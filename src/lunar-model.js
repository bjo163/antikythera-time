import { MEAN_SYNODIC_MONTH_NS, NS_PER_DAY } from './constants.js';
import { utcDateToTt } from './scales.js';

export const LUNAR_REFERENCE_NEW_MOON_UTC = '2000-01-06T18:14:00Z';
export const LUNAR_REFERENCE_NEW_MOON_TT = utcDateToTt(new Date(LUNAR_REFERENCE_NEW_MOON_UTC));

function modulo(value, modulus) {
  const r = value % modulus;
  return r < 0n ? r + modulus : r;
}

function phaseName(phase) {
  const names = ['New Moon','Waxing Crescent','First Quarter','Waxing Gibbous','Full Moon','Waning Gibbous','Last Quarter','Waning Crescent'];
  return names[Math.floor((phase * 8 + 0.5) % 8)];
}

export function lunarModelAtTt(tt) {
  const elapsed = tt.nsSinceJ2000 - LUNAR_REFERENCE_NEW_MOON_TT.nsSinceJ2000;
  const remainder = modulo(elapsed, MEAN_SYNODIC_MONTH_NS);
  const phase = Number(remainder) / Number(MEAN_SYNODIC_MONTH_NS);
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
  };
}

export function lunarModelAtUtc(date) {
  return lunarModelAtTt(utcDateToTt(date));
}
