import {getCosmologyPreset,inferCosmicAge,materializePresetParameters,parseCosmologyAgeQuery,propagateIndependentPreset} from '../src/cosmology/index.js';
export default async function handler(req,res){
  try{
    const parsed=parseCosmologyAgeQuery(req.query??{});let model,parameters,parameterSource=null,observationalSource=null,provenance=null;
    if(parsed.preset){const preset=getCosmologyPreset(parsed.preset);model=preset.model;parameters=materializePresetParameters(preset);parameterSource=preset.id;observationalSource=preset.provenance.dataset;provenance=preset.provenance;}
    else{model=parsed.model;parameters=parsed.parameters;}
    const estimate=inferCosmicAge(model,parameters,{parameterSource,observationalSource,normalization:{deriveOmegaK:parsed.deriveOmegaK??false}});
    let parameterUncertainty=null;
    if(parsed.preset&&parsed.uncertainty==='independent')parameterUncertainty=propagateIndependentPreset(parsed.preset,{samples:parsed.samples,seed:parsed.seed,integration:{absoluteTolerance:1e-9,relativeTolerance:1e-9}});
    return res.status(200).json({...estimate.toJSON(),parameterUncertainty,provenance});
  }catch(error){return res.status(400).json({error:error instanceof Error?error.message:'unknown error'});}
}
