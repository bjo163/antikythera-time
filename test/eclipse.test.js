import test from 'node:test';
import assert from 'node:assert/strict';
import { NASA_SAROS_DAYS, NASA_EXELIGMOS_DAYS, NASA_SOLAR_SAROS_139_REFERENCE, predictSaros, predictExeligmos, eclipseTimingResidualMinutes, validateSaros139 } from '../src/index.js';

test('NASA Saros period matches 223-month recurrence scale',()=>{
  assert.ok(Math.abs(NASA_SAROS_DAYS-6585.3223)<1e-9);
  assert.ok(Math.abs(NASA_EXELIGMOS_DAYS-19755.9669)<1e-9);
});

test('Saros 139 recurrence predicts 2042 and 2060 within 30 minutes of NASA greatest-eclipse TT',()=>{
  const r=validateSaros139();
  assert.equal(r.comparisons.length,2);
  assert.equal(r.allPass,true);
  for(const c of r.comparisons) assert.ok(c.absResidualMinutes<30);
});

test('Saros predictor preserves series and type while exposing timing drift',()=>{
  const seed=NASA_SOLAR_SAROS_139_REFERENCE[0];
  const p=predictSaros(seed,1),ref=NASA_SOLAR_SAROS_139_REFERENCE[1];
  assert.equal(p.saros,139);
  assert.equal(p.type,'TOTAL');
  assert.notEqual(eclipseTimingResidualMinutes(p,ref),0);
});

test('Exeligmos is exactly three Saros recurrences in the cycle model',()=>{
  const seed=NASA_SOLAR_SAROS_139_REFERENCE[0];
  assert.equal(predictExeligmos(seed,1).jdTt,predictSaros(seed,3).jdTt);
});
