use mtime_core::{CoordinateTime, QualityClass, Tt};
use mtime_hijri::{
    criterion_margins, CalendarProfile, GeometrySemantics, HijriAstronomicalState,
    NativeMTimeHijriInput,
};
use mtime_temporal::MTimeEngine;

fn main() {
    let state = MTimeEngine::digital()
        .from_tt(CoordinateTime::<Tt>::new(2_461_118.5, 0.0, 0.0).unwrap())
        .unwrap();

    // Geometry remains an independent astronomy/observer product.
    let astronomy = HijriAstronomicalState {
        conjunction_jd_tt: Some(2_461_117.9),
        sunset_jd_ut1: Some(2_461_118.0),
        moon_altitude_topocentric_deg: 3.15,
        elongation_geocentric_deg: 6.55,
        geometry_semantics: GeometrySemantics::mabims_required(),
        moon_age_hours: Some(14.4),
        moon_lag_minutes: Some(31.0),
        site_id: "DEMO-SITE".into(),
        ephemeris_source: "explicit reference fixture for native-M-Time demo".into(),
        quality: QualityClass::Reference,
    };

    let input = NativeMTimeHijriInput::new(state, astronomy);
    let profile = CalendarProfile::mabims_indonesia_2026();
    let result = profile.evaluate_native(&input);
    let margins = criterion_margins(&profile, &input.astronomy);

    println!("mtime_schema={}", input.mtime.profile_id);
    println!("instant_key={}", input.instant_key().0);
    println!("calendar_profile={}@{}", result.profile_id, result.profile_version);
    println!("criterion_met={:?}", result.met);
    for margin in margins {
        println!(
            "clause={} actual={:?} threshold={} signed_margin={:?} unit={}",
            margin.clause_id,
            margin.actual,
            margin.threshold,
            margin.signed_margin,
            margin.unit
        );
    }
    println!("observation_layer=SEPARATE");
    println!("authority_layer=SEPARATE");
}
