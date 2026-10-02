use std::{env, fs};
use mtime_astro::Body;
use mtime_spk::SpkEphemeris;

fn main() {
    let path = env::args().nth(1).expect("usage: de440_probe <de440s.bsp> [jd_tdb]");
    let jd = env::args().nth(2).map(|x| x.parse::<f64>().expect("jd_tdb")).unwrap_or(2_461_119.0);
    let bytes = fs::read(path).expect("read SPK");
    let eph = SpkEphemeris::from_bytes(&bytes).expect("parse SPK");
    let moon = eph.geocentric_vector_km(Body::Moon, (jd, 0.0)).expect("Moon vector");
    let sun = eph.geocentric_vector_km(Body::Sun, (jd, 0.0)).expect("Sun vector");
    let elong = eph.geometric_elongation_deg((jd, 0.0)).expect("elongation");
    println!("jd_tdb={jd:.9}");
    println!("moon_geo_km={:.9},{:.9},{:.9}", moon[0], moon[1], moon[2]);
    println!("sun_geo_km={:.9},{:.9},{:.9}", sun[0], sun[1], sun[2]);
    println!("elongation_geometric_deg={elong:.12}");
}
