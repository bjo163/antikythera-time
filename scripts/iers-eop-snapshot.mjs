#!/usr/bin/env node
import fs from 'node:fs/promises';
import { parseIersFinals2000A } from '../src/eop/iers.js';

const url='https://datacenter.iers.org/data/latestVersion/finals.all.iau2000.txt';
let text,last;
for(let attempt=1;attempt<=5;attempt++){
  try{
    const r=await fetch(url,{headers:{'User-Agent':'antikythera-time-v2-eop'}});
    if(!r.ok)throw new Error('HTTP '+r.status);
    text=await r.text();break;
  }catch(e){last=e;if(attempt<5)await new Promise(r=>setTimeout(r,attempt*3000));}
}
if(!text)throw last;
const rows=parseIersFinals2000A(text);
if(rows.length<1000)throw new Error('unexpected IERS EOP row count '+rows.length);
const latest=rows.at(-1),observed=rows.filter(r=>r.evidence==='OBSERVED').at(-1);
const report={source:url,retrievedAt:new Date().toISOString(),rowCount:rows.length,first:rows[0],latest,latestObserved:observed};
await fs.mkdir('artifacts',{recursive:true});
await fs.writeFile('artifacts/iers-eop-snapshot.json',JSON.stringify(report,null,2));
console.log(JSON.stringify({rowCount:rows.length,first:rows[0].date,latest:latest.date,latestObserved:observed?.date},null,2));
