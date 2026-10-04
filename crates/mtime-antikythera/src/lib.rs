use std::f64::consts::PI;

pub const J2000_JD_TT: f64 = 2_451_545.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceLabel {
    SurvivingEvidence,
    StronglyIndicatedReconstruction,
    ReconstructedModel,
    Hypothetical,
    ModernReference,
    ModernDigitalCorrection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cycle {
    pub id: &'static str,
    pub period_days: f64,
    pub evidence: EvidenceLabel,
}

impl Cycle {
    #[must_use]
    pub fn phase(self, elapsed_days: f64) -> f64 {
        elapsed_days.rem_euclid(self.period_days) / self.period_days
    }

    #[must_use]
    pub fn phase_angle_deg(self, elapsed_days: f64, epoch_angle_deg: f64) -> f64 {
        wrap_deg(epoch_angle_deg + self.phase(elapsed_days) * 360.0)
    }

    #[must_use]
    pub fn recurrence_jd(self, seed_jd: f64, cycles: i32) -> f64 {
        seed_jd + f64::from(cycles) * self.period_days
    }
}

pub const TROPICAL_YEAR: Cycle = Cycle {
    id: "TROPICAL_YEAR",
    period_days: 365.242_189_7,
    evidence: EvidenceLabel::ModernReference,
};
pub const MEAN_SYNODIC_MONTH: Cycle = Cycle {
    id: "MEAN_SYNODIC_MONTH",
    period_days: 29.530_588_853,
    evidence: EvidenceLabel::ModernReference,
};
pub const SIDEREAL_MONTH: Cycle = Cycle {
    id: "SIDEREAL_MONTH",
    period_days: 27.321_661_547,
    evidence: EvidenceLabel::ModernReference,
};
pub const ANOMALISTIC_MONTH: Cycle = Cycle {
    id: "ANOMALISTIC_MONTH",
    period_days: 27.554_549_88,
    evidence: EvidenceLabel::ModernReference,
};
pub const DRACONIC_MONTH: Cycle = Cycle {
    id: "DRACONIC_MONTH",
    period_days: 27.212_220_817,
    evidence: EvidenceLabel::ModernReference,
};
pub const METONIC_CYCLE: Cycle = Cycle {
    id: "METONIC_19_YEARS",
    period_days: 6_939.688,
    evidence: EvidenceLabel::SurvivingEvidence,
};
pub const SAROS: Cycle = Cycle {
    id: "SAROS_223_SYNODIC_MONTHS",
    period_days: 6_585.322_3,
    evidence: EvidenceLabel::SurvivingEvidence,
};
pub const EXELIGMOS: Cycle = Cycle {
    id: "EXELIGMOS_3_SAROS",
    period_days: 19_755.966_9,
    evidence: EvidenceLabel::SurvivingEvidence,
};
pub const NODE_REGRESSION: Cycle = Cycle {
    id: "LUNAR_NODE_REGRESSION",
    period_days: 6_798.383,
    evidence: EvidenceLabel::ModernReference,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RationalRelation {
    pub numerator: u64,
    pub denominator: u64,
}

impl RationalRelation {
    #[must_use]
    pub const fn new(numerator: u64, denominator: u64) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    #[must_use]
    pub fn ratio(self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    #[must_use]
    pub const fn compose(self, other: Self) -> Self {
        Self {
            numerator: self.numerator.saturating_mul(other.numerator),
            denominator: self.denominator.saturating_mul(other.denominator),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignedRelation {
    pub numerator: i64,
    pub denominator: u64,
    pub evidence: EvidenceLabel,
}

impl SignedRelation {
    #[must_use]
    pub const fn new(numerator: i64, denominator: u64, evidence: EvidenceLabel) -> Self {
        Self {
            numerator,
            denominator,
            evidence,
        }
    }

    #[must_use]
    pub fn ratio(self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

pub const METONIC_LUNATIONS_PER_YEARS: RationalRelation = RationalRelation::new(235, 19);
pub const SIDEREAL_MONTHS_PER_SYNODIC_MONTHS: RationalRelation = RationalRelation::new(254, 235);
pub const SAROS_SYNODIC_MONTHS: u16 = 223;
pub const NODE_TURNS_PER_SAROS_RECONSTRUCTION: SignedRelation =
    SignedRelation::new(-12, 223, EvidenceLabel::StronglyIndicatedReconstruction);
pub const NODE_COMPACT_RELATION_2021_MODEL: SignedRelation =
    SignedRelation::new(-5, 93, EvidenceLabel::ReconstructedModel);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VirtualGear {
    pub id: &'static str,
    pub teeth: u32,
    pub evidence: EvidenceLabel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GearMesh {
    pub driver: VirtualGear,
    pub driven: VirtualGear,
    pub external_mesh: bool,
}

impl GearMesh {
    #[must_use]
    pub fn ratio(self) -> f64 {
        let magnitude = f64::from(self.driver.teeth) / f64::from(self.driven.teeth);
        if self.external_mesh {
            -magnitude
        } else {
            magnitude
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GearTrain {
    pub id: &'static str,
    pub stages: Vec<GearMesh>,
    pub evidence: EvidenceLabel,
}

impl GearTrain {
    #[must_use]
    pub fn total_ratio(&self) -> f64 {
        self.stages.iter().fold(1.0, |acc, stage| acc * stage.ratio())
    }

    #[must_use]
    pub fn output_turns(&self, input_turns: f64) -> f64 {
        input_turns * self.total_ratio()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AntikytheraComponent {
    pub id: &'static str,
    pub label: &'static str,
    pub evidence: EvidenceLabel,
}

pub const EVIDENCE_MANIFEST: &[AntikytheraComponent] = &[
    AntikytheraComponent {
        id: "metonic",
        label: "Metonic dial",
        evidence: EvidenceLabel::SurvivingEvidence,
    },
    AntikytheraComponent {
        id: "saros",
        label: "Saros eclipse dial",
        evidence: EvidenceLabel::SurvivingEvidence,
    },
    AntikytheraComponent {
        id: "lunar-anomaly",
        label: "Lunar anomaly mechanism",
        evidence: EvidenceLabel::SurvivingEvidence,
    },
    AntikytheraComponent {
        id: "planet-inscriptions",
        label: "Five classical planets in front-cosmos inscriptions",
        evidence: EvidenceLabel::SurvivingEvidence,
    },
    AntikytheraComponent {
        id: "superior-planets",
        label: "Mars/Jupiter/Saturn gearing",
        evidence: EvidenceLabel::ReconstructedModel,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineProfile {
    HistoricalReconstructionV1,
    MTimeDigitalV1,
}

impl MachineProfile {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::HistoricalReconstructionV1 => "ANTIKYTHERA_HISTORICAL_RECONSTRUCTION_V1",
            Self::MTimeDigitalV1 => "MTIME_DIGITAL_ANTIKYTHERA_V1",
        }
    }

    #[must_use]
    pub const fn evidence(self) -> EvidenceLabel {
        match self {
            Self::HistoricalReconstructionV1 => EvidenceLabel::ReconstructedModel,
            Self::MTimeDigitalV1 => EvidenceLabel::ModernDigitalCorrection,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DialState {
    pub phase: f64,
    pub angle_deg: f64,
}

impl DialState {
    #[must_use]
    pub fn new(angle_deg: f64) -> Self {
        let angle_deg = wrap_deg(angle_deg);
        Self {
            phase: angle_deg / 360.0,
            angle_deg,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AntikytheraState {
    pub profile: MachineProfile,
    pub jd_tt: f64,
    pub elapsed_days_from_epoch: f64,
    pub solar_longitude: DialState,
    pub lunar_longitude: DialState,
    pub lunar_phase: DialState,
    pub lunar_node: DialState,
    pub solar_year_phase: f64,
    pub synodic_phase: f64,
    pub sidereal_phase: f64,
    pub anomalistic_phase: f64,
    pub draconic_phase: f64,
    pub metonic_phase: f64,
    pub saros_phase: f64,
    pub exeligmos_phase: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineError {
    NonFiniteInstant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AntikytheraMachine {
    pub epoch_jd_tt: f64,
    pub profile: MachineProfile,
}

impl AntikytheraMachine {
    #[must_use]
    pub const fn new(epoch_jd_tt: f64, profile: MachineProfile) -> Self {
        Self {
            epoch_jd_tt,
            profile,
        }
    }

    #[must_use]
    pub const fn historical() -> Self {
        Self::new(J2000_JD_TT, MachineProfile::HistoricalReconstructionV1)
    }

    #[must_use]
    pub const fn digital() -> Self {
        Self::new(J2000_JD_TT, MachineProfile::MTimeDigitalV1)
    }

    pub fn state_at_tt(&self, jd_tt: f64) -> Result<AntikytheraState, MachineError> {
        if !jd_tt.is_finite() || !self.epoch_jd_tt.is_finite() {
            return Err(MachineError::NonFiniteInstant);
        }

        let elapsed = jd_tt - self.epoch_jd_tt;
        let solar_longitude_deg = match self.profile {
            MachineProfile::HistoricalReconstructionV1 => historical_solar_longitude_deg(elapsed),
            MachineProfile::MTimeDigitalV1 => digital_solar_longitude_deg(elapsed),
        };
        let lunar_longitude_deg = match self.profile {
            MachineProfile::HistoricalReconstructionV1 => historical_lunar_longitude_deg(elapsed),
            MachineProfile::MTimeDigitalV1 => digital_lunar_longitude_deg(elapsed),
        };
        let phase_angle = wrap_deg(lunar_longitude_deg - solar_longitude_deg);
        let node_longitude = NODE_REGRESSION.phase_angle_deg(elapsed, 125.044_52);

        Ok(AntikytheraState {
            profile: self.profile,
            jd_tt,
            elapsed_days_from_epoch: elapsed,
            solar_longitude: DialState::new(solar_longitude_deg),
            lunar_longitude: DialState::new(lunar_longitude_deg),
            lunar_phase: DialState::new(phase_angle),
            lunar_node: DialState::new(node_longitude),
            solar_year_phase: TROPICAL_YEAR.phase(elapsed),
            synodic_phase: wrap_deg(297.850_192_1 + elapsed * 360.0 / MEAN_SYNODIC_MONTH.period_days)
                / 360.0,
            sidereal_phase: wrap_deg(218.316_447_7 + elapsed * 360.0 / SIDEREAL_MONTH.period_days)
                / 360.0,
            anomalistic_phase: wrap_deg(
                134.963_396_4 + elapsed * 360.0 / ANOMALISTIC_MONTH.period_days,
            ) / 360.0,
            draconic_phase: wrap_deg(
                93.272_095 + elapsed * 360.0 / DRACONIC_MONTH.period_days,
            ) / 360.0,
            metonic_phase: METONIC_CYCLE.phase(elapsed),
            saros_phase: SAROS.phase(elapsed),
            exeligmos_phase: EXELIGMOS.phase(elapsed),
        })
    }
}

#[must_use]
pub fn historical_solar_longitude_deg(elapsed_days_from_j2000: f64) -> f64 {
    wrap_deg(280.466_46 + 0.985_647_36 * elapsed_days_from_j2000)
}

#[must_use]
pub fn historical_lunar_longitude_deg(elapsed_days_from_j2000: f64) -> f64 {
    let mean = wrap_deg(218.316_447_7 + 13.176_396_48 * elapsed_days_from_j2000);
    let anomaly = wrap_deg(134.963_396_4 + 13.064_992_95 * elapsed_days_from_j2000);
    // The surviving mechanism contains lunar-anomaly gearing. The numerical
    // amplitude here is a modernized reconstruction parameter, not a claim
    // that an exact ancient coefficient survives.
    wrap_deg(mean + 6.289 * sin_deg(anomaly))
}

#[must_use]
pub fn digital_solar_longitude_deg(elapsed_days_from_j2000: f64) -> f64 {
    let t = elapsed_days_from_j2000 / 36_525.0;
    let mean = wrap_deg(280.466_46 + 0.985_647_36 * elapsed_days_from_j2000);
    let anomaly = wrap_deg(357.529_11 + 0.985_600_28 * elapsed_days_from_j2000);
    let center = (1.914_602 - 0.004_817 * t - 0.000_014 * t * t) * sin_deg(anomaly)
        + (0.019_993 - 0.000_101 * t) * sin_deg(2.0 * anomaly)
        + 0.000_289 * sin_deg(3.0 * anomaly);
    wrap_deg(mean + center)
}

#[must_use]
pub fn digital_lunar_longitude_deg(elapsed_days_from_j2000: f64) -> f64 {
    let d = elapsed_days_from_j2000;
    let l = wrap_deg(218.316_447_7 + 13.176_396_48 * d);
    let m_moon = wrap_deg(134.963_396_4 + 13.064_992_95 * d);
    let elongation = wrap_deg(297.850_192_1 + 12.190_749_12 * d);
    let f = wrap_deg(93.272_095 + 13.229_350_24 * d);
    let m_sun = wrap_deg(357.529_11 + 0.985_600_28 * d);

    let correction = 6.289 * sin_deg(m_moon)
        + 1.274 * sin_deg(2.0 * elongation - m_moon)
        + 0.658 * sin_deg(2.0 * elongation)
        + 0.214 * sin_deg(2.0 * m_moon)
        - 0.186 * sin_deg(m_sun)
        - 0.059 * sin_deg(2.0 * elongation - 2.0 * m_moon)
        - 0.057 * sin_deg(2.0 * elongation - m_sun - m_moon)
        + 0.053 * sin_deg(2.0 * elongation + m_moon)
        + 0.046 * sin_deg(2.0 * elongation - m_sun)
        + 0.041 * sin_deg(m_sun - m_moon)
        - 0.035 * sin_deg(elongation)
        - 0.031 * sin_deg(m_sun + m_moon)
        - 0.015 * sin_deg(2.0 * f - 2.0 * elongation)
        + 0.011 * sin_deg(2.0 * elongation - 4.0 * m_moon);

    wrap_deg(l + correction)
}

#[must_use]
pub fn shortest_angle_deg(a_deg: f64, b_deg: f64) -> f64 {
    ((a_deg - b_deg + 180.0).rem_euclid(360.0) - 180.0).abs()
}

#[must_use]
pub fn wrap_deg(value: f64) -> f64 {
    value.rem_euclid(360.0)
}

fn sin_deg(value: f64) -> f64 {
    (value * PI / 180.0).sin()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saros_recurrence() {
        assert!((SAROS.recurrence_jd(2_460_409.263, 1) - 2_466_994.585_3).abs() < 1e-9);
    }

    #[test]
    fn exeligmos_is_three_saros() {
        assert!((EXELIGMOS.period_days - 3.0 * SAROS.period_days).abs() < 1e-9);
    }

    #[test]
    fn reconstruction_is_labelled() {
        assert_eq!(
            EVIDENCE_MANIFEST[4].evidence,
            EvidenceLabel::ReconstructedModel
        );
    }

    #[test]
    fn historical_period_relations_are_explicit() {
        assert_eq!(METONIC_LUNATIONS_PER_YEARS, RationalRelation::new(235, 19));
        assert_eq!(
            SIDEREAL_MONTHS_PER_SYNODIC_MONTHS,
            RationalRelation::new(254, 235)
        );
        assert_eq!(SAROS_SYNODIC_MONTHS, 223);
        assert!((NODE_COMPACT_RELATION_2021_MODEL.ratio() + 5.0 / 93.0).abs() < 1e-15);
    }

    #[test]
    fn virtual_gear_train_composes_ratios() {
        let a = VirtualGear {
            id: "a",
            teeth: 20,
            evidence: EvidenceLabel::ReconstructedModel,
        };
        let b = VirtualGear {
            id: "b",
            teeth: 40,
            evidence: EvidenceLabel::ReconstructedModel,
        };
        let c = VirtualGear {
            id: "c",
            teeth: 10,
            evidence: EvidenceLabel::ReconstructedModel,
        };
        let train = GearTrain {
            id: "test",
            stages: vec![
                GearMesh {
                    driver: a,
                    driven: b,
                    external_mesh: true,
                },
                GearMesh {
                    driver: b,
                    driven: c,
                    external_mesh: true,
                },
            ],
            evidence: EvidenceLabel::ReconstructedModel,
        };
        assert!((train.total_ratio() - 2.0).abs() < 1e-12);
        assert!((train.output_turns(3.0) - 6.0).abs() < 1e-12);
    }

    #[test]
    fn machine_exposes_all_core_cycles() {
        let s = AntikytheraMachine::historical()
            .state_at_tt(J2000_JD_TT + 100.0)
            .unwrap();
        for phase in [
            s.solar_year_phase,
            s.synodic_phase,
            s.sidereal_phase,
            s.anomalistic_phase,
            s.draconic_phase,
            s.metonic_phase,
            s.saros_phase,
            s.exeligmos_phase,
        ] {
            assert!((0.0..1.0).contains(&phase));
        }
        assert_eq!(
            s.profile,
            MachineProfile::HistoricalReconstructionV1
        );
    }

    #[test]
    fn digital_profile_is_distinct_from_historical_reconstruction() {
        let jd = J2000_JD_TT + 9_500.0;
        let h = AntikytheraMachine::historical().state_at_tt(jd).unwrap();
        let d = AntikytheraMachine::digital().state_at_tt(jd).unwrap();
        assert!(shortest_angle_deg(h.solar_longitude.angle_deg, d.solar_longitude.angle_deg) > 0.01);
        assert!(shortest_angle_deg(h.lunar_longitude.angle_deg, d.lunar_longitude.angle_deg) > 0.01);
        assert_eq!(d.profile, MachineProfile::MTimeDigitalV1);
    }

    #[test]
    fn non_finite_instant_is_rejected() {
        assert_eq!(
            AntikytheraMachine::digital().state_at_tt(f64::NAN),
            Err(MachineError::NonFiniteInstant)
        );
    }
}
