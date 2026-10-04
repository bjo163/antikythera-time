use mtime_core::EarthObserver;
use mtime_eop::{interpolate, parse_finals2000a, utc_jd_to_ut1_jd};
use mtime_hilal::{
    find_solar_altitude_crossing_utc, jd_to_utc, HilalEngine, SolarCrossingDirection,
};
use mtime_timescales::NasaSimpleDtr;
use mtime_worship::{
    conjunction_before_fajr_ut1, SolarEvent, SolarEventKind, SolarThresholdProfile,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: validate_wellington_diyanet <de440s.bsp> <finals.all>");
        std::process::exit(2);
    }

    let spk = std::fs::read(&args[1]).expect("read DE440 SPK");
    let eop_text = std::fs::read_to_string(&args[2]).expect("read IERS EOP");
    let eop = parse_finals2000a(&eop_text);
    let engine = HilalEngine::from_spk_bytes(&spk).expect("load DE440");
    let observer = EarthObserver::new(174.7772, -41.2889, 0.0).unwrap();
    let profile = SolarThresholdProfile::diyanet_imsak_fajr_18_2026();

    // 4 Oct 2026 in Wellington is NZDT (UTC+13). Diyanet publishes imsak 05:16,
    // corresponding to 2026-10-03 16:16 UTC.
    let current = find_solar_altitude_crossing_utc(
        &engine,
        2_461_317.083_333_333_5,
        2_461_317.25,
        observer,
        &eop,
        &NasaSimpleDtr,
        profile.sun_altitude_deg,
        SolarCrossingDirection::Rising,
        0.1,
    )
    .expect("compute Wellington -18 degree fajr");
    let current_utc = jd_to_utc(current.jd_utc).unwrap();
    let official_unix = 1_791_044_160_i64;
    let residual_minutes =
        (current_utc.unix_seconds - official_unix).unsigned_abs() as f64 / 60.0;

    println!("profile_id={}", profile.id);
    println!("target_altitude_deg={}", profile.sun_altitude_deg);
    println!("computed_utc_unix={}", current_utc.unix_seconds);
    println!("diyanet_published_utc_unix={official_unix}");
    println!("published_time_residual_minutes={residual_minutes:.3}");
    assert!(
        residual_minutes <= 10.0,
        "computed astronomical -18° crossing differs from published Diyanet imsak by >10 min"
    );

    // Diyanet's Shawwal 1447 statement publishes conjunction at
    // 2026-03-19 01:24 UTC. Compare it with computed Wellington fajr for
    // 20 March local time (the following Wellington dawn).
    let shawwal_fajr = find_solar_altitude_crossing_utc(
        &engine,
        2_461_119.083_333_333_5,
        2_461_119.25,
        observer,
        &eop,
        &NasaSimpleDtr,
        profile.sun_altitude_deg,
        SolarCrossingDirection::Rising,
        0.1,
    )
    .expect("compute Shawwal Wellington fajr");

    let conjunction_jd_utc = 2_461_118.558_333_333;
    let conjunction_eop =
        interpolate(&eop, conjunction_jd_utc - 2_400_000.5).expect("conjunction EOP");
    let conjunction_jd_ut1 = utc_jd_to_ut1_jd(conjunction_jd_utc, conjunction_eop);
    let fajr_event = SolarEvent {
        jd_ut1: shawwal_fajr.jd_ut1,
        altitude_deg: profile.sun_altitude_deg,
        kind: SolarEventKind::FajrThreshold,
    };
    let before = conjunction_before_fajr_ut1(conjunction_jd_ut1, fajr_event).unwrap();
    let shawwal_utc = jd_to_utc(shawwal_fajr.jd_utc).unwrap();

    println!("shawwal_wellington_fajr_utc_unix={}", shawwal_utc.unix_seconds);
    println!("conjunction_before_wellington_fajr={before}");
    assert!(before);
}
