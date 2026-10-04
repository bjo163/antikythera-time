use mtime_core::{CoordinateTime, QualityClass, Tt};
use mtime_hijri::{CalendarProfile, GeometrySemantics, HijriAstronomicalState};
use mtime_profile::bundled_diyanet_1978_global;
use mtime_timescales::tt_to_tcg;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mtime_version() -> String {
    env!("CARGO_PKG_VERSION").into()
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
        assert_eq!(mtime_version(), env!("CARGO_PKG_VERSION"));
        assert!(mabims_id_2026_pass(3.0, 6.4));
        assert!(!mabims_id_2026_pass(2.9, 6.4));
        assert!(!diyanet_1978_site_pass(4.0, 7.0));
        assert!(diyanet_1978_site_pass(5.0, 8.0));
    }
}


#[wasm_bindgen]
pub fn antikythera_state_json(jd_tt: f64, digital: bool) -> Result<String, JsError> {
    let machine = if digital {
        mtime_antikythera::AntikytheraMachine::digital()
    } else {
        mtime_antikythera::AntikytheraMachine::historical()
    };
    let state = machine
        .state_at_tt(jd_tt)
        .map_err(|_| JsError::new("invalid Antikythera instant"))?;
    Ok(format!(
        "{{\"profile_id\":\"{}\",\"jd_tt\":{:.9},\"solar_longitude_deg\":{:.9},\"lunar_longitude_deg\":{:.9},\"lunar_phase_deg\":{:.9},\"node_deg\":{:.9},\"metonic_phase\":{:.12},\"saros_phase\":{:.12},\"exeligmos_phase\":{:.12}}}",
        state.profile.id(),
        state.jd_tt,
        state.solar_longitude.angle_deg,
        state.lunar_longitude.angle_deg,
        state.lunar_phase.angle_deg,
        state.lunar_node.angle_deg,
        state.metonic_phase,
        state.saros_phase,
        state.exeligmos_phase,
    ))
}

#[wasm_bindgen]
pub fn mtime_clock_json(unix_seconds: f64) -> Result<String, JsError> {
    if !unix_seconds.is_finite()
        || unix_seconds < i64::MIN as f64
        || unix_seconds > i64::MAX as f64
    {
        return Err(JsError::new("invalid Unix second"));
    }
    let whole = unix_seconds.floor() as i64;
    let nanos = ((unix_seconds - whole as f64) * 1e9)
        .round()
        .clamp(0.0, 999_999_999.0) as u32;
    let packet = mtime_clock::digital_packet_from_utc(mtime_timescales::UtcInstant {
        unix_seconds: whole,
        nanoseconds: nanos,
    })
    .map_err(|error| JsError::new(&error.to_string()))?;
    packet
        .to_json()
        .map_err(|error| JsError::new(&error.to_string()))
}
