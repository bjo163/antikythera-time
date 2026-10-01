import { MEAN_SYNODIC_MONTH_NS } from './constants.js';
import { UTime } from './utime.js';

function assertPositiveBigInt(name, value) {
  if (typeof value !== 'bigint' || value <= 0n) {
    throw new TypeError(`${name} must be a positive bigint`);
  }
}

function modulo(value, modulus) {
  const r = value % modulus;
  return r < 0n ? r + modulus : r;
}

/**
 * A digital analogue of the Antikythera idea: represent a long astronomical
 * period as a repeatable cycle and expose its phase/index mechanically-like.
 */
export class AstronomicalCycle {
  constructor({ name, periodNs, basis, evidence }) {
    assertPositiveBigInt('periodNs', periodNs);
    this.name = name;
    this.periodNs = periodNs;
    this.basis = basis;
    this.evidence = evidence;
    Object.freeze(this);
  }

  positionAt(time) {
    if (!(time instanceof UTime)) throw new TypeError('time must be UTime');
    const remainderNs = modulo(time.nsSinceJ2000, this.periodNs);
    const scaled = (remainderNs * 1_000_000_000n) / this.periodNs;
    return {
      cycle: this.name,
      remainderNs,
      phase: Number(scaled) / 1_000_000_000,
      note: 'phase is relative to the U-Time/J2000 origin, not a reconstructed ancient dial zero-point',
    };
  }
}

export const METONIC_MONTHS = 235n;
export const SAROS_MONTHS = 223n;

export const METONIC_CYCLE = new AstronomicalCycle({
  name: 'Metonic',
  periodNs: METONIC_MONTHS * MEAN_SYNODIC_MONTH_NS,
  basis: '235 mean synodic months; historically represented as a 19-year calendar cycle',
  evidence: 'Antikythera upper back dial / Metonic cycle',
});

export const SAROS_CYCLE = new AstronomicalCycle({
  name: 'Saros',
  periodNs: SAROS_MONTHS * MEAN_SYNODIC_MONTH_NS,
  basis: '223 mean synodic months; eclipse-cycle layer',
  evidence: 'Antikythera lower back dial / Saros eclipse prediction',
});

export function gearRatio(driverTeeth, drivenTeeth) {
  assertPositiveBigInt('driverTeeth', driverTeeth);
  assertPositiveBigInt('drivenTeeth', drivenTeeth);
  return { numerator: driverTeeth, denominator: drivenTeeth };
}

export function applyGearRatio(inputTurns, ratio) {
  if (typeof inputTurns !== 'number' || !Number.isFinite(inputTurns)) {
    throw new TypeError('inputTurns must be a finite number');
  }
  return inputTurns * Number(ratio.numerator) / Number(ratio.denominator);
}
