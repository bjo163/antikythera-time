use std::{env, fs};

fn main() {
    let topo_path = env::args().nth(1).expect("topocentric Horizons response path");
    let sun_path = env::args().nth(2).expect("geocentric Sun vector response path");
    let moon_path = env::args().nth(3).expect("geocentric Moon vector response path");

    let topo_text = fs::read_to_string(topo_path).expect("read topocentric response");
    let sun_text = fs::read_to_string(sun_path).expect("read Sun vector response");
    let moon_text = fs::read_to_string(moon_path).expect("read Moon vector response");

    let topo =
        mtime_jpl::parse_single_topocentric_moon_4(&topo_text).expect("parse Horizons quantity 4");
    let sun = mtime_jpl::parse_single_geometric_vector(&sun_text).expect("parse Sun geometric vector");
    let moon = mtime_jpl::parse_single_geometric_vector(&moon_text).expect("parse Moon geometric vector");
    let h = mtime_jpl::combine_hilal_reference_from_vectors(topo, sun, moon)
        .expect("combine MABIMS reference geometry");

    println!(
        "moon_azimuth_topocentric_deg={} moon_altitude_topocentric_deg={} elongation_geocentric_geometric_deg={}",
        h.topocentric_azimuth_deg, h.moon_altitude_topocentric_deg, h.elongation_geocentric_deg
    );
}
