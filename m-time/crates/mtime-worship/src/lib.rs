use mtime_resolution::TemporalResolution;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum SacredMonthBoundary{Ramadan,Syawal,Zulhijjah}
#[derive(Debug,Clone,PartialEq)]
pub struct WorshipResolution{pub boundary:SacredMonthBoundary,pub resolution:TemporalResolution}
impl WorshipResolution{
 pub fn display_status(&self)->String{
  let computed=if self.resolution.criterion.pass{"criterion satisfied"}else{"criterion not satisfied"};
  let observed=if self.resolution.observations.is_empty(){"no observation data".into()}else{format!("{} observation record(s)",self.resolution.observations.len())};
  let official=self.resolution.official_calendar_result.clone().unwrap_or_else(||"no official decision".into());
  format!("computed: {computed}; observed: {observed}; official: {official}")
 }
}
#[cfg(test)]
mod tests{use super::*;#[test]fn three_boundaries_are_distinct(){assert_ne!(SacredMonthBoundary::Ramadan,SacredMonthBoundary::Syawal);assert_ne!(SacredMonthBoundary::Syawal,SacredMonthBoundary::Zulhijjah);}}
