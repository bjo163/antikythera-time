use std::{env, fs};

fn main() {
    let topo_path = env::args().nth(1).expect("topocentric Horizons response path");
    let geo_path = env::args().nth(2).expect("geocentric Horizons response path");

    let topo_text = fs::read_to_string(topo_path).expect("read topocentric response");
    let geo_text = fs::read_to_string(geo_path).expect("read geocentric response");

    let topo =
        mtime_jpl::parse_single_topocentric_moon_4(&topo_text).expect("parse Horizons quantity 4");
    let geo = mtime_jpl::parse_single_geocentric_elongation_23(&geo_text)
        .expect("parse Horizons quantity 23");
    let h = mtime_jpl::combine_hilal_reference(topo, geo);

    println!(
        "moon_azimuth_topocentric_deg={} moon_altitude_topocentric_deg={} elongation_geocentric_deg={}",
        h.topocentric_azimuth_deg, h.moon_altitude_topocentric_deg, h.elongation_geocentric_deg
    );
}
