use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeScale { UTC, TAI, TT, UT1, TCG, TDB, TCB }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferenceFrame { ITRS, GCRS, BCRS, Unspecified }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceState { Observed, Measured, Calculated, Modeled, Inferred, Reconstructed, Speculative, TextualReference }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityClass { Reference, HighPrecision, Approximate, Reconstruction, Conceptual }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Provenance { pub source: String, pub version: Option<String>, pub retrieved_at: Option<String> }

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Uncertainty { pub seconds: f64 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurationSi { nanos: i128 }
impl DurationSi {
    pub const NANOS_PER_SECOND: i128 = 1_000_000_000;
    pub fn from_seconds(seconds: i64) -> Self { Self { nanos: seconds as i128 * Self::NANOS_PER_SECOND } }
    pub fn from_nanos(nanos: i128) -> Self { Self { nanos } }
    pub fn nanos(self) -> i128 { self.nanos }
    pub fn checked_add(self, other: Self) -> Option<Self> { self.nanos.checked_add(other.nanos).map(Self::from_nanos) }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CoordinateTime { pub d1: f64, pub d2: f64, pub scale: TimeScale, pub frame: ReferenceFrame }
impl CoordinateTime {
    pub fn new(d1: f64, d2: f64, scale: TimeScale, frame: ReferenceFrame) -> Result<Self, MTimeError> {
        if !d1.is_finite() || !d2.is_finite() { return Err(MTimeError::NonFinite); }
        Ok(Self { d1, d2, scale, frame })
    }
    pub fn jd(self) -> f64 { self.d1 + self.d2 }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EarthObserver { pub id: String, pub longitude_deg: f64, pub latitude_deg: f64, pub height_m: f64, pub datum: String }
impl EarthObserver {
    pub fn wgs84(id: impl Into<String>, lon: f64, lat: f64, height_m: f64) -> Result<Self, MTimeError> {
        if !lon.is_finite() || !lat.is_finite() || !height_m.is_finite() { return Err(MTimeError::NonFinite); }
        if !(-90.0..=90.0).contains(&lat) { return Err(MTimeError::InvalidLatitude(lat)); }
        Ok(Self { id: id.into(), longitude_deg: lon, latitude_deg: lat, height_m, datum: "WGS84".into() })
    }
}

#[derive(Debug, Error)]
pub enum MTimeError {
    #[error("non-finite numeric value")] NonFinite,
    #[error("invalid latitude {0}")] InvalidLatitude(f64),
    #[error("semantic mismatch: {0}")] SemanticMismatch(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn duration_is_integer_and_checked() { let a = DurationSi::from_seconds(86_400); assert_eq!(a.nanos(), 86_400_000_000_000); assert_eq!(a.checked_add(a).unwrap().nanos(), 172_800_000_000_000); }
    #[test] fn coordinate_keeps_scale_frame() { let t = CoordinateTime::new(2451545.0, 0.0, TimeScale::TT, ReferenceFrame::GCRS).unwrap(); assert_eq!(t.jd(), 2451545.0); assert_eq!(t.scale, TimeScale::TT); }
    #[test] fn observer_validates_latitude() { assert!(EarthObserver::wgs84("x", 106.8, -6.3, 10.0).is_ok()); assert!(EarthObserver::wgs84("x", 0.0, 91.0, 0.0).is_err()); }
}
