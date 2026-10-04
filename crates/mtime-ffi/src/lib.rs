use mtime_antikythera::{AntikytheraMachine, J2000_JD_TT};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MTimeCStateV1 {
    pub valid: u8,
    pub profile: u32,
    pub linear_ns_hi: i64,
    pub linear_ns_lo: u64,
    pub jd_tt: f64,
    pub sun_deg: f64,
    pub moon_deg: f64,
    pub phase_deg: f64,
    pub node_deg: f64,
    pub solar_year_phase: f64,
    pub synodic_phase: f64,
    pub sidereal_phase: f64,
    pub anomalistic_phase: f64,
    pub draconic_phase: f64,
    pub metonic_phase: f64,
    pub saros_phase: f64,
    pub exeligmos_phase: f64,
}

impl MTimeCStateV1 {
    fn invalid() -> Self {
        Self {
            valid: 0,
            profile: 0,
            linear_ns_hi: 0,
            linear_ns_lo: 0,
            jd_tt: f64::NAN,
            sun_deg: f64::NAN,
            moon_deg: f64::NAN,
            phase_deg: f64::NAN,
            node_deg: f64::NAN,
            solar_year_phase: f64::NAN,
            synodic_phase: f64::NAN,
            sidereal_phase: f64::NAN,
            anomalistic_phase: f64::NAN,
            draconic_phase: f64::NAN,
            metonic_phase: f64::NAN,
            saros_phase: f64::NAN,
            exeligmos_phase: f64::NAN,
        }
    }
}

/// C ABI v1: compute Software Antikythera state from TT Julian Date.
/// experimental_v2 == 0 selects Digital V1; non-zero selects V2 Experimental.
/// The function returns by value and uses no caller-provided raw pointers.
#[no_mangle]
pub extern "C" fn mtime_state_v1_from_tt(jd_tt: f64, experimental_v2: u8) -> MTimeCStateV1 {
    if !jd_tt.is_finite() {
        return MTimeCStateV1::invalid();
    }
    let machine = if experimental_v2 == 0 {
        AntikytheraMachine::digital()
    } else {
        AntikytheraMachine::digital_v2_experimental()
    };
    let Ok(state) = machine.state_at_tt(jd_tt) else {
        return MTimeCStateV1::invalid();
    };
    let linear = ((jd_tt - J2000_JD_TT) * 86_400_000_000_000.0).round() as i128;
    MTimeCStateV1 {
        valid: 1,
        profile: if experimental_v2 == 0 { 1 } else { 2 },
        linear_ns_hi: (linear >> 64) as i64,
        linear_ns_lo: linear as u64,
        jd_tt,
        sun_deg: state.solar_longitude.angle_deg,
        moon_deg: state.lunar_longitude.angle_deg,
        phase_deg: state.lunar_phase.angle_deg,
        node_deg: state.lunar_node.angle_deg,
        solar_year_phase: state.solar_year_phase,
        synodic_phase: state.synodic_phase,
        sidereal_phase: state.sidereal_phase,
        anomalistic_phase: state.anomalistic_phase,
        draconic_phase: state.draconic_phase,
        metonic_phase: state.metonic_phase,
        saros_phase: state.saros_phase,
        exeligmos_phase: state.exeligmos_phase,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_abi_returns_valid_j2000_state_without_raw_pointers() {
        let s = mtime_state_v1_from_tt(J2000_JD_TT, 0);
        assert_eq!(s.valid, 1);
        assert_eq!(s.profile, 1);
        assert_eq!(s.linear_ns_hi, 0);
        assert_eq!(s.linear_ns_lo, 0);
    }

    #[test]
    fn c_abi_exposes_experimental_profile_separately() {
        let a = mtime_state_v1_from_tt(J2000_JD_TT + 10_000.0, 0);
        let b = mtime_state_v1_from_tt(J2000_JD_TT + 10_000.0, 1);
        assert_eq!(b.profile, 2);
        assert_ne!(a.moon_deg, b.moon_deg);
        assert_eq!(a.sun_deg, b.sun_deg);
    }
}
