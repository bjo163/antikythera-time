use mtime_core::{CoordinateTime,ReferenceFrame,TimeScale};

pub const TT_MINUS_TAI_SECONDS:f64=32.184;
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct LeapSecondEntry {pub effective_utc:&'static str,pub tai_minus_utc:i32}
pub const LEAP_SECONDS:&[LeapSecondEntry]=&[
 LeapSecondEntry{effective_utc:"1972-01-01",tai_minus_utc:10},
 LeapSecondEntry{effective_utc:"1972-07-01",tai_minus_utc:11},
 LeapSecondEntry{effective_utc:"1973-01-01",tai_minus_utc:12},
 LeapSecondEntry{effective_utc:"1974-01-01",tai_minus_utc:13},
 LeapSecondEntry{effective_utc:"1975-01-01",tai_minus_utc:14},
 LeapSecondEntry{effective_utc:"1976-01-01",tai_minus_utc:15},
 LeapSecondEntry{effective_utc:"1977-01-01",tai_minus_utc:16},
 LeapSecondEntry{effective_utc:"1978-01-01",tai_minus_utc:17},
 LeapSecondEntry{effective_utc:"1979-01-01",tai_minus_utc:18},
 LeapSecondEntry{effective_utc:"1980-01-01",tai_minus_utc:19},
 LeapSecondEntry{effective_utc:"1981-07-01",tai_minus_utc:20},
 LeapSecondEntry{effective_utc:"1982-07-01",tai_minus_utc:21},
 LeapSecondEntry{effective_utc:"1983-07-01",tai_minus_utc:22},
 LeapSecondEntry{effective_utc:"1985-07-01",tai_minus_utc:23},
 LeapSecondEntry{effective_utc:"1988-01-01",tai_minus_utc:24},
 LeapSecondEntry{effective_utc:"1990-01-01",tai_minus_utc:25},
 LeapSecondEntry{effective_utc:"1991-01-01",tai_minus_utc:26},
 LeapSecondEntry{effective_utc:"1992-07-01",tai_minus_utc:27},
 LeapSecondEntry{effective_utc:"1993-07-01",tai_minus_utc:28},
 LeapSecondEntry{effective_utc:"1994-07-01",tai_minus_utc:29},
 LeapSecondEntry{effective_utc:"1996-01-01",tai_minus_utc:30},
 LeapSecondEntry{effective_utc:"1997-07-01",tai_minus_utc:31},
 LeapSecondEntry{effective_utc:"1999-01-01",tai_minus_utc:32},
 LeapSecondEntry{effective_utc:"2006-01-01",tai_minus_utc:33},
 LeapSecondEntry{effective_utc:"2009-01-01",tai_minus_utc:34},
 LeapSecondEntry{effective_utc:"2012-07-01",tai_minus_utc:35},
 LeapSecondEntry{effective_utc:"2015-07-01",tai_minus_utc:36},
 LeapSecondEntry{effective_utc:"2017-01-01",tai_minus_utc:37},
];
pub fn gregorian_to_jd(year:i32,month:u8,day:u8)->f64{
 let(mut y,mut m)=(year,month as i32);if m<=2{y-=1;m+=12}let a=(y as f64/100.0).floor();let b=2.0-a+(a/4.0).floor();
 (365.25*(y as f64+4716.0)).floor()+(30.6001*(m as f64+1.0)).floor()+day as f64+b-1524.5
}
fn parse_ymd(s:&str)->Option<(i32,u8,u8)>{
 let mut it=s.split('-');Some((it.next()?.parse().ok()?,it.next()?.parse().ok()?,it.next()?.parse().ok()?))
}
pub fn tai_minus_utc_at_jd(jd_utc:f64)->Option<i32>{
 if jd_utc<gregorian_to_jd(1972,1,1){return None}
 let mut out=None;
 for e in LEAP_SECONDS{let(y,m,d)=parse_ymd(e.effective_utc)?;if jd_utc>=gregorian_to_jd(y,m,d){out=Some(e.tai_minus_utc)}else{break}}
 out
}
pub fn utc_to_tai(utc:CoordinateTime)->Result<CoordinateTime,&'static str>{
 if utc.scale!=TimeScale::UTC{return Err("UTC required")}
 let off=tai_minus_utc_at_jd(utc.jd()).ok_or("UTC leap-second table supports 1972+ only")? as f64;
 ct(utc.jd1,utc.jd2+off/DAY,TimeScale::TAI,ReferenceFrame::GCRS)
}
pub fn tai_to_utc(tai:CoordinateTime)->Result<CoordinateTime,&'static str>{
 if tai.scale!=TimeScale::TAI{return Err("TAI required")}
 // Iterate because TAI input does not directly reveal the UTC-era offset.
 let mut jd=tai.jd();let mut off=37.0;
 for _ in 0..3{jd=tai.jd()-off/DAY;off=tai_minus_utc_at_jd(jd).ok_or("TAI instant maps before supported UTC era")? as f64;}
 ct(tai.jd1,tai.jd2-off/DAY,TimeScale::UTC,ReferenceFrame::GCRS)
}
pub fn tai_to_tt(tai:CoordinateTime)->Result<CoordinateTime,&'static str>{
 if tai.scale!=TimeScale::TAI{return Err("TAI required");}
 Ok(CoordinateTime::new(tai.jd1,tai.jd2+TT_MINUS_TAI_SECONDS/86400.0,TimeScale::TT,ReferenceFrame::GCRS)?)
}
pub fn tt_to_tai(tt:CoordinateTime)->Result<CoordinateTime,&'static str>{
 if tt.scale!=TimeScale::TT{return Err("TT required");}
 Ok(CoordinateTime::new(tt.jd1,tt.jd2-TT_MINUS_TAI_SECONDS/86400.0,TimeScale::TAI,ReferenceFrame::GCRS)?)
}
#[cfg(test)]
mod tests{use super::*;
 #[test]fn tt_tai_roundtrip(){let t=CoordinateTime::new(2451545.0,0.1,TimeScale::TAI,ReferenceFrame::GCRS).unwrap();let b=tt_to_tai(tai_to_tt(t).unwrap()).unwrap();assert!((b.jd()-t.jd()).abs()<1e-12);}
 #[test]fn current_table_ends_at_37(){assert_eq!(LEAP_SECONDS.last().unwrap().tai_minus_utc,37);assert_eq!(tai_minus_utc_at_jd(gregorian_to_jd(2026,1,1)),Some(37));}
 #[test]fn utc_tai_roundtrip_2026(){let u=CoordinateTime::new(gregorian_to_jd(2026,3,19),0.25,TimeScale::UTC,ReferenceFrame::GCRS).unwrap();let back=tai_to_utc(utc_to_tai(u).unwrap()).unwrap();assert!((back.jd()-u.jd()).abs()<1e-12);}
 #[test]fn pre_1972_utc_requires_separate_policy(){let u=CoordinateTime::new(gregorian_to_jd(1960,1,1),0.0,TimeScale::UTC,ReferenceFrame::GCRS).unwrap();assert!(utc_to_tai(u).is_err());}
 #[test]fn all_sofa_reference_vectors_pass(){for v in sofa_reference_vectors(){assert!(v.pass(),"{}: {} vs {}",v.id,v.actual,v.expected)}}
 #[test]fn cross_scale_input_is_rejected(){let tt=CoordinateTime::new(2451545.0,0.0,TimeScale::TT,ReferenceFrame::GCRS).unwrap();assert!(tcb_to_tdb(tt).is_err());}
}


