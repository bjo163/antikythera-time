use mtime_antikythera::SAROS;
use mtime_hijri::{mabims_indonesia_2026,evaluate_profile,HijriAstronomicalState,ObserverSite};
use mtime_cosmology::flat_lcdm_age;

#[derive(Debug,Clone,PartialEq)]
pub struct GoldenResult{pub id:&'static str,pub actual:f64,pub expected:f64,pub tolerance:f64}
impl GoldenResult{pub fn pass(&self)->bool{(self.actual-self.expected).abs()<=self.tolerance}}

pub fn run_golden()->Vec<GoldenResult>{
 let state=HijriAstronomicalState{conjunction_jd_tt:2460000.0,sunset_jd_utc:2460000.5,moon_topocentric_altitude_deg:3.0,moon_sun_geocentric_elongation_deg:6.4,moon_illumination_fraction:None,moon_age_hours:None,moon_lag_minutes:None,site:ObserverSite{id:"golden".into(),latitude_deg:0.0,longitude_deg:0.0,height_m:0.0,timezone:"UTC".into(),datum:"WGS84".into()},provenance:vec![]};
 let mabims=if evaluate_profile(&mabims_indonesia_2026(),&state).pass{1.0}else{0.0};
 let age=flat_lcdm_age(67.36,0.3153,0.6847).unwrap().age_gyr;
 vec![
  GoldenResult{id:"SAROS_ONE",actual:SAROS.advance(2460409.263,1),expected:2466994.5853,tolerance:1e-9},
  GoldenResult{id:"MABIMS_BOUNDARY_PASS",actual:mabims,expected:1.0,tolerance:0.0},
  GoldenResult{id:"PLANCK_LIKE_AGE",actual:age,expected:13.796,tolerance:0.02},
 ]
}
pub fn all_pass()->bool{run_golden().iter().all(|x|x.pass())}
#[cfg(test)]
mod tests{use super::*;#[test]fn golden_vectors_pass(){for x in run_golden(){assert!(x.pass(),"{} actual {}",x.id,x.actual)}}}
