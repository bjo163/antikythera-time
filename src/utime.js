import { J2000_JD_TT, JULIAN_YEAR_NS, NS_PER_DAY, NS_PER_SECOND } from './constants.js';

export const TimeScale = Object.freeze({ TT: 'TT', TAI: 'TAI', UTC: 'UTC', TCG: 'TCG', TDB: 'TDB', TCB: 'TCB' });
export const ReferenceFrame = Object.freeze({ GCRS: 'GCRS', BCRS: 'BCRS', UNSPECIFIED: 'UNSPECIFIED' });

function assertBigInt(name, value) {
  if (typeof value !== 'bigint') throw new TypeError(`${name} must be bigint`);
}

export class UTime {
  constructor({ nsSinceJ2000 = 0n, scale = TimeScale.TT, frame = ReferenceFrame.GCRS, uncertaintyNs = 0n } = {}) {
    assertBigInt('nsSinceJ2000', nsSinceJ2000);
    assertBigInt('uncertaintyNs', uncertaintyNs);
    if (uncertaintyNs < 0n) throw new RangeError('uncertaintyNs cannot be negative');
    this.nsSinceJ2000 = nsSinceJ2000;
    this.scale = scale;
    this.frame = frame;
    this.uncertaintyNs = uncertaintyNs;
    Object.freeze(this);
  }
  static j2000TT() { return new UTime(); }
  static fromSeconds(seconds, options = {}) {
    assertBigInt('seconds', seconds);
    return new UTime({ ...options, nsSinceJ2000: seconds * NS_PER_SECOND });
  }
  static fromNanoseconds(nsSinceJ2000, options = {}) { return new UTime({ ...options, nsSinceJ2000 }); }
  plusNanoseconds(deltaNs) {
    assertBigInt('deltaNs', deltaNs);
    return new UTime({ nsSinceJ2000: this.nsSinceJ2000 + deltaNs, scale: this.scale, frame: this.frame, uncertaintyNs: this.uncertaintyNs });
  }
  plusSeconds(deltaSeconds) { assertBigInt('deltaSeconds', deltaSeconds); return this.plusNanoseconds(deltaSeconds * NS_PER_SECOND); }
  difference(other) {
    if (!(other instanceof UTime)) throw new TypeError('other must be UTime');
    if (this.scale !== other.scale) throw new Error(`time-scale mismatch: ${this.scale} vs ${other.scale}`);
    if (this.frame !== other.frame) throw new Error(`reference-frame mismatch: ${this.frame} vs ${other.frame}`);
    return this.nsSinceJ2000 - other.nsSinceJ2000;
  }
  julianDateTT() {
    if (this.scale !== TimeScale.TT) throw new Error('julianDateTT() requires TT; perform a real scale conversion first');
    return J2000_JD_TT + Number(this.nsSinceJ2000) / Number(NS_PER_DAY);
  }
  julianYearsSince(other) { return Number(this.difference(other)) / Number(JULIAN_YEAR_NS); }
  toScale(targetScale) {
    if (targetScale === this.scale) return this;
    throw new Error(`U-Time core has no implicit ${this.scale}->${targetScale} conversion. Use the validated scale module.`);
  }
  toJSON() {
    return { protocol: 'U-Time', version: '2.0.0', unit: 'SI nanosecond', epoch: 'J2000.0 (JD 2451545.0 TT)', nsSinceJ2000: this.nsSinceJ2000.toString(), scale: this.scale, frame: this.frame, uncertaintyNs: this.uncertaintyNs.toString() };
  }
}
