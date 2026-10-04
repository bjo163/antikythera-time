use mtime_antikythera::{
    AntikytheraMachine, AntikytheraState, MachineProfile, J2000_JD_TT,
};
use mtime_core::{CoordinateTime, TemporalError, Tt};
use mtime_timescales::{
    tt_to_tdb, utc_to_tt, DtrProvider, NasaSimpleDtr, UtcInstant,
};

pub const MTIME_STATE_VERSION: &str = "MTS-1";
pub const SI_NANOSECONDS_PER_DAY: f64 = 86_400.0 * 1_000_000_000.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReferenceTimes {
    pub tt_jd: f64,
    pub tdb_jd: f64,
    pub tdb_minus_tt_seconds: f64,
    pub uncertainty_seconds: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MTimeState {
    pub state_version: &'static str,
    pub profile: MachineProfile,
    pub linear_si_nanoseconds_from_j2000_tt: i128,
    pub reference: ReferenceTimes,
    pub cycles: AntikytheraState,
}

impl MTimeState {
    #[must_use]
    pub fn cycle_vector(self) -> [f64; 8] {
        [
            self.cycles.solar_year_phase,
            self.cycles.synodic_phase,
            self.cycles.sidereal_phase,
            self.cycles.anomalistic_phase,
            self.cycles.draconic_phase,
            self.cycles.metonic_phase,
            self.cycles.saros_phase,
            self.cycles.exeligmos_phase,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MTimeEngine {
    pub machine: AntikytheraMachine,
}

impl MTimeEngine {
    #[must_use]
    pub const fn historical() -> Self {
        Self {
            machine: AntikytheraMachine::historical(),
        }
    }

    #[must_use]
    pub const fn digital() -> Self {
        Self {
            machine: AntikytheraMachine::digital(),
        }
    }

    pub fn from_utc(self, utc: UtcInstant) -> Result<MTimeState, TemporalError> {
        let tt = utc_to_tt(utc)?;
        self.from_tt(tt)
    }

    pub fn from_tt(self, tt: CoordinateTime<Tt>) -> Result<MTimeState, TemporalError> {
        let dtr = NasaSimpleDtr.dtr(&tt)?;
        let tdb = tt_to_tdb(&tt, dtr)?;
        let cycles = self
            .machine
            .state_at_tt(tt.jd())
            .map_err(|_| TemporalError::InvalidInput("Antikythera machine state"))?;
        let linear_si_nanoseconds_from_j2000_tt =
            ((tt.jd() - J2000_JD_TT) * SI_NANOSECONDS_PER_DAY).round() as i128;

        Ok(MTimeState {
            state_version: MTIME_STATE_VERSION,
            profile: self.machine.profile,
            linear_si_nanoseconds_from_j2000_tt,
            reference: ReferenceTimes {
                tt_jd: tt.jd(),
                tdb_jd: tdb.jd(),
                tdb_minus_tt_seconds: dtr.seconds,
                uncertainty_seconds: tdb.uncertainty_seconds(),
            },
            cycles,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mtime_core::CoordinateTime;
    use mtime_timescales::J2000_UTC_UNIX_SECONDS;

    #[test]
    fn j2000_utc_maps_to_zero_linear_coordinate() {
        let whole = J2000_UTC_UNIX_SECONDS.floor() as i64;
        let nanos = ((J2000_UTC_UNIX_SECONDS - whole as f64) * 1e9).round() as u32;
        let state = MTimeEngine::digital()
            .from_utc(UtcInstant {
                unix_seconds: whole,
                nanoseconds: nanos,
            })
            .unwrap();
        assert_eq!(state.linear_si_nanoseconds_from_j2000_tt, 0);
        assert_eq!(state.state_version, "MTS-1");
        assert_eq!(state.profile, MachineProfile::MTimeDigitalV1);
    }

    #[test]
    fn one_tt_day_is_one_si_day_in_linear_coordinate() {
        let a = MTimeEngine::digital()
            .from_tt(CoordinateTime::<Tt>::new(J2000_JD_TT, 0.0, 0.0).unwrap())
            .unwrap();
        let b = MTimeEngine::digital()
            .from_tt(CoordinateTime::<Tt>::new(J2000_JD_TT, 1.0, 0.0).unwrap())
            .unwrap();
        assert_eq!(
            b.linear_si_nanoseconds_from_j2000_tt - a.linear_si_nanoseconds_from_j2000_tt,
            86_400_000_000_000_i128
        );
    }

    #[test]
    fn linear_coordinate_is_profile_independent_but_cycles_are_not() {
        let tt = CoordinateTime::<Tt>::new(J2000_JD_TT, 9_500.0, 0.0).unwrap();
        let historical = MTimeEngine::historical().from_tt(tt).unwrap();
        let digital = MTimeEngine::digital().from_tt(tt).unwrap();
        assert_eq!(
            historical.linear_si_nanoseconds_from_j2000_tt,
            digital.linear_si_nanoseconds_from_j2000_tt
        );
        assert_ne!(
            historical.cycles.solar_longitude.angle_deg,
            digital.cycles.solar_longitude.angle_deg
        );
    }

    #[test]
    fn cycle_vector_is_normalized() {
        let state = MTimeEngine::digital()
            .from_tt(CoordinateTime::<Tt>::new(J2000_JD_TT, 1234.5, 0.0).unwrap())
            .unwrap();
        assert!(state
            .cycle_vector()
            .iter()
            .all(|phase| (0.0..1.0).contains(phase)));
    }
}


pub const MTIME_WIRE_VERSION: &str = "MTS-2";

#[derive(Debug, Clone, PartialEq)]
pub struct MTimeWireV2 {
    pub profile_id: String,
    pub linear_si_nanoseconds_from_j2000_tt: i128,
    pub tt_jd: f64,
    pub tdb_jd: f64,
    pub reference_uncertainty_seconds: f64,
    pub dial_deg: [f64; 4],
    pub cycle_phase: [f64; 8],
}

impl MTimeWireV2 {
    #[must_use]
    pub fn from_state(state: MTimeState) -> Self {
        Self {
            profile_id: state.profile.id().to_string(),
            linear_si_nanoseconds_from_j2000_tt: state.linear_si_nanoseconds_from_j2000_tt,
            tt_jd: state.reference.tt_jd,
            tdb_jd: state.reference.tdb_jd,
            reference_uncertainty_seconds: state.reference.uncertainty_seconds,
            dial_deg: [
                state.cycles.solar_longitude.angle_deg,
                state.cycles.lunar_longitude.angle_deg,
                state.cycles.lunar_phase.angle_deg,
                state.cycles.lunar_node.angle_deg,
            ],
            cycle_phase: state.cycle_vector(),
        }
    }

    #[must_use]
    pub fn instant_key(&self) -> MTimeInstantKey {
        MTimeInstantKey(self.linear_si_nanoseconds_from_j2000_tt)
    }

    #[must_use]
    pub fn to_canonical_json(&self) -> String {
        let cycles = self.cycle_phase.iter().map(|v| format!("{v:.15}")).collect::<Vec<_>>().join(",");
        let dials = self.dial_deg.iter().map(|v| format!("{v:.15}")).collect::<Vec<_>>().join(",");
        format!(
            "{{\"schema\":\"MTS-2\",\"profile_id\":\"{}\",\"linear_si_nanoseconds_from_j2000_tt\":\"{}\",\"tt_jd\":{:.15},\"tdb_jd\":{:.15},\"reference_uncertainty_seconds\":{:.15},\"dial_deg\":[{}],\"cycle_phase\":[{}]}}",
            self.profile_id,
            self.linear_si_nanoseconds_from_j2000_tt,
            self.tt_jd,
            self.tdb_jd,
            self.reference_uncertainty_seconds,
            dials,
            cycles
        )
    }

    #[must_use]
    pub fn encode_binary(&self) -> Vec<u8> {
        let profile = self.profile_id.as_bytes();
        assert!(profile.len() <= u16::MAX as usize);
        let mut out = Vec::with_capacity(4 + 2 + profile.len() + 16 + 8 * 15);
        out.extend_from_slice(b"MTS2");
        out.extend_from_slice(&(profile.len() as u16).to_be_bytes());
        out.extend_from_slice(profile);
        out.extend_from_slice(&self.linear_si_nanoseconds_from_j2000_tt.to_be_bytes());
        for value in [
            self.tt_jd,
            self.tdb_jd,
            self.reference_uncertainty_seconds,
        ] {
            out.extend_from_slice(&value.to_bits().to_be_bytes());
        }
        for value in self.dial_deg {
            out.extend_from_slice(&value.to_bits().to_be_bytes());
        }
        for value in self.cycle_phase {
            out.extend_from_slice(&value.to_bits().to_be_bytes());
        }
        out
    }

    pub fn decode_binary(bytes: &[u8]) -> Result<Self, TemporalError> {
        if bytes.len() < 6 || &bytes[..4] != b"MTS2" {
            return Err(TemporalError::InvalidInput("MTS-2 magic"));
        }
        let profile_len = u16::from_be_bytes([bytes[4], bytes[5]]) as usize;
        let expected = 4 + 2 + profile_len + 16 + (3 + 4 + 8) * 8;
        if bytes.len() != expected {
            return Err(TemporalError::InvalidInput("MTS-2 length"));
        }
        let mut cursor = 6;
        let profile_id = core::str::from_utf8(&bytes[cursor..cursor + profile_len])
            .map_err(|_| TemporalError::InvalidInput("MTS-2 profile UTF-8"))?
            .to_string();
        cursor += profile_len;

        let mut i128_bytes = [0u8; 16];
        i128_bytes.copy_from_slice(&bytes[cursor..cursor + 16]);
        let linear = i128::from_be_bytes(i128_bytes);
        cursor += 16;

        fn take_f64(bytes: &[u8], cursor: &mut usize) -> f64 {
            let mut raw = [0u8; 8];
            raw.copy_from_slice(&bytes[*cursor..*cursor + 8]);
            *cursor += 8;
            f64::from_bits(u64::from_be_bytes(raw))
        }

        let tt_jd = take_f64(bytes, &mut cursor);
        let tdb_jd = take_f64(bytes, &mut cursor);
        let reference_uncertainty_seconds = take_f64(bytes, &mut cursor);
        let mut dial_deg = [0.0; 4];
        for value in &mut dial_deg {
            *value = take_f64(bytes, &mut cursor);
        }
        let mut cycle_phase = [0.0; 8];
        for value in &mut cycle_phase {
            *value = take_f64(bytes, &mut cursor);
        }
        if !tt_jd.is_finite()
            || !tdb_jd.is_finite()
            || !reference_uncertainty_seconds.is_finite()
            || !dial_deg.iter().all(|v| v.is_finite())
            || !cycle_phase.iter().all(|v| v.is_finite() && (0.0..1.0).contains(v))
        {
            return Err(TemporalError::InvalidInput("MTS-2 non-finite/phase"));
        }
        Ok(Self {
            profile_id,
            linear_si_nanoseconds_from_j2000_tt: linear,
            tt_jd,
            tdb_jd,
            reference_uncertainty_seconds,
            dial_deg,
            cycle_phase,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MTimeInstantKey(pub i128);

#[cfg(test)]
mod mts2_tests {
    use super::*;

    #[test]
    fn mts2_binary_round_trip_is_lossless() {
        let state = MTimeEngine::digital()
            .from_tt(CoordinateTime::<Tt>::new(J2000_JD_TT, 12_345.678, 0.0).unwrap())
            .unwrap();
        let wire = MTimeWireV2::from_state(state);
        let decoded = MTimeWireV2::decode_binary(&wire.encode_binary()).unwrap();
        assert_eq!(wire, decoded);
        assert_eq!(wire.instant_key(), decoded.instant_key());
    }

    #[test]
    fn mts2_json_keeps_i128_as_decimal_string() {
        let state = MTimeEngine::digital()
            .from_tt(CoordinateTime::<Tt>::new(J2000_JD_TT, 1.0, 0.0).unwrap())
            .unwrap();
        let json = MTimeWireV2::from_state(state).to_canonical_json();
        assert!(json.contains("\"schema\":\"MTS-2\""));
        assert!(json.contains("\"linear_si_nanoseconds_from_j2000_tt\":\"86400000000000\""));
    }

    #[test]
    fn instant_order_is_linear_coordinate_order() {
        let a = MTimeInstantKey(-1);
        let b = MTimeInstantKey(0);
        let c = MTimeInstantKey(1);
        assert!(a < b && b < c);
    }
}
