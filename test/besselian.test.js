import test from 'node:test';import assert from 'node:assert/strict';
import { NASA_2026_02_17,evaluateBesselian,besselianReferenceSelfTest } from '../src/index.js';
test('NASA 2026 Besselian t0 coefficients reproduce published values',()=>{const e=evaluateBesselian(NASA_2026_02_17,12);assert.equal(e.x,0.321954);assert.equal(e.y,-0.926971);assert.ok(Math.abs(e.muDeg-356.514404)<1e-9);});
test('Besselian polynomial evolves continuously away from t0',()=>{const a=evaluateBesselian(NASA_2026_02_17,12),b=evaluateBesselian(NASA_2026_02_17,12.1);assert.notEqual(a.x,b.x);assert.ok(Number.isFinite(b.axisDistanceEarthRadii));});
test('Besselian self-test passes while preserving generation boundary',()=>{const r=besselianReferenceSelfTest();assert.equal(r.pass,true);assert.match(r.boundary,/not independent element generation/);});
