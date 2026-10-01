import { normalizeCosmologyParameters } from './parameters.js';
export function cplDarkEnergyRelativeDensity(a,w0,wa){if(!(a>0&&a<=1))throw new RangeError('scale factor a must be in (0,1]');return Math.pow(a,-3*(1+w0+wa))*Math.exp(-3*wa*(1-a));}
export function expansionE2(a,normalized){
  if(!(a>0&&a<=1))throw new RangeError('scale factor a must be in (0,1]');
  const {model,parameters:p}=normalized;
  const radiation=p.omegaR/Math.pow(a,4),matter=p.omegaM/Math.pow(a,3),curvature=(p.omegaK??0)/Math.pow(a,2);
  let e2;
  if(model==='flat-lcdm'||model==='lcdm')e2=radiation+matter+curvature+p.omegaLambda;
  else if(model==='w0wa-cdm')e2=radiation+matter+curvature+p.omegaDE*cplDarkEnergyRelativeDensity(a,p.w0,p.wa);
  else throw new RangeError(`unsupported model ${model}`);
  if(!(e2>0)||!Number.isFinite(e2))throw new RangeError(`E(a)^2 is non-positive or non-finite at a=${a}`);
  return e2;
}
export function expansionE(a,normalized){return Math.sqrt(expansionE2(a,normalized));}
export function buildCosmology(model,input,options={}){return normalizeCosmologyParameters(model,input,options);}
