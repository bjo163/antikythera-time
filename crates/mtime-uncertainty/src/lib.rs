#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UncertaintyKind {
    Numerical,
    Reference,
    Model,
    Observer,
    Environment,
    PolicyMargin,
    DataAge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Seconds,
    Degrees,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UncertaintyComponent {
    pub id: &'static str,
    pub kind: UncertaintyKind,
    pub unit: Unit,
    pub magnitude: f64,
    pub coverage: &'static str,
    pub provenance: &'static str,
}

impl UncertaintyComponent {
    #[must_use]
    pub fn is_known(self) -> bool {
        self.magnitude.is_finite() && self.magnitude >= 0.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UncertaintyBudget {
    pub components: Vec<UncertaintyComponent>,
}

impl UncertaintyBudget {
    #[must_use]
    pub fn new(components: Vec<UncertaintyComponent>) -> Self {
        Self { components }
    }

    #[must_use]
    pub fn known_for_unit(&self, unit: Unit) -> bool {
        self.components
            .iter()
            .filter(|c| c.unit == unit)
            .all(|c| c.is_known())
    }

    #[must_use]
    pub fn conservative_sum(&self, unit: Unit) -> Option<f64> {
        let matching = self.components.iter().filter(|c| c.unit == unit).collect::<Vec<_>>();
        if matching.is_empty() || matching.iter().any(|c| !c.is_known()) {
            return None;
        }
        Some(matching.iter().map(|c| c.magnitude).sum())
    }

    #[must_use]
    pub fn explain(&self) -> String {
        if self.components.is_empty() {
            return "uncertainty budget has no declared components".into();
        }
        self.components
            .iter()
            .map(|c| {
                format!(
                    "{}:{:?}:{:?}:{}:{}:{}",
                    c.id, c.kind, c.unit, c.magnitude, c.coverage, c.provenance
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[must_use]
    pub fn status(&self) -> &'static str {
        if self.components.iter().all(|c| c.is_known()) {
            "BOUNDED_BY_DECLARED_COMPONENTS"
        } else {
            "UNCERTAINTY_INCOMPLETE"
        }
    }
}

pub const M6_SUN_ABSOLUTE_P95_DEG: f64 = 0.006_319_944;
pub const M6_MOON_ABSOLUTE_P95_DEG: f64 = 0.305_809_037;
pub const M6_MOON_PHASE_P95_DEG: f64 = 0.304_293_570;

#[must_use]
pub fn m6_digital_angular_budget() -> UncertaintyBudget {
    UncertaintyBudget::new(vec![
        UncertaintyComponent {
            id: "M6_SUN_MODEL_P95",
            kind: UncertaintyKind::Model,
            unit: Unit::Degrees,
            magnitude: M6_SUN_ABSOLUTE_P95_DEG,
            coverage: "P95, monthly 1900-2100",
            provenance: "M6 DE440 mean-ecliptic-of-date calibration",
        },
        UncertaintyComponent {
            id: "M6_MOON_MODEL_P95",
            kind: UncertaintyKind::Model,
            unit: Unit::Degrees,
            magnitude: M6_MOON_ABSOLUTE_P95_DEG,
            coverage: "P95, monthly 1900-2100",
            provenance: "M6 DE440 mean-ecliptic-of-date calibration",
        },
        UncertaintyComponent {
            id: "M6_PHASE_MODEL_P95",
            kind: UncertaintyKind::Model,
            unit: Unit::Degrees,
            magnitude: M6_MOON_PHASE_P95_DEG,
            coverage: "P95, monthly 1900-2100",
            provenance: "M6 DE440 mean-ecliptic-of-date calibration",
        },
    ])
}

#[must_use]
pub fn reference_time_budget(reference_seconds: f64) -> UncertaintyBudget {
    UncertaintyBudget::new(vec![UncertaintyComponent {
        id: "REFERENCE_TIME",
        kind: UncertaintyKind::Reference,
        unit: Unit::Seconds,
        magnitude: reference_seconds,
        coverage: "declared conversion/provider uncertainty",
        provenance: "M-Time timescale/reference layer",
    }])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m6_budget_is_explicit_and_known() {
        let b = m6_digital_angular_budget();
        assert_eq!(b.status(), "BOUNDED_BY_DECLARED_COMPONENTS");
        assert_eq!(b.components.len(), 3);
        assert!(b.conservative_sum(Unit::Degrees).unwrap() > M6_MOON_ABSOLUTE_P95_DEG);
    }

    #[test]
    fn unknown_component_blocks_bound_status() {
        let b = UncertaintyBudget::new(vec![UncertaintyComponent {
            id: "UNKNOWN",
            kind: UncertaintyKind::Environment,
            unit: Unit::Degrees,
            magnitude: f64::NAN,
            coverage: "unknown",
            provenance: "missing",
        }]);
        assert_eq!(b.status(), "UNCERTAINTY_INCOMPLETE");
        assert_eq!(b.conservative_sum(Unit::Degrees), None);
    }
}
