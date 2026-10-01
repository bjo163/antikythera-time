use mtime_astro::Site;use mtime_solar_provider::state_at;
fn main(){
 let site=Site{id:"Jakarta".into(),latitude_deg:-6.2,longitude_deg:106.8,height_m:10.0,datum:"WGS84".into()};
 let s=state_at(2461118.95,&site).expect("provider state");
 println!("{{\"jd_utc\":2461118.95,\"altitude_deg\":{:.12},\"elongation_deg\":{:.12}}}",s.moon_topocentric_altitude_deg,s.moon_sun_geocentric_elongation_deg);
}
