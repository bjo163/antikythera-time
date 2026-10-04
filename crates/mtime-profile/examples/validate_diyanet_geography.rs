use mtime_core::QualityClass;
use mtime_geospatial::{AmericasMainlandGeoProvider, RegisteredSite};
use mtime_hijri::{GeometrySemantics, HijriAstronomicalState};
use mtime_profile::{bundled_diyanet_1978_global, evaluate_compiled_profile_with_providers, WellingtonFajrProvider};
use mtime_worship::{SolarEvent, SolarEventKind};

struct FixedWellingtonFajr;

impl WellingtonFajrProvider for FixedWellingtonFajr {
    fn fajr_event(&self) -> Option<SolarEvent> {
        Some(SolarEvent {
            jd_ut1: 2_460_000.25,
            altitude_deg: -18.0,
            kind: SolarEventKind::FajrThreshold,
        })
    }
}

fn state(site_id: &str) -> HijriAstronomicalState {
    HijriAstronomicalState {
        conjunction_jd_tt: None,
        sunset_jd_ut1: None,
        moon_altitude_topocentric_deg: 5.5,
        elongation_geocentric_deg: 8.5,
        geometry_semantics: GeometrySemantics::mabims_required(),
        moon_age_hours: None,
        moon_lag_minutes: None,
        site_id: site_id.into(),
        ephemeris_source: "integration fixture".into(),
        quality: QualityClass::Reference,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: validate_diyanet_geography <ne_110m_land.geojson>");
        std::process::exit(2);
    }

    let payload = std::fs::read(&args[1]).expect("read Natural Earth GeoJSON");
    let provider = AmericasMainlandGeoProvider::from_natural_earth_110m_land(
        &payload,
        vec![
            RegisteredSite::new("NEW_YORK", -74.0060, 40.7128).unwrap(),
            RegisteredSite::new("SANTIAGO", -70.6693, -33.4489).unwrap(),
            RegisteredSite::new("HAVANA", -82.3666, 23.1136).unwrap(),
            RegisteredSite::new("WELLINGTON", 174.7772, -41.2889).unwrap(),
        ],
    )
    .unwrap();
    let profile = bundled_diyanet_1978_global();

    let mainland = evaluate_compiled_profile_with_providers(
        &profile,
        &[state("NEW_YORK")],
        Some(2_460_000.20),
        &provider,
        &FixedWellingtonFajr,
    )
    .unwrap();
    assert_eq!(mainland.complete_rule_met, Some(true));

    let island = evaluate_compiled_profile_with_providers(
        &profile,
        &[state("HAVANA")],
        Some(2_460_000.20),
        &provider,
        &FixedWellingtonFajr,
    )
    .unwrap();
    assert_eq!(island.complete_rule_met, Some(false));

    let outside = evaluate_compiled_profile_with_providers(
        &profile,
        &[state("WELLINGTON")],
        Some(2_460_000.20),
        &provider,
        &FixedWellingtonFajr,
    )
    .unwrap();
    assert_eq!(outside.complete_rule_met, Some(false));

    let unknown = evaluate_compiled_profile_with_providers(
        &profile,
        &[state("UNKNOWN_SITE")],
        Some(2_460_000.20),
        &provider,
        &FixedWellingtonFajr,
    )
    .unwrap();
    assert_eq!(unknown.complete_rule_met, None);

    println!("new_york_complete_rule={:?}", mainland.complete_rule_met);
    println!("havana_complete_rule={:?}", island.complete_rule_met);
    println!("wellington_complete_rule={:?}", outside.complete_rule_met);
    println!("unknown_site_complete_rule={:?}", unknown.complete_rule_met);
}
