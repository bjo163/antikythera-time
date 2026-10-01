use std::{env,fs,process};
use mtime_protocol::from_json;
fn main(){
 let files:Vec<String>=env::args().skip(1).collect();if files.is_empty(){eprintln!("usage: conformance <record.json> [...]");process::exit(2)}
 let mut failed=false;
 for f in files{
  match fs::read_to_string(&f).ok().and_then(|s|from_json(&s).ok()){
   Some(r)=>match r.validate(){Ok(())=>println!("PASS {f}"),Err(e)=>{eprintln!("FAIL {f}: {}",e.join("; "));failed=true}},
   None=>{eprintln!("FAIL {f}: parse error");failed=true}
  }
 }
 if failed{process::exit(1)}
}
