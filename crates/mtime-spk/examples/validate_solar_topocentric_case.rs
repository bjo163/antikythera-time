use std::{env, fs};

use mtime_astro::{topocentric_horizon_iau2006, Body};
use mtime_core::EarthObserver;
use mtime_eop::{interpolate, parse_finals2000a, utc_jd_to_ut1_jd};
use mtime_jpl::parse_single_topocentric_quantity_4;
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
    let target_altitude_deg: f64 = args
        .next()
        .unwrap_or_else(|| "-18".into())
        .parse()
        .expect("target altitude");

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
    let sun = eph
        .geocentric_vector_km(Body::Sun, (p.d1, p.d2))
        .expect("Sun vector");

    let observer = EarthObserver::new(lon, lat, height_m).expect("observer");
    let ours = topocentric_horizon_iau2006(
        sun,
        observer,
        (tt.jd_parts().d1, tt.jd_parts().d2),
        (2_400_000.5, jd_ut1 - 2_400_000.5),
        eop.xp_arcsec,
        eop.yp_arcsec,
    )
    .expect("IAU topocentric");

    let htxt = fs::read_to_string(horizons_path).expect("Horizons");
    let href = parse_single_topocentric_quantity_4(&htxt).expect("Horizons quantity 4");

    let elevation_error_deg = (ours.altitude_deg - href.elevation_deg).abs();
    let direction_error_deg = horizon_direction_separation_deg(
        ours.altitude_deg,
        ours.azimuth_deg,
        href.elevation_deg,
        href.azimuth_deg,
    );
    let horizons_target_residual_deg = (href.elevation_deg - target_altitude_deg).abs();

    println!("case={label}");
    println!("jd_utc={jd_utc:.12}");
    println!("ours_altitude_deg={:.12}", ours.altitude_deg);
    println!("horizons_altitude_deg={:.12}", href.elevation_deg);
    println!("altitude_error_deg={elevation_error_deg:.12}");
    println!("direction_error_deg={direction_error_deg:.12}");
    println!("horizons_target_residual_deg={horizons_target_residual_deg:.12}");

    if elevation_error_deg > 0.001
        || direction_error_deg > 0.001
        || horizons_target_residual_deg > 0.0015
    {
        eprintln!("solar worship-time oracle gate failed");
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
    UtcInstant {
        unix_seconds: seconds,
        nanoseconds: nanoseconds as u32,
    }
}

fn horizon_direction_separation_deg(
    altitude_a_deg: f64,
    azimuth_a_deg: f64,
    altitude_b_deg: f64,
    azimuth_b_deg: f64,
) -> f64 {
    let alt_a = altitude_a_deg.to_radians();
    let alt_b = altitude_b_deg.to_radians();
    let daz = (azimuth_a_deg - azimuth_b_deg).to_radians();
    (
        alt_a.sin() * alt_b.sin()
            + alt_a.cos() * alt_b.cos() * daz.cos()
    )
    .clamp(-1.0, 1.0)
    .acos()
    .to_degrees()
}
