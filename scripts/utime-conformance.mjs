#!/usr/bin/env node
import fs from 'node:fs/promises';
import { checkV2Record } from '../src/v2/conformance.js';
const files=process.argv.slice(2);if(!files.length){console.error('usage: node scripts/utime-conformance.mjs record.json [...]');process.exit(2);}
let failed=0;
for(const file of files){const data=JSON.parse(await fs.readFile(file,'utf8'));const records=Array.isArray(data)?data:[data];for(const [i,r] of records.entries()){const c=checkV2Record(r);console.log(JSON.stringify({file,index:i,...c}));if(!c.pass)failed++;}}
if(failed)process.exit(1);
