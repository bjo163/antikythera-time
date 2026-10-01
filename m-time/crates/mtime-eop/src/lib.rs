#[derive(Debug,Clone,PartialEq)]
pub struct EopRecord{pub mjd:f64,pub xp_arcsec:f64,pub yp_arcsec:f64,pub ut1_minus_utc_seconds:f64,pub observed:bool}
pub fn parse_finals_iau2000(text:&str)->Vec<EopRecord>{
 let mut out=Vec::new();
 for line in text.lines(){
  let p:Vec<&str>=line.split_whitespace().collect(); if p.len()<11{continue}
  let mjd=match p[3].parse::<f64>(){Ok(v)=>v,Err(_)=>continue};
  let mut i=4; let pole_flag=if matches!(p.get(i),Some(&"I")|Some(&"P")){let x=p[i];i+=1;Some(x)}else{None};
  let xp=match p.get(i).and_then(|x|x.parse::<f64>().ok()){Some(v)=>v,None=>continue};i+=2;
  let yp=match p.get(i).and_then(|x|x.parse::<f64>().ok()){Some(v)=>v,None=>continue};i+=2;
  let ut1_flag=if matches!(p.get(i),Some(&"I")|Some(&"P")){let x=p[i];i+=1;Some(x)}else{None};
  let ut1=match p.get(i).and_then(|x|x.parse::<f64>().ok()){Some(v)=>v,None=>continue};
  out.push(EopRecord{mjd,xp_arcsec:xp,yp_arcsec:yp,ut1_minus_utc_seconds:ut1,observed:pole_flag==Some("I")&&ut1_flag==Some("I")});
 }
 out
}
pub fn interpolate(rows:&[EopRecord],mjd:f64)->Result<EopRecord,&'static str>{
 if rows.len()<2{return Err("at least two EOP rows required")}
 if mjd<rows[0].mjd||mjd>rows[rows.len()-1].mjd{return Err("outside EOP coverage")}
 for w in rows.windows(2){if mjd>=w[0].mjd&&mjd<=w[1].mjd{let t=(mjd-w[0].mjd)/(w[1].mjd-w[0].mjd);let f=|a:f64,b:f64|a+(b-a)*t;return Ok(EopRecord{mjd,xp_arcsec:f(w[0].xp_arcsec,w[1].xp_arcsec),yp_arcsec:f(w[0].yp_arcsec,w[1].yp_arcsec),ut1_minus_utc_seconds:f(w[0].ut1_minus_utc_seconds,w[1].ut1_minus_utc_seconds),observed:w[0].observed&&w[1].observed})}}
 Err("interval not found")
}
#[cfg(test)]
mod tests{use super::*;#[test]fn parses_fixture(){let s="23 1 1 59945.00 I 0.062781 0.000012 0.200308 0.000009 I -0.0198681 0.0000071 0.1595 0.0041\n23 1 2 59946.00 I 0.061000 0.000012 0.201000 0.000009 I -0.0208681 0.0000071 0.1600 0.0041";let r=parse_finals_iau2000(s);assert_eq!(r.len(),2);let m=interpolate(&r,59945.5).unwrap();assert!((m.ut1_minus_utc_seconds+0.0203681).abs()<1e-10);}}
