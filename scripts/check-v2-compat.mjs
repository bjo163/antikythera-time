#!/usr/bin/env node
import fs from 'node:fs/promises';
const js=JSON.parse(await fs.readFile('artifacts/v1-js.json','utf8'));
const py=JSON.parse(await fs.readFile('artifacts/v1-python.json','utf8'));
const rs=JSON.parse(await fs.readFile('artifacts/v2-rust.json','utf8'));
const impl=[js,py,rs];
const fields=[
 ['TT_TO_TCG',x=>x.ttToTcgD2,1e-12],
 ['PLANCK_AGE',x=>x.planckAgeGyr,5e-6],
 ['SAROS',x=>x.sarosJdTt,1e-9],
 ['MARS_X',x=>x.mars.x,1e-12],
 ['MARS_Y',x=>x.mars.y,1e-12],
 ['MARS_Z',x=>x.mars.z,1e-12],
];
const checks=[];
for(const [id,get,tol] of fields){
 const vals=impl.map(get),spread=Math.max(...vals)-Math.min(...vals);
 checks.push({id,values:Object.fromEntries(impl.map((x,i)=>[x.implementation,vals[i]])),spread,tolerance:tol,pass:spread<=tol});
}
const report={version:'2.0.0',implementations:impl.map(x=>x.implementation),checks,allPass:checks.every(x=>x.pass)};
await fs.writeFile('artifacts/v2-compatibility.json',JSON.stringify(report,null,2));console.log(JSON.stringify(report,null,2));if(!report.allPass)process.exit(1);
