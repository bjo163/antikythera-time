use mtime_core::Provenance;

#[derive(Debug,Clone,PartialEq)]
pub enum Metric {
  MoonTopocentricAltitudeDeg,
  MoonSunGeocentricElongationDeg,
  MoonAgeHours,
  MoonLagMinutes,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Op { Ge, Gt, Le, Lt }

#[derive(Debug,Clone,PartialEq)]
pub struct Threshold { pub metric:Metric, pub op:Op, pub value:f64, pub unit:&'static str }

impl Threshold {
 pub fn evaluate(&self, actual:f64)->bool{match self.op{Op::Ge=>actual>=self.value,Op::Gt=>actual>self.value,Op::Le=>actual<=self.value,Op::Lt=>actual<self.value}}
}

#[derive(Debug,Clone,PartialEq)]
pub enum CriterionExpr {
 Threshold(Threshold),
 All(Vec<CriterionExpr>),
 Any(Vec<CriterionExpr>),
}

#[derive(Debug,Clone,PartialEq)]
pub struct CalendarProfile {
 pub id:String,
 pub version:String,
 pub effective_from:Option<String>,
 pub criteria:CriterionExpr,
 pub provenance:Provenance,
}

#[derive(Debug,Clone,PartialEq)]
pub struct CriterionClauseResult {pub label:String,pub actual:f64,pub required:String,pub pass:bool}

#[derive(Debug,Clone,PartialEq)]
pub struct CriterionResult {pub profile_id:String,pub profile_version:String,pub clauses:Vec<CriterionClauseResult>,pub pass:bool}
