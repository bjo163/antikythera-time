import { omegaRadiationFromH0 } from './parameters.js';
const ref=x=>Object.freeze(x);
export const COSMOLOGY_PRESETS=Object.freeze({
  planck2018:ref({id:'planck2018',label:'Planck 2018 base ΛCDM (TT,TE,EE+lowE+lensing)',model:'flat-lcdm',parameters:{H0:67.36,omegaM:0.3153},uncertainties:{H0:0.54,omegaM:0.0073},publishedAgeGyr:{mean:13.797,sigma:0.023},provenance:{publication:'Planck 2018 results. VI. Cosmological parameters',journal:'Astronomy & Astrophysics 641, A6 (2020)',doi:'10.1051/0004-6361/201833910',dataset:'Planck final full-mission CMB',model:'base flat ΛCDM',publicationDate:'2020-09-11',officialSource:'Planck Legacy Archive / A&A'}}),
  'desi-dr2-lcdm-cmb':ref({id:'desi-dr2-lcdm-cmb',label:'DESI DR2 BAO + CMB, ΛCDM',model:'flat-lcdm',parameters:{H0:68.17,omegaM:0.3027},uncertainties:{H0:0.28,omegaM:0.0036},provenance:{publication:'DESI DR2 Results II: Measurements of Baryon Acoustic Oscillations and Cosmological Constraints',journal:'Physical Review D 112, 083515 (2025)',arxiv:'2503.14738',dataset:'DESI DR2 BAO + baseline CMB combination',model:'flat ΛCDM',publicationDate:'2025-10-06',officialSource:'DESI Collaboration / Physical Review D',table:'Table V'}}),
  'desi-dr2-lcdm-curved-cmb':ref({id:'desi-dr2-lcdm-curved-cmb',label:'DESI DR2 BAO + CMB, ΛCDM + ΩK',model:'lcdm',parameters:{H0:68.50,omegaM:0.3034,omegaK:0.0023},uncertainties:{H0:0.33,omegaM:0.0037,omegaK:0.0011},provenance:{publication:'DESI DR2 Results II',journal:'Physical Review D 112, 083515 (2025)',arxiv:'2503.14738',dataset:'DESI DR2 BAO + CMB',model:'ΛCDM + curvature',publicationDate:'2025-10-06',officialSource:'DESI Collaboration / Physical Review D',table:'Table V'}}),
  'desi-dr2-w0wa-cmb-desy5':ref({id:'desi-dr2-w0wa-cmb-desy5',label:'DESI DR2 BAO + CMB + DESY5, w0waCDM',model:'w0wa-cdm',parameters:{H0:66.74,omegaM:0.3191,w0:-0.752,wa:-0.86},uncertainties:{H0:0.56,omegaM:0.0056,w0:0.057,wa:0.215},asymmetricUncertainties:{wa:{minus:0.20,plus:0.23}},provenance:{publication:'DESI DR2 Results II',journal:'Physical Review D 112, 083515 (2025)',arxiv:'2503.14738',dataset:'DESI DR2 BAO + CMB + DES Year 5 supernovae',model:'CPL w0waCDM',publicationDate:'2025-10-06',officialSource:'DESI Collaboration / Physical Review D',table:'Table V',note:'Level-1 independent uncertainty is only an approximation; use released DESI covariance/chains for faithful posterior age inference.'}})
});
export function getCosmologyPreset(id){const p=COSMOLOGY_PRESETS[String(id).toLowerCase()];if(!p)throw new RangeError(`unknown cosmology preset: ${id}`);return p;}
export function materializePresetParameters(preset,overrides={}){
  const base={...preset.parameters,...overrides},omegaR=omegaRadiationFromH0(base.H0);
  if(preset.model==='flat-lcdm')return {...base,omegaR,omegaLambda:1-omegaR-base.omegaM};
  if(preset.model==='lcdm')return {...base,omegaR,omegaLambda:1-omegaR-base.omegaM-base.omegaK};
  if(preset.model==='w0wa-cdm')return {...base,omegaR,omegaK:base.omegaK??0,omegaDE:1-omegaR-base.omegaM-(base.omegaK??0)};
  throw new RangeError(`unsupported preset model: ${preset.model}`);
}
export function listCosmologyPresets(){return Object.values(COSMOLOGY_PRESETS).map(p=>({id:p.id,label:p.label,model:p.model,parameters:materializePresetParameters(p),uncertainties:p.uncertainties,provenance:p.provenance}));}
