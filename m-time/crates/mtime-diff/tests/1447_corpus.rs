use mtime_diff::{CalendarTrace,DiffCategory,explain_calendar_trace_difference};

fn trace(sky:&str,scope:&str,rule:&str,outcome:&str,policy:&str,obs:Option<&str>,jurisdiction:&str,authority:&str,result:&str)->CalendarTrace{
 CalendarTrace{sky_event_id:sky.into(),observer_scope:scope.into(),rule_id:rule.into(),rule_version:"1447-profile".into(),rule_outcome:outcome.into(),observation_policy:policy.into(),observation_result:obs.map(str::to_string),jurisdiction:jurisdiction.into(),authority:authority.into(),official_result:result.into()}
}

#[test]
fn ramadan_1447_explains_one_day_difference(){
 let gov=trace("conjunction-2026-02-17","Indonesia","MABIMS-ID","not satisfied; hilal below horizon","hisab + rukyatulhilal + Sidang Isbat",Some("national rukyat network"),"Indonesia","Kemenag RI","1 Ramadan = 2026-02-19");
 let khgt=trace("conjunction-2026-02-17","global","KHGT-MUHAMMADIYAH","PKG2 satisfied","global hisab / PKG2",None,"global Muhammadiyah calendar","PP Muhammadiyah","1 Ramadan = 2026-02-18");
 let d=explain_calendar_trace_difference(&gov,&khgt);
 assert!(!d.iter().any(|x|x.category==DiffCategory::PhysicalInput));
 assert!(d.iter().any(|x|x.category==DiffCategory::Criterion));
 assert!(d.iter().any(|x|x.category==DiffCategory::Authority));
}

#[test]
fn zulhijjah_1447_can_share_date_but_keep_distinct_reasoning(){
 let gov=trace("conjunction-2026-05-16","Indonesia","MABIMS-ID","criterion satisfied on observation day","hisab + rukyatulhilal + Sidang Isbat",Some("88 sites; 2 accepted positive sightings"),"Indonesia","Kemenag RI","1 Zulhijjah = 2026-05-18");
 let khgt=trace("conjunction-2026-05-16","global","KHGT-MUHAMMADIYAH","PKG1/PKG2 not satisfied for prior-day start","global hisab",None,"global Muhammadiyah calendar","PP Muhammadiyah","1 Zulhijjah = 2026-05-18");
 let d=explain_calendar_trace_difference(&gov,&khgt);
 assert_eq!(gov.official_result,khgt.official_result);
 assert!(d.iter().any(|x|x.category==DiffCategory::Criterion));
 assert!(d.iter().any(|x|x.category==DiffCategory::Observation));
}
