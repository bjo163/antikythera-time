use serde::{Deserialize, Serialize};
use mtime_core::EarthObserver;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HijriAstronomicalState {
    pub event_id: String,
    pub observer: EarthObserver,
    pub conjunction_jd_tt: f64,
    pub sunset_jd_utc: f64,
    pub moon_altitude_deg: f64,
    pub elongation_deg: f64,
    pub illumination_fraction: f64,
    pub lunar_age_hours: f64,
    pub lag_minutes: f64,
    pub ephemeris_provenance: String,
    pub refraction_model: String,
}
impl HijriAstronomicalState {
    pub fn synthetic(alt: f64, elong: f64) -> Self {
        Self { event_id: "synthetic".into(), observer: EarthObserver::wgs84("synthetic", 0.0, 0.0, 0.0).unwrap(), conjunction_jd_tt: 2451545.0, sunset_jd_utc: 2451545.5, moon_altitude_deg: alt, elongation_deg: elong, illumination_fraction: 0.01, lunar_age_hours: 12.0, lag_minutes: 30.0, ephemeris_provenance: "synthetic fixture".into(), refraction_model: "none".into() }
    }
}
