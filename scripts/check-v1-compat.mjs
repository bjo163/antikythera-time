#!/usr/bin/env node
import fs from 'node:fs/promises';
const js=JSON.parse(await fs.readFile('artifacts/v1-js.json','utf8'));
const py=JSON.parse(await fs.readFile('artifacts/v1-python.json','utf8'));
const checks=[
  {id:'TT_TO_TCG',error:Math.abs(js.ttToTcgD2-py.ttToTcgD2),tolerance:1e-12},
  {id:'PLANCK_AGE',error:Math.abs(js.planckAgeGyr-py.planckAgeGyr),tolerance:5e-6},
  {id:'SAROS',error:Math.abs(js.sarosJdTt-py.sarosJdTt),tolerance:1e-9},
  {id:'MARS_X',error:Math.abs(js.mars.x-py.mars.x),tolerance:1e-12},
  {id:'MARS_Y',error:Math.abs(js.mars.y-py.mars.y),tolerance:1e-12},
  {id:'MARS_Z',error:Math.abs(js.mars.z-py.mars.z),tolerance:1e-12},
].map(x=>({...x,pass:x.error<=x.tolerance}));
const report={version:'1.0.0',implementations:[js.implementation,py.implementation],checks,allPass:checks.every(x=>x.pass)};
console.log(JSON.stringify(report,null,2));
await fs.writeFile('artifacts/v1-compatibility.json',JSON.stringify(report,null,2));
if(!report.allPass)process.exit(1);
