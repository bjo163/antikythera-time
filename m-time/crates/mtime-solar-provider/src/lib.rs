use mtime_astro::{angular_separation_deg,BodyState,EphemerisProvider,Site,SunMoonState};
use mtime_core::Provenance;
use mtime_hijri::{HijriAstronomicalState,ObserverSite};
use serde_json::Value;

pub const PROVIDER_ID:&str="solar-ephemeris-0.2.0-adapter";
pub const QUALITY_CLASS:&str="EXPERIMENTAL_OFFLINE_PROVIDER";

fn body<'a>(v:&'a Value,name:&str)->Result<&'a Value,String>{
 v["bodies"].as_array().and_then(|a|a.iter().find(|x|x["name"].as_str()==Some(name))).ok_or_else(||format!("body {name} missing"))
}
fn num(v:&Value,key:&str)->Result<f64,String>{v[key].as_f64().ok_or_else(||format!("{key} missing"))}

pub fn snapshot_json(jd_utc:f64,site:&Site)->Result<Value,String>{
 let raw=solar_ephemeris::sky_snapshot_json(jd_utc,site.latitude_deg,site.longitude_deg,site.height_m);
 let v:Value=serde_json::from_str(&raw).map_err(|e|e.to_string())?;
 if let Some(e)=v["error"].as_str(){return Err(e.to_string())}
 Ok(v)
}

pub fn state_at(jd_utc:f64,site:&Site)->Result<SunMoonState,String>{
 let v=snapshot_json(jd_utc,site)?;
 let sun=body(&v,"Sun")?;let moon=body(&v,"Moon")?;
 let sun_state=BodyState{right_ascension_deg:num(sun,"geocentric_apparent_ra_deg")?,declination_deg:num(sun,"geocentric_apparent_dec_deg")?,distance_au:num(sun,"geocentric_range_km")?/149_597_870.7};
 let moon_state=BodyState{right_ascension_deg:num(moon,"geocentric_apparent_ra_deg")?,declination_deg:num(moon,"geocentric_apparent_dec_deg")?,distance_au:num(moon,"geocentric_range_km")?/149_597_870.7};
 let elong=angular_separation_deg(sun_state.right_ascension_deg,sun_state.declination_deg,moon_state.right_ascension_deg,moon_state.declination_deg);
 Ok(SunMoonState{
  jd_tt:num(&v["time"],"jd_tt")?,site:site.clone(),sun:sun_state,moon:moon_state,
  moon_topocentric_altitude_deg:num(moon,"alt_deg")?,
  moon_sun_geocentric_elongation_deg:elong,
  illumination_fraction:(1.0-elong.to_radians().cos())/2.0,
  provenance:vec![Provenance{source:"solar-ephemeris 0.2.0 offline analytic provider".into(),version:Some("0.2.0".into()),retrieved_at:None}],
 })
}

#[derive(Debug,Clone)]
pub struct SolarEphemerisProvider;
impl EphemerisProvider for SolarEphemerisProvider{
 fn id(&self)->&str{PROVIDER_ID}
 fn sun_moon_state(&self,jd_tt:f64,site:&Site)->Result<SunMoonState,String>{
  // Provider input surface is UTC. The bootstrap interface cannot safely infer UTC from TT
  // without full leap-second context, so this trait method is deliberately rejected.
  let _=(jd_tt,site);
  Err("use state_at(jd_utc, site); implicit TT->UTC is prohibited".into())
 }
}

fn signed_lon_diff_deg(jd_tt:f64)->f64{
 let t=solar_ephemeris::time::centuries(jd_tt);
 let dpsi=solar_ephemeris::time::nutation_deg(t).0;
 let moon=solar_ephemeris::elpmpp02::moon_apparent_ecliptic(jd_tt,dpsi).0;
 let sun=solar_ephemeris::planets::sun_apparent_ecliptic(jd_tt,dpsi).0;
 ((moon-sun+180.0).rem_euclid(360.0))-180.0
}

