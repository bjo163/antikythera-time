use mtime_core::TemporalError;
use mtime_temporal::{MTimeEngine, MTimeState};
use mtime_timescales::UtcInstant;
use serde::Serialize;

pub const MCLOCK_PACKET_VERSION: &str = "MCLOCK-1";
pub const MCLOCK_STATUS: &str = "RESEARCH_PROTOTYPE";

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MClockPacket {
    pub packet_version: String,
    pub status: String,
    pub mtime_state_version: String,
    pub profile_id: String,
    pub linear_si_nanoseconds_from_j2000_tt: String,
    pub tt_jd: f64,
    pub tdb_jd: f64,
    pub tdb_minus_tt_seconds: f64,
    pub uncertainty_seconds: f64,
    pub solar_longitude_deg: f64,
    pub lunar_longitude_deg: f64,
    pub lunar_phase_deg: f64,
    pub lunar_node_deg: f64,
    pub solar_year_phase: f64,
    pub synodic_phase: f64,
    pub sidereal_phase: f64,
    pub anomalistic_phase: f64,
    pub draconic_phase: f64,
    pub metonic_phase: f64,
    pub saros_phase: f64,
    pub exeligmos_phase: f64,
}

impl MClockPacket {
    #[must_use]
    pub fn from_state(state: MTimeState) -> Self {
        Self {
            packet_version: MCLOCK_PACKET_VERSION.into(),
            status: MCLOCK_STATUS.into(),
            mtime_state_version: state.state_version.into(),
            profile_id: state.profile.id().into(),
            linear_si_nanoseconds_from_j2000_tt: state.linear_si_nanoseconds_from_j2000_tt.to_string(),
            tt_jd: state.reference.tt_jd,
            tdb_jd: state.reference.tdb_jd,
            tdb_minus_tt_seconds: state.reference.tdb_minus_tt_seconds,
            uncertainty_seconds: state.reference.uncertainty_seconds,
            solar_longitude_deg: state.cycles.solar_longitude.angle_deg,
            lunar_longitude_deg: state.cycles.lunar_longitude.angle_deg,
            lunar_phase_deg: state.cycles.lunar_phase.angle_deg,
            lunar_node_deg: state.cycles.lunar_node.angle_deg,
            solar_year_phase: state.cycles.solar_year_phase,
            synodic_phase: state.cycles.synodic_phase,
            sidereal_phase: state.cycles.sidereal_phase,
            anomalistic_phase: state.cycles.anomalistic_phase,
            draconic_phase: state.cycles.draconic_phase,
            metonic_phase: state.cycles.metonic_phase,
            saros_phase: state.cycles.saros_phase,
            exeligmos_phase: state.cycles.exeligmos_phase,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    #[must_use]
    pub fn render_text(&self) -> String {
        format!(
            "M-CLOCK {} [{}]\nprofile      {}\nTT JD        {:.9}\nTDB JD       {:.9}\nSun dial     {:9.4} deg\nMoon dial    {:9.4} deg\nLunar phase {:9.4} deg\nNode dial    {:9.4} deg\nMetonic      {:.9}\nSaros        {:.9}\nExeligmos    {:.9}\n",
            self.packet_version,self.status,self.profile_id,self.tt_jd,self.tdb_jd,
            self.solar_longitude_deg,self.lunar_longitude_deg,self.lunar_phase_deg,
            self.lunar_node_deg,self.metonic_phase,self.saros_phase,self.exeligmos_phase
        )
    }
}

pub fn digital_packet_from_utc(utc: UtcInstant) -> Result<MClockPacket, TemporalError> {
    Ok(MClockPacket::from_state(MTimeEngine::digital().from_utc(utc)?))
}

pub fn historical_packet_from_utc(utc: UtcInstant) -> Result<MClockPacket, TemporalError> {
    Ok(MClockPacket::from_state(MTimeEngine::historical().from_utc(utc)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mtime_timescales::J2000_UTC_UNIX_SECONDS;

    fn j2000() -> UtcInstant {
        let whole = J2000_UTC_UNIX_SECONDS.floor() as i64;
        let nanos = ((J2000_UTC_UNIX_SECONDS - whole as f64) * 1e9).round() as u32;
        UtcInstant { unix_seconds: whole, nanoseconds: nanos }
    }

    #[test]
    fn packet_is_deterministic_at_j2000() {
        let p = digital_packet_from_utc(j2000()).unwrap();
        assert_eq!(p.packet_version, "MCLOCK-1");
        assert_eq!(p.mtime_state_version, "MTS-1");
        assert_eq!(p.linear_si_nanoseconds_from_j2000_tt, "0");
        assert_eq!(p.profile_id, "MTIME_DIGITAL_ANTIKYTHERA_V1");
    }

    #[test]
    fn json_keeps_linear_coordinate_as_string() {
        let json = digital_packet_from_utc(j2000()).unwrap().to_json().unwrap();
        assert!(json.contains("\"linear_si_nanoseconds_from_j2000_tt\": \"0\""));
    }

    #[test]
    fn historical_and_digital_share_linear_time() {
        let utc = UtcInstant { unix_seconds: 1_767_225_600, nanoseconds: 0 };
        let h = historical_packet_from_utc(utc).unwrap();
        let d = digital_packet_from_utc(utc).unwrap();
        assert_eq!(h.linear_si_nanoseconds_from_j2000_tt,d.linear_si_nanoseconds_from_j2000_tt);
        assert_ne!(h.lunar_longitude_deg,d.lunar_longitude_deg);
    }
}
