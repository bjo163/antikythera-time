#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum CosmologyModel { FlatLambdaCdm, LambdaCdm, W0WaCdm }

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct CosmologyParameters {
 pub h0_km_s_mpc:f64,
 pub omega_r:f64,
 pub omega_m:f64,
 pub omega_k:f64,
 pub omega_de:f64,
 pub w0:f64,
 pub wa:f64,
}

#[derive(Debug,Clone,PartialEq)]
pub struct CosmicAgeInference{
 pub model:CosmologyModel,
 pub parameters:CosmologyParameters,
 pub age_gyr:f64,
 pub numerical_error_gyr:f64,
 pub provenance:String,
}

#[derive(Debug,Clone,PartialEq)]
pub struct PosteriorAgeSummary {
 pub count:usize,
 pub total_weight:f64,
 pub mean_gyr:f64,
 pub stddev_gyr:f64,
 pub p16_gyr:f64,
 pub median_gyr:f64,
 pub p84_gyr:f64,
}

const MPC_M:f64=149_597_870_700.0*(648000.0/std::f64::consts::PI)*1_000_000.0;
const GYR_S:f64=1e9*31_557_600.0;

pub fn omega_radiation_from_h0(h0:f64)->f64{
 let h=h0/100.0;2.4728e-5*(1.0+0.22710731766*3.046)/(h*h)
}

impl CosmologyParameters {
 pub fn flat_lcdm(h0:f64,omega_m:f64)->Result<Self,&'static str>{
  if !(h0>0.0&&omega_m>=0.0){return Err("H0>0 and omega_m>=0 required")}
  let omega_r=omega_radiation_from_h0(h0);
  let omega_de=1.0-omega_r-omega_m;
  if omega_de<0.0{return Err("derived dark-energy density is negative")}
  Ok(Self{h0_km_s_mpc:h0,omega_r,omega_m,omega_k:0.0,omega_de,w0:-1.0,wa:0.0})
 }
 pub fn validate(&self)->Result<(),&'static str>{
  if ![self.h0_km_s_mpc,self.omega_r,self.omega_m,self.omega_k,self.omega_de,self.w0,self.wa].iter().all(|x|x.is_finite()){return Err("finite cosmology parameters required")}
  if self.h0_km_s_mpc<=0.0||self.omega_r<0.0||self.omega_m<0.0||self.omega_de<0.0{return Err("unphysical cosmology density/H0")}
  Ok(())
 }
}

pub fn expansion_e2(model:CosmologyModel,p:&CosmologyParameters,a:f64)->Result<f64,&'static str>{
 p.validate()?;if !(a>0.0&&a<=1.0){return Err("scale factor must be (0,1]")}
 let de=match model{
  CosmologyModel::FlatLambdaCdm|CosmologyModel::LambdaCdm=>p.omega_de,
  CosmologyModel::W0WaCdm=>{
   let power=-3.0*(1.0+p.w0+p.wa);
   p.omega_de*a.powf(power)*(-3.0*p.wa*(1.0-a)).exp()
  }
 };
 let k=match model{CosmologyModel::FlatLambdaCdm=>0.0,_=>p.omega_k};
 let e2=p.omega_r/a.powi(4)+p.omega_m/a.powi(3)+k/a.powi(2)+de;
 if !(e2>0.0&&e2.is_finite()){return Err("non-positive/non-finite expansion rate")}
 Ok(e2)
}

fn simpson<F:Fn(f64)->f64>(f:&F,n:usize)->f64{
 let n=if n%2==0{n}else{n+1};let h=1.0/n as f64;let mut s=f(0.0)+f(1.0);
 for i in 1..n{s+=if i%2==1{4.0*f(i as f64*h)}else{2.0*f(i as f64*h)}} s*h/3.0
}

pub fn infer_cosmic_age(model:CosmologyModel,p:CosmologyParameters)->Result<CosmicAgeInference,&'static str>{
 p.validate()?;
 if model==CosmologyModel::FlatLambdaCdm && p.omega_k.abs()>1e-12{return Err("flat model requires omega_k=0")}
 let f=|x:f64|{
  if x==0.0{return 0.0}
  let a=x*x;
  match expansion_e2(model,&p,a){Ok(e2)=>2.0/(x*e2.sqrt()),Err(_)=>f64::NAN}
 };
 let a=simpson(&f,100_000);let b=simpson(&f,200_000);
 if !a.is_finite()||!b.is_finite(){return Err("age integration failed")}
 let h0si=p.h0_km_s_mpc*1000.0/MPC_M;
 Ok(CosmicAgeInference{model,parameters:p,age_gyr:b/h0si/GYR_S,numerical_error_gyr:(b-a).abs()/h0si/GYR_S,provenance:"declared cosmology model/parameters; no Antikythera or revelation numerical prior".into()})
}

