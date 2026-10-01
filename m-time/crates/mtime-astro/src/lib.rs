use mtime_core::Provenance;
#[derive(Debug,Clone,PartialEq)]
pub struct Site{pub id:String,pub latitude_deg:f64,pub longitude_deg:f64,pub height_m:f64,pub datum:String}
#[derive(Debug,Clone,PartialEq)]
pub struct BodyState{pub right_ascension_deg:f64,pub declination_deg:f64,pub distance_au:f64}
#[derive(Debug,Clone,PartialEq)]
pub struct SunMoonState{
 pub jd_tt:f64,pub site:Site,pub sun:BodyState,pub moon:BodyState,
 pub moon_topocentric_altitude_deg:f64,pub moon_sun_geocentric_elongation_deg:f64,
 pub illumination_fraction:f64,pub provenance:Vec<Provenance>
}
pub trait EphemerisProvider{
 fn id(&self)->&str;
 fn sun_moon_state(&self,jd_tt:f64,site:&Site)->Result<SunMoonState,String>;
}
#[derive(Debug,Clone)]
pub struct FixtureEphemeris{pub state:SunMoonState,pub provider_id:String}
impl EphemerisProvider for FixtureEphemeris{
 fn id(&self)->&str{&self.provider_id}
 fn sun_moon_state(&self,_:f64,_:&Site)->Result<SunMoonState,String>{Ok(self.state.clone())}
}
pub fn angular_separation_deg(ra1:f64,dec1:f64,ra2:f64,dec2:f64)->f64{
 let r=std::f64::consts::PI/180.0;let(a,b,c,d)=(ra1*r,dec1*r,ra2*r,dec2*r);
 let cos=b.sin()*d.sin()+b.cos()*d.cos()*(a-c).cos();cos.clamp(-1.0,1.0).acos()/r
}
#[cfg(test)]
mod tests{use super::*;#[test]fn separation_is_zero_for_same_point(){assert!(angular_separation_deg(10.,20.,10.,20.)<1e-6);}#[test]fn separation_opposite_ra_equator(){assert!((angular_separation_deg(0.,0.,180.,0.)-180.).abs()<1e-9);}}
