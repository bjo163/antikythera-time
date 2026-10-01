#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceState {
    Observed, Measured, Calculated, Modeled, Inferred, Reconstructed, Speculative, TextualReference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityClass { Reference, HighPrecision, Approximate, Reconstruction, Conceptual }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeScale { UTC, TAI, TT, UT1, TCG, TDB, TCB }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceFrame { ITRS, GCRS, BCRS, EclipticJ2000, Unspecified }

#[derive(Debug, Clone, PartialEq)]
pub struct Provenance {
    pub source: String,
    pub version: Option<String>,
    pub retrieved_at: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Uncertainty {
    pub absolute_seconds: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordinateTime {
    pub jd1: f64,
    pub jd2: f64,
    pub scale: TimeScale,
    pub frame: ReferenceFrame,
}

impl CoordinateTime {
    pub fn new(jd1:f64,jd2:f64,scale:TimeScale,frame:ReferenceFrame)->Result<Self,&'static str>{
        if !jd1.is_finite() || !jd2.is_finite() { return Err("finite two-part JD required"); }
        Ok(Self{jd1,jd2,scale,frame})
    }
    pub fn jd(&self)->f64 { self.jd1 + self.jd2 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurationSi { nanos: i128 }

impl DurationSi {
    pub const fn from_nanos(nanos:i128)->Self{Self{nanos}}
    pub fn from_seconds(seconds:i64)->Option<Self>{
        (seconds as i128).checked_mul(1_000_000_000).map(Self::from_nanos)
    }
    pub const fn nanos(&self)->i128{self.nanos}
    pub fn checked_add(self, other:Self)->Option<Self>{self.nanos.checked_add(other.nanos).map(Self::from_nanos)}
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test] fn duration_is_integer_and_overflow_safe(){
   let a=DurationSi::from_seconds(86400).unwrap();
   assert_eq!(a.nanos(),86_400_000_000_000);
   assert!(DurationSi::from_nanos(i128::MAX).checked_add(DurationSi::from_nanos(1)).is_none());
 }
 #[test] fn two_part_jd_preserves_semantics(){
   let t=CoordinateTime::new(2451545.0,0.25,TimeScale::TT,ReferenceFrame::GCRS).unwrap();
   assert_eq!(t.scale,TimeScale::TT); assert_eq!(t.jd(),2451545.25);
 }
}
