import { inferCosmicAge,einsteinDeSitterAnalyticAgeSeconds,flatMatterLambdaAnalyticAgeSeconds } from './age.js';
import { hubbleKmSmpcToSI } from './parameters.js';
import { getCosmologyPreset,materializePresetParameters } from './references.js';
const relError=(a,b)=>Math.abs(a-b)/Math.max(Math.abs(b),Number.MIN_VALUE);
export function validateH0Dimensions(H0=70){const si=hubbleKmSmpcToSI(H0);return{H0,H0SI:si,hubbleTimeSeconds:1/si,pass:si>1e-19&&si<1e-16};}
export function validateEinsteinDeSitter({H0=70,tolerance=1e-8}={}){const numeric=inferCosmicAge('flat-lcdm',{H0,omegaR:0,omegaM:1,omegaLambda:0},{normalization:{flatnessTolerance:1e-12,consistencyMode:'reject'}}),analytic=einsteinDeSitterAnalyticAgeSeconds(H0),error=relError(numeric.result.seconds,analytic);return{numeric:numeric.result.seconds,analytic,relativeError:error,pass:error<=tolerance};}
export function validateFlatMatterLambda({H0=70,omegaM=.3,tolerance=1e-8}={}){const omegaLambda=1-omegaM,numeric=inferCosmicAge('flat-lcdm',{H0,omegaR:0,omegaM,omegaLambda},{normalization:{flatnessTolerance:1e-12,consistencyMode:'reject'}}),analytic=flatMatterLambdaAnalyticAgeSeconds({H0,omegaM,omegaLambda}),error=relError(numeric.result.seconds,analytic);return{numeric:numeric.result.seconds,analytic,relativeError:error,pass:error<=tolerance};}
export function validatePlanckSanity({toleranceGyr=.05}={}){const preset=getCosmologyPreset('planck2018'),params=materializePresetParameters(preset),result=inferCosmicAge(preset.model,params,{parameterSource:preset.id,observationalSource:preset.provenance.dataset}),delta=result.result.gyr-preset.publishedAgeGyr.mean;return{computedGyr:result.result.gyr,published:preset.publishedAgeGyr,deltaGyr:delta,pass:Math.abs(delta)<=toleranceGyr};}
export function integrationStability(model,parameters){const settings=[{absoluteTolerance:1e-8,relativeTolerance:1e-8},{absoluteTolerance:1e-10,relativeTolerance:1e-10},{absoluteTolerance:1e-12,relativeTolerance:1e-12}],results=settings.map(integration=>inferCosmicAge(model,parameters,{integration}).result.gyr),span=Math.max(...results)-Math.min(...results);return{settings,results,spanGyr:span,pass:span<1e-6};}
export function phaseAgeDegeneracy(period,phase,nA=1,nB=10){if(!(period>0)||!Number.isFinite(phase))throw new RangeError('invalid period/phase');const tA=(phase+nA)*period,tB=(phase+nB)*period;return{phase,period,tA,tB,samePhaseModuloPeriod:Math.abs(((tA-tB)/period)-Math.round((tA-tB)/period))<1e-12,absoluteAgeDetermined:false};}
export function rejectAntikytheraBigBangSemanticMisuse(claim){const s=String(claim).toLowerCase();if(s.includes('antikythera')&&(s.includes('big bang')||s.includes('age of the universe')||s.includes('umur semesta')))throw new Error('SEMANTIC_BOUNDARY: Antikythera cycles encode recurrence/phase, not the Big Bang date or cosmic age.');return true;}


export function parameterSensitivity() {
  const flat = (H0, omegaM) => inferCosmicAge('flat-lcdm', { H0, omegaR: 0, omegaM, omegaLambda: 1 - omegaM }).result.gyr;
  const curved = (omegaLambda) => inferCosmicAge('lcdm', { H0: 70, omegaR: 0, omegaM: 0.3, omegaLambda, omegaK: 1 - 0.3 - omegaLambda }).result.gyr;
  const cpl = (w0, wa) => inferCosmicAge('w0wa-cdm', { H0: 70, omegaR: 0, omegaM: 0.3, omegaDE: 0.7, omegaK: 0, w0, wa }).result.gyr;
  return {
    H0: { low: flat(67, 0.3), high: flat(73, 0.3), expected: 'higher H0 -> lower age at fixed dimensionless expansion history' },
    omegaM: { low: flat(70, 0.25), high: flat(70, 0.35), expected: 'more matter in flat closure -> younger inferred age' },
    omegaLambda: { low: curved(0.60), high: curved(0.75), expected: 'larger Lambda contribution at fixed H0 and matter changes the expansion history and age' },
    w0: { minusOne: cpl(-1, 0), lessNegative: cpl(-0.9, 0), expected: 'changing dark-energy equation of state changes the integrated expansion history' },
    wa: { zero: cpl(-1, 0), negative: cpl(-1, -0.4), expected: 'evolving dark energy changes the integrated expansion history' },
  };
}
