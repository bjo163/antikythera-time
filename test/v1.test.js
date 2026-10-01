import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import { makeInstantRecord, makeDurationRecord, makeCosmicAgeRecord, makeModelResultRecord, U_TIME_V1_VERSION } from '../src/index.js';

test('v1 protocol version is frozen at 1.0.0',()=>assert.equal(U_TIME_V1_VERSION,'1.0.0'));

test('instant record carries scale frame uncertainty and provenance',()=>{
  const r=makeInstantRecord({d1:2451545,d2:0,scale:'TT',frame:'GCRS',uncertaintySeconds:1e-9,provenance:{source:'test'}});
  assert.equal(r.protocol,'U-Time');assert.equal(r.version,'1.0.0');assert.equal(r.kind,'instant');assert.equal(r.value.scale,'TT');assert.equal(r.provenance.source,'test');
});

test('duration record remains distinct from instant',()=>{
  const r=makeDurationRecord({seconds:10,provenance:{source:'SI'}});
  assert.equal(r.kind,'duration');assert.equal(r.quantity,'elapsed_time');
});

test('cosmic age record is inference and explicitly not an absolute clock',()=>{
  const r=makeCosmicAgeRecord({seconds:4e17,gyr:13.8,model:'flat-lcdm',parameters:{H0:67},provenance:{source:'test'}});
  assert.equal(r.status,'INFERRED');assert.equal(r.boundary.absoluteCosmicClock,false);
});

test('model result requires an explicit evidence state and provenance',()=>{
  const r=makeModelResultRecord({quantity:'planet_position',value:{x:1},model:'JPL_APPROX',validity:'1800-2050',evidence:'MODELED',provenance:{source:'JPL'}});
  assert.equal(r.status,'MODELED');assert.equal(r.context.validity,'1800-2050');
});

test('v1 JSON schema is parseable and pins protocol/version',()=>{
  const s=JSON.parse(fs.readFileSync(new URL('../spec/utime-v1.schema.json',import.meta.url),'utf8'));
  assert.equal(s.properties.protocol.const,'U-Time');assert.equal(s.properties.version.const,'1.0.0');
  assert.ok(s.required.includes('provenance'));
});

test('v1 golden vectors are versioned and cover four domains',()=>{
  const g=JSON.parse(fs.readFileSync(new URL('../spec/golden-vectors.json',import.meta.url),'utf8'));
  assert.equal(g.version,'1.0.0');
  assert.deepEqual(new Set(g.vectors.map(x=>x.kind)),new Set(['coordinate_transform','cosmic_age','eclipse_recurrence','planetary_approx']));
});
