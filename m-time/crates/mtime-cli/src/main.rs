use mtime_hijri::{mabims_indonesia_2026,evaluate_profile,HijriAstronomicalState,ObserverSite};
fn main(){
 let state=HijriAstronomicalState{
  conjunction_jd_tt:2460000.0,sunset_jd_utc:2460000.5,
  moon_topocentric_altitude_deg:3.2,moon_sun_geocentric_elongation_deg:6.6,
  moon_illumination_fraction:None,moon_age_hours:None,moon_lag_minutes:None,
  site:ObserverSite{id:"demo".into(),latitude_deg:-6.2,longitude_deg:106.8,height_m:10.0,timezone:"Asia/Jakarta".into(),datum:"WGS84".into()},
  provenance:vec![]
 };
 let r=evaluate_profile(&mabims_indonesia_2026(),&state);
 println!("M-Time bootstrap: profile={} pass={}",r.profile_id,r.pass);
}
