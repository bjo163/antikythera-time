import test from 'node:test';
import assert from 'node:assert/strict';
import { parseCobayaText, reproduceCosmicAgeFromCobaya } from '../src/cosmology/cobaya.js';
import { inferCosmicAge } from '../src/cosmology/age.js';
import { omegaRadiationFromH0 } from '../src/cosmology/parameters.js';

test('Cobaya parser reads header, weights and derived columns',()=>{
  const text='# weight minuslogpost H0 omegam age chi2\n2 10 70 0.3 13.466 20\n1 11 68 0.31 13.70 22\n';
  const p=parseCobayaText(text);
  assert.deepEqual(p.columns,['weight','minuslogpost','H0','omegam','age','chi2']);
  assert.equal(p.rows.length,2);
  assert.equal(p.rows[0].weight,2);
});

test('posterior reproducer computes weighted cosmic age and compares official age when present',()=>{
  const r70=omegaRadiationFromH0(70),r68=omegaRadiationFromH0(68);
  const a=inferCosmicAge('flat-lcdm',{H0:70,omegaR:r70,omegaM:.3,omegaLambda:1-r70-.3}).result.gyr;
  const b=inferCosmicAge('flat-lcdm',{H0:68,omegaR:r68,omegaM:.31,omegaLambda:1-r68-.31}).result.gyr;
  const text=`# weight minuslogpost H0 omegam age\n2 1 70 .3 ${a}\n1 1 68 .31 ${b}\n`;
  const r=reproduceCosmicAgeFromCobaya(parseCobayaText(text),{model:'flat-lcdm',maxSamples:10});
  assert.equal(r.usedSamples,2);
  assert.ok(r.computedAgeGyr.mean>13);
  assert.ok(Math.abs(r.engineMinusOfficialGyr.mean)<0.01);
});

test('posterior reproducer rejects missing required cosmology columns',()=>{
  const text='# weight minuslogpost foo bar\n1 1 2 3\n';
  assert.throws(()=>reproduceCosmicAgeFromCobaya(parseCobayaText(text),{model:'flat-lcdm'}));
});
