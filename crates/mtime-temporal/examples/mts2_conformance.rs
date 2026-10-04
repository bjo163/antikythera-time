use mtime_antikythera::AntikytheraMachine;
use mtime_core::{CoordinateTime, Tt};
use mtime_temporal::{MTimeEngine, MTimeWireV2};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn conformance_epochs() -> Vec<f64> {
    let mut epochs = vec![
        2_396_758.5, // 1850-01-01 validated start
        2_415_020.5, // 1900
        2_433_282.5, // 1950
        2_451_545.0, // J2000
        2_469_807.5, // 2050
        2_488_069.5, // 2100
        2_506_330.5, // 2149-12-31-ish validated end
    ];
    // Deterministic differential corpus spanning the validated interval.
    // The LCG is only a repeatable vector generator, not a physical model.
    let start = 2_396_758.5_f64;
    let span = 2_506_331.5_f64 - start;
    let mut x = 0x4d54494d45_u64;
    for _ in 0..128 {
        x = x.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        let u = (x >> 11) as f64 / ((1_u64 << 53) as f64);
        epochs.push(start + u * span);
    }
    epochs
}

fn main() {
    let epochs = conformance_epochs();
    for jd in epochs {
        let state = MTimeEngine::digital()
            .from_tt(CoordinateTime::<Tt>::new(jd, 0.0, 0.0).unwrap())
            .unwrap();
        let wire = MTimeWireV2::from_state(state);
        println!(
            "V|{:.9}|{}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{}",
            jd,
            wire.linear_si_nanoseconds_from_j2000_tt,
            wire.dial_deg[0],
            wire.dial_deg[1],
            wire.dial_deg[2],
            wire.dial_deg[3],
            wire.cycle_phase[0],
            wire.cycle_phase[1],
            wire.cycle_phase[2],
            wire.cycle_phase[3],
            wire.cycle_phase[4],
            wire.cycle_phase[5],
            wire.cycle_phase[6],
            wire.cycle_phase[7],
            hex(&wire.encode_binary())
        );

        let experimental = AntikytheraMachine::digital_v2_experimental()
            .state_at_tt(jd)
            .unwrap();
        println!(
            "E|{:.9}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}|{:.12}",
            jd,
            experimental.solar_longitude.angle_deg,
            experimental.lunar_longitude.angle_deg,
            experimental.lunar_phase.angle_deg,
            experimental.lunar_node.angle_deg,
            experimental.solar_year_phase,
            experimental.synodic_phase,
            experimental.sidereal_phase,
            experimental.anomalistic_phase,
            experimental.draconic_phase,
            experimental.metonic_phase,
            experimental.saros_phase,
            experimental.exeligmos_phase,
        );
    }
}