pub const LG:f64=6.969290134e-10;
pub const LB:f64=1.550519768e-8;
pub const TDB0_SECONDS:f64=-6.55e-5;
const DAY:f64=86400.0;
const DJM0:f64=2400000.5;
const DJM77:f64=43144.0;

fn ct(d1:f64,d2:f64,scale:TimeScale,frame:ReferenceFrame)->Result<CoordinateTime,&'static str>{
 CoordinateTime::new(d1,d2,scale,frame)
}

pub fn tt_to_tcg(tt:CoordinateTime)->Result<CoordinateTime,&'static str>{
 if tt.scale!=TimeScale::TT{return Err("TT required")}
 let t77t=DJM77+TT_MINUS_TAI_SECONDS/DAY;let rate=LG/(1.0-LG);
 let(d1,d2)=if tt.jd1.abs()>tt.jd2.abs(){(tt.jd1,tt.jd2+((tt.jd1-DJM0)+(tt.jd2-t77t))*rate)}else{(tt.jd1+((tt.jd2-DJM0)+(tt.jd1-t77t))*rate,tt.jd2)};
 ct(d1,d2,TimeScale::TCG,ReferenceFrame::GCRS)
}
pub fn tcg_to_tt(tcg:CoordinateTime)->Result<CoordinateTime,&'static str>{
 if tcg.scale!=TimeScale::TCG{return Err("TCG required")}
 let t77t=DJM77+TT_MINUS_TAI_SECONDS/DAY;
 let(d1,d2)=if tcg.jd1.abs()>tcg.jd2.abs(){(tcg.jd1,tcg.jd2-((tcg.jd1-DJM0)+(tcg.jd2-t77t))*LG)}else{(tcg.jd1-((tcg.jd2-DJM0)+(tcg.jd1-t77t))*LG,tcg.jd2)};
 ct(d1,d2,TimeScale::TT,ReferenceFrame::GCRS)
}
pub fn tcb_to_tdb(tcb:CoordinateTime)->Result<CoordinateTime,&'static str>{
 if tcb.scale!=TimeScale::TCB{return Err("TCB required")}
 let t77td=DJM0+DJM77;let t77tf=TT_MINUS_TAI_SECONDS/DAY;let tdb0=TDB0_SECONDS/DAY;
 let(d1,d2)=if tcb.jd1.abs()>tcb.jd2.abs(){let d=tcb.jd1-t77td;(tcb.jd1,tcb.jd2+tdb0-(d+(tcb.jd2-t77tf))*LB)}else{let d=tcb.jd2-t77td;(tcb.jd1+tdb0-(d+(tcb.jd1-t77tf))*LB,tcb.jd2)};
 ct(d1,d2,TimeScale::TDB,ReferenceFrame::BCRS)
}
pub fn tdb_to_tcb(tdb:CoordinateTime)->Result<CoordinateTime,&'static str>{
 if tdb.scale!=TimeScale::TDB{return Err("TDB required")}
 let t77td=DJM0+DJM77;let t77tf=TT_MINUS_TAI_SECONDS/DAY;let tdb0=TDB0_SECONDS/DAY;let rate=LB/(1.0-LB);
 let(d1,d2)=if tdb.jd1.abs()>tdb.jd2.abs(){let d=t77td-tdb.jd1;let f=tdb.jd2-tdb0;(tdb.jd1,f-(d-(f-t77tf))*rate)}else{let d=t77td-tdb.jd2;let f=tdb.jd1-tdb0;(f-(d-(f-t77tf))*rate,tdb.jd2)};
 ct(d1,d2,TimeScale::TCB,ReferenceFrame::BCRS)
}
pub fn tt_to_tdb(tt:CoordinateTime,dtr_seconds:f64)->Result<CoordinateTime,&'static str>{
 if tt.scale!=TimeScale::TT{return Err("TT required")}if !dtr_seconds.is_finite(){return Err("finite TDB-TT required")}
 let dd=dtr_seconds/DAY;let(d1,d2)=if tt.jd1.abs()>tt.jd2.abs(){(tt.jd1,tt.jd2+dd)}else{(tt.jd1+dd,tt.jd2)};
 ct(d1,d2,TimeScale::TDB,ReferenceFrame::BCRS)
}
pub fn tdb_to_tt(tdb:CoordinateTime,dtr_seconds:f64)->Result<CoordinateTime,&'static str>{
 if tdb.scale!=TimeScale::TDB{return Err("TDB required")}if !dtr_seconds.is_finite(){return Err("finite TDB-TT required")}
 let dd=dtr_seconds/DAY;let(d1,d2)=if tdb.jd1.abs()>tdb.jd2.abs(){(tdb.jd1,tdb.jd2-dd)}else{(tdb.jd1-dd,tdb.jd2)};
 ct(d1,d2,TimeScale::TT,ReferenceFrame::GCRS)
}

