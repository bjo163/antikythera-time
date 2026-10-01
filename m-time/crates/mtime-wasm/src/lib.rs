use wasm_bindgen::prelude::*;
use mtime_hijri::{mabims_indonesia_2026,evaluate_profile,HijriAstronomicalState,ObserverSite};

#[wasm_bindgen]
pub fn mabims_pass(altitude_deg:f64,elongation_deg:f64)->bool{
 let s=HijriAstronomicalState{conjunction_jd_tt:0.0,sunset_jd_utc:0.0,moon_topocentric_altitude_deg:altitude_deg,moon_sun_geocentric_elongation_deg:elongation_deg,moon_illumination_fraction:None,moon_age_hours:None,moon_lag_minutes:None,site:ObserverSite{id:"wasm".into(),latitude_deg:0.0,longitude_deg:0.0,height_m:0.0,timezone:"UTC".into(),datum:"WGS84".into()},provenance:vec![]};
 evaluate_profile(&mabims_indonesia_2026(),&s).pass
}
