use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use mtime_antikythera::{
    digital_lunar_longitude_with_selection, shortest_angle_deg, wrap_deg,
    AntikytheraMachine, CorrectionSelection, DIGITAL_LUNAR_CORRECTIONS_V1, J2000_JD_TT,
};
use mtime_astro::{icrf_vector_to_mean_ecliptic_of_date, Body};
use mtime_core::{CoordinateTime, Tt};
use mtime_spk::SpkEphemeris;
use mtime_timescales::{tt_to_tdb, DtrProvider, NasaSimpleDtr};

#[derive(Default)]
struct Stats {
    values: Vec<f64>,
}
impl Stats {
    fn push(&mut self, v: f64) { self.values.push(v.abs()); }
    fn p95(&self) -> f64 {
        let mut v=self.values.clone(); v.sort_by(f64::total_cmp);
        v[((v.len()-1) as f64*0.95).round() as usize]
    }
    fn max(&self) -> f64 { self.values.iter().copied().fold(0.0,f64::max) }
    fn mean(&self) -> f64 { self.values.iter().sum::<f64>()/self.values.len() as f64 }
}

#[derive(Debug, Clone, Copy)]
enum Basis {
    SinDraconic,
    CosDraconic,
    Sin2Draconic,
    Cos2Draconic,
    SinAnomalistic,
    CosAnomalistic,
    SinSynodic,
    CosSynodic,
}
impl Basis {
    const ALL: [Self;8]=[
        Self::SinDraconic,Self::CosDraconic,Self::Sin2Draconic,Self::Cos2Draconic,
        Self::SinAnomalistic,Self::CosAnomalistic,Self::SinSynodic,Self::CosSynodic
    ];
    fn id(self)->&'static str {
        match self {
            Self::SinDraconic=>"SIN_DRACONIC",
            Self::CosDraconic=>"COS_DRACONIC",
            Self::Sin2Draconic=>"SIN_2_DRACONIC",
            Self::Cos2Draconic=>"COS_2_DRACONIC",
            Self::SinAnomalistic=>"SIN_ANOMALISTIC",
            Self::CosAnomalistic=>"COS_ANOMALISTIC",
            Self::SinSynodic=>"SIN_SYNODIC",
            Self::CosSynodic=>"COS_SYNODIC",
        }
    }
    fn value(self, state:&mtime_antikythera::AntikytheraState)->f64 {
        let twopi=2.0*std::f64::consts::PI;
        match self {
            Self::SinDraconic=>(twopi*state.draconic_phase).sin(),
            Self::CosDraconic=>(twopi*state.draconic_phase).cos(),
            Self::Sin2Draconic=>(2.0*twopi*state.draconic_phase).sin(),
            Self::Cos2Draconic=>(2.0*twopi*state.draconic_phase).cos(),
            Self::SinAnomalistic=>(twopi*state.anomalistic_phase).sin(),
            Self::CosAnomalistic=>(twopi*state.anomalistic_phase).cos(),
            Self::SinSynodic=>(twopi*state.synodic_phase).sin(),
            Self::CosSynodic=>(twopi*state.synodic_phase).cos(),
        }
    }
}

