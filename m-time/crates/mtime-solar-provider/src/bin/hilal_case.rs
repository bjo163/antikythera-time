use mtime_astro::Site;
use mtime_hijri::{mabims_indonesia_2026,evaluate_profile};
use mtime_solar_provider::hilal_state_for_local_day;
fn main(){
 let site=Site{id:"Jakarta".into(),latitude_deg:-6.2,longitude_deg:106.8,height_m:10.0,datum:"WGS84".into()};
 let state=hilal_state_for_local_day(2461118.5,&site).expect("hilal state");
 let result=evaluate_profile(&mabims_indonesia_2026(),&state);
 println!("site={} sunset_jd_utc={:.9} conjunction_jd_tt={:.9} altitude_deg={:.6} elongation_deg={:.6} moon_age_h={:.3} lag_min={:?} mabims_pass={}",
  state.site.id,state.sunset_jd_utc,state.conjunction_jd_tt,state.moon_topocentric_altitude_deg,state.moon_sun_geocentric_elongation_deg,state.moon_age_hours.unwrap_or(f64::NAN),state.moon_lag_minutes,result.pass);
}
