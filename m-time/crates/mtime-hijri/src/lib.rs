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
 #[test] fn khgt_syawal_1447_pkg1_matches_published_route(){
  let scan=KhgtScan{conjunction_utc_hour:1.391111,conjunction_before_new_zealand_dawn:true,points:vec![GlobalHilalPoint{id:"published-first-qualifying-site".into(),sunset_utc_hour:15.400833,moon_geocentric_altitude_deg:6.4889,moon_sun_geocentric_elongation_deg:8.0,in_american_landmass:false}]};
  let r=evaluate_khgt_muhammadiyah_2026(&scan);assert!(r.pass);assert_eq!(r.route,KhgtRoute::Pkg1Before24Utc);
 }
 #[test] fn khgt_ramadan_1447_pkg2_route_is_representable(){
  let scan=KhgtScan{conjunction_utc_hour:12.019167,conjunction_before_new_zealand_dawn:true,points:vec![GlobalHilalPoint{id:"Alaska-published-point".into(),sunset_utc_hour:25.0,moon_geocentric_altitude_deg:5.3931,moon_sun_geocentric_elongation_deg:8.0031,in_american_landmass:true}]};
  let r=evaluate_khgt_muhammadiyah_2026(&scan);assert!(r.pass);assert_eq!(r.route,KhgtRoute::Pkg2AmericanTransfer);
 }
}


#[derive(Debug,Clone,PartialEq)]
pub struct GlobalHilalPoint {
 pub id:String,
 pub sunset_utc_hour:f64,
 pub moon_geocentric_altitude_deg:f64,
 pub moon_sun_geocentric_elongation_deg:f64,
 pub in_american_landmass:bool,
}

#[derive(Debug,Clone,PartialEq)]
pub struct KhgtScan {
 pub conjunction_utc_hour:f64,
 pub conjunction_before_new_zealand_dawn:bool,
 pub points:Vec<GlobalHilalPoint>,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum KhgtRoute { Pkg1Before24Utc, Pkg2AmericanTransfer, NotSatisfied }

#[derive(Debug,Clone,PartialEq)]
pub struct KhgtResult {
 pub route:KhgtRoute,
 pub triggering_site:Option<String>,
 pub pass:bool,
 pub profile_id:&'static str,
 pub profile_version:&'static str,
}

fn khgt_5_8(p:&GlobalHilalPoint)->bool {
 p.moon_geocentric_altitude_deg>=5.0 && p.moon_sun_geocentric_elongation_deg>=8.0
}

/// Muhammadiyah KHGT profile represented from its published 2026 methodology.
/// Astronomy supplies candidate global sunset points; this function applies the
/// calendrical/global-transfer rule only.
pub fn evaluate_khgt_muhammadiyah_2026(scan:&KhgtScan)->KhgtResult {
 if let Some(p)=scan.points.iter().find(|p|p.sunset_utc_hour<24.0 && khgt_5_8(p)){
  return KhgtResult{route:KhgtRoute::Pkg1Before24Utc,triggering_site:Some(p.id.clone()),pass:true,profile_id:"KHGT-MUHAMMADIYAH",profile_version:"MUNAS-XXXII/KEP-86-2025"};
 }
 if scan.conjunction_before_new_zealand_dawn {
  if let Some(p)=scan.points.iter().find(|p|p.sunset_utc_hour>=24.0 && p.in_american_landmass && khgt_5_8(p)){
   return KhgtResult{route:KhgtRoute::Pkg2AmericanTransfer,triggering_site:Some(p.id.clone()),pass:true,profile_id:"KHGT-MUHAMMADIYAH",profile_version:"MUNAS-XXXII/KEP-86-2025"};
  }
 }
 KhgtResult{route:KhgtRoute::NotSatisfied,triggering_site:None,pass:false,profile_id:"KHGT-MUHAMMADIYAH",profile_version:"MUNAS-XXXII/KEP-86-2025"}
}
