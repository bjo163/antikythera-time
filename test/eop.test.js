import test from 'node:test';
import assert from 'node:assert/strict';
import { parseIersFinals2000A,interpolateEop,ut1DateFromUtc,makeEarthObserver,observerDtdbGeometry,makeSpacecraftObserver,weakFieldProperTimeRate } from '../src/index.js';

const fixture=`23  1  1  59945.00 I  0.062781 0.000012  0.200308 0.000009 I -0.0198681 0.0000071  0.1595 0.0041
23  1  2  59946.00 I  0.061000 0.000012  0.201000 0.000009 I -0.0208681 0.0000071  0.1600 0.0041`;

test('IERS finals parser extracts polar motion and UT1-UTC',()=>{
  const r=parseIersFinals2000A(fixture);assert.equal(r.length,2);assert.equal(r[0].mjd,59945);assert.equal(r[0].ut1MinusUtcSeconds,-0.0198681);assert.equal(r[0].evidence,'OBSERVED');
});
test('EOP interpolation is deterministic',()=>{
  const r=parseIersFinals2000A(fixture),m=interpolateEop(r,59945.5);assert.ok(Math.abs(m.ut1MinusUtcSeconds+0.0203681)<1e-10);assert.equal(m.interpolated,true);
});
test('UTC to UT1 applies IERS UT1-UTC',()=>{
  const d=new Date('2023-01-01T00:00:00Z'),u=ut1DateFromUtc(d,{ut1MinusUtcSeconds:0.2});assert.equal(u.getTime()-d.getTime(),200);
});
test('WGS84 observer yields SOFA-compatible u/v geometry',()=>{
  const o=makeEarthObserver({longitudeDeg:106.8,latitudeDeg:-6.3,heightMeters:50});const g=observerDtdbGeometry(o);assert.ok(g.uKm>6000);assert.ok(g.vKm<0);assert.ok(Math.abs(g.elongRad)>1);
});
test('spacecraft observer preserves declared state and frame',()=>{
  const o=makeSpacecraftObserver({id:'demo',positionKm:[1,2,3],velocityKmS:[4,5,6],epoch:'J2000'});assert.equal(o.frame,'BCRS');assert.equal(o.positionKm[2],3);
});
test('weak-field proper-time rate decreases with speed',()=>{
  assert.ok(weakFieldProperTimeRate({potentialM2S2:0,velocityMS:1000})<1);
});
