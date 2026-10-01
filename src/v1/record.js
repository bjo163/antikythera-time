export const U_TIME_V1_VERSION='1.0.0';
export const U_TIME_V1_EVIDENCE=Object.freeze([
  'OBSERVED','MEASURED','CALCULATED','MODELED','INFERRED','RECONSTRUCTED','SPECULATIVE','TEXTUAL_REFERENCE'
]);

function finite(name,v){if(!Number.isFinite(v))throw new TypeError(name+' must be finite');return v;}
function provenance(p){if(!p||typeof p!=='object'||!p.source)throw new TypeError('provenance.source is required');return p;}

export function makeInstantRecord({d1,d2,scale,frame,uncertaintySeconds=0,provenance:prov,evidence='CALCULATED'}){
  finite('d1',d1);finite('d2',d2);finite('uncertaintySeconds',uncertaintySeconds);
  if(uncertaintySeconds<0)throw new RangeError('uncertaintySeconds must be non-negative');
  if(!U_TIME_V1_EVIDENCE.includes(evidence))throw new RangeError('invalid evidence state');
  return Object.freeze({
    protocol:'U-Time',version:U_TIME_V1_VERSION,kind:'instant',quantity:'coordinate_time',status:evidence,
    value:{d1,d2,scale,frame},uncertainty:{seconds:uncertaintySeconds},provenance:provenance(prov)
  });
}

export function makeDurationRecord({seconds,uncertaintySeconds=0,provenance:prov,evidence='CALCULATED'}){
  finite('seconds',seconds);finite('uncertaintySeconds',uncertaintySeconds);
  return Object.freeze({protocol:'U-Time',version:U_TIME_V1_VERSION,kind:'duration',quantity:'elapsed_time',status:evidence,value:{seconds},uncertainty:{seconds:uncertaintySeconds},provenance:provenance(prov)});
}

export function makeCosmicAgeRecord({seconds,gyr,model,parameters,uncertaintyGyr=null,provenance:prov}){
  finite('seconds',seconds);finite('gyr',gyr);
  return Object.freeze({protocol:'U-Time',version:U_TIME_V1_VERSION,kind:'inference',quantity:'age_of_universe',status:'INFERRED',value:{seconds,gyr},context:{model,parameters},uncertainty:{gyr:uncertaintyGyr},provenance:provenance(prov),boundary:{absoluteCosmicClock:false}});
}

export function makeModelResultRecord({quantity,value,model,validity,evidence='MODELED',uncertainty=null,provenance:prov}){
  if(!U_TIME_V1_EVIDENCE.includes(evidence))throw new RangeError('invalid evidence state');
  return Object.freeze({protocol:'U-Time',version:U_TIME_V1_VERSION,kind:'model_result',quantity,status:evidence,value,context:{model,validity},uncertainty,provenance:provenance(prov)});
}
