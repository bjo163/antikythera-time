import { adaptiveSimpson } from './integrate.js';
import { expansionE, buildCosmology } from './models.js';
import { GYR_SECONDS,JULIAN_YEAR_SECONDS_NUMBER,hubbleKmSmpcToSI } from './parameters.js';
export class CosmicAgeEstimate{
  constructor(payload){this.quantity='age_of_universe';this.status='MODEL_DEPENDENT_INFERENCE';Object.assign(this,payload);Object.freeze(this);}
  toJSON(){return {...this};}
}
export function inferCosmicAge(model,input,{integration={},parameterSource=null,observationalSource=null,assumptions=[],normalization={}}={}){
  const normalized=buildCosmology(model,input,normalization),H0SI=hubbleKmSmpcToSI(normalized.parameters.H0);
  const integrand=x=>{if(x===0)return 0;const a=x*x;return 2/(x*expansionE(a,normalized));};
  const numerical=adaptiveSimpson(integrand,0,1,integration);
  const ageSeconds=numerical.value/H0SI,numericalUncertaintySeconds=numerical.errorEstimate/H0SI;
  return new CosmicAgeEstimate({
    result:{seconds:ageSeconds,julianYears:ageSeconds/JULIAN_YEAR_SECONDS_NUMBER,gyr:ageSeconds/GYR_SECONDS},
    model,parameters:normalized.parameters,parameterSource,observationalSource,
    numericalUncertainty:{seconds:numericalUncertaintySeconds,gyr:numericalUncertaintySeconds/GYR_SECONDS,integrationErrorDimensionless:numerical.errorEstimate},
    parameterUncertainty:null,
    modelDependence:'The inferred age changes with the cosmological model and observational parameter posterior.',
    method:{equation:'t0 = H0^-1 * integral_0^1 da / (a E(a))',integration:numerical,variableTransform:'a = x^2',H0SI},
    assumptions:['homogeneous and isotropic FLRW background','General Relativity background Friedmann equation',...assumptions],
    validation:{warnings:normalized.warnings,consistency:normalized.consistency},
    scientificBoundary:{antikytheraDirectlyDeterminesCosmicAge:false,scriptureUsedAsNumericalPrior:false,isAbsoluteCosmicClock:false,isUTimeTimestamp:false},
  });
}
export function einsteinDeSitterAnalyticAgeSeconds(H0){return 2/(3*hubbleKmSmpcToSI(H0));}
export function flatMatterLambdaAnalyticAgeSeconds({H0,omegaM,omegaLambda}){
  if(!(omegaM>0)||!(omegaLambda>0))throw new RangeError('analytic matter+Lambda solution needs positive omegaM and omegaLambda');
  return 2/(3*hubbleKmSmpcToSI(H0)*Math.sqrt(omegaLambda))*Math.asinh(Math.sqrt(omegaLambda/omegaM));
}
