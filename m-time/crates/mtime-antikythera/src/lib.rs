use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ratio { pub num: i64, pub den: i64 }
impl Ratio {
    pub fn new(num: i64, den: i64) -> Self { assert!(den != 0); let g = gcd(num.abs(), den.abs()); let sign = if den < 0 { -1 } else { 1 }; Self { num: sign * num / g, den: sign * den / g } }
    pub fn compose(self, other: Self) -> Self { Self::new(self.num * other.num, self.den * other.den) }
    pub fn value(self) -> f64 { self.num as f64 / self.den as f64 }
}
const fn gcd(mut a: i64, mut b: i64) -> i64 { while b != 0 { let r = a % b; a = b; b = r; } if a == 0 { 1 } else { a } }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cycle { pub id: String, pub period_days: f64, pub evidence: String }
impl Cycle {
    pub fn phase(&self, jd: f64, epoch_jd: f64) -> f64 { ((jd - epoch_jd) / self.period_days).rem_euclid(1.0) }
    pub fn recurrence(&self, jd: f64, n: i32) -> f64 { jd + self.period_days * n as f64 }
}

pub fn metonic() -> Cycle { Cycle { id: "METONIC".into(), period_days: 6939.688_166, evidence: "SURVIVING_EVIDENCE: 235 synodic months / 19 years".into() } }
pub fn saros() -> Cycle { Cycle { id: "SAROS".into(), period_days: 6585.3223, evidence: "SURVIVING_EVIDENCE: 223 lunar months".into() } }
pub fn exeligmos() -> Cycle { Cycle { id: "EXELIGMOS".into(), period_days: 3.0 * 6585.3223, evidence: "RECONSTRUCTED/KNOWN EXTENSION: 3 Saros".into() } }
pub fn anomalistic_month() -> Cycle { Cycle { id: "ANOMALISTIC_MONTH".into(), period_days: 27.55455, evidence: "MODERN_ASTRONOMICAL_PERIOD".into() } }
pub fn draconic_month() -> Cycle { Cycle { id: "DRACONIC_MONTH".into(), period_days: 27.21222, evidence: "MODERN_ASTRONOMICAL_PERIOD".into() } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn ratio_reduces() { assert_eq!(Ratio::new(470, 38), Ratio { num: 235, den: 19 }); }
    #[test] fn saros_recurrence() { assert!((saros().recurrence(2460409.263, 1) - 2466994.5853).abs() < 1e-9); }
    #[test] fn phase_wraps() { assert!(saros().phase(100.0, 100.0).abs() < 1e-12); assert!(saros().phase(100.0 + saros().period_days, 100.0).abs() < 1e-12); }
}
