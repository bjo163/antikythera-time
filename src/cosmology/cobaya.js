import { inferCosmicAge } from './age.js';
import { summarizeSamples } from './uncertainty.js';
import { omegaRadiationFromH0 } from './parameters.js';

function normalizeName(name){return String(name).trim().toLowerCase().replace(/[^a-z0-9_]/g,'');}
function firstExisting(row,names){
  for(const n of names){const key=Object.keys(row).find(k=>normalizeName(k)===normalizeName(n));if(key&&Number.isFinite(row[key]))return row[key];}
  return undefined;
}

export function parseCobayaText(text,{maxRows=Infinity,stride=1}={}){
  if(typeof text!=='string')throw new TypeError('text must be string');
  const lines=text.split(/\r?\n/);
  const headerLine=lines.find(line=>line.trim().startsWith('#'));
  if(!headerLine)throw new Error('Cobaya header not found');
  const columns=headerLine.replace(/^\s*#\s*/,'').trim().split(/\s+/);
  if(columns.length<3||normalizeName(columns[0])!=='weight')throw new Error('unexpected Cobaya header');
  const rows=[];
  let seen=0;
  for(const line of lines){
    const s=line.trim();
    if(!s||s.startsWith('#'))continue;
    if((seen++%stride)!==0)continue;
    const values=s.split(/\s+/).map(Number);
    if(values.length!==columns.length||values.some(v=>!Number.isFinite(v)))continue;
    const row={};for(let i=0;i<columns.length;i++)row[columns[i]]=values[i];
    rows.push(row);if(rows.length>=maxRows)break;
  }
  return {columns,rows};
}

export function cobayaRowToCosmology(row,model){
  const H0=firstExisting(row,['H0','hubble','H_0']);
  const omegaM=firstExisting(row,['omegam','Omega_m','omega_m']);
  if(!(H0>0)||!(omegaM>=0))throw new Error('Cobaya row lacks H0/omegam');
  const omegaR=omegaRadiationFromH0(H0);
  if(model==='flat-lcdm')return {H0,omegaR,omegaM,omegaLambda:1-omegaR-omegaM};
  if(model==='lcdm'){
    const omegaK=firstExisting(row,['omegak','Omega_k','omega_k']);
    if(!Number.isFinite(omegaK))throw new Error('curved chain row lacks omegak');
    return {H0,omegaR,omegaM,omegaK,omegaLambda:1-omegaR-omegaM-omegaK};
  }
  if(model==='w0wa-cdm'){
    const w0=firstExisting(row,['w','w0','w_0']);
    const wa=firstExisting(row,['wa','w_a']);
    const omegaK=firstExisting(row,['omegak','Omega_k','omega_k'])??0;
    if(!Number.isFinite(w0)||!Number.isFinite(wa))throw new Error('w0wa chain row lacks w0/wa');
    return {H0,omegaR,omegaM,omegaK,omegaDE:1-omegaR-omegaM-omegaK,w0,wa};
  }
  throw new RangeError('unsupported chain model');
}

function weightedSummary(items){
  const valid=items.filter(x=>Number.isFinite(x.value)&&Number.isFinite(x.weight)&&x.weight>0);
  if(!valid.length)throw new Error('no valid weighted samples');
  const sorted=[...valid].sort((a,b)=>a.value-b.value),total=sorted.reduce((s,x)=>s+x.weight,0);
  const mean=sorted.reduce((s,x)=>s+x.value*x.weight,0)/total;
  const variance=sorted.reduce((s,x)=>s+x.weight*(x.value-mean)**2,0)/total;
  const q=p=>{const target=p*total;let c=0;for(const x of sorted){c+=x.weight;if(c>=target)return x.value;}return sorted.at(-1).value;};
  return {count:valid.length,totalWeight:total,mean,median:q(.5),standardDeviation:Math.sqrt(variance),p16:q(.16),p84:q(.84),p2_5:q(.025),p97_5:q(.975)};
}

export function reproduceCosmicAgeFromCobaya(parsed,{model,maxSamples=5000,stride=1,integration={absoluteTolerance:1e-8,relativeTolerance:1e-8}}={}){
  if(!parsed?.rows?.length)throw new TypeError('parsed Cobaya rows required');
  const ages=[],officialAges=[],residuals=[];let used=0,skipped=0,firstError=null;
  for(let i=0;i<parsed.rows.length&&used<maxSamples;i+=stride){
    const row=parsed.rows[i],weight=firstExisting(row,['weight'])??1;
    try{
      const params=cobayaRowToCosmology(row,model);
      const computed=inferCosmicAge(model,params,{integration}).result.gyr;
      ages.push({value:computed,weight});
      const official=firstExisting(row,['age','age_gyr','agegyr']);
      if(Number.isFinite(official)){officialAges.push({value:official,weight});residuals.push({value:computed-official,weight});}
      used++;
    }catch(error){skipped++;if(!firstError)firstError=error instanceof Error?error.message:String(error);}
  }
  if(!ages.length)throw new Error('No valid cosmology samples. First error: '+firstError+'; columns: '+parsed.columns.join(','));
  const result={model,usedSamples:used,skippedSamples:skipped,firstSkippedError:firstError,computedAgeGyr:weightedSummary(ages),officialAgeGyr:officialAges.length?weightedSummary(officialAges):null,engineMinusOfficialGyr:residuals.length?weightedSummary(residuals):null,columns:parsed.columns};
  if(result.engineMinusOfficialGyr)result.validation={meanAbsOffsetUpperBoundGyr:Math.max(Math.abs(result.engineMinusOfficialGyr.p2_5),Math.abs(result.engineMinusOfficialGyr.p97_5)),meanOffsetGyr:result.engineMinusOfficialGyr.mean};
  return result;
}

export function summarizeUnweightedComputedAges(values){return summarizeSamples(values);}
