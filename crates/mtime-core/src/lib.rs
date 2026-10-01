use core::fmt;
use core::marker::PhantomData;

pub const NS_PER_SECOND: i128 = 1_000_000_000;
pub const SECONDS_PER_DAY: f64 = 86_400.0;
pub const J2000_JD_TT: f64 = 2_451_545.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DurationSi { nanoseconds: i128 }
impl DurationSi {
    #[must_use] pub const fn from_nanoseconds(nanoseconds:i128)->Self{Self{nanoseconds}}
    #[must_use] pub const fn from_seconds(seconds:i128)->Self{Self{nanoseconds:seconds.saturating_mul(NS_PER_SECOND)}}
    #[must_use] pub const fn nanoseconds(self)->i128{self.nanoseconds}
    #[must_use] pub fn seconds_f64(self)->f64{self.nanoseconds as f64/NS_PER_SECOND as f64}
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct TwoPartJulianDate{pub d1:f64,pub d2:f64}
impl TwoPartJulianDate{
    pub fn new(d1:f64,d2:f64)->Result<Self,TemporalError>{if !d1.is_finite()||!d2.is_finite(){return Err(TemporalError::NonFinite);}Ok(Self{d1,d2})}
    #[must_use] pub fn jd(self)->f64{self.d1+self.d2}
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ReferenceFrame{Gcrs,Bcrs,Itrs,Unspecified}
pub trait TimeScale:Copy+fmt::Debug+'static{const NAME:&'static str;const DEFAULT_FRAME:ReferenceFrame;}
macro_rules! scale{($n:ident,$l:literal,$f:expr)=>{#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub struct $n;impl TimeScale for $n{const NAME:&'static str=$l;const DEFAULT_FRAME:ReferenceFrame=$f;}}}
scale!(Tt,"TT",ReferenceFrame::Gcrs);scale!(Tcg,"TCG",ReferenceFrame::Gcrs);scale!(Tdb,"TDB",ReferenceFrame::Bcrs);scale!(Tcb,"TCB",ReferenceFrame::Bcrs);

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct CoordinateTime<S:TimeScale>{jd:TwoPartJulianDate,uncertainty_seconds:f64,_scale:PhantomData<S>}
impl<S:TimeScale> CoordinateTime<S>{
    pub fn new(d1:f64,d2:f64,uncertainty_seconds:f64)->Result<Self,TemporalError>{if !uncertainty_seconds.is_finite()||uncertainty_seconds<0.0{return Err(TemporalError::InvalidUncertainty);}Ok(Self{jd:TwoPartJulianDate::new(d1,d2)?,uncertainty_seconds,_scale:PhantomData})}
    #[must_use] pub const fn jd_parts(&self)->TwoPartJulianDate{self.jd}
    #[must_use] pub fn jd(&self)->f64{self.jd.jd()}
    #[must_use] pub const fn uncertainty_seconds(&self)->f64{self.uncertainty_seconds}
    #[must_use] pub const fn frame(&self)->ReferenceFrame{S::DEFAULT_FRAME}
    #[must_use] pub const fn scale_name(&self)->&'static str{S::NAME}
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Uncertainty{Exact,AbsoluteSeconds(f64),IntervalSeconds{minus:f64,plus:f64},Unknown}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum EvidenceState{Observed,Measured,Calculated,Modeled,Inferred,Reconstructed,Speculative,TextualReference}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum QualityClass{Reference,HighPrecision,Approximate,Reconstruction,Conceptual}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Provenance{pub source:String,pub source_version:Option<String>,pub retrieved_at:Option<String>}
impl Provenance{#[must_use]pub fn new(source:impl Into<String>)->Self{Self{source:source.into(),source_version:None,retrieved_at:None}}}

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct EarthObserver{pub longitude_deg:f64,pub latitude_deg:f64,pub height_m:f64}
impl EarthObserver{
    pub fn new(longitude_deg:f64,latitude_deg:f64,height_m:f64)->Result<Self,TemporalError>{if !longitude_deg.is_finite()||!latitude_deg.is_finite()||!height_m.is_finite(){return Err(TemporalError::NonFinite);}if !(-90.0..=90.0).contains(&latitude_deg)||!(-180.0..=180.0).contains(&longitude_deg){return Err(TemporalError::InvalidObserver);}Ok(Self{longitude_deg,latitude_deg,height_m})}
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum TemporalError{NonFinite,InvalidUncertainty,InvalidObserver,OutsideSupportedRange,MissingReferenceData,InvalidInput(&'static str)}
impl fmt::Display for TemporalError{fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{write!(f,"{self:?}")}}
impl std::error::Error for TemporalError{}

#[cfg(test)]mod tests{use super::*;#[test]fn duration_uses_i128_nanoseconds(){let d=DurationSi::from_seconds(86_400);assert_eq!(d.nanoseconds(),86_400_i128*NS_PER_SECOND);}#[test]fn two_part_jd_preserves_scale_type(){let t=CoordinateTime::<Tt>::new(J2000_JD_TT,0.0,0.0).unwrap();assert_eq!(t.scale_name(),"TT");assert_eq!(t.frame(),ReferenceFrame::Gcrs);}#[test]fn observer_rejects_invalid_latitude(){assert!(EarthObserver::new(0.0,91.0,0.0).is_err());}}
