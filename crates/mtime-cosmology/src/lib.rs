use mtime_core::{EvidenceState,Provenance,QualityClass,TemporalError};
const MPC_M:f64=3.085_677_581_491_367e22;const GYR_SECONDS:f64=1e9*31_557_600.0;
#[derive(Debug,Clone,Copy,PartialEq)]pub enum CosmologyModel{FlatLambdaCdm{h0_km_s_mpc:f64,omega_r:f64,omega_m:f64,omega_lambda:f64},LambdaCdm{h0_km_s_mpc:f64,omega_r:f64,omega_m:f64,omega_lambda:f64,omega_k:f64},W0WaCdm{h0_km_s_mpc:f64,omega_r:f64,omega_m:f64,omega_de:f64,omega_k:f64,w0:f64,wa:f64}}
impl CosmologyModel{fn h0(self)->f64{match self{Self::FlatLambdaCdm{h0_km_s_mpc,..}|Self::LambdaCdm{h0_km_s_mpc,..}|Self::W0WaCdm{h0_km_s_mpc,..}=>h0_km_s_mpc}}fn e2(self,a:f64)->f64{match self{Self::FlatLambdaCdm{omega_r,omega_m,omega_lambda,..}=>omega_r/a.powi(4)+omega_m/a.powi(3)+omega_lambda,Self::LambdaCdm{omega_r,omega_m,omega_lambda,omega_k,..}=>omega_r/a.powi(4)+omega_m/a.powi(3)+omega_k/a.powi(2)+omega_lambda,Self::W0WaCdm{omega_r,omega_m,omega_de,omega_k,w0,wa,..}=>{let de=omega_de*a.powf(-3.0*(1.0+w0+wa))*(-3.0*wa*(1.0-a)).exp();omega_r/a.powi(4)+omega_m/a.powi(3)+omega_k/a.powi(2)+de}}}}
#[derive(Debug,Clone,PartialEq)]pub struct CosmicAgeInference{pub gyr:f64,pub seconds:f64,pub integration_error_gyr:f64,pub model:CosmologyModel,pub evidence:EvidenceState,pub quality:QualityClass,pub provenance:Provenance,pub absolute_cosmic_clock:bool}
fn simpson(f:&impl Fn(f64)->f64,a:f64,b:f64)->f64{let c=(a+b)/2.0;(b-a)*(f(a)+4.0*f(c)+f(b))/6.0}
fn adaptive(f:&impl Fn(f64)->f64,a:f64,b:f64,eps:f64,whole:f64,depth:u32)->(f64,f64){let c=(a+b)/2.0;let left=simpson(f,a,c);let right=simpson(f,c,b);let delta=left+right-whole;if depth==0||delta.abs()<=15.0*eps{(left+right+delta/15.0,delta.abs()/15.0)}else{let(lv,le)=adaptive(f,a,c,eps/2.0,left,depth-1);let(rv,re)=adaptive(f,c,b,eps/2.0,right,depth-1);(lv+rv,le+re)}}
pub fn infer_age(model:CosmologyModel,tolerance:f64,provenance:Provenance)->Result<CosmicAgeInference,TemporalError>{if !tolerance.is_finite()||tolerance<=0.0||!model.h0().is_finite()||model.h0()<=0.0{return Err(TemporalError::InvalidInput("invalid cosmology input"));}let f=|x:f64|{if x==0.0{0.0}else{let a=x*x;let e2=model.e2(a);if e2<=0.0||!e2.is_finite(){f64::NAN}else{2.0/(x*e2.sqrt())}}};let whole=simpson(&f,0.0,1.0);if !whole.is_finite(){return Err(TemporalError::InvalidInput("non-positive expansion history"));}let(integral,err)=adaptive(&f,0.0,1.0,tolerance,whole,28);let h0_si=model.h0()*1000.0/MPC_M;let seconds=integral/h0_si;Ok(CosmicAgeInference{gyr:seconds/GYR_SECONDS,seconds,integration_error_gyr:(err/h0_si)/GYR_SECONDS,model,evidence:EvidenceState::Inferred,quality:QualityClass::Reference,provenance,absolute_cosmic_clock:false})}
#[must_use]pub fn planck_2018_flat_reference()->CosmologyModel{let h0=67.36;let h=h0/100.0;let omega_r=2.4728e-5*(1.0+0.227_107_317_66*3.046)/(h*h);let omega_m=0.3153;CosmologyModel::FlatLambdaCdm{h0_km_s_mpc:h0,omega_r,omega_m,omega_lambda:1.0-omega_r-omega_m}}
#[cfg(test)]mod tests{use super::*;#[test]fn planck_age_is_near_reference(){let r=infer_age(planck_2018_flat_reference(),1e-10,Provenance::new("Planck 2018")).unwrap();assert!((r.gyr-13.797).abs()<0.02);assert!(!r.absolute_cosmic_clock);}#[test]fn eds_matches_two_thirds_hubble_time(){let m=CosmologyModel::FlatLambdaCdm{h0_km_s_mpc:70.0,omega_r:0.0,omega_m:1.0,omega_lambda:0.0};let r=infer_age(m,1e-10,Provenance::new("analytic")).unwrap();let h0=70.0*1000.0/MPC_M;let expected=(2.0/(3.0*h0))/GYR_SECONDS;assert!((r.gyr-expected).abs()<1e-7);}}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CosmologyProfileKind {
    ReferenceFit,
    SensitivityScenario,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CosmologyProfile {
    pub id: &'static str,
    pub kind: CosmologyProfileKind,
    pub model: CosmologyModel,
    pub provenance: &'static str,
    pub operational_clock_input: bool,
}

