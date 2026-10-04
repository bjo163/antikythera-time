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

    // Shawwal 1447: Diyanet publishes conjunction at 2026-03-19 01:24 UTC
    // and requires conjunction before Wellington/New Zealand fajr. The
    // following Wellington dawn is 20 March local time (NZDT, UTC+13).
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
    .expect("compute Shawwal Wellington -18 degree fajr");

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
    let fajr_utc = jd_to_utc(shawwal_fajr.jd_utc).unwrap();
    let conjunction_utc = jd_to_utc(conjunction_jd_utc).unwrap();
    let delta_hours =
        (fajr_utc.unix_seconds - conjunction_utc.unix_seconds) as f64 / 3600.0;

    println!("profile_id={}", profile.id);
    println!("profile_version={}", profile.version);
    println!("target_altitude_deg={}", profile.sun_altitude_deg);
    println!("wellington_fajr_utc_unix={}", fajr_utc.unix_seconds);
    println!("wellington_fajr_jd_ut1={:.12}", shawwal_fajr.jd_ut1);
    println!("solver_residual_deg={:.12}", shawwal_fajr.residual_deg);
    println!("conjunction_utc_unix={}", conjunction_utc.unix_seconds);
    println!("conjunction_to_fajr_hours={delta_hours:.6}");
    println!("conjunction_before_wellington_fajr={before}");

    // Sanity bounds for a Wellington early-morning event on 20 March NZDT:
    // 15:00..18:00 UTC on 19 March.
    assert!((1_773_932_400..=1_773_943_200).contains(&fajr_utc.unix_seconds));
    assert!(shawwal_fajr.residual_deg.abs() < 1e-4);
    assert!(before);
}
