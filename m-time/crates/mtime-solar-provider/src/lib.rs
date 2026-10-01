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


fn state_at_with_solar_eop(jd_utc:f64,site:&Site,eop:solar_ephemeris::earth_orientation::EarthOrientation,provenance_label:String)->Result<SunMoonState,String>{
 use solar_ephemeris::{coords,earth_orientation,elpmpp02,planets,time,timescales::AstroTime};
 if !jd_utc.is_finite(){return Err("finite JD UTC required".into())}
 let astro=AstroTime::from_jd_utc_with_eop(jd_utc,eop);
 let t=time::centuries(astro.jd_tt);let(dpsi,deps)=time::nutation_deg(t);let eps=time::mean_obliquity_deg(t)+deps;
 let(observer_lat,observer_lon)=earth_orientation::corrected_observer_geodetic(site.latitude_deg,site.longitude_deg,astro.eop.xp_arcsec,astro.eop.yp_arcsec);
 let lst=(time::gast_deg(astro.jd_ut1,dpsi,eps)+observer_lon).rem_euclid(360.0);let(rho_sin,rho_cos)=coords::observer_rho(observer_lat,site.height_m);
 let(slon,slat,sdist_au)=planets::sun_apparent_ecliptic(astro.jd_tt,dpsi);let(sra,sdec)=coords::ecl_to_equ(slon,slat,eps);
 let(mlon,mlat,mdist_km)=elpmpp02::moon_apparent_ecliptic(astro.jd_tt,dpsi);let(mra,mdec)=coords::ecl_to_equ(mlon,mlat,eps);
 let(mra_t,mdec_t)=coords::topocentric(mra,mdec,mdist_km,lst,rho_sin,rho_cos);let(mgeo_alt,_)=coords::alt_az(mra,mdec,lst,observer_lat);let(malt,_)=coords::alt_az(mra_t,mdec_t,lst,observer_lat);
 let elong=angular_separation_deg(sra,sdec,mra,mdec);
 Ok(SunMoonState{jd_tt:astro.jd_tt,site:site.clone(),
  sun:BodyState{right_ascension_deg:sra,declination_deg:sdec,distance_au:Some(sdist_au)},
  moon:BodyState{right_ascension_deg:mra,declination_deg:mdec,distance_au:Some(mdist_km/coords::AU_KM)},
  moon_geocentric_altitude_deg:mgeo_alt,moon_topocentric_altitude_deg:malt,moon_sun_geocentric_elongation_deg:elong,
  illumination_fraction:(1.0-elong.to_radians().cos())/2.0,
  provenance:vec![Provenance{source:provenance_label,version:Some("solar-ephemeris=0.2.0".into()),retrieved_at:None}]})
}

pub fn state_at_with_iers(jd_utc:f64,site:&Site,eop:&mtime_eop::EopRecord)->Result<SunMoonState,String>{
 use solar_ephemeris::earth_orientation::{EarthOrientation,Quality};
 let q=if eop.observed{Quality::Rapid}else{Quality::Predicted};
 let se=EarthOrientation{dut1_seconds:eop.ut1_minus_utc_seconds,xp_arcsec:eop.xp_arcsec,yp_arcsec:eop.yp_arcsec,dut1_uncertainty_seconds:if eop.observed{0.001}else{0.1},source:"external IERS finals.all IAU2000 via M-Time",quality:q};
 state_at_with_solar_eop(jd_utc,site,se,format!("solar-ephemeris numerical model + M-Time IERS finals.all EOP MJD {:.5}",eop.mjd))
}

pub fn state_at(jd_utc:f64,site:&Site)->Result<SunMoonState,String>{
 use solar_ephemeris::{coords,earth_orientation,elpmpp02,planets,time,timescales::AstroTime};
 if !jd_utc.is_finite(){return Err("finite JD UTC required".into())}
 let astro=AstroTime::from_jd_utc(jd_utc);
 let t=time::centuries(astro.jd_tt);
 let (dpsi,deps)=time::nutation_deg(t);
 let eps=time::mean_obliquity_deg(t)+deps;
 let (observer_lat,observer_lon)=earth_orientation::corrected_observer_geodetic(site.latitude_deg,site.longitude_deg,astro.eop.xp_arcsec,astro.eop.yp_arcsec);
 let lst=(time::gast_deg(astro.jd_ut1,dpsi,eps)+observer_lon).rem_euclid(360.0);
 let (rho_sin,rho_cos)=coords::observer_rho(observer_lat,site.height_m);

 let (slon,slat,sdist_au)=planets::sun_apparent_ecliptic(astro.jd_tt,dpsi);
 let (sra,sdec)=coords::ecl_to_equ(slon,slat,eps);
 let (sra_t,sdec_t)=coords::topocentric(sra,sdec,sdist_au*coords::AU_KM,lst,rho_sin,rho_cos);
 let (salt,_)=coords::alt_az(sra_t,sdec_t,lst,observer_lat);

 let (mlon,mlat,mdist_km)=elpmpp02::moon_apparent_ecliptic(astro.jd_tt,dpsi);
 let (mra,mdec)=coords::ecl_to_equ(mlon,mlat,eps);
 let (mra_t,mdec_t)=coords::topocentric(mra,mdec,mdist_km,lst,rho_sin,rho_cos);
 let (mgeo_alt,_)=coords::alt_az(mra,mdec,lst,observer_lat);
 let (malt,_)=coords::alt_az(mra_t,mdec_t,lst,observer_lat);

 let sun_state=BodyState{right_ascension_deg:sra,declination_deg:sdec,distance_au:Some(sdist_au)};
 let moon_state=BodyState{right_ascension_deg:mra,declination_deg:mdec,distance_au:Some(mdist_km/coords::AU_KM)};
 let elong=angular_separation_deg(sra,sdec,mra,mdec);
 let eop_quality=format!("{:?}",astro.eop.quality);
 let _=salt; // used by sunset fast path below; retained here for symmetric reduction.
 Ok(SunMoonState{
  jd_tt:astro.jd_tt,site:site.clone(),sun:sun_state,moon:moon_state,
  moon_geocentric_altitude_deg:mgeo_alt,
  moon_topocentric_altitude_deg:malt,
  moon_sun_geocentric_elongation_deg:elong,
  illumination_fraction:(1.0-elong.to_radians().cos())/2.0,
  provenance:vec![Provenance{source:format!("solar-ephemeris 0.2.0 direct numeric provider; EOP quality={eop_quality}"),version:Some("0.2.0".into()),retrieved_at:None}],
 })
}

