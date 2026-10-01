import { U_TIME_EVIDENCE,U_TIME_QUALITY,U_TIME_V2_VERSION } from './record.js';
export function checkV2Record(record){
  const errors=[];
  if(!record||typeof record!=='object')return{pass:false,errors:['record must be object']};
  if(record.protocol!=='U-Time')errors.push('protocol must be U-Time');
  if(record.version!==U_TIME_V2_VERSION)errors.push('version must be 2.0.0');
  if(!['instant','duration','inference','model_result','observation'].includes(record.kind))errors.push('invalid kind');
  if(!record.quantity)errors.push('quantity required');
  if(!U_TIME_EVIDENCE.includes(record.status))errors.push('invalid evidence status');
  if(!U_TIME_QUALITY.includes(record.quality))errors.push('invalid quality');
  if(!record.value||typeof record.value!=='object')errors.push('value object required');
  if(!Array.isArray(record.provenance)||record.provenance.length===0||record.provenance.some(x=>!x?.source))errors.push('provenance[] with source required');
  if(record.kind==='instant'&&(!record.value.scale||!record.value.frame))errors.push('instant scale/frame required');
  if(record.quantity==='age_of_universe'&&record.kind!=='inference')errors.push('cosmic age must be inference');
  if(record.status==='TEXTUAL_REFERENCE'&&record.quality!=='CONCEPTUAL')errors.push('textual reference must use CONCEPTUAL quality');
  if(record.quality==='APPROXIMATE'&&!record.validity)errors.push('approximate model requires validity');
  return{pass:errors.length===0,errors};
}
