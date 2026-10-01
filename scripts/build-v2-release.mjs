#!/usr/bin/env node
import fs from 'node:fs/promises';import path from 'node:path';import crypto from 'node:crypto';
const roots=['spec','src/v2','reference-python','reference-rust','benchmarks'];
const singles=['README.md','GOVERNANCE.md','CONTRIBUTING.md','SECURITY.md','CHANGELOG.md','CITATION.cff','docs/preprint.md','docs/standardization-readiness.md','docs/standards-mapping.md'];
async function walk(p){const st=await fs.stat(p);if(st.isFile())return[p];const out=[];for(const n of (await fs.readdir(p)).sort())out.push(...await walk(path.join(p,n)));return out;}
const files=[];for(const r of roots)files.push(...await walk(r));for(const f of singles)files.push(f);
const entries=[];for(const f of [...new Set(files)].sort()){const b=await fs.readFile(f);entries.push({path:f,bytes:b.length,sha256:crypto.createHash('sha256').update(b).digest('hex')});}
const manifest={protocol:'U-Time',version:'2.0.0',status:'STANDARDIZATION_CANDIDATE',createdAt:new Date().toISOString(),files:entries};
await fs.mkdir('artifacts',{recursive:true});await fs.writeFile('artifacts/utime-2.0.0-manifest.json',JSON.stringify(manifest,null,2));console.log(JSON.stringify({files:entries.length},null,2));
