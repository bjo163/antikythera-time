use std::{env, fs};

use mtime_astro::{topocentric_horizon_iau2006, Body};
use mtime_core::EarthObserver;
use mtime_eop::{interpolate, parse_finals2000a, utc_jd_to_ut1_jd};
use mtime_jpl::parse_single_topocentric_moon_4;
use mtime_spk::SpkEphemeris;
use mtime_timescales::{tt_to_tdb, DtrProvider, NasaSimpleDtr, UtcInstant, utc_to_tt};

const JD_UNIX_EPOCH: f64 = 2_440_587.5;
const SECONDS_PER_DAY: f64 = 86_400.0;

fn main() {
    let mut args = env::args().skip(1);
    let spk_path = args.next().expect("SPK path");
    let horizons_path = args.next().expect("Horizons observer text");
    let iers_path = args.next().expect("IERS finals path");
    let lon: f64 = args.next().expect("longitude deg").parse().expect("longitude");
    let lat: f64 = args.next().expect("latitude deg").parse().expect("latitude");
    let height_m: f64 = args.next().expect("height m").parse().expect("height");
    let jd_utc: f64 = args.next().expect("JD UTC").parse().expect("JD");
    let label = args.next().unwrap_or_else(|| "CASE".into());

    let utc = utc_from_jd(jd_utc);
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

    let observer = EarthObserver::new(lon, lat, height_m).expect("observer");
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

    println!("case={label}");
    println!("jd_utc={jd_utc:.12}");
    println!("longitude_deg={lon:.8}");
    println!("latitude_deg={lat:.8}");
    println!("ours_altitude_deg={:.12}", ours.altitude_deg);
    println!("horizons_altitude_deg={:.12}", href.elevation_deg);
    println!("altitude_error_deg={elevation_error_deg:.12}");
    println!("ours_azimuth_deg={:.12}", ours.azimuth_deg);
    println!("horizons_azimuth_deg={:.12}", href.azimuth_deg);
    println!("azimuth_error_deg={az_error_deg:.12}");
    println!("ut1_minus_utc_seconds={:.9}", eop.ut1_minus_utc_seconds);

    if elevation_error_deg > 0.001 || az_error_deg > 0.001 {
        eprintln!("matrix reference gate failed: residual exceeds 0.001 deg");
        std::process::exit(1);
    }
}

fn utc_from_jd(jd_utc: f64) -> UtcInstant {
    let total = (jd_utc - JD_UNIX_EPOCH) * SECONDS_PER_DAY;
    let mut seconds = total.floor() as i64;
    let mut nanoseconds = ((total - seconds as f64) * 1e9).round() as i64;
    if nanoseconds >= 1_000_000_000 {
        seconds += 1;
        nanoseconds -= 1_000_000_000;
    }
    if nanoseconds < 0 {
        seconds -= 1;
        nanoseconds += 1_000_000_000;
    }
    UtcInstant { unix_seconds: seconds, nanoseconds: nanoseconds as u32 }
}

fn circular_delta_deg(a: f64, b: f64) -> f64 {
    (a - b + 180.0).rem_euclid(360.0) - 180.0
}
