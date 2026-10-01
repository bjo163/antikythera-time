import { NS_PER_DAY } from '../constants.js';
import { ReferenceFrame, TimeScale, UTime } from '../utime.js';
import { REL_J2000_JD } from './constants.js';

const ALLOWED = new Set([TimeScale.TT, TimeScale.TCG, TimeScale.TDB, TimeScale.TCB]);

export class CoordinateTime {
  constructor({
    d1,
    d2,
    scale,
    frame,
    uncertaintySeconds = 0,
    provenance = null,
  }) {
    if (!Number.isFinite(d1) || !Number.isFinite(d2)) throw new TypeError('d1/d2 must be finite');
    if (!ALLOWED.has(scale)) throw new RangeError('CoordinateTime scale must be TT, TCG, TDB or TCB');
    if (!Number.isFinite(uncertaintySeconds) || uncertaintySeconds < 0) {
      throw new RangeError('uncertaintySeconds must be finite and non-negative');
    }
    this.d1 = d1;
    this.d2 = d2;
    this.scale = scale;
    this.frame = frame;
    this.uncertaintySeconds = uncertaintySeconds;
    this.provenance = provenance;
    Object.freeze(this);
  }

  get jd() { return this.d1 + this.d2; }

  toJSON() {
    return {
      type: 'CoordinateTime',
      d1: this.d1,
      d2: this.d2,
      jd: this.jd,
      scale: this.scale,
      frame: this.frame,
      uncertaintySeconds: this.uncertaintySeconds,
      provenance: this.provenance,
    };
  }
}

export function coordinateFrameForScale(scale) {
  if (scale === TimeScale.TT || scale === TimeScale.TCG) return ReferenceFrame.GCRS;
  if (scale === TimeScale.TDB || scale === TimeScale.TCB) return ReferenceFrame.BCRS;
  throw new RangeError('unsupported relativistic coordinate time scale');
}

export function coordinateFromTwoPartJD(scale, d1, d2, options = {}) {
  return new CoordinateTime({
    d1,
    d2,
    scale,
    frame: options.frame ?? coordinateFrameForScale(scale),
    uncertaintySeconds: options.uncertaintySeconds ?? 0,
    provenance: options.provenance ?? null,
  });
}

// Converts the existing exact-ish BigInt J2000 TT storage into a two-part JD.
// The day remainder stays below one day, protecting far more precision than
// collapsing the complete JD into one floating-point number.
export function ttCoordinateFromUTime(value) {
  if (!(value instanceof UTime) || value.scale !== TimeScale.TT) {
    throw new TypeError('ttCoordinateFromUTime requires TT UTime');
  }

  let days = value.nsSinceJ2000 / NS_PER_DAY;
  let remainder = value.nsSinceJ2000 % NS_PER_DAY;
  if (remainder < 0n) {
    remainder += NS_PER_DAY;
    days -= 1n;
  }

  return coordinateFromTwoPartJD(
    TimeScale.TT,
    REL_J2000_JD + Number(days),
    Number(remainder) / Number(NS_PER_DAY),
    {
      uncertaintySeconds: Number(value.uncertaintyNs) / 1e9,
      provenance: 'UTime nsSinceJ2000 -> two-part JD(TT)',
    },
  );
}
