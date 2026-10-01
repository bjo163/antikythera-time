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
pub fn tai_to_tt(tai:CoordinateTime)->Result<CoordinateTime,&'static str>{
 if tai.scale!=TimeScale::TAI{return Err("TAI required");}
 Ok(CoordinateTime::new(tai.jd1,tai.jd2+TT_MINUS_TAI_SECONDS/86400.0,TimeScale::TT,ReferenceFrame::GCRS)?)
}
pub fn tt_to_tai(tt:CoordinateTime)->Result<CoordinateTime,&'static str>{
 if tt.scale!=TimeScale::TT{return Err("TT required");}
 Ok(CoordinateTime::new(tt.jd1,tt.jd2-TT_MINUS_TAI_SECONDS/86400.0,TimeScale::TAI,ReferenceFrame::GCRS)?)
}
#[cfg(test)]
mod tests{use super::*;#[test]fn tt_tai_roundtrip(){let t=CoordinateTime::new(2451545.0,0.1,TimeScale::TAI,ReferenceFrame::GCRS).unwrap();let b=tt_to_tai(tai_to_tt(t).unwrap()).unwrap();assert!((b.jd()-t.jd()).abs()<1e-12);}#[test]fn current_table_ends_at_37(){assert_eq!(LEAP_SECONDS.last().unwrap().tai_minus_utc,37);}}