#[derive(Debug,Clone,PartialEq)]
pub struct SofaVectorResult{pub id:&'static str,pub actual:f64,pub expected:f64,pub tolerance_days:f64}
impl SofaVectorResult{pub fn pass(&self)->bool{(self.actual-self.expected).abs()<=self.tolerance_days}}
pub fn sofa_reference_vectors()->Vec<SofaVectorResult>{
 vec![
  SofaVectorResult{id:"TT_TO_TCG",actual:tt_to_tcg(ct(2453750.5,0.892482639,TimeScale::TT,ReferenceFrame::GCRS).unwrap()).unwrap().jd2,expected:0.8924900312508587,tolerance_days:1e-12},
  SofaVectorResult{id:"TCG_TO_TT",actual:tcg_to_tt(ct(2453750.5,0.892862531,TimeScale::TCG,ReferenceFrame::GCRS).unwrap()).unwrap().jd2,expected:0.8928551387488817,tolerance_days:1e-12},
  SofaVectorResult{id:"TCB_TO_TDB",actual:tcb_to_tdb(ct(2453750.5,0.893019599,TimeScale::TCB,ReferenceFrame::BCRS).unwrap()).unwrap().jd2,expected:0.8928551362746343,tolerance_days:1e-12},
  SofaVectorResult{id:"TDB_TO_TCB",actual:tdb_to_tcb(ct(2453750.5,0.892855137,TimeScale::TDB,ReferenceFrame::BCRS).unwrap()).unwrap().jd2,expected:0.8930195997253657,tolerance_days:1e-12},
  SofaVectorResult{id:"TT_TO_TDB",actual:tt_to_tdb(ct(2453750.5,0.892855139,TimeScale::TT,ReferenceFrame::GCRS).unwrap(),-0.000201).unwrap().jd2,expected:0.8928551366736111,tolerance_days:1e-12},
  SofaVectorResult{id:"TDB_TO_TT",actual:tdb_to_tt(ct(2453750.5,0.892855137,TimeScale::TDB,ReferenceFrame::BCRS).unwrap(),-0.000201).unwrap().jd2,expected:0.8928551393263889,tolerance_days:1e-12},
 ]
}
