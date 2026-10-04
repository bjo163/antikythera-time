#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum EopEvidence{Observed,Predicted}
#[derive(Debug,Clone,Copy,PartialEq)]pub struct EopRecord{pub mjd:f64,pub xp_arcsec:f64,pub yp_arcsec:f64,pub ut1_minus_utc_seconds:f64,pub lod_milliseconds:Option<f64>,pub evidence:EopEvidence}
#[derive(Debug,Clone,PartialEq,Eq)]pub enum EopError{InvalidLine,InsufficientData,OutsideCoverage,NonFinite}
fn parse_flagged_value<'a>(tokens:&'a [&'a str],i:&mut usize)->Option<(Option<&'a str>,f64)>{
    let token=*tokens.get(*i)?;
    if token=="I"||token=="P"{
        let flag=Some(token);
        *i+=1;
        let value=tokens.get(*i)?.parse::<f64>().ok()?;
        *i+=1;
        return Some((flag,value));
    }
    if token.len()>1{
        let (head,tail)=token.split_at(1);
        if (head=="I"||head=="P")&&!tail.is_empty(){
            if let Ok(value)=tail.parse::<f64>(){
                *i+=1;
                return Some((Some(head),value));
            }
        }
    }
    let value=token.parse::<f64>().ok()?;
    *i+=1;
    Some((None,value))
}

pub fn parse_finals2000a(text:&str)->Vec<EopRecord>{
    let mut rows=Vec::new();
    for raw in text.lines(){
        let p=raw.split_whitespace().collect::<Vec<_>>();
        if p.len()<9{continue;}
        let Some((mjd_index,mjd))=p.iter().take(5).enumerate().find_map(|(index,token)|{
            token.parse::<f64>().ok().filter(|value|*value>30_000.0&&*value<100_000.0).map(|value|(index,value))
        })else{continue;};
        let mut i=mjd_index+1;

        let Some((pole_flag,xp))=parse_flagged_value(&p,&mut i)else{continue;};
        if p.get(i).and_then(|x|x.parse::<f64>().ok()).is_none(){continue;}
        i+=1;
        let Some(yp)=p.get(i).and_then(|x|x.parse::<f64>().ok())else{continue;};
        i+=1;
        if p.get(i).and_then(|x|x.parse::<f64>().ok()).is_none(){continue;}
        i+=1;

        let Some((ut1_flag,ut1))=parse_flagged_value(&p,&mut i)else{continue;};
        if p.get(i).and_then(|x|x.parse::<f64>().ok()).is_none(){continue;}
        i+=1;
        let lod=p.get(i).and_then(|x|x.parse::<f64>().ok());

        if [mjd,xp,yp,ut1].iter().all(|v|v.is_finite()){
            rows.push(EopRecord{
                mjd,
                xp_arcsec:xp,
                yp_arcsec:yp,
                ut1_minus_utc_seconds:ut1,
                lod_milliseconds:lod,
                evidence:if pole_flag==Some("I")&&ut1_flag==Some("I"){
                    EopEvidence::Observed
                }else{
                    EopEvidence::Predicted
                },
            });
        }
    }
    rows.sort_by(|a,b|a.mjd.total_cmp(&b.mjd));
    rows
}
fn lerp(a:f64,b:f64,t:f64)->f64{a+(b-a)*t}
pub fn interpolate(rows:&[EopRecord],mjd:f64)->Result<EopRecord,EopError>{if !mjd.is_finite(){return Err(EopError::NonFinite);}if rows.len()<2{return Err(EopError::InsufficientData);}if mjd<rows[0].mjd||mjd>rows[rows.len()-1].mjd{return Err(EopError::OutsideCoverage);}let mut lo=0usize;let mut hi=rows.len()-1;while hi-lo>1{let mid=(lo+hi)/2;if rows[mid].mjd<=mjd{lo=mid}else{hi=mid}}if (mjd-rows[lo].mjd).abs()<f64::EPSILON{return Ok(rows[lo]);}let a=rows[lo];let b=rows[hi];let t=(mjd-a.mjd)/(b.mjd-a.mjd);Ok(EopRecord{mjd,xp_arcsec:lerp(a.xp_arcsec,b.xp_arcsec,t),yp_arcsec:lerp(a.yp_arcsec,b.yp_arcsec,t),ut1_minus_utc_seconds:lerp(a.ut1_minus_utc_seconds,b.ut1_minus_utc_seconds,t),lod_milliseconds:match(a.lod_milliseconds,b.lod_milliseconds){(Some(x),Some(y))=>Some(lerp(x,y,t)),_=>None},evidence:if a.evidence==EopEvidence::Observed&&b.evidence==EopEvidence::Observed{EopEvidence::Observed}else{EopEvidence::Predicted}})}
#[must_use]pub fn utc_jd_to_ut1_jd(utc_jd:f64,eop:EopRecord)->f64{utc_jd+eop.ut1_minus_utc_seconds/86_400.0}
#[cfg(test)]mod tests{use super::*;const FIX:&str="23 1 1 59945.00 I 0.062781 0.000012 0.200308 0.000009 I -0.0198681 0.0000071 0.1595 0.0041\n23 1 2 59946.00 I 0.061000 0.000012 0.201000 0.000009 I -0.0208681 0.0000071 0.1600 0.0041";const FUSED_NEGATIVE:&str="26 921 61304.00 I 0.185007 0.000091 0.328594 0.000090 I-0.0106308 0.0000220 0.7149 0.0156 P 0.138 0.128 0.188 0.160\n26 922 61305.00 I 0.183762 0.000090 0.328161 0.000091 I-0.0114081 0.0000226 0.8498 0.0168 P 0.137 0.128 0.192 0.160";#[test]fn parses_iers_fixture(){let r=parse_finals2000a(FIX);assert_eq!(r.len(),2);assert_eq!(r[0].evidence,EopEvidence::Observed);assert!((r[0].ut1_minus_utc_seconds+0.0198681).abs()<1e-12);}#[test]fn interpolates_ut1(){let r=parse_finals2000a(FIX);let e=interpolate(&r,59945.5).unwrap();assert!((e.ut1_minus_utc_seconds+0.0203681).abs()<1e-10);}#[test]fn rejects_outside_coverage(){let r=parse_finals2000a(FIX);assert_eq!(interpolate(&r,59944.0),Err(EopError::OutsideCoverage));}
#[test]fn parses_official_fused_negative_ut1_flag(){let r=parse_finals2000a(FUSED_NEGATIVE);assert_eq!(r.len(),2);assert!((r[0].ut1_minus_utc_seconds+0.0106308).abs()<1e-12);assert_eq!(r[0].evidence,EopEvidence::Observed);let mid=interpolate(&r,61304.5).unwrap();assert!(mid.ut1_minus_utc_seconds<0.0);}}
