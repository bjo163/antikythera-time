use std::f64::consts::PI;

pub const J2000_JD_TT: f64 = 2_451_545.0;
pub const DIGITAL_VALIDATED_START_JD_TT: f64 = 2_396_758.5; // 1850-01-01
pub const DIGITAL_VALIDATED_END_EXCLUSIVE_JD_TT: f64 = 2_506_331.5; // 2150-01-01

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
pub struct HistoricalSource {
    pub id: &'static str,
    pub citation: &'static str,
    pub doi: &'static str,
    pub scope: &'static str,
}

pub const HISTORICAL_SOURCES: &[HistoricalSource] = &[
    HistoricalSource {
        id: "FREETH_2006_NATURE",
        citation: "Freeth et al., Nature 444, 587-591 (2006)",
        doi: "10.1038/nature05357",
        scope: "X-ray/tomography-backed gearing, lunar anomaly mechanism, inscriptions",
    },
    HistoricalSource {
        id: "FREETH_2008_NATURE",
        citation: "Freeth et al., Nature 454, 614-617 (2008)",
        doi: "10.1038/nature07130",
        scope: "Metonic calendar, Olympiad display and Saros eclipse-prediction dial",
    },
    HistoricalSource {
        id: "FREETH_2021_SCI_REP",
        citation: "Freeth et al., Scientific Reports 11, 5821 (2021)",
        doi: "10.1038/s41598-021-84310-w",
        scope: "front-cosmos reconstruction constrained by surviving inscriptions and gear evidence",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HistoricalEvidenceRecord {
    pub component_id: &'static str,
    pub claim: &'static str,
    pub evidence: EvidenceLabel,
    pub source_ids: &'static [&'static str],
    pub uncertainty_note: &'static str,
}

pub const HISTORICAL_EVIDENCE_REGISTRY: &[HistoricalEvidenceRecord] = &[
    HistoricalEvidenceRecord {
        component_id: "metonic",
        claim: "Upper rear dial implements a 19-year Metonic calendar cycle.",
        evidence: EvidenceLabel::SurvivingEvidence,
        source_ids: &["FREETH_2008_NATURE"],
        uncertainty_note: "Dial/cycle evidence is strong; exact lost pointer mechanics remain reconstruction-dependent.",
    },
    HistoricalEvidenceRecord {
        component_id: "saros",
        claim: "Lower rear dial implements a 223-lunation Saros eclipse-prediction cycle.",
        evidence: EvidenceLabel::SurvivingEvidence,
        source_ids: &["FREETH_2008_NATURE"],
        uncertainty_note: "Eclipse-glyph interpretation is evidence-backed; missing fragments limit complete mechanical detail.",
    },
    HistoricalEvidenceRecord {
        component_id: "lunar-anomaly",
        claim: "A pin-and-slot / epicyclic mechanism represents lunar anomaly.",
        evidence: EvidenceLabel::SurvivingEvidence,
        source_ids: &["FREETH_2006_NATURE"],
        uncertainty_note: "Mechanism function is strongly supported; software coefficient values are not claimed to be surviving ancient numerical constants.",
    },
    HistoricalEvidenceRecord {
        component_id: "planet-inscriptions",
        claim: "Front-cosmos inscriptions refer to the five classical planets and their displayed motions.",
        evidence: EvidenceLabel::SurvivingEvidence,
        source_ids: &["FREETH_2021_SCI_REP"],
        uncertainty_note: "Inscriptions survive; much of the original front gearing does not.",
    },
    HistoricalEvidenceRecord {
        component_id: "front-cosmos-gearing-2021",
        claim: "A specific front-cosmos gearing topology can mechanize the inscription-constrained planetary periods.",
        evidence: EvidenceLabel::ReconstructedModel,
        source_ids: &["FREETH_2021_SCI_REP"],
        uncertainty_note: "This is a scholarly reconstruction model, not direct surviving evidence of every gear.",
    },
    HistoricalEvidenceRecord {
        component_id: "node-compact-5-93",
        claim: "A compact -5/93 node relation is used in a modern reconstruction model.",
        evidence: EvidenceLabel::ReconstructedModel,
        source_ids: &["FREETH_2021_SCI_REP"],
        uncertainty_note: "Model relation must remain separately labelled from surviving gear evidence.",
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoricalReconstructionProfile {
    ConservativeRearDialsV1,
    Freeth2021FrontCosmosV1,
}

impl HistoricalReconstructionProfile {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::ConservativeRearDialsV1 => "ANTIKYTHERA_CONSERVATIVE_REAR_DIALS_V1",
            Self::Freeth2021FrontCosmosV1 => "ANTIKYTHERA_FREETH_2021_FRONT_COSMOS_V1",
        }
    }

    #[must_use]
    pub const fn scope(self) -> &'static [&'static str] {
        match self {
            Self::ConservativeRearDialsV1 => &["metonic", "saros", "lunar-anomaly"],
            Self::Freeth2021FrontCosmosV1 => &[
                "metonic",
                "saros",
                "lunar-anomaly",
                "planet-inscriptions",
                "front-cosmos-gearing-2021",
                "node-compact-5-93",
            ],
        }
    }
}

