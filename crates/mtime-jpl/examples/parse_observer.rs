use std::{env, fs};
fn main() {
    let path = env::args().nth(1).expect("Horizons response path");
    let text = fs::read_to_string(path).expect("read response");
    let x = mtime_jpl::parse_single_moon_observer_4_23(&text).expect("parse Horizons quantities 4,23");
    println!(
        "moon_azimuth_deg={} moon_elevation_deg={} solar_elongation_deg={}",
        x.azimuth_deg, x.elevation_deg, x.solar_elongation_deg
    );
}
