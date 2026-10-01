use mtime_astro::Site;
use mtime_core::{CoordinateTime,ReferenceFrame,TimeScale};
use mtime_solar_provider::{find_conjunction_tt,state_at,sunset_jd_utc};
use mtime_timescales::tt_to_tai;

fn tt_jd_to_utc_jd_2026(jd_tt:f64)->f64{
    let tt=CoordinateTime::new(jd_tt.floor(),jd_tt-jd_tt.floor(),TimeScale::TT,ReferenceFrame::GCRS).unwrap();
    let tai=tt_to_tai(tt).unwrap();
    // TAI-UTC = 37 s throughout 2026.
    tai.jd()-37.0/86400.0
}
fn main(){
    let makkah=Site{id:"Makkah".into(),latitude_deg:21.4225,longitude_deg:39.8262,height_m:300.0,datum:"WGS84".into()};
    let conjunction_tt=find_conjunction_tt(2461117.5,2461119.0).expect("conjunction");
    let conjunction_utc=tt_jd_to_utc_jd_2026(conjunction_tt);
    let sunset=sunset_jd_utc(2461118.5,&makkah).expect("Makkah sunset");
    let state=state_at(sunset,&makkah).expect("state at sunset");
    println!("{{\"conjunction_jd_tt\":{:.12},\"conjunction_jd_utc\":{:.12},\"makkah_sunset_jd_utc\":{:.12},\"makkah_geocentric_altitude_deg\":{:.12},\"makkah_topocentric_altitude_deg\":{:.12},\"makkah_elongation_deg\":{:.12}}}",
      conjunction_tt,conjunction_utc,sunset,state.moon_geocentric_altitude_deg,state.moon_topocentric_altitude_deg,state.moon_sun_geocentric_elongation_deg);
}
