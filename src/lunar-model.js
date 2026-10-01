import { MEAN_SYNODIC_MONTH_NS, NS_PER_DAY } from './constants.js';
import { utcDateToTt } from './scales.js';

// NASA/GSFC Six Millennium phase catalog: New Moon 2000-01-06 18:14 UT.
export const LUNAR_REFERENCE_NEW_MOON_UTC = '2000-01-06T18:14:00Z';
export const LUNAR_REFERENCE_NEW_MOON_TT = utcDateToTt(new Date(LUNAR_REFERENCE_NEW_MOON_UTC));
export const BASELINE_SYNODIC_PERIOD_SECONDS = Number(MEAN_SYNODIC_MONTH_NS) / 1e9;
export const DEFAULT_LUNAR_MODEL_PARAMS = Object.freeze({
  periodSeconds: BASELINE_SYNODIC_PERIOD_SECONDS,
  epochOffsetSeconds: 0,
});

function modulo(value, modulus) {
  const r = value % modulus;
  return r < 0n ? r + modulus : r;
}

function phaseName(phase) {
  const names = ['New Moon','Waxing Crescent','First Quarter','Waxing Gibbous','Full Moon','Waning Gibbous','Last Quarter','Waning Crescent'];
  return names[Math.floor((phase * 8 + 0.5) % 8)];
}

function normalizeParams(params = DEFAULT_LUNAR_MODEL_PARAMS) {
  const periodSeconds = Number(params.periodSeconds ?? BASELINE_SYNODIC_PERIOD_SECONDS);
  const epochOffsetSeconds = Number(params.epochOffsetSeconds ?? 0);
  if (!Number.isFinite(periodSeconds) || periodSeconds <= 0) throw new RangeError('periodSeconds must be finite and positive');
  if (!Number.isFinite(epochOffsetSeconds)) throw new RangeError('epochOffsetSeconds must be finite');
  return { periodSeconds, epochOffsetSeconds };
}

export function lunarModelAtTtWithParams(tt, params = DEFAULT_LUNAR_MODEL_PARAMS) {
  const p = normalizeParams(params);
  const periodNs = BigInt(Math.round(p.periodSeconds * 1e9));
  const epochOffsetNs = BigInt(Math.round(p.epochOffsetSeconds * 1e9));
  const elapsed = tt.nsSinceJ2000 - (LUNAR_REFERENCE_NEW_MOON_TT.nsSinceJ2000 + epochOffsetNs);
  const remainder = modulo(elapsed, periodNs);
  const phase = Number(remainder) / Number(periodNs);
  const illuminationPercent = 50 * (1 - Math.cos(2 * Math.PI * phase));
  const phaseAngleDeg = Math.abs(180 - 360 * phase);
  const ageDays = Number(remainder) / Number(NS_PER_DAY);
  return {
    phase,
    phaseName: phaseName(phase),
    ageDays,
    illuminationPercent,
    phaseAngleDeg,
    waxing: phase < 0.5,
    model: 'mean-synodic-cycle',
    calibrationUtc: LUNAR_REFERENCE_NEW_MOON_UTC,
    params: p,
  };
}

export function lunarModelAtTt(tt) {
  return lunarModelAtTtWithParams(tt, DEFAULT_LUNAR_MODEL_PARAMS);
}

export function lunarModelAtUtcWithParams(date, params = DEFAULT_LUNAR_MODEL_PARAMS) {
  return lunarModelAtTtWithParams(utcDateToTt(date), params);
}

export function lunarModelAtUtc(date) {
  return lunarModelAtUtcWithParams(date, DEFAULT_LUNAR_MODEL_PARAMS);
}
