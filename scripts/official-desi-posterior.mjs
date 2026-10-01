#!/usr/bin/env node
import https from 'node:https';
import fs from 'node:fs/promises';
import { parseCobayaText, reproduceCosmicAgeFromCobaya } from '../src/cosmology/cobaya.js';

const ROOT='https://data.desi.lbl.gov/public/papers/y3/bao-cosmo-params/cobaya/';
const registry={
  'desi-dr2-lcdm-cmb':{root:'base/',model:'flat-lcdm',must:['desi-bao-all','planck-act-dr6-lensing'],avoid:['desy5sn','pantheonplus','union3']},
  'desi-dr2-lcdm-curved-cmb':{root:'base_omegak/',model:'lcdm',must:['desi-bao-all','planck-act-dr6-lensing'],avoid:['desy5sn','pantheonplus','union3']},
  'desi-dr2-w0wa-cmb-desy5':{root:'base_w_wa/',model:'w0wa-cdm',must:['desi-bao-all','planck-act-dr6-lensing','desy5sn'],avoid:[]},
};

function links(html){return [...html.matchAll(/href="([^"]+\/?)"/g)].map(m=>m[1]).filter(x=>!x.startsWith('?')&&!x.startsWith('/'));}
function httpsTextOnce(url,timeoutMs=90000){
  return new Promise((resolve,reject)=>{
    const req=https.get(url,{headers:{'User-Agent':'antikythera-time-posterior-reproducer/0.7'}},res=>{
      if(res.statusCode>=300&&res.statusCode<400&&res.headers.location){res.resume();resolve(httpsTextOnce(new URL(res.headers.location,url).toString(),timeoutMs));return;}
      if(res.statusCode!==200){res.resume();reject(new Error('HTTP '+res.statusCode+' for '+url));return;}
      res.setEncoding('utf8');let data='';res.on('data',chunk=>data+=chunk);res.on('end',()=>resolve(data));
    });
    req.setTimeout(timeoutMs,()=>req.destroy(new Error('timeout after '+timeoutMs+'ms for '+url)));
    req.on('error',reject);
  });
}
async function getText(url){
  let last;
  for(let attempt=1;attempt<=5;attempt++){
    try{return await httpsTextOnce(url,90000);}catch(error){last=error;console.error('download attempt',attempt,'failed:',error.message);if(attempt<5)await new Promise(r=>setTimeout(r,attempt*3000));}
  }
  throw last;
}
async function discover(cfg){
  const url=ROOT+cfg.root,html=await getText(url);
  const candidates=links(html).filter(x=>x.endsWith('/')&&cfg.must.every(k=>x.includes(k))&&!cfg.avoid.some(k=>x.includes(k)));
  if(!candidates.length)throw new Error(`No official DESI chain directory matched ${JSON.stringify(cfg.must)} under ${url}`);
  candidates.sort((a,b)=>a.length-b.length);
  return {rootUrl:url,directory:candidates[0],allCandidates:candidates};
}
async function run(id){
  const cfg=registry[id];if(!cfg)throw new Error(`unknown dataset id: ${id}`);
  const found=await discover(cfg),dirUrl=found.rootUrl+found.directory,listing=await getText(dirUrl);
  const chainFiles=links(listing).filter(x=>/^chain\.\d+\.txt$/.test(x));
  if(!chainFiles.length)throw new Error('No chain.N.txt files found');
  const merged={columns:null,rows:[]};
  for(const file of chainFiles){
    const text=await getText(dirUrl+file),parsed=parseCobayaText(text,{maxRows:Infinity,stride:1});
    if(!merged.columns)merged.columns=parsed.columns;
    if(merged.columns.join('\t')!==parsed.columns.join('\t'))throw new Error('chain column mismatch');
    merged.rows.push(...parsed.rows);
  }
  const stride=Math.max(1,Math.floor(merged.rows.length/6000));
  const result=reproduceCosmicAgeFromCobaya(merged,{model:cfg.model,maxSamples:6000,stride});
  return {dataset:id,officialDirectory:dirUrl,downloadedChains:chainFiles,totalParsedRows:merged.rows.length,stride,...result};
}

const ids=process.argv.slice(2);const selected=ids.length?ids:Object.keys(registry);
const results=[];
for(const id of selected){console.error('Processing',id);results.push(await run(id));}
const payload={generatedAt:new Date().toISOString(),source:'DESI DR2 official public Cobaya chains',results};
await fs.mkdir('artifacts',{recursive:true});
await fs.writeFile('artifacts/desi-official-age-posterior.json',JSON.stringify(payload,null,2));
const md=['# DESI DR2 Official-Chain Cosmic Age Reproduction','','Generated: '+payload.generatedAt,''];
for(const r of results){
  md.push('## '+r.dataset,'', '- Official directory: '+r.officialDirectory, '- Parsed rows: '+r.totalParsedRows, '- Engine samples: '+r.usedSamples, '- Computed age: '+r.computedAgeGyr.mean.toFixed(6)+' ± '+r.computedAgeGyr.standardDeviation.toFixed(6)+' Gyr (weighted posterior mean ± SD)', '- 68% interval: ['+r.computedAgeGyr.p16.toFixed(6)+', '+r.computedAgeGyr.p84.toFixed(6)+'] Gyr');
  if(r.officialAgeGyr){md.push('- Cobaya derived age: '+r.officialAgeGyr.mean.toFixed(6)+' ± '+r.officialAgeGyr.standardDeviation.toFixed(6)+' Gyr','- Engine − Cobaya mean age: '+r.engineMinusOfficialGyr.mean.toExponential(4)+' Gyr');}
  else md.push('- Cobaya chain has no recognized derived `age` column; posterior age was recomputed from chain cosmological parameters.');
  md.push('');
}
await fs.writeFile('artifacts/desi-official-age-posterior.md',md.join('\n'));
console.log(JSON.stringify(payload,null,2));
