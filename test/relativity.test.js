import test from 'node:test';
import assert from 'node:assert/strict';
import {
  CoordinateTime,
  ReferenceFrame,
  TimeScale,
  UTime,
  coordinateDeltaSeconds,
  coordinateFromTwoPartJD,
  runRelativityReferenceVectors,
  tcbToTdbCoordinate,
  tcgToTtCoordinate,
  tdbToTcbCoordinate,
  tdbToTtCoordinate,
  ttCoordinateFromUTime,
  ttToTcgCoordinate,
  ttToTdbCoordinate,
} from '../src/index.js';

test('all six IAU SOFA reference vectors pass',()=>{
  const r=runRelativityReferenceVectors();
  assert.equal(r.total,6);
  assert.equal(r.passCount,6);
  assert.equal(r.allPass,true);
});

test('TT -> TCG matches SOFA validation vector',()=>{
  const out=ttToTcgCoordinate(coordinateFromTwoPartJD(TimeScale.TT,2453750.5,0.892482639));
  assert.ok(Math.abs(out.d2-0.8924900312508587113)<=1e-12);
  assert.equal(out.scale,TimeScale.TCG);
  assert.equal(out.frame,ReferenceFrame.GCRS);
});

test('TCG -> TT matches SOFA validation vector',()=>{
  const out=tcgToTtCoordinate(coordinateFromTwoPartJD(TimeScale.TCG,2453750.5,0.892862531));
  assert.ok(Math.abs(out.d2-0.8928551387488816828)<=1e-12);
});

test('TCB -> TDB matches SOFA validation vector',()=>{
  const out=tcbToTdbCoordinate(coordinateFromTwoPartJD(TimeScale.TCB,2453750.5,0.893019599));
  assert.ok(Math.abs(out.d2-0.8928551362746343397)<=1e-12);
  assert.equal(out.frame,ReferenceFrame.BCRS);
});

test('TDB -> TCB matches SOFA validation vector',()=>{
  const out=tdbToTcbCoordinate(coordinateFromTwoPartJD(TimeScale.TDB,2453750.5,0.892855137));
  assert.ok(Math.abs(out.d2-0.8930195997253656716)<=1e-12);
});

test('TT -> TDB with explicit dtr matches SOFA validation vector',()=>{
  const out=ttToTdbCoordinate(
    coordinateFromTwoPartJD(TimeScale.TT,2453750.5,0.892855139),
    {seconds:-0.000201,uncertaintySeconds:1e-9,provenance:'SOFA reference vector'},
  );
  assert.ok(Math.abs(out.d2-0.8928551366736111111)<=1e-12);
  assert.equal(out.scale,TimeScale.TDB);
  assert.equal(out.frame,ReferenceFrame.BCRS);
  assert.ok(out.uncertaintySeconds>=1e-9);
});

test('TDB -> TT with explicit dtr matches SOFA validation vector',()=>{
  const out=tdbToTtCoordinate(
    coordinateFromTwoPartJD(TimeScale.TDB,2453750.5,0.892855137),
    -0.000201,
  );
  assert.ok(Math.abs(out.d2-0.8928551393263888889)<=1e-12);
});

test('TT <-> TDB refuses missing dtr instead of inventing a model',()=>{
  const tt=coordinateFromTwoPartJD(TimeScale.TT,2451545.0,0);
  assert.throws(()=>ttToTdbCoordinate(tt),/requires dtr/);
  const tdb=coordinateFromTwoPartJD(TimeScale.TDB,2451545.0,0);
  assert.throws(()=>tdbToTtCoordinate(tdb),/requires dtr/);
});

test('TT <-> TCG roundtrip is stable at sub-microsecond scale',()=>{
  const tt=coordinateFromTwoPartJD(TimeScale.TT,2451545.0,0.123456789);
  const back=tcgToTtCoordinate(ttToTcgCoordinate(tt));
  assert.ok(Math.abs(coordinateDeltaSeconds(back,tt))<1e-6);
});

test('TDB <-> TCB roundtrip is stable at sub-microsecond scale',()=>{
  const tdb=coordinateFromTwoPartJD(TimeScale.TDB,2451545.0,0.234567891);
  const back=tcbToTdbCoordinate(tdbToTcbCoordinate(tdb));
  assert.ok(Math.abs(coordinateDeltaSeconds(back,tdb))<1e-6);
});

test('legacy UTime TT converts to a separate two-part CoordinateTime',()=>{
  const t=UTime.j2000TT().plusSeconds(86400n);
  const c=ttCoordinateFromUTime(t);
  assert.ok(c instanceof CoordinateTime);
  assert.equal(c.scale,TimeScale.TT);
  assert.equal(c.frame,ReferenceFrame.GCRS);
  assert.equal(c.d1,2451546.0);
  assert.equal(c.d2,0);
});

test('CoordinateTime rejects unsupported civil scale',()=>{
  assert.throws(()=>coordinateFromTwoPartJD(TimeScale.UTC,2451545,0),/TT, TCG, TDB or TCB/);
});
