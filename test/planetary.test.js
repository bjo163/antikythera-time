import test from 'node:test';
import assert from 'node:assert/strict';
import { JPL_APPROX_PLANETS, jplApproxHeliocentric, jplApproxGeocentric, vectorErrorAu } from '../src/index.js';

test('JPL approximate layer covers Earth and five classical planets',()=>{
  for(const name of ['Mercury','Venus','Earth','Mars','Jupiter','Saturn']) assert.ok(JPL_APPROX_PLANETS.includes(name));
});

test('planetary vectors are finite inside 1800-2050 validity interval',()=>{
  for(const name of JPL_APPROX_PLANETS){
    const p=jplApproxHeliocentric(name,2451545.0);
    for(const key of ['x','y','z','radiusAu','longitudeDeg','latitudeDeg']) assert.ok(Number.isFinite(p[key]));
    assert.ok(p.radiusAu>0);
  }
});

test('geocentric Earth is the declared observer origin',()=>{
  const e=jplApproxGeocentric('Earth',2451545.0);
  assert.deepEqual([e.x,e.y,e.z],[0,0,0]);
});

test('planetary approximation rejects epochs outside its JPL validity range',()=>{
  assert.throws(()=>jplApproxHeliocentric('Mars',2300000),/1800-2050/);
  assert.throws(()=>jplApproxHeliocentric('Mars',2500000),/1800-2050/);
});

test('vector error is Euclidean in AU',()=>{
  assert.equal(vectorErrorAu({x:0,y:0,z:0},{x:3,y:4,z:0}),5);
});
