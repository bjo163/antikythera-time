export const ASTRONOMICAL_UNIT_METERS = 149_597_870_700;
export const PARSEC_METERS = ASTRONOMICAL_UNIT_METERS * 648_000 / Math.PI;
export const MPC_METERS = PARSEC_METERS * 1_000_000;
export const JULIAN_YEAR_SECONDS_NUMBER = 31_557_600;
export const GYR_SECONDS = 1e9 * JULIAN_YEAR_SECONDS_NUMBER;
export const DEFAULT_TCMB_K = 2.7255;
export const DEFAULT_NEFF = 3.046;

export function assertFinite(name,value){if(!Number.isFinite(value))throw new TypeError(`${name} must be finite`);return value;}
export function hubbleKmSmpcToSI(H0){assertFinite('H0',H0);if(H0<=0)throw new RangeError('H0 must be positive');return H0*1000/MPC_METERS;}
export function omegaRadiationFromH0(H0,{Tcmb=DEFAULT_TCMB_K,Neff=DEFAULT_NEFF}={}){
  assertFinite('Tcmb',Tcmb);assertFinite('Neff',Neff);if(Tcmb<=0||Neff<0)throw new RangeError('invalid radiation parameters');
  const h=H0/100, omegaGammaH2=2.4728e-5*Math.pow(Tcmb/2.7255,4);
  return omegaGammaH2*(1+0.22710731766*Neff)/(h*h);
}
function nonNegative(name,value){assertFinite(name,value);if(value<0)throw new RangeError(`${name} must be non-negative`);return value;}
function flatnessDiagnostic(p,tolerance){
  const sum=p.omegaR+p.omegaM+(p.omegaDE??p.omegaLambda)+(p.omegaK??0);
  return {densitySum:sum,deviationFromUnity:sum-1,tolerance,consistent:Math.abs(sum-1)<=tolerance};
}

export function normalizeCosmologyParameters(model,input,{flatnessTolerance=5e-4,consistencyMode='warning',deriveOmegaK=false}={}){
  if(!input||typeof input!=='object')throw new TypeError('input parameters are required');
  const H0=assertFinite('H0',Number(input.H0));if(H0<=0)throw new RangeError('H0 must be positive');
  const omegaR=input.omegaR==null?omegaRadiationFromH0(H0):nonNegative('omegaR',Number(input.omegaR));
  const omegaM=nonNegative('omegaM',Number(input.omegaM));const warnings=[];let p;
  if(model==='flat-lcdm'){
    const omegaLambda=input.omegaLambda==null?1-omegaR-omegaM:nonNegative('omegaLambda',Number(input.omegaLambda));
    p={H0,omegaR,omegaM,omegaLambda,omegaK:0};const diag=flatnessDiagnostic(p,flatnessTolerance);
    if(!diag.consistent){const msg=`flat density sum ${diag.densitySum} differs from 1 by ${diag.deviationFromUnity}`;if(consistencyMode==='reject')throw new RangeError(msg);warnings.push(msg);}
    return {model,parameters:p,warnings,consistency:diag};
  }
  if(model==='lcdm'){
    const omegaLambda=nonNegative('omegaLambda',Number(input.omegaLambda));let omegaK;
    if(input.omegaK!=null)omegaK=assertFinite('omegaK',Number(input.omegaK));
    else if(deriveOmegaK)omegaK=1-omegaR-omegaM-omegaLambda;
    else throw new RangeError('non-flat lcdm requires explicit omegaK or deriveOmegaK=true');
    p={H0,omegaR,omegaM,omegaLambda,omegaK};return {model,parameters:p,warnings,consistency:flatnessDiagnostic(p,flatnessTolerance)};
  }
  if(model==='w0wa-cdm'){
    const w0=assertFinite('w0',Number(input.w0)),wa=assertFinite('wa',Number(input.wa));
    const omegaK=input.omegaK==null?0:assertFinite('omegaK',Number(input.omegaK));
    const omegaDE=input.omegaDE==null?1-omegaR-omegaM-omegaK:nonNegative('omegaDE',Number(input.omegaDE));
    p={H0,omegaR,omegaM,omegaDE,omegaK,w0,wa};if(omegaDE<0)throw new RangeError('omegaDE must be non-negative');
    if(!(omegaM>0||omegaR>0))throw new RangeError('model needs matter or radiation for a finite early-time age');
    if(w0+wa>=0)warnings.push('w0 + wa >= 0: early dark-energy behavior may violate the intended early matter-dominated regime');
    return {model,parameters:p,warnings,consistency:flatnessDiagnostic(p,flatnessTolerance)};
  }
  throw new RangeError(`unsupported cosmology model: ${model}`);
}
function numberFromQuery(query,keys){for(const key of keys){if(query[key]!=null&&query[key]!==''){const n=Number(query[key]);if(!Number.isFinite(n))throw new RangeError(`${key} must be finite`);return n;}}}
export function parseCosmologyAgeQuery(query={}){
  const model=String(query.model??'flat-lcdm').toLowerCase(),preset=query.preset==null?null:String(query.preset).toLowerCase();
  const explicitKeys=['H0','h0','OmegaM','omegaM','OmegaLambda','omegaLambda','OmegaR','omegaR','OmegaK','omegaK','OmegaDE','omegaDE','w0','wa'];
  if(preset&&explicitKeys.some(k=>query[k]!=null))throw new RangeError('do not mix preset with explicit cosmology parameters');
  if(preset)return {preset,uncertainty:query.uncertainty??'none',samples:Number(query.samples??2000),seed:Number(query.seed??20261001)};
  const parameters={H0:numberFromQuery(query,['H0','h0']),omegaM:numberFromQuery(query,['OmegaM','omegaM']),omegaLambda:numberFromQuery(query,['OmegaLambda','omegaLambda']),omegaR:numberFromQuery(query,['OmegaR','omegaR']),omegaK:numberFromQuery(query,['OmegaK','omegaK']),omegaDE:numberFromQuery(query,['OmegaDE','omegaDE']),w0:numberFromQuery(query,['w0']),wa:numberFromQuery(query,['wa'])};
  if(parameters.H0==null||parameters.omegaM==null)throw new RangeError('explicit request requires H0 and OmegaM');
  return {model,parameters,deriveOmegaK:String(query.deriveOmegaK??'false')==='true',uncertainty:'none'};
}
