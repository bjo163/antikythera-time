use std::{env, fs};

use mtime_core::EarthObserver;
use mtime_eop::parse_finals2000a;
use mtime_hilal::{
    find_solar_altitude_crossing_utc, HilalEngine, SolarCrossingDirection,
};
use mtime_timescales::NasaSimpleDtr;

fn main() {
    let mut args = env::args().skip(1);
    let spk_path = args.next().expect("SPK path");
    let iers_path = args.next().expect("IERS path");
    let lon: f64 = args.next().expect("longitude").parse().expect("longitude");
    let lat: f64 = args.next().expect("latitude").parse().expect("latitude");
    let height_m: f64 = args.next().expect("height").parse().expect("height");
    let start_jd: f64 = args.next().expect("start JD").parse().expect("start JD");
    let end_jd: f64 = args.next().expect("end JD").parse().expect("end JD");
    let target: f64 = args.next().expect("target altitude").parse().expect("target");
    let label = args.next().unwrap_or_else(|| "CASE".into());

    let spk = fs::read(spk_path).expect("read SPK");
    let eop_text = fs::read_to_string(iers_path).expect("read IERS");
    let eop = parse_finals2000a(&eop_text);
    let engine = HilalEngine::from_spk_bytes(&spk).expect("load DE440");
    let observer = EarthObserver::new(lon, lat, height_m).expect("observer");

    let solution = find_solar_altitude_crossing_utc(
        &engine,
        start_jd,
        end_jd,
        observer,
        &eop,
        &NasaSimpleDtr,
        target,
        SolarCrossingDirection::Rising,
        0.05,
    )
    .expect("solve rising solar crossing");

    println!("case={label}");
    println!("jd_utc={:.12}", solution.jd_utc);
    println!("jd_ut1={:.12}", solution.jd_ut1);
    println!("target_altitude_deg={target:.12}");
    println!("solver_residual_deg={:.12}", solution.residual_deg);

    if solution.residual_deg.abs() > 0.0001 {
        eprintln!("solar crossing residual exceeds 0.0001 deg");
        std::process::exit(1);
    }
}