#[must_use]
pub fn source_by_id(id: &str) -> Option<&'static HistoricalSource> {
    HISTORICAL_SOURCES.iter().find(|source| source.id == id)
}

#[must_use]
pub fn historical_evidence_for(component_id: &str) -> Option<&'static HistoricalEvidenceRecord> {
    HISTORICAL_EVIDENCE_REGISTRY
        .iter()
        .find(|record| record.component_id == component_id)
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineProfile {
    HistoricalReconstructionV1,
    MTimeDigitalV1,
    MTimeDigitalV2Experimental,
}

impl MachineProfile {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::HistoricalReconstructionV1 => "ANTIKYTHERA_HISTORICAL_RECONSTRUCTION_V1",
            Self::MTimeDigitalV1 => "MTIME_DIGITAL_ANTIKYTHERA_V1",
            Self::MTimeDigitalV2Experimental => "MTIME_DIGITAL_ANTIKYTHERA_V2_EXPERIMENTAL",
        }
    }

    #[must_use]
    pub const fn evidence(self) -> EvidenceLabel {
        match self {
            Self::HistoricalReconstructionV1 => EvidenceLabel::ReconstructedModel,
            Self::MTimeDigitalV1 | Self::MTimeDigitalV2Experimental => EvidenceLabel::ModernDigitalCorrection,
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
    OutsideValidatedRange,
    NoValidatedInterval,
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

    #[must_use]
    pub const fn digital_v2_experimental() -> Self {
        Self::new(J2000_JD_TT, MachineProfile::MTimeDigitalV2Experimental)
    }

    #[must_use]
    pub const fn validated_interval(&self) -> Option<(f64, f64)> {
        match self.profile {
            MachineProfile::MTimeDigitalV1 | MachineProfile::MTimeDigitalV2Experimental => {
                Some((DIGITAL_VALIDATED_START_JD_TT, DIGITAL_VALIDATED_END_EXCLUSIVE_JD_TT))
            }
            MachineProfile::HistoricalReconstructionV1 => None,
        }
    }

    pub fn state_at_tt_validated(&self, jd_tt: f64) -> Result<AntikytheraState, MachineError> {
        let Some((start, end)) = self.validated_interval() else {
            return Err(MachineError::NoValidatedInterval);
        };
        if !(start..end).contains(&jd_tt) {
            return Err(MachineError::OutsideValidatedRange);
        }
        self.state_at_tt(jd_tt)
    }

    pub fn state_at_tt(&self, jd_tt: f64) -> Result<AntikytheraState, MachineError> {
        if !jd_tt.is_finite() || !self.epoch_jd_tt.is_finite() {
            return Err(MachineError::NonFiniteInstant);
        }

        let elapsed = jd_tt - self.epoch_jd_tt;
        let solar_longitude_deg = match self.profile {
            MachineProfile::HistoricalReconstructionV1 => historical_solar_longitude_deg(elapsed),
            MachineProfile::MTimeDigitalV1 | MachineProfile::MTimeDigitalV2Experimental => {
                digital_solar_longitude_deg(elapsed)
            },
        };
        let lunar_longitude_deg = match self.profile {
            MachineProfile::HistoricalReconstructionV1 => historical_lunar_longitude_deg(elapsed),
            MachineProfile::MTimeDigitalV1 => digital_lunar_longitude_deg(elapsed),
            MachineProfile::MTimeDigitalV2Experimental => {
                digital_lunar_longitude_v2_experimental_deg(elapsed)
            },
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LunarArgument {
    MoonAnomaly,
    TwoElongationMinusMoonAnomaly,
    TwoElongation,
    TwoMoonAnomaly,
    SunAnomaly,
    TwoElongationMinusTwoMoonAnomaly,
    TwoElongationMinusSunMinusMoonAnomaly,
    TwoElongationPlusMoonAnomaly,
    TwoElongationMinusSun,
    SunMinusMoonAnomaly,
    Elongation,
    SunPlusMoonAnomaly,
    TwoLatitudeMinusTwoElongation,
    TwoElongationMinusFourMoonAnomaly,
    TwoDraconicPhase,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CorrectionTerm {
    pub id: &'static str,
    pub coefficient_deg: f64,
    pub argument: LunarArgument,
    pub evidence: EvidenceLabel,
    pub enabled_by_default: bool,
    pub provenance: &'static str,
}

pub const M8_SIN_2_DRACONIC_COEFFICIENT_DEG: f64 = -0.113_859_414_732;

pub const M8_EXPERIMENTAL_CORRECTION: CorrectionTerm = CorrectionTerm {
    id: "M8_SIN_2_DRACONIC",
    coefficient_deg: M8_SIN_2_DRACONIC_COEFFICIENT_DEG,
    argument: LunarArgument::TwoDraconicPhase,
    evidence: EvidenceLabel::ModernDigitalCorrection,
    enabled_by_default: false,
    provenance: "fit 1900-1999 monthly DE440; validated 2000-2100; M8 run 37204075486",
};

pub const DIGITAL_LUNAR_CORRECTIONS_V1: &[CorrectionTerm] = &[
    CorrectionTerm { id: "L1_MOON_ANOMALY", coefficient_deg: 6.289, argument: LunarArgument::MoonAnomaly, evidence: EvidenceLabel::ModernDigitalCorrection, enabled_by_default: true, provenance: "compact modern lunar-series baseline" },
    CorrectionTerm { id: "L2_2D_MINUS_M", coefficient_deg: 1.274, argument: LunarArgument::TwoElongationMinusMoonAnomaly, evidence: EvidenceLabel::ModernDigitalCorrection, enabled_by_default: true, provenance: "compact modern lunar-series baseline" },
    CorrectionTerm { id: "L3_2D", coefficient_deg: 0.658, argument: LunarArgument::TwoElongation, evidence: EvidenceLabel::ModernDigitalCorrection, enabled_by_default: true, provenance: "compact modern lunar-series baseline" },
    CorrectionTerm { id: "L4_2M", coefficient_deg: 0.214, argument: LunarArgument::TwoMoonAnomaly, evidence: EvidenceLabel::ModernDigitalCorrection, enabled_by_default: true, provenance: "compact modern lunar-series baseline" },
    CorrectionTerm { id: "L5_SUN_ANOMALY", coefficient_deg: -0.186, argument: LunarArgument::SunAnomaly, evidence: EvidenceLabel::ModernDigitalCorrection, enabled_by_default: true, provenance: "compact modern lunar-series baseline" },
    CorrectionTerm { id: "L6_2D_MINUS_2M", coefficient_deg: -0.059, argument: LunarArgument::TwoElongationMinusTwoMoonAnomaly, evidence: EvidenceLabel::ModernDigitalCorrection, enabled_by_default: true, provenance: "compact modern lunar-series baseline" },
    CorrectionTerm { id: "L7_2D_MINUS_MSUN_MINUS_M", coefficient_deg: -0.057, argument: LunarArgument::TwoElongationMinusSunMinusMoonAnomaly, evidence: EvidenceLabel::ModernDigitalCorrection, enabled_by_default: true, provenance: "compact modern lunar-series baseline" },
    CorrectionTerm { id: "L8_2D_PLUS_M", coefficient_deg: 0.053, argument: LunarArgument::TwoElongationPlusMoonAnomaly, evidence: EvidenceLabel::ModernDigitalCorrection, enabled_by_default: true, provenance: "compact modern lunar-series baseline" },
    CorrectionTerm { id: "L9_2D_MINUS_MSUN", coefficient_deg: 0.046, argument: LunarArgument::TwoElongationMinusSun, evidence: EvidenceLabel::ModernDigitalCorrection, enabled_by_default: true, provenance: "compact modern lunar-series baseline" },
    CorrectionTerm { id: "L10_MSUN_MINUS_M", coefficient_deg: 0.041, argument: LunarArgument::SunMinusMoonAnomaly, evidence: EvidenceLabel::ModernDigitalCorrection, enabled_by_default: true, provenance: "compact modern lunar-series baseline" },
    CorrectionTerm { id: "L11_D", coefficient_deg: -0.035, argument: LunarArgument::Elongation, evidence: EvidenceLabel::ModernDigitalCorrection, enabled_by_default: true, provenance: "compact modern lunar-series baseline" },
    CorrectionTerm { id: "L12_MSUN_PLUS_M", coefficient_deg: -0.031, argument: LunarArgument::SunPlusMoonAnomaly, evidence: EvidenceLabel::ModernDigitalCorrection, enabled_by_default: true, provenance: "compact modern lunar-series baseline" },
    CorrectionTerm { id: "L13_2F_MINUS_2D", coefficient_deg: -0.015, argument: LunarArgument::TwoLatitudeMinusTwoElongation, evidence: EvidenceLabel::ModernDigitalCorrection, enabled_by_default: true, provenance: "compact modern lunar-series baseline" },
    CorrectionTerm { id: "L14_2D_MINUS_4M", coefficient_deg: 0.011, argument: LunarArgument::TwoElongationMinusFourMoonAnomaly, evidence: EvidenceLabel::ModernDigitalCorrection, enabled_by_default: true, provenance: "compact modern lunar-series baseline" },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CorrectionSelection<'a> {
    pub disabled_ids: &'a [&'a str],
}

impl<'a> CorrectionSelection<'a> {
    #[must_use]
    pub const fn all_enabled() -> Self {
        Self { disabled_ids: &[] }
    }

    #[must_use]
    pub fn is_enabled(self, term: CorrectionTerm) -> bool {
        term.enabled_by_default && !self.disabled_ids.iter().any(|id| *id == term.id)
    }
}

#[must_use]
pub fn digital_lunar_longitude_deg(elapsed_days_from_j2000: f64) -> f64 {
    digital_lunar_longitude_with_selection(
        elapsed_days_from_j2000,
        CorrectionSelection::all_enabled(),
    )
}

#[must_use]
pub fn digital_lunar_longitude_with_selection(
    elapsed_days_from_j2000: f64,
    selection: CorrectionSelection<'_>,
) -> f64 {
    let d = elapsed_days_from_j2000;
    let l = wrap_deg(218.316_447_7 + 13.176_396_48 * d);
    let m_moon = wrap_deg(134.963_396_4 + 13.064_992_95 * d);
    let elongation = wrap_deg(297.850_192_1 + 12.190_749_12 * d);
    let f = wrap_deg(93.272_095 + 13.229_350_24 * d);
    let m_sun = wrap_deg(357.529_11 + 0.985_600_28 * d);

    let argument_deg = |argument: LunarArgument| match argument {
        LunarArgument::MoonAnomaly => m_moon,
        LunarArgument::TwoElongationMinusMoonAnomaly => 2.0 * elongation - m_moon,
        LunarArgument::TwoElongation => 2.0 * elongation,
        LunarArgument::TwoMoonAnomaly => 2.0 * m_moon,
        LunarArgument::SunAnomaly => m_sun,
        LunarArgument::TwoElongationMinusTwoMoonAnomaly => 2.0 * elongation - 2.0 * m_moon,
        LunarArgument::TwoElongationMinusSunMinusMoonAnomaly => 2.0 * elongation - m_sun - m_moon,
        LunarArgument::TwoElongationPlusMoonAnomaly => 2.0 * elongation + m_moon,
        LunarArgument::TwoElongationMinusSun => 2.0 * elongation - m_sun,
        LunarArgument::SunMinusMoonAnomaly => m_sun - m_moon,
        LunarArgument::Elongation => elongation,
        LunarArgument::SunPlusMoonAnomaly => m_sun + m_moon,
        LunarArgument::TwoLatitudeMinusTwoElongation => 2.0 * f - 2.0 * elongation,
        LunarArgument::TwoElongationMinusFourMoonAnomaly => 2.0 * elongation - 4.0 * m_moon,
        LunarArgument::TwoDraconicPhase => 2.0 * f,
    };

    let correction = DIGITAL_LUNAR_CORRECTIONS_V1
        .iter()
        .copied()
        .filter(|term| selection.is_enabled(*term))
        .map(|term| term.coefficient_deg * sin_deg(argument_deg(term.argument)))
        .sum::<f64>();

    wrap_deg(l + correction)
}


#[must_use]
pub fn digital_lunar_longitude_v2_experimental_deg(elapsed_days_from_j2000: f64) -> f64 {
    let base = digital_lunar_longitude_deg(elapsed_days_from_j2000);
    let draconic_phase = wrap_deg(
        93.272_095 + elapsed_days_from_j2000 * 360.0 / DRACONIC_MONTH.period_days,
    ) / 360.0;
    let correction = M8_SIN_2_DRACONIC_COEFFICIENT_DEG
        * (4.0 * PI * draconic_phase).sin();
    wrap_deg(base + correction)
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

    #[test]
    fn historical_registry_sources_resolve() {
        for record in HISTORICAL_EVIDENCE_REGISTRY {
            assert!(!record.source_ids.is_empty());
            for source_id in record.source_ids {
                assert!(source_by_id(source_id).is_some(), "missing source {source_id}");
            }
        }
        assert_eq!(
            historical_evidence_for("front-cosmos-gearing-2021")
                .unwrap()
                .evidence,
            EvidenceLabel::ReconstructedModel
        );
    }

    #[test]
    fn historical_profiles_keep_reconstruction_scope_explicit() {
        assert!(!HistoricalReconstructionProfile::ConservativeRearDialsV1
            .scope()
            .contains(&"front-cosmos-gearing-2021"));
        assert!(HistoricalReconstructionProfile::Freeth2021FrontCosmosV1
            .scope()
            .contains(&"front-cosmos-gearing-2021"));
    }

    #[test]
    fn all_digital_lunar_terms_are_modern_and_individually_ablatable() {
        assert_eq!(DIGITAL_LUNAR_CORRECTIONS_V1.len(), 14);
        assert!(DIGITAL_LUNAR_CORRECTIONS_V1
            .iter()
            .all(|term| term.evidence == EvidenceLabel::ModernDigitalCorrection));
        let d = 9_500.0;
        let full = digital_lunar_longitude_deg(d);
        for term in DIGITAL_LUNAR_CORRECTIONS_V1 {
            let ablated = digital_lunar_longitude_with_selection(
                d,
                CorrectionSelection {
                    disabled_ids: &[term.id],
                },
            );
            assert!(shortest_angle_deg(full, ablated) > 0.0);
        }
    }

    #[test]
    fn correction_registry_refactor_preserves_legacy_formula_at_reference_epoch() {
        let d = 12_345.678;
        let l = wrap_deg(218.316_447_7 + 13.176_396_48 * d);
        let m_moon = wrap_deg(134.963_396_4 + 13.064_992_95 * d);
        let elongation = wrap_deg(297.850_192_1 + 12.190_749_12 * d);
        let f = wrap_deg(93.272_095 + 13.229_350_24 * d);
        let m_sun = wrap_deg(357.529_11 + 0.985_600_28 * d);
        let legacy = wrap_deg(
            l + 6.289 * sin_deg(m_moon)
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
                + 0.011 * sin_deg(2.0 * elongation - 4.0 * m_moon),
        );
        assert!(shortest_angle_deg(legacy, digital_lunar_longitude_deg(d)) < 1e-12);
    }


    #[test]
    fn experimental_v2_is_explicit_and_default_stays_v1() {
        assert_eq!(AntikytheraMachine::digital().profile, MachineProfile::MTimeDigitalV1);
        assert_eq!(
            AntikytheraMachine::digital_v2_experimental().profile,
            MachineProfile::MTimeDigitalV2Experimental
        );
        assert!(!M8_EXPERIMENTAL_CORRECTION.enabled_by_default);
        assert_eq!(
            M8_EXPERIMENTAL_CORRECTION.evidence,
            EvidenceLabel::ModernDigitalCorrection
        );
    }

    #[test]
    fn experimental_v2_changes_only_lunar_path() {
        let jd = J2000_JD_TT + 12_345.0;
        let v1 = AntikytheraMachine::digital().state_at_tt(jd).unwrap();
        let v2 = AntikytheraMachine::digital_v2_experimental().state_at_tt(jd).unwrap();
        assert_eq!(v1.solar_longitude, v2.solar_longitude);
        assert!(shortest_angle_deg(v1.lunar_longitude.angle_deg, v2.lunar_longitude.angle_deg) > 0.001);
    }


    #[test]
    fn validated_digital_api_fails_closed_outside_m16_interval() {
        let m = AntikytheraMachine::digital();
        assert!(m.state_at_tt_validated(DIGITAL_VALIDATED_START_JD_TT).is_ok());
        assert!(m.state_at_tt_validated(DIGITAL_VALIDATED_END_EXCLUSIVE_JD_TT - 1.0).is_ok());
        assert_eq!(
            m.state_at_tt_validated(DIGITAL_VALIDATED_START_JD_TT - 1.0),
            Err(MachineError::OutsideValidatedRange)
        );
        assert_eq!(
            AntikytheraMachine::historical().state_at_tt_validated(J2000_JD_TT),
            Err(MachineError::NoValidatedInterval)
        );
    }

}