fn direct_sun_altitude_deg(jd_utc:f64,site:&Site)->Result<f64,String>{
 use solar_ephemeris::{coords,earth_orientation,planets,time,timescales::AstroTime};
 let astro=AstroTime::from_jd_utc(jd_utc);let t=time::centuries(astro.jd_tt);let(dpsi,deps)=time::nutation_deg(t);let eps=time::mean_obliquity_deg(t)+deps;
 let(observer_lat,observer_lon)=earth_orientation::corrected_observer_geodetic(site.latitude_deg,site.longitude_deg,astro.eop.xp_arcsec,astro.eop.yp_arcsec);
 let lst=(time::gast_deg(astro.jd_ut1,dpsi,eps)+observer_lon).rem_euclid(360.0);let(rho_sin,rho_cos)=coords::observer_rho(observer_lat,site.height_m);
 let(lon,lat,dist)=planets::sun_apparent_ecliptic(astro.jd_tt,dpsi);let(ra,dec)=coords::ecl_to_equ(lon,lat,eps);let(ra_t,dec_t)=coords::topocentric(ra,dec,dist*coords::AU_KM,lst,rho_sin,rho_cos);
 Ok(coords::alt_az(ra_t,dec_t,lst,observer_lat).0)
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

fn event_jd(body:&Value,name:&str)->Option<f64>{
 let e=&body["events"][name];
 e.as_f64().or_else(||e["jd"].as_f64())
}
fn sun_geometric_altitude_deg(jd_utc:f64,site:&Site)->Result<f64,String>{direct_sun_altitude_deg(jd_utc,site)}
/// Local sunset fallback: solve geometric Sun-centre altitude = -0.8333 deg.
/// The fixed threshold is explicitly an approximate standard-atmosphere model.
/// A higher-fidelity provider may expose its own dynamic semidiameter/refraction event.
fn fallback_sunset_jd_utc(jd_utc:f64,site:&Site)->Result<f64,String>{
 let offset=site.longitude_deg/360.0;
 let start=((jd_utc-0.5+offset).floor()+0.5)-offset;
 let target=-0.8333;
 let steps=96usize;let mut a=start;let mut fa=sun_geometric_altitude_deg(a,site)?-target;
 for i in 1..=steps{
  let b=start+i as f64/steps as f64;let fb=sun_geometric_altitude_deg(b,site)?-target;
  if fa>0.0&&fb<=0.0{
   let(mut lo,mut hi,mut flo)=(a,b,fa);
   for _ in 0..60{let mid=(lo+hi)/2.0;let fm=sun_geometric_altitude_deg(mid,site)?-target;if (hi-lo)*86400.0<0.05{return Ok(mid)}if flo.signum()==fm.signum(){lo=mid;flo=fm}else{hi=mid}}
   return Ok((lo+hi)/2.0)
  }
  a=b;fa=fb;
 }
 Err("sunset crossing not found in local mean-solar day".into())
}
pub fn sunset_jd_utc(jd_utc:f64,site:&Site)->Result<f64,String>{
 // Direct numeric fallback is authoritative for this adapter; snapshot events remain diagnostic.
 fallback_sunset_jd_utc(jd_utc,site)
}

pub fn hilal_state_for_local_day(jd_utc:f64,site:&Site)->Result<HijriAstronomicalState,String>{
 let sunset=sunset_jd_utc(jd_utc,site)?;
 let state=state_at(sunset,site)?;
 let sunset_tt=state.jd_tt;
 let conjunction=find_conjunction_tt(sunset_tt-3.0,sunset_tt+0.25)?;
 if conjunction>sunset_tt{return Err("nearest conjunction occurs after sunset; requested day is pre-conjunction".into())}
 let lag=None;
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
 #[test]fn conjunction_solver_finds_root(){let root=find_conjunction_tt(2461115.0,2461120.0).unwrap();assert!(signed_lon_diff_deg(root).abs()<1e-5);}
 #[test]fn implicit_tt_to_utc_is_rejected(){let p=SolarEphemerisProvider;assert!(p.sun_moon_state(2461041.0,&jakarta()).is_err());}
}