#[must_use]
pub fn planck_2018_profile() -> CosmologyProfile {
    CosmologyProfile {
        id: "PLANCK_2018_FLAT_LCDM_REFERENCE",
        kind: CosmologyProfileKind::ReferenceFit,
        model: planck_2018_flat_reference(),
        provenance: "Planck-2018-like reference parameters already used by mtime-cosmology",
        operational_clock_input: false,
    }
}

#[must_use]
pub fn high_h0_sensitivity_profile() -> CosmologyProfile {
    let h0 = 73.0;
    let h = h0 / 100.0;
    let omega_r = 2.4728e-5 * (1.0 + 0.227_107_317_66 * 3.046) / (h * h);
    let omega_m = 0.3153;
    CosmologyProfile {
        id: "H0_73_SENSITIVITY_ONLY",
        kind: CosmologyProfileKind::SensitivityScenario,
        model: CosmologyModel::FlatLambdaCdm {
            h0_km_s_mpc: h0,
            omega_r,
            omega_m,
            omega_lambda: 1.0 - omega_r - omega_m,
        },
        provenance: "M-Time sensitivity scenario; not an asserted preferred cosmological fit",
        operational_clock_input: false,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CosmicEpochMapping {
    pub id: String,
    pub inferred_age_gyr: f64,
    pub model_profile_id: String,
    pub research_only: bool,
    pub operational_mtime_coordinate: Option<i128>,
}

#[must_use]
pub fn research_epoch_mapping(
    id: impl Into<String>,
    inference: &CosmicAgeInference,
    profile: CosmologyProfile,
) -> CosmicEpochMapping {
    CosmicEpochMapping {
        id: id.into(),
        inferred_age_gyr: inference.gyr,
        model_profile_id: profile.id.into(),
        research_only: true,
        operational_mtime_coordinate: None,
    }
}

#[cfg(test)]
mod cosmology_isolation_tests {
    use super::*;

    #[test]
    fn alternative_models_change_inferred_age_but_never_operational_clock() {
        let p = planck_2018_profile();
        let s = high_h0_sensitivity_profile();
        let a = infer_age(p.model, 1e-9, Provenance::new(p.provenance)).unwrap();
        let b = infer_age(s.model, 1e-9, Provenance::new(s.provenance)).unwrap();
        assert!((a.gyr - b.gyr).abs() > 0.5);
        assert!(!p.operational_clock_input && !s.operational_clock_input);
        assert_eq!(
            research_epoch_mapping("cosmic-age", &a, p).operational_mtime_coordinate,
            None
        );
    }
}
