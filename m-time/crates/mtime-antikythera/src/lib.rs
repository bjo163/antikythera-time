#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cycle { pub period_days: f64, pub label: &'static str }

impl Cycle {
 pub const fn new(period_days:f64,label:&'static str)->Self{Self{period_days,label}}
 pub fn phase(&self, elapsed_days:f64)->f64{
   let mut x=(elapsed_days/self.period_days).fract();
   if x<0.0{x+=1.0;} x
 }
 pub fn advance(&self, jd:f64, cycles:i64)->f64{jd+self.period_days*cycles as f64}
}

pub const METONIC_YEARS:f64=19.0;
pub const METONIC_LUNAR_MONTHS:i32=235;
pub const SAROS:Cycle=Cycle::new(6585.3223,"Saros");
pub const EXELIGMOS:Cycle=Cycle::new(19755.9669,"Exeligmos");
pub const SYNODIC_MONTH:Cycle=Cycle::new(29.530588853,"Synodic month");
pub const ANOMALISTIC_MONTH:Cycle=Cycle::new(27.55454988,"Anomalistic month");
pub const DRACONIC_MONTH:Cycle=Cycle::new(27.21222082,"Draconic month");

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum HistoricalEvidence { Surviving, StronglyIndicatedReconstruction, ReconstructedModel, Hypothetical }

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct MechanismClaim { pub id:&'static str, pub evidence:HistoricalEvidence }

pub const CLAIMS:&[MechanismClaim]=&[
 MechanismClaim{id:"metonic-dial",evidence:HistoricalEvidence::Surviving},
 MechanismClaim{id:"saros-dial",evidence:HistoricalEvidence::Surviving},
 MechanismClaim{id:"lunar-anomaly",evidence:HistoricalEvidence::Surviving},
 MechanismClaim{id:"planet-inscriptions",evidence:HistoricalEvidence::Surviving},
 MechanismClaim{id:"superior-planet-gearing",evidence:HistoricalEvidence::ReconstructedModel},
];

#[cfg(test)]
mod tests{
 use super::*;
 #[test] fn saros_recurrence(){assert!((SAROS.advance(2460409.263,1)-2466994.5853).abs()<1e-9);}
 #[test] fn phase_wraps(){assert!((SYNODIC_MONTH.phase(SYNODIC_MONTH.period_days)-0.0).abs()<1e-12);}
 #[test] fn evidence_is_not_flattened(){assert_ne!(CLAIMS[3].evidence,CLAIMS[4].evidence);}
}
