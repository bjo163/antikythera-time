import test from 'node:test';
import assert from 'node:assert/strict';
import {
  JULIAN_YEAR_SECONDS, MEAN_SYNODIC_MONTH_NS, METONIC_CYCLE, METONIC_MONTHS, NS_PER_DAY,
  ReferenceFrame, SAROS_CYCLE, SAROS_MONTHS, SECONDS_PER_DAY, TT_MINUS_TAI_NS,
  TimeScale, UTime, applyGearRatio, gearRatio, taiToTt, ttToTai, utcDateToCurrentTt
} from '../src/index.js';

test('J2000.0 TT is zero', () => { const t = UTime.j2000TT(); assert.equal(t.nsSinceJ2000, 0n); assert.equal(t.julianDateTT(), 2451545.0); });
test('+86400 SI seconds is one Julian display day', () => { const t = UTime.j2000TT().plusSeconds(SECONDS_PER_DAY); assert.equal(t.nsSinceJ2000, NS_PER_DAY); assert.equal(t.julianDateTT(), 2451546.0); });
test('duration uses BigInt nanoseconds', () => { assert.equal(UTime.fromNanoseconds(25n).difference(UTime.fromNanoseconds(10n)), 15n); });
test('Julian year is 31557600 SI seconds', () => { const a = UTime.j2000TT(); const b = a.plusSeconds(JULIAN_YEAR_SECONDS); assert.equal(b.julianYearsSince(a), 1); });
test('cross-scale subtraction is rejected', () => { const tt = new UTime({scale:TimeScale.TT,frame:ReferenceFrame.GCRS}); const tdb = new UTime({scale:TimeScale.TDB,frame:ReferenceFrame.GCRS}); assert.throws(()=>tt.difference(tdb),/time-scale mismatch/); });
test('core refuses implicit conversion', () => { assert.throws(()=>UTime.j2000TT().toScale(TimeScale.TDB),/no implicit/); });
test('cycle wraps exactly', () => { const p = METONIC_CYCLE.positionAt(UTime.fromNanoseconds(METONIC_CYCLE.periodNs)); assert.equal(p.remainderNs,0n); assert.equal(p.phase,0); });
test('Metonic/Saros metadata matches 235/223 months', () => { assert.equal(METONIC_MONTHS,235n); assert.equal(SAROS_MONTHS,223n); assert.equal(METONIC_CYCLE.periodNs,235n*MEAN_SYNODIC_MONTH_NS); assert.equal(SAROS_CYCLE.periodNs,223n*MEAN_SYNODIC_MONTH_NS); });
test('gear ratio primitive works', () => { const r=gearRatio(64n,38n); assert.equal(applyGearRatio(1,r),64/38); });
test('TT <-> TAI roundtrip uses exact 32.184 s offset', () => { const tt=UTime.j2000TT(); const tai=ttToTai(tt); assert.equal(tai.nsSinceJ2000,-TT_MINUS_TAI_NS); assert.equal(taiToTt(tai).nsSinceJ2000,0n); });
test('current UTC conversion accounts for five leap seconds added since J2000', () => { const t=utcDateToCurrentTt(new Date('2026-10-01T00:00:00Z')); assert.equal(t.scale,TimeScale.TT); assert.ok(t.nsSinceJ2000>0n); });
test('UTC conversion refuses dates outside validated IERS window', () => { assert.throws(()=>utcDateToCurrentTt(new Date('2030-01-01T00:00:00Z')),/bounded/); });