pub fn flat_lcdm_age(h0:f64,omega_m:f64,omega_lambda:f64)->Result<CosmicAgeInference,&'static str>{
 let omega_r=0.0;let p=CosmologyParameters{h0_km_s_mpc:h0,omega_r,omega_m,omega_k:0.0,omega_de:omega_lambda,w0:-1.0,wa:0.0};
 infer_cosmic_age(CosmologyModel::FlatLambdaCdm,p)
}

fn weighted_quantile(mut x:Vec<(f64,f64)>,q:f64)->f64{
 x.sort_by(|a,b|a.0.partial_cmp(&b.0).unwrap());let total:x_compat::F64=x.iter().map(|v|v.1).sum();
 let target=q*total.0;let mut s=0.0;for(v,w)in x{s+=w;if s>=target{return v}}f64::NAN
}
mod x_compat{
 #[derive(Clone,Copy)]pub struct F64(pub f64);
 impl std::iter::Sum<f64> for F64{fn sum<I:Iterator<Item=f64>>(iter:I)->Self{Self(iter.fold(0.0,|a,b|a+b))}}
}

pub fn summarize_age_chain(samples:&[(CosmologyModel,CosmologyParameters,f64)])->Result<PosteriorAgeSummary,&'static str>{
 if samples.is_empty(){return Err("posterior samples required")}
 let mut ages=Vec::new();let mut total=0.0;let mut weighted=0.0;
 for(model,p,w)in samples{
  if !w.is_finite()||*w<=0.0{return Err("positive finite posterior weights required")}
  let age=infer_cosmic_age(*model,*p)?.age_gyr;ages.push((age,*w));total+=*w;weighted+=age**w;
 }
 let mean=weighted/total;let var=ages.iter().map(|(a,w)|w*(a-mean)*(a-mean)).sum::<f64>()/total;
 Ok(PosteriorAgeSummary{count:ages.len(),total_weight:total,mean_gyr:mean,stddev_gyr:var.sqrt(),p16_gyr:weighted_quantile(ages.clone(),0.16),median_gyr:weighted_quantile(ages.clone(),0.5),p84_gyr:weighted_quantile(ages,0.84)})
}

pub fn reject_antikythera_big_bang_claim()->Result<(),&'static str>{Err("Antikythera cycle phase cannot determine absolute elapsed cosmic history")}

#[cfg(test)]
mod tests{
 use super::*;
 #[test]fn planck_like_age_is_sane(){let p=CosmologyParameters::flat_lcdm(67.36,0.3153).unwrap();let r=infer_cosmic_age(CosmologyModel::FlatLambdaCdm,p).unwrap();assert!(r.age_gyr>13.7&&r.age_gyr<13.9);assert!(r.numerical_error_gyr<1e-3);}
 #[test]fn curved_model_changes_age(){let mut p=CosmologyParameters::flat_lcdm(68.5,0.3034).unwrap();p.omega_k=0.0023;p.omega_de-=0.0023;let r=infer_cosmic_age(CosmologyModel::LambdaCdm,p).unwrap();assert!(r.age_gyr>13.0&&r.age_gyr<14.5);}
 #[test]fn cpl_reduces_to_lambda_at_minus_one_zero(){let p=CosmologyParameters::flat_lcdm(70.0,0.3).unwrap();let a=infer_cosmic_age(CosmologyModel::FlatLambdaCdm,p).unwrap();let b=infer_cosmic_age(CosmologyModel::W0WaCdm,p).unwrap();assert!((a.age_gyr-b.age_gyr).abs()<1e-10);}
 #[test]fn posterior_summary_is_weighted(){let a=CosmologyParameters::flat_lcdm(67.0,0.31).unwrap();let b=CosmologyParameters::flat_lcdm(68.0,0.31).unwrap();let s=summarize_age_chain(&[(CosmologyModel::FlatLambdaCdm,a,3.0),(CosmologyModel::FlatLambdaCdm,b,1.0)]).unwrap();assert_eq!(s.count,2);assert!(s.p16_gyr<=s.median_gyr&&s.median_gyr<=s.p84_gyr);}
 #[test]fn semantic_boundary(){assert!(reject_antikythera_big_bang_claim().is_err());}
}
