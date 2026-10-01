export const U_TIME_V2_VERSION='2.0.0';
export const U_TIME_V2_STATUS='STANDARDIZATION_CANDIDATE';
export const U_TIME_EVIDENCE=Object.freeze(['OBSERVED','MEASURED','CALCULATED','MODELED','INFERRED','RECONSTRUCTED','SPECULATIVE','TEXTUAL_REFERENCE']);
export const U_TIME_QUALITY=Object.freeze(['REFERENCE','HIGH_PRECISION','APPROXIMATE','RECONSTRUCTION','CONCEPTUAL']);

function finite(name,v){if(!Number.isFinite(v))throw new TypeError(name+' must be finite');return v;}
function sourceList(p){const a=Array.isArray(p)?p:[p];if(!a.length||a.some(x=>!x||!x.source))throw new TypeError('provenance source required');return a.map(x=>({...x}));}
function base({kind,quantity,status,quality='REFERENCE',value,context={},uncertainty=null,provenance,algorithm=null,validity=null,observer=null,referenceData=null,transformChain=[]}){
  if(!U_TIME_EVIDENCE.includes(status))throw new RangeError('invalid evidence state');
  if(!U_TIME_QUALITY.includes(quality))throw new RangeError('invalid quality class');
  return Object.freeze({protocol:'U-Time',version:U_TIME_V2_VERSION,specStatus:U_TIME_V2_STATUS,kind,quantity,status,quality,value,context,uncertainty,provenance:sourceList(provenance),algorithm,validity,observer,referenceData,transformChain:[...transformChain]});
}
export function makeV2Instant({d1,d2,scale,frame,uncertaintySeconds=0,provenance,observer=null,algorithm=null,referenceData=null,transformChain=[]}){
  finite('d1',d1);finite('d2',d2);finite('uncertaintySeconds',uncertaintySeconds);
  return base({kind:'instant',quantity:'coordinate_time',status:'CALCULATED',quality:'REFERENCE',value:{d1,d2,scale,frame},uncertainty:{seconds:uncertaintySeconds},provenance,observer,algorithm,referenceData,transformChain});
}
export function makeV2ModelResult({quantity,value,model,quality='APPROXIMATE',status='MODELED',validity,uncertainty=null,provenance,observer=null,referenceData=null,algorithm=null}){
  return base({kind:'model_result',quantity,status,quality,value,context:{model},validity,uncertainty,provenance,observer,referenceData,algorithm});
}
export function makeV2Inference({quantity,value,model,parameters,uncertainty,provenance,referenceData=null,algorithm=null}){
  return base({kind:'inference',quantity,status:'INFERRED',quality:'REFERENCE',value,context:{model,parameters},uncertainty,provenance,referenceData,algorithm});
}
export function upgradeV1Record(v1){
  if(!v1||v1.protocol!=='U-Time'||v1.version!=='1.0.0')throw new TypeError('U-Time v1 record required');
  return Object.freeze({...v1,version:U_TIME_V2_VERSION,specStatus:U_TIME_V2_STATUS,quality:v1.kind==='model_result'?'APPROXIMATE':'REFERENCE',provenance:sourceList(v1.provenance),observer:null,referenceData:null,algorithm:null,validity:v1.context?.validity??null,transformChain:[]});
}
