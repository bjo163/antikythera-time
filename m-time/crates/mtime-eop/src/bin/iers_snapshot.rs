use std::{env,fs};
use mtime_eop::parse_finals_iau2000;
fn main(){
 let path=env::args().nth(1).expect("usage: iers_snapshot <finals.all>");
 let text=fs::read_to_string(path).expect("read file");
 let rows=parse_finals_iau2000(&text);
 assert!(rows.len()>1000,"unexpectedly few EOP rows: {}",rows.len());
 let first=&rows[0];let last=&rows[rows.len()-1];let observed=rows.iter().filter(|x|x.observed).last().unwrap_or(last);
 println!("rows={} first_mjd={} last_mjd={} latest_observed_mjd={}",rows.len(),first.mjd,last.mjd,observed.mjd);
}
