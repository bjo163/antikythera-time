use std::{env, fs};

use mtime_astro::Body;
use mtime_jpl::parse_single_geometric_vector;
use mtime_spk::SpkEphemeris;

fn error_km(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        eprintln!("usage: compare_de440s_horizons <de440s.bsp> <sun.txt> <moon.txt>");
        std::process::exit(2);
    }

    let bsp = fs::read(&args[1]).expect("read DE440s kernel");
    let sun_text = fs::read_to_string(&args[2]).expect("read Horizons Sun vector");
    let moon_text = fs::read_to_string(&args[3]).expect("read Horizons Moon vector");

    let eph = SpkEphemeris::from_bytes(&bsp).expect("parse SPK");
    let jd_tdb = (2_461_119.0, 0.0);

    let sun_spk = eph
        .geocentric_vector_km(Body::Sun, jd_tdb)
        .expect("Sun vector from SPK");
    let moon_spk = eph
        .geocentric_vector_km(Body::Moon, jd_tdb)
        .expect("Moon vector from SPK");

    let sun_h = parse_single_geometric_vector(&sun_text).expect("parse Horizons Sun");
    let moon_h = parse_single_geometric_vector(&moon_text).expect("parse Horizons Moon");

    let sun_err = error_km(sun_spk, [sun_h.x, sun_h.y, sun_h.z]);
    let moon_err = error_km(moon_spk, [moon_h.x, moon_h.y, moon_h.z]);

    println!("DE440s offline vs Horizons @ JD TDB 2461119.0");
    println!("sun_error_km={sun_err:.9}");
    println!("moon_error_km={moon_err:.9}");

    // Horizons uses the DE440/441 family for major-body work. 100 km is
    // deliberately conservative: this gate detects frame/time/origin mistakes,
    // while the log preserves the measured residual for future tightening.
    let threshold_km = 100.0;
    if sun_err > threshold_km || moon_err > threshold_km {
        eprintln!(
            "offline SPK residual exceeds {threshold_km} km gate: Sun={sun_err} Moon={moon_err}"
        );
        std::process::exit(1);
    }
}
