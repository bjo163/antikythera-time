import test from 'node:test';
import assert from 'node:assert/strict';
import {
  JULIAN_YEAR_SECONDS, MEAN_SYNODIC_MONTH_NS, METONIC_CYCLE, METONIC_MONTHS, NS_PER_DAY,
  ReferenceFrame, SAROS_CYCLE, SAROS_MONTHS, SECONDS_PER_DAY, TT_MINUS_TAI_NS,
  TimeScale, UTime, applyGearRatio, compareLunarReference, gearRatio, julianDateToUtcDate,
  lunarModelAtUtc, parseHorizonsObserverRows, summarizeValidation, taiMinusUtcAt, taiToTt,
  ttToTai, utcDateToTt
} from '../src/index.js';

test('J2000.0 TT is zero',()=>{const t=UTime.j2000TT();assert.equal(t.nsSinceJ2000,0n);assert.equal(t.julianDateTT(),2451545.0);});
test('+86400 SI seconds is one Julian display day',()=>{const t=UTime.j2000TT().plusSeconds(SECONDS_PER_DAY);assert.equal(t.nsSinceJ2000,NS_PER_DAY);assert.equal(t.julianDateTT(),2451546.0);});
test('duration uses BigInt nanoseconds',()=>assert.equal(UTime.fromNanoseconds(25n).difference(UTime.fromNanoseconds(10n)),15n));
test('Julian year is 31557600 SI seconds',()=>{const a=UTime.j2000TT();assert.equal(a.plusSeconds(JULIAN_YEAR_SECONDS).julianYearsSince(a),1);});
test('cross-scale subtraction is rejected',()=>{const tt=new UTime({scale:TimeScale.TT,frame:ReferenceFrame.GCRS});const tdb=new UTime({scale:TimeScale.TDB,frame:ReferenceFrame.GCRS});assert.throws(()=>tt.difference(tdb),/time-scale mismatch/);});
test('core refuses implicit conversion',()=>assert.throws(()=>UTime.j2000TT().toScale(TimeScale.TDB),/no implicit/));
test('cycle wraps exactly',()=>{const p=METONIC_CYCLE.positionAt(UTime.fromNanoseconds(METONIC_CYCLE.periodNs));assert.equal(p.remainderNs,0n);assert.equal(p.phase,0);});
test('Metonic/Saros metadata matches 235/223 months',()=>{assert.equal(METONIC_MONTHS,235n);assert.equal(SAROS_MONTHS,223n);assert.equal(METONIC_CYCLE.periodNs,235n*MEAN_SYNODIC_MONTH_NS);assert.equal(SAROS_CYCLE.periodNs,223n*MEAN_SYNODIC_MONTH_NS);});
test('gear ratio primitive works',()=>{const r=gearRatio(64n,38n);assert.equal(applyGearRatio(1,r),64/38);});
test('TT <-> TAI roundtrip uses exact 32.184 s offset',()=>{const tt=UTime.j2000TT();const tai=ttToTai(tt);assert.equal(tai.nsSinceJ2000,-TT_MINUS_TAI_NS);assert.equal(taiToTt(tai).nsSinceJ2000,0n);});
test('leap table returns 32 seconds at J2000-era UTC',()=>assert.equal(taiMinusUtcAt(new Date('2000-01-06T18:14:00Z')),32n));
test('leap table returns 37 seconds in 2026',()=>assert.equal(taiMinusUtcAt(new Date('2026-10-01T00:00:00Z')),37n));
test('UTC->TT maps J2000 UTC representation to zero',()=>assert.equal(utcDateToTt(new Date('2000-01-01T11:58:55.816Z')).nsSinceJ2000,0n));
test('UTC conversion refuses unsupported future dates',()=>assert.throws(()=>utcDateToTt(new Date('2030-01-01T00:00:00Z')),/supported only/));
test('lunar model is new Moon at NASA/GSFC calibration instant',()=>{const m=lunarModelAtUtc(new Date('2000-01-06T18:14:00Z'));assert.ok(m.phase<1e-12);assert.ok(m.illuminationPercent<1e-10);});
test('lunar model is near full Moon after half a mean synodic month',()=>{const halfMs=Number(MEAN_SYNODIC_MONTH_NS/2n)/1e6;const m=lunarModelAtUtc(new Date(Date.parse('2000-01-06T18:14:00Z')+halfMs));assert.ok(m.illuminationPercent>99.999);assert.ok(m.phaseAngleDeg<0.01);});
test('Horizons parser extracts JD, illumination and phase angle',()=>{const fixture='header\n$$SOE\n2460310.500000000, , , 78.1250, 55.4400,\n$$EOE\nfooter';const [r]=parseHorizonsObserverRows(fixture);assert.equal(r.jd,2460310.5);assert.equal(r.illuminatedPercent,78.125);assert.equal(r.phaseAngleDeg,55.44);});
test('Julian Date converter maps Unix epoch',()=>assert.equal(julianDateToUtcDate(2440587.5).toISOString(),'1970-01-01T00:00:00.000Z'));
test('validation comparison produces signed and absolute errors',()=>{const s=compareLunarReference({date:new Date('2000-01-06T18:14:00Z'),illuminatedPercent:1,phaseAngleDeg:179});assert.ok(s.error.illuminationPoints<0);assert.equal(s.error.absIlluminationPoints,Math.abs(s.error.illuminationPoints));});
test('validation summary reports MAE/RMSE/max',()=>{const a=compareLunarReference({date:new Date('2000-01-06T18:14:00Z'),illuminatedPercent:1,phaseAngleDeg:179});const b=compareLunarReference({date:new Date('2000-01-21T12:00:00Z'),illuminatedPercent:99,phaseAngleDeg:1});const s=summarizeValidation([a,b]);assert.equal(s.sampleCount,2);assert.ok(s.illumination.maePoints>=0);assert.ok(s.phaseAngle.rmseDeg>=0);});
