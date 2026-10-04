use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use mtime_antikythera::{shortest_angle_deg, AntikytheraMachine};
use mtime_astro::{icrf_vector_to_mean_ecliptic_of_date, Body};
use mtime_core::{CoordinateTime, Tt};
use mtime_spk::SpkEphemeris;
use mtime_timescales::{tt_to_tdb, DtrProvider, NasaSimpleDtr};

#[derive(Default)]
struct Stats { values: Vec<f64> }
impl Stats {
    fn push(&mut self,v:f64){self.values.push(v);}
    fn p95(&self)->f64{
        let mut v=self.values.clone(); v.sort_by(f64::total_cmp);
        v[((v.len()-1) as f64*0.95).round() as usize]
    }
    fn max(&self)->f64{self.values.iter().copied().fold(0.0,f64::max)}
}

fn main(){
    let mut args=env::args().skip(1);
    let spk_path=args.next().expect("usage: validate_antikythera_longspan <de440s.bsp> <out-dir>");
    let out_dir=args.next().expect("out-dir");
    fs::create_dir_all(&out_dir).unwrap();
    let bytes=fs::read(spk_path).unwrap();
    let eph=SpkEphemeris::from_bytes(&bytes).unwrap();
    let v1=AntikytheraMachine::digital();
    let v2=AntikytheraMachine::digital_v2_experimental();

    let mut csv=BufWriter::new(File::create(Path::new(&out_dir).join("longspan-partitions.csv")).unwrap());
    writeln!(csv,"partition,start_year,end_year,count,v1_p95_deg,v1_max_deg,v2_p95_deg,v2_max_deg,p95_improvement_deg,max_improvement_deg").unwrap();

    let partitions=[("PRE1900",1850,1899),("TRAIN",1900,1999),("VALIDATION",2000,2099),("FUTURE",2100,2149),("FULL",1850,2149)];
    for (name,start,end) in partitions {
        let (s1,s2)=range_stats(&eph,&v1,&v2,start,end);
        writeln!(csv,"{name},{start},{end},{},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9}",
            s1.values.len(),s1.p95(),s1.max(),s2.p95(),s2.max(),
            s1.p95()-s2.p95(),s1.max()-s2.max()).unwrap();
        println!("partition={name} years={start}-{end} count={} v1_p95_deg={:.9} v1_max_deg={:.9} v2_p95_deg={:.9} v2_max_deg={:.9} p95_improvement_deg={:.9} max_improvement_deg={:.9}",
            s1.values.len(),s1.p95(),s1.max(),s2.p95(),s2.max(),s1.p95()-s2.p95(),s1.max()-s2.max());
    }

    let (full1,full2)=range_stats(&eph,&v1,&v2,1850,2149);
    println!("===== M16 LONG-SPAN FALSIFICATION =====");
    println!("validated_interval=1850-01-01..2149-12-31");
    println!("monthly_epochs={}",full1.values.len());
    println!("v1_full_p95_deg={:.9}",full1.p95());
    println!("v1_full_max_deg={:.9}",full1.max());
    println!("v2_full_p95_deg={:.9}",full2.p95());
    println!("v2_full_max_deg={:.9}",full2.max());
    println!("v2_p95_improves={}",full2.p95()<full1.p95());
    println!("v2_max_improves={}",full2.max()<full1.max());

    assert_eq!(full1.values.len(),3600);
}

fn range_stats(
    eph:&SpkEphemeris<'_>,
    v1:&AntikytheraMachine,
    v2:&AntikytheraMachine,
    start:i32,
    end:i32,
)->(Stats,Stats){
    let mut a=Stats::default(); let mut b=Stats::default();
    for year in start..=end {
        for month in 1..=12 {
            let jd=gregorian_to_jd(year,month,1,12.0);
            let reference=reference_moon(eph,jd);
            let s1=v1.state_at_tt(jd).unwrap();
            let s2=v2.state_at_tt(jd).unwrap();
            a.push(shortest_angle_deg(s1.lunar_longitude.angle_deg,reference));
            b.push(shortest_angle_deg(s2.lunar_longitude.angle_deg,reference));
        }
    }
    (a,b)
}

fn reference_moon(eph:&SpkEphemeris<'_>,jd_tt:f64)->f64{
    let tt=CoordinateTime::<Tt>::new(jd_tt,0.0,0.0).unwrap();
    let dtr=NasaSimpleDtr.dtr(&tt).unwrap();
    let tdb=tt_to_tdb(&tt,dtr).unwrap();
    let p=tdb.jd_parts(); let t=tt.jd_parts();
    icrf_vector_to_mean_ecliptic_of_date(
        eph.geocentric_vector_km(Body::Moon,(p.d1,p.d2)).unwrap(),
        (t.d1,t.d2)
    ).unwrap().longitude_deg
}

fn gregorian_to_jd(year:i32,month:u32,day:u32,hour:f64)->f64{
    let (y,m)=if month<=2{(year-1,month as i32+12)}else{(year,month as i32)};
    let a=(y as f64/100.0).floor() as i32;
    let b=2-a+a/4;
    (365.25*(y+4716) as f64).floor()+(30.6001*(m+1) as f64).floor()+day as f64+b as f64-1524.5+hour/24.0
}
