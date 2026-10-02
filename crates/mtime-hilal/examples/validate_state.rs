use std::{env, fs};

use mtime_core::EarthObserver;
use mtime_eop::parse_finals2000a;
use mtime_hilal::HilalEngine;
use mtime_jpl::parse_single_topocentric_moon_4;
use mtime_timescales::{NasaSimpleDtr, UtcInstant};

fn main() {
    let mut args = env::args().skip(1);
    let spk_path = args.next().expect("SPK path");
    let horizons_path = args.next().expect("Horizons observer text");
    let iers_path = args.next().expect("IERS finals path");

    let bytes = fs::read(spk_path).expect("SPK");
    let engine = HilalEngine::from_spk_bytes(&bytes).expect("Hilal engine");
    let eop_rows = parse_finals2000a(&fs::read_to_string(iers_path).expect("IERS"));
    let observer = EarthObserver::new(106.8272, -6.1754, 8.0).expect("observer");
    let utc = UtcInstant {
        unix_seconds: 1_773_914_400,
        nanoseconds: 0,
    };
    let state = engine
        .state_at_utc(utc, observer, "JAKARTA_FIXED_ORACLE", &eop_rows, &NasaSimpleDtr)
        .expect("integrated state");

    let href = parse_single_topocentric_moon_4(
        &fs::read_to_string(horizons_path).expect("Horizons"),
    )
    .expect("Horizons parse");

    let altitude_error =
        (state.moon_altitude_topocentric_deg - href.elevation_deg).abs();

    println!("integrated_state_altitude_deg={:.12}", state.moon_altitude_topocentric_deg);
    println!("horizons_altitude_deg={:.12}", href.elevation_deg);
    println!("integrated_altitude_error_deg={altitude_error:.12}");
    println!("integrated_elongation_deg={:.12}", state.elongation_geocentric_deg);
    println!("quality={:?}", state.quality);
    println!("ephemeris_source={}", state.ephemeris_source);

    if altitude_error > 0.001 {
        eprintln!("integrated hilal state gate failed: altitude residual > 0.001 deg");
        std::process::exit(1);
    }
}