fn main(){
    let mut args=env::args().skip(1);
    let spk_path=args.next().expect("usage: ablate_antikythera_corrections <de440s.bsp> <out-dir>");
    let out_dir=args.next().expect("out-dir");
    fs::create_dir_all(&out_dir).unwrap();
    let bytes=fs::read(spk_path).unwrap();
    let eph=SpkEphemeris::from_bytes(&bytes).unwrap();
    let machine=AntikytheraMachine::digital();

    let mut ablation=BufWriter::new(File::create(Path::new(&out_dir).join("ablation.csv")).unwrap());
    writeln!(ablation,"term_id,validation_p95_deg,validation_max_deg,baseline_p95_deg,baseline_max_deg,p95_delta_deg,max_delta_deg").unwrap();

    let baseline=validation_stats(&eph,&machine,&[]);
    for term in DIGITAL_LUNAR_CORRECTIONS_V1 {
        let stats=validation_stats(&eph,&machine,&[term.id]);
        writeln!(ablation,"{},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9}",
            term.id,stats.p95(),stats.max(),baseline.p95(),baseline.max(),
            stats.p95()-baseline.p95(),stats.max()-baseline.max()).unwrap();
    }

    let mut candidates=BufWriter::new(File::create(Path::new(&out_dir).join("candidate-fit.csv")).unwrap());
    writeln!(candidates,"basis,coefficient_deg,train_p95_before,train_p95_after,validation_p95_before,validation_p95_after,validation_max_before,validation_max_after,accepted").unwrap();

    let mut accepted_count=0usize;
    for basis in Basis::ALL {
        let coefficient=fit_coefficient(&eph,&machine,basis,1900,1999);
        let train_before=range_stats(&eph,&machine,1900,1999,None);
        let train_after=range_stats(&eph,&machine,1900,1999,Some((basis,coefficient)));
        let val_before=range_stats(&eph,&machine,2000,2100,None);
        let val_after=range_stats(&eph,&machine,2000,2100,Some((basis,coefficient)));
        let accepted=train_after.p95()<train_before.p95()
            && val_after.p95()<val_before.p95()
            && val_after.max()<=val_before.max()*1.05;
        if accepted { accepted_count+=1; }
        writeln!(candidates,"{},{:.12},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{}",
            basis.id(),coefficient,train_before.p95(),train_after.p95(),
            val_before.p95(),val_after.p95(),val_before.max(),val_after.max(),accepted).unwrap();
        println!("candidate={} coeff_deg={:.12} accepted={} train_p95_before={:.9} train_p95_after={:.9} validation_p95_before={:.9} validation_p95_after={:.9} validation_max_before={:.9} validation_max_after={:.9}",
            basis.id(),coefficient,accepted,train_before.p95(),train_after.p95(),
            val_before.p95(),val_after.p95(),val_before.max(),val_after.max());
    }

    println!("===== M8 CORRECTION ABLATION =====");
    println!("default_terms={}",DIGITAL_LUNAR_CORRECTIONS_V1.len());
    println!("validation_baseline_p95_deg={:.9}",baseline.p95());
    println!("validation_baseline_max_deg={:.9}",baseline.max());
    println!("accepted_experimental_candidates={accepted_count}");
    println!("default_profile_changed=false");
}

fn validation_stats(eph:&SpkEphemeris<'_>, machine:&AntikytheraMachine, disabled:&[&str])->Stats {
    let mut s=Stats::default();
    for year in 2000..=2100 {
        for month in 1..=12 {
            let jd=gregorian_to_jd(year,month,1,12.0);
            let reference=reference_moon(eph,jd);
            let elapsed=jd-J2000_JD_TT;
            let model=digital_lunar_longitude_with_selection(elapsed,CorrectionSelection{disabled_ids:disabled});
            s.push(signed_angle(reference-model));
        }
    }
    s
}

fn fit_coefficient(eph:&SpkEphemeris<'_>, machine:&AntikytheraMachine, basis:Basis, start:i32, end:i32)->f64 {
    let mut xy=0.0; let mut xx=0.0;
    for year in start..=end {
        for month in 1..=12 {
            let jd=gregorian_to_jd(year,month,1,12.0);
            let state=machine.state_at_tt(jd).unwrap();
            let x=basis.value(&state);
            let y=signed_angle(reference_moon(eph,jd)-state.lunar_longitude.angle_deg);
            xy+=x*y; xx+=x*x;
        }
    }
    xy/xx
}

fn range_stats(eph:&SpkEphemeris<'_>, machine:&AntikytheraMachine, start:i32, end:i32, candidate:Option<(Basis,f64)>)->Stats {
    let mut s=Stats::default();
    for year in start..=end {
        for month in 1..=12 {
            let jd=gregorian_to_jd(year,month,1,12.0);
            let state=machine.state_at_tt(jd).unwrap();
            let mut model=state.lunar_longitude.angle_deg;
            if let Some((basis,coefficient))=candidate {
                model=wrap_deg(model+coefficient*basis.value(&state));
            }
            s.push(signed_angle(reference_moon(eph,jd)-model));
        }
    }
    s
}

fn signed_angle(v:f64)->f64 { (v+180.0).rem_euclid(360.0)-180.0 }

fn reference_moon(eph:&SpkEphemeris<'_>,jd_tt:f64)->f64 {
    let tt=CoordinateTime::<Tt>::new(jd_tt,0.0,0.0).unwrap();
    let dtr=NasaSimpleDtr.dtr(&tt).unwrap();
    let tdb=tt_to_tdb(&tt,dtr).unwrap();
    let p=tdb.jd_parts();
    let t=tt.jd_parts();
    icrf_vector_to_mean_ecliptic_of_date(
        eph.geocentric_vector_km(Body::Moon,(p.d1,p.d2)).unwrap(),
        (t.d1,t.d2)
    ).unwrap().longitude_deg
}

fn gregorian_to_jd(year:i32,month:u32,day:u32,hour:f64)->f64 {
    let (y,m)=if month<=2 {(year-1,month as i32+12)} else {(year,month as i32)};
    let a=(y as f64/100.0).floor() as i32;
    let b=2-a+a/4;
    (365.25*(y+4716) as f64).floor()+(30.6001*(m+1) as f64).floor()
        +day as f64+b as f64-1524.5+hour/24.0
}
