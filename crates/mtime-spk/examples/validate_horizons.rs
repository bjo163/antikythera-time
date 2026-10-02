use std::{env, fs};
use mtime_astro::Body;
use mtime_jpl::parse_single_geometric_vector;
use mtime_spk::SpkEphemeris;

fn max_abs_delta(a: [f64; 3], b: [f64; 3]) -> f64 {
    (a[0] - b[0]).abs().max((a[1] - b[1]).abs()).max((a[2] - b[2]).abs())
}

fn main() {
    let mut args = env::args().skip(1);
    let spk_path = args.next().expect("SPK path");
    let sun_txt = args.next().expect("Horizons Sun text");
    let moon_txt = args.next().expect("Horizons Moon text");
    let jd = args.next().unwrap_or_else(|| "2461119.0".into()).parse::<f64>().expect("JD TDB");

    let bytes = fs::read(spk_path).expect("read SPK");
    let eph = SpkEphemeris::from_bytes(&bytes).expect("parse SPK");
    let sun = eph.geocentric_vector_km(Body::Sun, (jd, 0.0)).expect("SPK Sun");
    let moon = eph.geocentric_vector_km(Body::Moon, (jd, 0.0)).expect("SPK Moon");

    let sun_h = parse_single_geometric_vector(&fs::read_to_string(sun_txt).expect("Sun text")).expect("parse Sun");
    let moon_h = parse_single_geometric_vector(&fs::read_to_string(moon_txt).expect("Moon text")).expect("parse Moon");
    let sun_ref = [sun_h.x, sun_h.y, sun_h.z];
    let moon_ref = [moon_h.x, moon_h.y, moon_h.z];

    let sun_max_km = max_abs_delta(sun, sun_ref);
    let moon_max_km = max_abs_delta(moon, moon_ref);
    let elong = eph.geometric_elongation_deg((jd, 0.0)).expect("elongation");

    println!("SPK/Horizons geometric vector comparison at JD TDB {jd:.9}");
    println!("sun_max_component_error_km={sun_max_km:.9}");
    println!("moon_max_component_error_km={moon_max_km:.9}");
    println!("spk_geometric_elongation_deg={elong:.12}");

    // This gate is intentionally much looser than the expected DE440/Horizons
    // agreement. It detects wrong centers/frames/time semantics, not rounding.
    if sun_max_km > 10.0 || moon_max_km > 10.0 {
        eprintln!("reference gate failed: expected each max component error <= 10 km");
        std::process::exit(1);
    }
}
