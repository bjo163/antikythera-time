use mtime_core::{EvidenceState,QualityClass};
#[derive(Debug,Clone,Copy,PartialEq)]pub struct TextualTemporalReference{pub corpus:&'static str,pub reference:&'static str,pub concepts:&'static [&'static str],pub evidence:EvidenceState,pub quality:QualityClass,pub numerical_scientific_prior:Option<f64>,pub note:&'static str}
pub const TEMPORAL_ONTOLOGY:&[TextualTemporalReference]=&[
TextualTemporalReference{corpus:"Qur'an",reference:"2:189",concepts:&["hilal","mawaqit","people","hajj"],evidence:EvidenceState::TextualReference,quality:QualityClass::Conceptual,numerical_scientific_prior:None,note:"New moons/hilal connected conceptually with appointed times and Hajj."},
TextualTemporalReference{corpus:"Qur'an",reference:"10:5",concepts:&["sun","moon","manazil","years","hisab"],evidence:EvidenceState::TextualReference,quality:QualityClass::Conceptual,numerical_scientific_prior:None,note:"Celestial order, lunar stages, years and reckoning."},
TextualTemporalReference{corpus:"Qur'an",reference:"55:5",concepts:&["sun","moon","reckoning"],evidence:EvidenceState::TextualReference,quality:QualityClass::Conceptual,numerical_scientific_prior:None,note:"Sun and Moon associated with reckoning."},
TextualTemporalReference{corpus:"Qur'an",reference:"21:33",concepts:&["sun","moon","falak","motion"],evidence:EvidenceState::TextualReference,quality:QualityClass::Conceptual,numerical_scientific_prior:None,note:"Celestial motion/falak."},
TextualTemporalReference{corpus:"Torah / Hebrew Bible witness",reference:"Genesis 1:14",concepts:&["luminaries","signs","appointed-times","days","years"],evidence:EvidenceState::TextualReference,quality:QualityClass::Conceptual,numerical_scientific_prior:None,note:"Comparative textual concept only."},
TextualTemporalReference{corpus:"Psalms witness",reference:"Psalm 104:19",concepts:&["moon","appointed-times"],evidence:EvidenceState::TextualReference,quality:QualityClass::Conceptual,numerical_scientific_prior:None,note:"Comparative textual concept only."},
TextualTemporalReference{corpus:"Gospel witness",reference:"Mark 13:32",concepts:&["day","hour","epistemic-limit"],evidence:EvidenceState::TextualReference,quality:QualityClass::Conceptual,numerical_scientific_prior:None,note:"Epistemic-limit ontology only; not chronology prediction."}];
#[must_use]pub fn any_numerical_prior()->bool{TEMPORAL_ONTOLOGY.iter().any(|x|x.numerical_scientific_prior.is_some())}
#[cfg(test)]mod tests{use super::*;#[test]fn texts_cannot_supply_scientific_priors(){assert!(!any_numerical_prior());for r in TEMPORAL_ONTOLOGY{assert_eq!(r.evidence,EvidenceState::TextualReference);assert_eq!(r.quality,QualityClass::Conceptual);}}}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemporalConcept {
    DayNight,
    Sun,
    Moon,
    LunarPhases,
    Months,
    Years,
    Reckoning,
    AppointedTimes,
    CelestialMotion,
    EpistemicLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextCorpus {
    Quran,
    TorahWitness,
    PsalmsWitness,
    GospelWitness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewStatus {
    SourceLinked,
    InterpretationNoteOnly,
    NeedsExternalScholarReview,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TemporalOntologyEntryV2 {
    pub id: &'static str,
    pub corpus: TextCorpus,
    pub canonical_reference: &'static str,
    pub source_language_id: &'static str,
    pub translation_provenance: &'static str,
    pub concepts: &'static [TemporalConcept],
    pub review: ReviewStatus,
    pub numerical_physics_value: Option<f64>,
    pub semantic_note: &'static str,
}

pub const TEMPORAL_ONTOLOGY_V2: &[TemporalOntologyEntryV2] = &[
    TemporalOntologyEntryV2 {
        id: "QURAN_2_189",
        corpus: TextCorpus::Quran,
        canonical_reference: "2:189",
        source_language_id: "ar-quran",
        translation_provenance: "reference identifier only; translations are external/versioned presentation",
        concepts: &[TemporalConcept::LunarPhases, TemporalConcept::AppointedTimes],
        review: ReviewStatus::SourceLinked,
        numerical_physics_value: None,
        semantic_note: "Hilal/new-moon phenomena are connected conceptually with appointed times and Hajj.",
    },
    TemporalOntologyEntryV2 {
        id: "QURAN_10_5",
        corpus: TextCorpus::Quran,
        canonical_reference: "10:5",
        source_language_id: "ar-quran",
        translation_provenance: "reference identifier only; translations are external/versioned presentation",
        concepts: &[TemporalConcept::Sun, TemporalConcept::Moon, TemporalConcept::LunarPhases, TemporalConcept::Years, TemporalConcept::Reckoning],
        review: ReviewStatus::SourceLinked,
        numerical_physics_value: None,
        semantic_note: "Celestial order and lunar stages are associated conceptually with years and reckoning.",
    },
    TemporalOntologyEntryV2 {
        id: "QURAN_55_5",
        corpus: TextCorpus::Quran,
        canonical_reference: "55:5",
        source_language_id: "ar-quran",
        translation_provenance: "reference identifier only; translations are external/versioned presentation",
        concepts: &[TemporalConcept::Sun, TemporalConcept::Moon, TemporalConcept::Reckoning],
        review: ReviewStatus::SourceLinked,
        numerical_physics_value: None,
        semantic_note: "Sun and Moon are associated with reckoning; no numerical astronomical constant is inferred.",
    },
    TemporalOntologyEntryV2 {
        id: "QURAN_21_33",
        corpus: TextCorpus::Quran,
        canonical_reference: "21:33",
        source_language_id: "ar-quran",
        translation_provenance: "reference identifier only; translations are external/versioned presentation",
        concepts: &[TemporalConcept::DayNight, TemporalConcept::Sun, TemporalConcept::Moon, TemporalConcept::CelestialMotion],
        review: ReviewStatus::SourceLinked,
        numerical_physics_value: None,
        semantic_note: "Celestial motion/falak is a semantic concept, not an orbital-parameter source.",
    },
    TemporalOntologyEntryV2 {
        id: "TORAH_GENESIS_1_14",
        corpus: TextCorpus::TorahWitness,
        canonical_reference: "Genesis 1:14",
        source_language_id: "he-masoretic-witness",
        translation_provenance: "comparative textual witness; translation/version must be supplied by presentation layer",
        concepts: &[TemporalConcept::Sun, TemporalConcept::Moon, TemporalConcept::AppointedTimes, TemporalConcept::DayNight, TemporalConcept::Years],
        review: ReviewStatus::NeedsExternalScholarReview,
        numerical_physics_value: None,
        semantic_note: "Comparative appointed-times/day/year concept only.",
    },
    TemporalOntologyEntryV2 {
        id: "PSALMS_104_19",
        corpus: TextCorpus::PsalmsWitness,
        canonical_reference: "Psalm 104:19",
        source_language_id: "he-masoretic-witness",
        translation_provenance: "comparative textual witness; translation/version must be supplied by presentation layer",
        concepts: &[TemporalConcept::Moon, TemporalConcept::AppointedTimes],
        review: ReviewStatus::NeedsExternalScholarReview,
        numerical_physics_value: None,
        semantic_note: "Comparative lunar appointed-times concept only.",
    },
    TemporalOntologyEntryV2 {
        id: "GOSPEL_MARK_13_32",
        corpus: TextCorpus::GospelWitness,
        canonical_reference: "Mark 13:32",
        source_language_id: "gr-nt-witness",
        translation_provenance: "comparative textual witness; translation/version must be supplied by presentation layer",
        concepts: &[TemporalConcept::DayNight, TemporalConcept::EpistemicLimit],
        review: ReviewStatus::NeedsExternalScholarReview,
        numerical_physics_value: None,
        semantic_note: "Epistemic-limit ontology only; not a chronology-prediction rule.",
    },
];

#[must_use]
pub fn ontology_can_change_physics() -> bool {
    TEMPORAL_ONTOLOGY_V2
        .iter()
        .any(|entry| entry.numerical_physics_value.is_some())
}

#[cfg(test)]
mod ontology_v2_tests {
    use super::*;

    #[test]
    fn ontology_contains_no_numerical_physics_injection() {
        assert!(!ontology_can_change_physics());
        assert!(TEMPORAL_ONTOLOGY_V2
            .iter()
            .all(|entry| entry.numerical_physics_value.is_none()));
    }

    #[test]
    fn comparative_witnesses_are_flagged_for_external_review() {
        assert!(TEMPORAL_ONTOLOGY_V2
            .iter()
            .filter(|entry| entry.corpus != TextCorpus::Quran)
            .all(|entry| entry.review == ReviewStatus::NeedsExternalScholarReview));
    }
}
