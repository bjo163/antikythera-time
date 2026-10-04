use std::{env, fs};

use mtime_antikythera::{shortest_angle_deg, wrap_deg, AntikytheraMachine};
use mtime_astro::Body;
use mtime_core::{CoordinateTime, Tt};
use mtime_spk::{vector_to_j2000_ecliptic, SpkEphemeris};
use mtime_timescales::{tt_to_tdb, DtrProvider, NasaSimpleDtr};

fn main() {
    let spk_path = env::args().nth(1).expect("usage: validate_antikythera_matrix <de440s.bsp>");
    let bytes = fs::read(spk_path).expect("read DE440");
    let eph = SpkEphemeris::from_bytes(&bytes).expect("parse DE440");
    let historical = AntikytheraMachine::historical();
    let digital = AntikytheraMachine::digital();

    let epochs = (0..12)
        .map(|i| 2_461_041.5 + f64::from(i) * 30.436_875)
        .collect::<Vec<_>>();

    let (base_sun, base_moon) = jpl_longitudes(&eph, epochs[0]);
    let base_h = historical.state_at_tt(epochs[0]).unwrap();
    let base_d = digital.state_at_tt(epochs[0]).unwrap();

    let mut max_h_sun = 0.0_f64;
    let mut max_h_moon = 0.0_f64;
    let mut max_h_phase = 0.0_f64;
    let mut max_d_sun = 0.0_f64;
    let mut max_d_moon = 0.0_f64;
    let mut max_d_phase = 0.0_f64;

    for (index, jd_tt) in epochs.into_iter().enumerate() {
        let (jpl_sun, jpl_moon) = jpl_longitudes(&eph, jd_tt);
        let jpl_phase = wrap_deg(jpl_moon - jpl_sun);
        let jpl_sun_delta = wrap_deg(jpl_sun - base_sun);
        let jpl_moon_delta = wrap_deg(jpl_moon - base_moon);

        let h = historical.state_at_tt(jd_tt).unwrap();
        let d = digital.state_at_tt(jd_tt).unwrap();

        let h_sun = shortest_angle_deg(
            wrap_deg(h.solar_longitude.angle_deg - base_h.solar_longitude.angle_deg),
            jpl_sun_delta,
        );
        let h_moon = shortest_angle_deg(
            wrap_deg(h.lunar_longitude.angle_deg - base_h.lunar_longitude.angle_deg),
            jpl_moon_delta,
        );
        let h_phase = shortest_angle_deg(h.lunar_phase.angle_deg, jpl_phase);

        let d_sun = shortest_angle_deg(
            wrap_deg(d.solar_longitude.angle_deg - base_d.solar_longitude.angle_deg),
            jpl_sun_delta,
        );
        let d_moon = shortest_angle_deg(
            wrap_deg(d.lunar_longitude.angle_deg - base_d.lunar_longitude.angle_deg),
            jpl_moon_delta,
        );
        let d_phase = shortest_angle_deg(d.lunar_phase.angle_deg, jpl_phase);

        max_h_sun = max_h_sun.max(h_sun);
        max_h_moon = max_h_moon.max(h_moon);
        max_h_phase = max_h_phase.max(h_phase);
        max_d_sun = max_d_sun.max(d_sun);
        max_d_moon = max_d_moon.max(d_moon);
        max_d_phase = max_d_phase.max(d_phase);

        println!("case={index:02}");
        println!("jd_tt={jd_tt:.9}");
        println!("historical_sun_relative_error_deg={h_sun:.9}");
        println!("historical_moon_relative_error_deg={h_moon:.9}");
        println!("historical_phase_error_deg={h_phase:.9}");
        println!("digital_sun_relative_error_deg={d_sun:.9}");
        println!("digital_moon_relative_error_deg={d_moon:.9}");
        println!("digital_phase_error_deg={d_phase:.9}");
    }

    println!("===== ANTYKITHERA CALIBRATION SUMMARY =====");
    println!("epochs=12");
    println!("historical_max_sun_relative_error_deg={max_h_sun:.9}");
    println!("historical_max_moon_relative_error_deg={max_h_moon:.9}");
    println!("historical_max_phase_error_deg={max_h_phase:.9}");
    println!("digital_max_sun_relative_error_deg={max_d_sun:.9}");
    println!("digital_max_moon_relative_error_deg={max_d_moon:.9}");
    println!("digital_max_phase_error_deg={max_d_phase:.9}");

    assert!(max_h_sun <= 4.5, "historical mean-Sun baseline exceeded");
    assert!(max_h_moon <= 8.0, "historical lunar reconstruction baseline exceeded");
    assert!(max_h_phase <= 8.0, "historical phase baseline exceeded");
    assert!(max_d_sun <= 0.25, "digital solar correction baseline exceeded");
    assert!(max_d_moon <= 1.5, "digital lunar correction baseline exceeded");
    assert!(max_d_phase <= 1.5, "digital lunar-phase correction baseline exceeded");
}

fn jpl_longitudes(eph: &SpkEphemeris<'_>, jd_tt: f64) -> (f64, f64) {
    let tt = CoordinateTime::<Tt>::new(jd_tt, 0.0, 0.0).unwrap();
    let dtr = NasaSimpleDtr.dtr(&tt).unwrap();
    let tdb = tt_to_tdb(&tt, dtr).unwrap();
    let p = tdb.jd_parts();
    let sun = vector_to_j2000_ecliptic(
        eph.geocentric_vector_km(Body::Sun, (p.d1, p.d2)).unwrap(),
    )
    .unwrap();
    let moon = vector_to_j2000_ecliptic(
        eph.geocentric_vector_km(Body::Moon, (p.d1, p.d2)).unwrap(),
    )
    .unwrap();
    (sun.longitude_deg, moon.longitude_deg)
}
