import { inferCosmicAge } from './age.js';
import { getCosmologyPreset, materializePresetParameters } from './references.js';
export function mulberry32(seed){let a=seed>>>0;return function(){a|=0;a=a+0x6D2B79F5|0;let t=Math.imul(a^a>>>15,1|a);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296;};}
function gaussian(rng){let u=0,v=0;while(u===0)u=rng();while(v===0)v=rng();return Math.sqrt(-2*Math.log(u))*Math.cos(2*Math.PI*v);}
function percentile(sorted,p){const x=(sorted.length-1)*p,lo=Math.floor(x),hi=Math.ceil(x);return lo===hi?sorted[lo]:sorted[lo]+(sorted[hi]-sorted[lo])*(x-lo);}
export function summarizeSamples(values){if(!Array.isArray(values)||!values.length)throw new TypeError('values must be non-empty');const sorted=[...values].sort((a,b)=>a-b),mean=values.reduce((a,b)=>a+b,0)/values.length,variance=values.reduce((s,x)=>s+(x-mean)**2,0)/Math.max(1,values.length-1);return{count:values.length,mean,median:percentile(sorted,.5),standardDeviation:Math.sqrt(variance),p16:percentile(sorted,.16),p84:percentile(sorted,.84),p2_5:percentile(sorted,.025),p97_5:percentile(sorted,.975)};}
function cholesky(matrix){const n=matrix.length,L=Array.from({length:n},()=>Array(n).fill(0));for(let i=0;i<n;i++)for(let j=0;j<=i;j++){let sum=matrix[i][j];for(let k=0;k<j;k++)sum-=L[i][k]*L[j][k];if(i===j){if(!(sum>0))throw new RangeError('covariance matrix must be positive definite');L[i][j]=Math.sqrt(sum);}else L[i][j]=sum/L[j][j];}return L;}
export function propagateIndependentPreset(presetId,{samples=2000,seed=20261001,integration={}}={}){
  if(!Number.isInteger(samples)||samples<10||samples>100000)throw new RangeError('samples must be an integer in [10,100000]');
  const preset=getCosmologyPreset(presetId),rng=mulberry32(seed),ages=[];
  for(let i=0;i<samples;i++){const sampled={};for(const[key,mu]of Object.entries(preset.parameters)){const sigma=preset.uncertainties?.[key];sampled[key]=sigma==null?mu:mu+sigma*gaussian(rng);}if(!(sampled.H0>0)||!(sampled.omegaM>=0)){i--;continue;}const params=materializePresetParameters(preset,sampled);ages.push(inferCosmicAge(preset.model,params,{integration,parameterSource:preset.id,observationalSource:preset.provenance.dataset}).result.gyr);}
  return{method:'independent-gaussian-monte-carlo',seed,assumptions:'Parameter correlations are ignored. Use covariance or posterior-chain mode when available.',ageGyr:summarizeSamples(ages)};
}
export function propagateCovariance({model,means,parameterNames,covariance,samples=2000,seed=20261001,completeParameters,integration={}}){
  if(!Array.isArray(parameterNames)||covariance.length!==parameterNames.length)throw new TypeError('parameterNames/covariance mismatch');const L=cholesky(covariance),rng=mulberry32(seed),ages=[];
  for(let s=0;s<samples;s++){const z=parameterNames.map(()=>gaussian(rng)),p={...means};for(let i=0;i<parameterNames.length;i++){let delta=0;for(let k=0;k<=i;k++)delta+=L[i][k]*z[k];p[parameterNames[i]]=means[parameterNames[i]]+delta;}const full=completeParameters?completeParameters(p):p;try{ages.push(inferCosmicAge(model,full,{integration}).result.gyr);}catch{s--;}}
  return{method:'covariance-aware-monte-carlo',seed,ageGyr:summarizeSamples(ages)};
}
function summarizeWeightedSamples(samples){const sorted=[...samples].sort((a,b)=>a.value-b.value),total=sorted.reduce((s,x)=>s+x.weight,0),mean=sorted.reduce((s,x)=>s+x.value*x.weight,0)/total,variance=sorted.reduce((s,x)=>s+x.weight*(x.value-mean)**2,0)/total;const wp=p=>{const target=p*total;let c=0;for(const x of sorted){c+=x.weight;if(c>=target)return x.value;}return sorted.at(-1).value;};return{count:sorted.length,totalWeight:total,mean,median:wp(.5),standardDeviation:Math.sqrt(variance),p16:wp(.16),p84:wp(.84),p2_5:wp(.025),p97_5:wp(.975)};}
export function evaluatePosteriorChain(chain,{model,mapParameters=x=>x,weightKey=null,integration={}}={}){
  if(!Array.isArray(chain)||!chain.length)throw new TypeError('chain must be non-empty');const ages=[];
  for(const row of chain){const estimate=inferCosmicAge(model,mapParameters(row),{integration}),weight=weightKey?Number(row[weightKey]):1;if(!Number.isFinite(weight)||weight<=0)continue;ages.push({value:estimate.result.gyr,weight});}
  if(!ages.length)throw new Error('no valid posterior samples');return{method:'posterior-chain-evaluation',samples:ages,ageGyr:summarizeWeightedSamples(ages),weighted:Boolean(weightKey)};
}
