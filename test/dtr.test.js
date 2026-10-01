import test from 'node:test';
import assert from 'node:assert/strict';
import { TimeScale, coordinateFromTwoPartJD, nasaSimpleDtr, NASA_SIMPLE_DTR_MODEL, ttToTdbCoordinate } from '../src/index.js';

test('NASA simple dtr provider is millisecond-scale and explicitly approximate',()=>{
  const tt=coordinateFromTwoPartJD(TimeScale.TT,2451545.0,0);
  const d=nasaSimpleDtr(tt);
  assert.equal(d.status,'APPROXIMATE_GEOCENTRIC');
  assert.ok(Math.abs(d.seconds)<0.002);
  assert.equal(d.model.id,NASA_SIMPLE_DTR_MODEL.id);
});

test('automatic approximate dtr can feed canonical TT->TDB wrapper with uncertainty',()=>{
  const tt=coordinateFromTwoPartJD(TimeScale.TT,2451545.0,0);
  const d=nasaSimpleDtr(tt);
  const tdb=ttToTdbCoordinate(tt,d);
  assert.equal(tdb.scale,TimeScale.TDB);
  assert.ok(tdb.uncertaintySeconds>=NASA_SIMPLE_DTR_MODEL.declaredModelUncertaintySeconds);
});
