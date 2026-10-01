#[derive(Debug,Clone,PartialEq)]
pub struct CosmicAgeInference{
 pub model:String,pub h0_km_s_mpc:f64,pub omega_m:f64,pub omega_lambda:f64,
 pub age_gyr:f64,pub numerical_error_gyr:f64,pub provenance:String
}
const MPC_M:f64=149_597_870_700.0*(648000.0/std::f64::consts::PI)*1_000_000.0;
const GYR_S:f64=1e9*31_557_600.0;
pub fn flat_lcdm_age(h0:f64,omega_m:f64,omega_lambda:f64)->Result<CosmicAgeInference,&'static str>{
 if !(h0>0.0&&omega_m>=0.0&&omega_lambda>=0.0){return Err("physical non-negative parameters and H0>0 required")}
 let f=|x:f64|{if x==0.0{0.0}else{let a=x*x;let e=(omega_m/a.powi(3)+omega_lambda).sqrt();2.0/(x*e)}};
 fn simpson<F:Fn(f64)->f64>(f:&F,n:usize)->f64{let n=if n%2==0{n}else{n+1};let h=1.0/n as f64;let mut s=f(0.0)+f(1.0);for i in 1..n{s+=if i%2==1{4.0*f(i as f64*h)}else{2.0*f(i as f64*h)}}s*h/3.0}
 let a=simpson(&f,200_000);let b=simpson(&f,400_000);let h0si=h0*1000.0/MPC_M;
 Ok(CosmicAgeInference{model:"flat-lcdm".into(),h0_km_s_mpc:h0,omega_m,omega_lambda,age_gyr:b/h0si/GYR_S,numerical_error_gyr:(b-a).abs()/h0si/GYR_S,provenance:"explicit model parameters; no Antikythera or revelation numerical prior".into()})
}
pub fn reject_antikythera_big_bang_claim()->Result<(),&'static str>{Err("Antikythera cycle phase cannot determine absolute elapsed cosmic history")}
#[cfg(test)]
mod tests{use super::*;#[test]fn planck_like_age_is_sane(){let r=flat_lcdm_age(67.36,0.3153,0.6847).unwrap();assert!(r.age_gyr>13.7&&r.age_gyr<13.9);assert!(r.numerical_error_gyr<1e-4);}#[test]fn semantic_boundary(){assert!(reject_antikythera_big_bang_claim().is_err());}}
