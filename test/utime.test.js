import test from 'node:test';
import assert from 'node:assert/strict';

import {
  JULIAN_YEAR_SECONDS,
  MEAN_SYNODIC_MONTH_NS,
  METONIC_CYCLE,
  METONIC_MONTHS,
  NS_PER_DAY,
  ReferenceFrame,
  SAROS_CYCLE,
  SAROS_MONTHS,
  SECONDS_PER_DAY,
  TimeScale,
  UTime,
  applyGearRatio,
  gearRatio,
} from '../src/index.js';

test('J2000.0 TT is zero in the U-Time coordinate', () => {
  const t = UTime.j2000TT();
  assert.equal(t.nsSinceJ2000, 0n);
  assert.equal(t.julianDateTT(), 2451545.0);
});

test('+86400 SI seconds advances exactly one U-Time day and one JD display day', () => {
  const t = UTime.j2000TT().plusSeconds(SECONDS_PER_DAY);
  assert.equal(t.nsSinceJ2000, NS_PER_DAY);
  assert.equal(t.julianDateTT(), 2451546.0);
});

test('duration uses BigInt nanoseconds', () => {
  const a = UTime.fromNanoseconds(10n);
  const b = UTime.fromNanoseconds(25n);
  assert.equal(b.difference(a), 15n);
});

test('Julian year is 31,557,600 SI seconds by definition', () => {
  const a = UTime.j2000TT();
  const b = a.plusSeconds(JULIAN_YEAR_SECONDS);
  assert.equal(b.julianYearsSince(a), 1);
});

test('cross-scale subtraction is rejected instead of silently producing fake precision', () => {
  const tt = new UTime({ scale: TimeScale.TT, frame: ReferenceFrame.GCRS });
  const tdb = new UTime({ scale: TimeScale.TDB, frame: ReferenceFrame.GCRS });
  assert.throws(() => tt.difference(tdb), /time-scale mismatch/);
});

test('v0.2 refuses unvalidated time-scale conversion', () => {
  const tt = UTime.j2000TT();
  assert.throws(() => tt.toScale(TimeScale.TDB), /no validated/);
});

test('cycle engine wraps exactly at one period', () => {
  const t = UTime.fromNanoseconds(METONIC_CYCLE.periodNs);
  const p = METONIC_CYCLE.positionAt(t);
  assert.equal(p.remainderNs, 0n);
  assert.equal(p.phase, 0);
});

test('Antikythera cycle metadata uses 235-month Metonic and 223-month Saros structure', () => {
  assert.equal(METONIC_MONTHS, 235n);
  assert.equal(SAROS_MONTHS, 223n);
  assert.equal(METONIC_CYCLE.periodNs, 235n * MEAN_SYNODIC_MONTH_NS);
  assert.equal(SAROS_CYCLE.periodNs, 223n * MEAN_SYNODIC_MONTH_NS);
});

test('generic gear-ratio engine composes an idealized rotation ratio', () => {
  const ratio = gearRatio(64n, 38n);
  assert.equal(applyGearRatio(1, ratio), 64 / 38);
});
