use mtime_astro::Site;
use mtime_solar_provider::state_at;
use serde_json::Value;

#[test]
fn frozen_jpl_hilal_corpus_stays_within_one_millidegree(){
    let raw=include_str!("../../../benchmarks/jpl-hilal-corpus-v1.json");
    let v:Value=serde_json::from_str(raw).unwrap();
    let threshold=v["offline_regression_threshold_deg"].as_f64().unwrap();
    for c in v["cases"].as_array().unwrap(){
        let site=Site{
            id:c["id"].as_str().unwrap().to_string(),
            latitude_deg:c["lat"].as_f64().unwrap(),
            longitude_deg:c["lon"].as_f64().unwrap(),
            height_m:c["height_m"].as_f64().unwrap(),
            datum:"WGS84".into()
        };
        let s=state_at(c["jd_utc"].as_f64().unwrap(),&site).unwrap();
        let alt_err=(s.moon_topocentric_altitude_deg-c["jpl_altitude_deg"].as_f64().unwrap()).abs();
        let elong_err=(s.moon_sun_geocentric_elongation_deg-c["jpl_elongation_deg"].as_f64().unwrap()).abs();
        assert!(alt_err<=threshold,"{} altitude error {}",site.id,alt_err);
        assert!(elong_err<=threshold,"{} elongation error {}",site.id,elong_err);
    }
}
