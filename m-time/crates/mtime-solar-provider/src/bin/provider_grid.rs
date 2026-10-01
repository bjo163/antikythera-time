use mtime_astro::Site;use mtime_solar_provider::state_at;
fn main(){
 let cases=[
  ("jakarta-ramadan",2461088.9166666665,-6.2,106.8,10.0),
  ("jakarta-syawal",2461118.95,-6.2,106.8,10.0),
  ("jakarta-zulhijjah",2461176.9166666665,-6.2,106.8,10.0),
  ("makkah-syawal",2461119.148611111,21.4225,39.8262,300.0),
  ("aceh-syawal",2461118.95,5.55,95.32,20.0),
  ("jayapura-syawal",2461118.95,-2.53,140.72,20.0),
  ("istanbul-syawal",2461119.15,41.0082,28.9784,40.0),
  ("newyork-syawal",2461119.2916666665,40.7128,-74.0060,10.0),
  ("auckland-syawal",2461118.80,-36.8485,174.7633,20.0),
 ];
 print!("[");
 for (i,(id,jd,lat,lon,h)) in cases.iter().enumerate(){
  let site=Site{id:(*id).into(),latitude_deg:*lat,longitude_deg:*lon,height_m:*h,datum:"WGS84".into()};
  let s=state_at(*jd,&site).expect("provider state");
  if i>0{print!(",")}
  print!("{{\"id\":\"{}\",\"jd_utc\":{},\"lat\":{},\"lon\":{},\"height_m\":{},\"altitude_deg\":{:.12},\"elongation_deg\":{:.12}}}",id,jd,lat,lon,h,s.moon_topocentric_altitude_deg,s.moon_sun_geocentric_elongation_deg);
 }
 println!("]");
}
