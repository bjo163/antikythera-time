#!/usr/bin/env node
import fs from 'node:fs/promises';
import { coordinateFromTwoPartJD } from '../src/relativity/coordinate-time.js';
import { nasaSimpleDtr } from '../src/relativity/dtr.js';
import { TimeScale } from '../src/utime.js';

const path=process.argv[2]??'artifacts/erfa-dtr-reference.json';
const reference=JSON.parse(await fs.readFile(path,'utf8'));
const errors=[];
for(const row of reference.samples){
  const tt=coordinateFromTwoPartJD(TimeScale.TT,row.jd1,row.jd2);
  const actual=nasaSimpleDtr(tt).seconds;
  errors.push({jd:row.jd1+row.jd2,errorSeconds:actual-row.erfaDtrSeconds});
}
const abs=errors.map(x=>Math.abs(x.errorSeconds));
const report={
  source:reference.source,
  sampleCount:errors.length,
  window:reference.window,
  maxAbsErrorSeconds:Math.max(...abs),
  meanAbsErrorSeconds:abs.reduce((a,b)=>a+b,0)/abs.length,
  rmsErrorSeconds:Math.sqrt(errors.reduce((s,x)=>s+x.errorSeconds*x.errorSeconds,0)/errors.length),
  thresholdSeconds:0.0001,
};
report.pass=report.maxAbsErrorSeconds<=report.thresholdSeconds;
await fs.mkdir('artifacts',{recursive:true});
await fs.writeFile('artifacts/erfa-dtr-benchmark.json',JSON.stringify(report,null,2));
console.log(JSON.stringify(report,null,2));
if(!report.pass)process.exit(1);
