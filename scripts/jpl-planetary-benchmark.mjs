#!/usr/bin/env node
import fs from 'node:fs/promises';
import { jplApproxHeliocentric, vectorErrorAu } from '../src/planetary/index.js';

const AU_KM=149597870.7;
const planets=['Mercury','Venus','Mars','Jupiter','Saturn'];
const jds=[2451545.0,2460676.5,2469442.5];
const thresholdsKm={Mercury:50000,Venus:50000,Mars:300000,Jupiter:4000000,Saturn:10000000};

function parseVector(text){
  const start=text.indexOf('$$SOE'),end=text.indexOf('$$EOE');
  if(start<0||end<=start)throw new Error('Horizons block missing');
  const block=text.slice(start+5,end);
  const vals=[...block.matchAll(/(?:X|Y|Z)\s*=\s*([+-]?[0-9.]+(?:E[+-]?\d+)?)/gi)].map(m=>Number(m[1]));
  if(vals.length<3)throw new Error('XYZ parse failed');
  return{x:vals[0],y:vals[1],z:vals[2]};
}
async function horizons(model,jd){
  const q=new URLSearchParams({format:'text',COMMAND:`'${model.horizonsId}'`,OBJ_DATA:"'NO'",MAKE_EPHEM:"'YES'",EPHEM_TYPE:"'VECTORS'",CENTER:"'500@10'",TLIST:`'${jd}'`,TLIST_TYPE:"'JD'",TIME_TYPE:"'TDB'",REF_PLANE:"'ECLIPTIC'",OUT_UNITS:"'AU-D'",VEC_TABLE:"'1'"});
  let last;
  for(let i=1;i<=4;i++){
    try{const r=await fetch('https://ssd.jpl.nasa.gov/api/horizons.api?'+q,{headers:{'User-Agent':'antikythera-time-v1-benchmark'}});if(!r.ok)throw new Error('HTTP '+r.status);return parseVector(await r.text());}catch(e){last=e;if(i<4)await new Promise(r=>setTimeout(r,i*2000));}
  }
  throw last;
}
const results=[];
for(const name of planets)for(const jd of jds){
  const model=jplApproxHeliocentric(name,jd),ref=await horizons(model,jd);
  const errorKm=vectorErrorAu(model,ref)*AU_KM;
  results.push({planet:name,jd,errorKm,thresholdKm:thresholdsKm[name],pass:errorKm<=thresholdsKm[name]});
  console.error(name,jd,errorKm.toFixed(1),'km');
}
const report={source:'JPL Horizons geometric heliocentric ecliptic vectors',results,allPass:results.every(x=>x.pass)};
await fs.mkdir('artifacts',{recursive:true});
await fs.writeFile('artifacts/jpl-planetary-benchmark.json',JSON.stringify(report,null,2));
console.log(JSON.stringify(report,null,2));
if(!report.allPass)process.exit(1);
