use mtime_core::{CoordinateTime, QualityClass, Tt};
use mtime_hijri::{CalendarProfile, GeometrySemantics, HijriAstronomicalState};
use mtime_profile::bundled_diyanet_1978_global;
use mtime_timescales::tt_to_tcg;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mtime_version() -> String {
    "0.2.0".into()
}

fn state(altitude: f64, elongation: f64) -> HijriAstronomicalState {
    HijriAstronomicalState {
        conjunction_jd_tt: None,
        geometry_semantics: GeometrySemantics::mabims_required(),
        sunset_jd_ut1: None,
        moon_altitude_topocentric_deg: altitude,
        elongation_geocentric_deg: elongation,
        moon_age_hours: None,
        moon_lag_minutes: None,
        site_id: "WASM".into(),
        ephemeris_source: "caller-supplied".into(),
        quality: QualityClass::Reference,
    }
}

#[wasm_bindgen]
pub fn mabims_id_2026_pass(
    moon_altitude_topocentric_deg: f64,
    elongation_geocentric_deg: f64,
) -> bool {
    CalendarProfile::mabims_indonesia_2026()
        .evaluate(&state(
            moon_altitude_topocentric_deg,
            elongation_geocentric_deg,
        ))
        .met
        == Some(true)
}

#[wasm_bindgen]
pub fn diyanet_1978_site_pass(
    moon_altitude_topocentric_deg: f64,
    elongation_geocentric_deg: f64,
) -> bool {
    bundled_diyanet_1978_global()
        .calendar
        .evaluate(&state(
            moon_altitude_topocentric_deg,
            elongation_geocentric_deg,
        ))
        .met
        == Some(true)
}

#[wasm_bindgen]
pub fn tt_to_tcg_second_part(d1: f64, d2: f64) -> Result<f64, JsError> {
    let tt = CoordinateTime::<Tt>::new(d1, d2, 0.0).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(tt_to_tcg(&tt)
        .map_err(|e| JsError::new(&e.to_string()))?
        .jd_parts()
        .d2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_profiles_use_same_rust_core() {
        assert!(mabims_id_2026_pass(3.0, 6.4));
        assert!(!mabims_id_2026_pass(2.9, 6.4));
        assert!(!diyanet_1978_site_pass(4.0, 7.0));
        assert!(diyanet_1978_site_pass(5.0, 8.0));
    }
}
