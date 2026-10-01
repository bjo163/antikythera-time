use std::f64::consts::PI;
use std::fs;

const DAY:f64=86400.0;
const LG:f64=6.969290134e-10;
const MJD0:f64=2400000.5;
const MJD1977:f64=43144.0;
const TT_MINUS_TAI:f64=32.184;
const AU_M:f64=149_597_870_700.0;
const MPC_M:f64=AU_M*(648000.0/PI)*1_000_000.0;
const GYR_S:f64=1e9*31_557_600.0;

fn tt_to_tcg(d1:f64,d2:f64)->(f64,f64){
 let t77t=MJD1977+TT_MINUS_TAI/DAY;
 let rate=LG/(1.0-LG);
 if d1.abs()>d2.abs(){(d1,d2+((d1-MJD0)+(d2-t77t))*rate)}
 else {(d1+((d2-MJD0)+(d1-t77t))*rate,d2)}
}
fn omega_r(h0:f64)->f64{let h=h0/100.0;2.4728e-5*(1.0+0.22710731766*3.046)/(h*h)}
fn planck_age()->f64{
 let h0=67.36;let om=0.3153;let orad=omega_r(h0);let ol=1.0-orad-om;
 let n=400000usize;let h=1.0/n as f64;let mut s=0.0;
 for i in 0..=n{
   let x=i as f64*h;
   let f=if x==0.0{0.0}else{let a=x*x;let e=(orad/a.powi(4)+om/a.powi(3)+ol).sqrt();2.0/(x*e)};
   let w=if i==0||i==n{1.0}else if i%2==1{4.0}else{2.0};
   s+=w*f;
 }
 let integral=s*h/3.0;let h0si=h0*1000.0/MPC_M;integral/h0si/GYR_S
}
fn mars()->(f64,f64,f64){
 let a:f64=1.52371034;let e:f64=0.09339410;let inc:f64=1.84969142;let l:f64=-4.55343205;let peri:f64=-23.94362959;let node:f64=49.55953891;
 let d2r=PI/180.0;let w=peri-node;let mut m=(l-peri+180.0)%360.0-180.0;if m < -180.0 {m+=360.0;}
 let estar=180.0/PI*e;let mut ee=m+estar*(m*d2r).sin();
 for _ in 0..20 {let dm=m-(ee-estar*(ee*d2r).sin());let de=dm/(1.0-e*(ee*d2r).cos());ee+=de;if de.abs()<=1e-10{break;}}
 let xp=a*((ee*d2r).cos()-e);let yp=a*(1.0-e*e).sqrt()*(ee*d2r).sin();
 let wr=w*d2r;let nr=node*d2r;let ir=inc*d2r;
 let (cw,sw,co,so,ci,si)=(wr.cos(),wr.sin(),nr.cos(),nr.sin(),ir.cos(),ir.sin());
 let x=(cw*co-sw*so*ci)*xp+(-sw*co-cw*so*ci)*yp;
 let y=(cw*so+sw*co*ci)*xp+(-sw*so+cw*co*ci)*yp;
 let z=(sw*si)*xp+(cw*si)*yp;(x,y,z)
}
fn main(){
 let (_,tcg2)=tt_to_tcg(2453750.5,0.892482639);let age=planck_age();let saros=2460409.263+6585.3223;let (x,y,z)=mars();
 let out=format!(r#"{{"implementation":"rust-independent","ttToTcgD2":{:.17},"planckAgeGyr":{:.12},"sarosJdTt":{:.10},"mars":{{"x":{:.17},"y":{:.17},"z":{:.17}}}}}"#,tcg2,age,saros,x,y,z);
 fs::create_dir_all("artifacts").unwrap();fs::write("artifacts/v2-rust.json",&out).unwrap();println!("{}",out);
}
