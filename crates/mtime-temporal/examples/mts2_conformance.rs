use mtime_core::{CoordinateTime, Tt};
use mtime_temporal::{MTimeEngine, MTimeWireV2};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn main() {
    let epochs = [
        2_415_020.5, // 1900-01-01-ish
        2_433_282.5, // 1950
        2_451_545.0, // J2000
        2_469_807.5, // 2050
        2_488_069.5, // 2100
    ];
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
    }
}
