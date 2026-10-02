use std::{env, fs};

use mtime_astro::{topocentric_geometric_horizon_from_vector, topocentric_horizon_iau2006, Body};
use mtime_core::EarthObserver;
use mtime_eop::{interpolate, parse_finals2000a, utc_jd_to_ut1_jd};
use mtime_jpl::parse_single_topocentric_moon_4;
use mtime_spk::SpkEphemeris;
use mtime_timescales::{tt_to_tdb, DtrProvider, NasaSimpleDtr, UtcInstant, utc_to_tt};

fn main() {
    let mut args = env::args().skip(1);
    let spk_path = args.next().expect("SPK path");
    let horizons_path = args.next().expect("Horizons observer text");
    let iers_path = args.next().expect("IERS finals path");

    // Fixed diagnostic epoch: 2026-03-19T10:00:00Z.
    let utc = UtcInstant {
        unix_seconds: 1_773_914_400,
        nanoseconds: 0,
    };
    let jd_utc = 2_461_118.916_666_666_5;
    let mjd_utc = jd_utc - 2_400_000.5;

    let tt = utc_to_tt(utc).expect("UTC->TT");
    let dtr = NasaSimpleDtr.dtr(&tt).expect("TDB-TT");
    let tdb = tt_to_tdb(&tt, dtr).expect("TT->TDB");
    let p = tdb.jd_parts();

    let eop_text = fs::read_to_string(iers_path).expect("IERS finals");
    let rows = parse_finals2000a(&eop_text);
    let eop = interpolate(&rows, mjd_utc).expect("IERS interpolation");
    let jd_ut1 = utc_jd_to_ut1_jd(jd_utc, eop);

    let bytes = fs::read(spk_path).expect("SPK");
    let eph = SpkEphemeris::from_bytes(&bytes).expect("parse SPK");
    let moon = eph
        .geocentric_vector_km(Body::Moon, (p.d1, p.d2))
        .expect("Moon vector");

    let observer = EarthObserver::new(106.8272, -6.1754, 8.0).expect("observer");
    let legacy =
        topocentric_geometric_horizon_from_vector(moon, observer, jd_ut1).expect("legacy topocentric");
    let ours = topocentric_horizon_iau2006(
        moon,
        observer,
        (tt.jd_parts().d1, tt.jd_parts().d2),
        (2_400_000.5, jd_ut1 - 2_400_000.5),
        eop.xp_arcsec,
        eop.yp_arcsec,
    )
    .expect("IAU topocentric");

    let htxt = fs::read_to_string(horizons_path).expect("Horizons");
    let href = parse_single_topocentric_moon_4(&htxt).expect("Horizons quantity 4");

    let elevation_error_deg = (ours.altitude_deg - href.elevation_deg).abs();
    let az_error_deg = circular_delta_deg(ours.azimuth_deg, href.azimuth_deg).abs();

    println!("M-Time topocentric diagnostic 2026-03-19T10:00:00Z");
    println!("legacy_altitude_deg={:.12}", legacy.altitude_deg);
    println!("ours_altitude_deg={:.12}", ours.altitude_deg);
    println!("horizons_altitude_deg={:.12}", href.elevation_deg);
    println!("altitude_error_deg={elevation_error_deg:.12}");
    println!("ours_azimuth_deg={:.12}", ours.azimuth_deg);
    println!("horizons_azimuth_deg={:.12}", href.azimuth_deg);
    println!("azimuth_error_deg={az_error_deg:.12}");
    println!("ut1_minus_utc_seconds={:.9}", eop.ut1_minus_utc_seconds);

    // IAU 2006/2000A + IERS EOP diagnostic gate.
    // Keep this at 0.1 deg until light-time/apparent-place semantics are
    // explicitly matched to the Horizons observer quantity.
    if elevation_error_deg > 0.1 || az_error_deg > 0.1 {
        eprintln!("diagnostic gate failed: residual exceeds 0.1 deg");
        std::process::exit(1);
    }
}

fn circular_delta_deg(a: f64, b: f64) -> f64 {
    (a - b + 180.0).rem_euclid(360.0) - 180.0
}
