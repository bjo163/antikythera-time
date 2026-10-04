use mtime_core::{CoordinateTime, Tt};
use mtime_temporal::MTimeEngine;
use mtime_worship::{
    bind_solar_event_to_mtime, NativeFastingWindow, SolarEvent, SolarEventKind,
    SolarThresholdProfile,
};

fn main() {
    let profile = SolarThresholdProfile::diyanet_imsak_fajr_18_2026();

    let fajr_state = MTimeEngine::digital()
        .from_tt(CoordinateTime::<Tt>::new(2_461_118.2, 0.0, 0.0).unwrap())
        .unwrap();
    let sunset_state = MTimeEngine::digital()
        .from_tt(CoordinateTime::<Tt>::new(2_461_118.7, 0.0, 0.0).unwrap())
        .unwrap();

    // UT1 event values remain explicit astronomical/reference results.
    let fajr = SolarEvent {
        jd_ut1: 2_461_118.20,
        altitude_deg: -18.0,
        kind: SolarEventKind::FajrThreshold,
    };
    let sunset = SolarEvent {
        jd_ut1: 2_461_118.70,
        altitude_deg: 0.0,
        kind: SolarEventKind::Sunset,
    };

    let start = bind_solar_event_to_mtime(fajr_state, fajr, &profile);
    let end = bind_solar_event_to_mtime(sunset_state, sunset, &profile);
    let window = NativeFastingWindow::new(start, end).unwrap();

    println!("profile={}", window.start.worship_profile_id);
    println!("start_mtime_ns={}", window.start.mtime.linear_si_nanoseconds_from_j2000_tt);
    println!("end_mtime_ns={}", window.end.mtime.linear_si_nanoseconds_from_j2000_tt);
    println!("start_event={:?}", window.start.event.kind);
    println!("end_event={:?}", window.end.event.kind);
    println!("reference_geometry_layer=SEPARATE");
}