pub fn find_conjunction_tt(start_jd_tt:f64,end_jd_tt:f64)->Result<f64,String>{
 if !(start_jd_tt.is_finite()&&end_jd_tt.is_finite()&&end_jd_tt>start_jd_tt){return Err("valid bracket required".into())}
 let steps=192usize;let mut a=start_jd_tt;let mut fa=signed_lon_diff_deg(a);
 for i in 1..=steps{
  let b=start_jd_tt+(end_jd_tt-start_jd_tt)*i as f64/steps as f64;
  let fb=signed_lon_diff_deg(b);
  if fa==0.0{return Ok(a)}
  if fa.signum()!=fb.signum() && fa.abs()<90.0 && fb.abs()<90.0{
   let(mut lo,mut hi,mut flo)=(a,b,fa);
   for _ in 0..80{
    let mid=(lo+hi)/2.0;let fm=signed_lon_diff_deg(mid);
    if (hi-lo)*86400.0<0.001{return Ok(mid)}
    if flo.signum()==fm.signum(){lo=mid;flo=fm}else{hi=mid}
   }
   return Ok((lo+hi)/2.0)
  }
  a=b;fa=fb;
 }
 Err("no conjunction root found in bracket".into())
}

pub fn sunset_jd_utc(jd_utc:f64,site:&Site)->Result<f64,String>{
 let v=snapshot_json(jd_utc,site)?;let sun=body(&v,"Sun")?;
 sun["events"]["set"]["jd"].as_f64().ok_or_else(||"Sun set unavailable in local solar day".into())
}

pub fn hilal_state_for_local_day(jd_utc:f64,site:&Site)->Result<HijriAstronomicalState,String>{
 let daily=snapshot_json(jd_utc,site)?;
 let sun=body(&daily,"Sun")?;let moon=body(&daily,"Moon")?;
 let sunset=sun["events"]["set"]["jd"].as_f64().ok_or("Sunset unavailable")?;
 let at_set=snapshot_json(sunset,site)?;
 let moon_set=body(&at_set,"Moon")?;
 let state=state_at(sunset,site)?;
 let sunset_tt=num(&at_set["time"],"jd_tt")?;
 let conjunction=find_conjunction_tt(sunset_tt-3.0,sunset_tt+0.25)?;
 if conjunction>sunset_tt{return Err("nearest conjunction occurs after sunset; requested day is pre-conjunction".into())}
 let moon_set_jd=moon["events"]["set"]["jd"].as_f64().or_else(||moon_set["events"]["set"]["jd"].as_f64());
 let lag=moon_set_jd.map(|x|(x-sunset)*1440.0);
 Ok(HijriAstronomicalState{
  conjunction_jd_tt:conjunction,sunset_jd_utc:sunset,
  moon_topocentric_altitude_deg:state.moon_topocentric_altitude_deg,
  moon_sun_geocentric_elongation_deg:state.moon_sun_geocentric_elongation_deg,
  moon_illumination_fraction:Some(state.illumination_fraction),
  moon_age_hours:Some((sunset_tt-conjunction)*24.0),moon_lag_minutes:lag,
  site:ObserverSite{id:site.id.clone(),latitude_deg:site.latitude_deg,longitude_deg:site.longitude_deg,height_m:site.height_m,timezone:"UNSPECIFIED".into(),datum:"WGS84".into()},
  provenance:state.provenance,
 })
}

#[cfg(test)]
mod tests{
 use super::*;
 fn jakarta()->Site{Site{id:"jakarta".into(),latitude_deg:-6.2,longitude_deg:106.8,height_m:10.0,datum:"WGS84".into()}}
 #[test]fn offline_snapshot_is_finite(){let s=state_at(2461041.0,&jakarta()).unwrap();assert!(s.moon_topocentric_altitude_deg.is_finite());assert!((0.0..=180.0).contains(&s.moon_sun_geocentric_elongation_deg));}
 #[test]fn conjunction_solver_finds_root(){let root=find_conjunction_tt(2461038.0,2461044.0).unwrap();assert!(signed_lon_diff_deg(root).abs()<1e-5);}
 #[test]fn implicit_tt_to_utc_is_rejected(){let p=SolarEphemerisProvider;assert!(p.sun_moon_state(2461041.0,&jakarta()).is_err());}
}
