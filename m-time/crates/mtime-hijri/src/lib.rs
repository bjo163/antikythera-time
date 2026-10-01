use mtime_calendar::{CalendarProfile,CriterionClauseResult,CriterionExpr,CriterionResult,Metric,Op,Threshold};
use mtime_core::Provenance;

#[derive(Debug,Clone,PartialEq)]
pub struct ObserverSite {
 pub id:String,
 pub latitude_deg:f64,
 pub longitude_deg:f64,
 pub height_m:f64,
 pub timezone:String,
 pub datum:String,
}

#[derive(Debug,Clone,PartialEq)]
pub struct HijriAstronomicalState {
 pub conjunction_jd_tt:f64,
 pub sunset_jd_utc:f64,
 pub moon_topocentric_altitude_deg:f64,
 pub moon_sun_geocentric_elongation_deg:f64,
 pub moon_illumination_fraction:Option<f64>,
 pub moon_age_hours:Option<f64>,
 pub moon_lag_minutes:Option<f64>,
 pub site:ObserverSite,
 pub provenance:Vec<Provenance>,
}

pub fn mabims_indonesia_2026()->CalendarProfile{
 CalendarProfile{
  id:"MABIMS-ID".into(),version:"PMA-1-2026".into(),effective_from:Some("2026-01-27".into()),
  criteria:CriterionExpr::All(vec![
   CriterionExpr::Threshold(Threshold{metric:Metric::MoonTopocentricAltitudeDeg,op:Op::Ge,value:3.0,unit:"deg"}),
   CriterionExpr::Threshold(Threshold{metric:Metric::MoonSunGeocentricElongationDeg,op:Op::Ge,value:6.4,unit:"deg"}),
  ]),
  provenance:Provenance{source:"Kementerian Agama RI — PMA No. 1 Tahun 2026 / MABIMS".into(),version:Some("2026".into()),retrieved_at:None},
 }
}

fn eval(expr:&CriterionExpr,state:&HijriAstronomicalState,clauses:&mut Vec<CriterionClauseResult>)->bool{
 match expr{
  CriterionExpr::Threshold(t)=>{
   let actual=match t.metric{
    Metric::MoonTopocentricAltitudeDeg=>state.moon_topocentric_altitude_deg,
    Metric::MoonSunGeocentricElongationDeg=>state.moon_sun_geocentric_elongation_deg,
    Metric::MoonAgeHours=>state.moon_age_hours.unwrap_or(f64::NAN),
    Metric::MoonLagMinutes=>state.moon_lag_minutes.unwrap_or(f64::NAN),
   };
   let pass=actual.is_finite()&&t.evaluate(actual);
   clauses.push(CriterionClauseResult{
    label:format!("{:?}",t.metric),actual,
    required:format!("{:?} {} {}",t.op,t.value,t.unit),pass});
   pass
  }
  CriterionExpr::All(v)=>v.iter().all(|x|eval(x,state,clauses)),
  CriterionExpr::Any(v)=>v.iter().any(|x|eval(x,state,clauses)),
 }
}

pub fn evaluate_profile(profile:&CalendarProfile,state:&HijriAstronomicalState)->CriterionResult{
 let mut clauses=Vec::new(); let pass=eval(&profile.criteria,state,&mut clauses);
 CriterionResult{profile_id:profile.id.clone(),profile_version:profile.version.clone(),clauses,pass}
}

#[cfg(test)]
mod tests{
 use super::*;
 fn state(a:f64,e:f64)->HijriAstronomicalState{
  HijriAstronomicalState{conjunction_jd_tt:2460000.0,sunset_jd_utc:2460000.5,moon_topocentric_altitude_deg:a,moon_sun_geocentric_elongation_deg:e,moon_illumination_fraction:None,moon_age_hours:None,moon_lag_minutes:None,site:ObserverSite{id:"jakarta".into(),latitude_deg:-6.2,longitude_deg:106.8,height_m:10.0,timezone:"Asia/Jakarta".into(),datum:"WGS84".into()},provenance:vec![]}
 }
 #[test] fn mabims_requires_both_thresholds(){let p=mabims_indonesia_2026();assert!(evaluate_profile(&p,&state(3.0,6.4)).pass);assert!(!evaluate_profile(&p,&state(2.99,6.4)).pass);assert!(!evaluate_profile(&p,&state(3.0,6.39)).pass);}
}
