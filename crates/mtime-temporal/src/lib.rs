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
