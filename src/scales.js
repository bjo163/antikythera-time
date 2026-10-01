import { NS_PER_SECOND } from './constants.js';
import { ReferenceFrame, TimeScale, UTime } from './utime.js';

export const TT_MINUS_TAI_NS = 32_184_000_000n;
export const J2000_TAI_MINUS_UTC_SECONDS = 32n;
export const CURRENT_TAI_MINUS_UTC_SECONDS = 37n;
export const CURRENT_UTC_OFFSET_VALID_FROM = '2017-01-01T00:00:00Z';
export const CURRENT_UTC_OFFSET_VALID_THROUGH = '2027-01-01T00:00:00Z';

export const J2000_UTC_UNIX_MS = Date.parse('2000-01-01T11:58:55.816Z');

export function ttToTai(tt) {
  if (!(tt instanceof UTime) || tt.scale !== TimeScale.TT) throw new TypeError('ttToTai requires a TT UTime');
  return new UTime({ nsSinceJ2000: tt.nsSinceJ2000 - TT_MINUS_TAI_NS, scale: TimeScale.TAI, frame: tt.frame, uncertaintyNs: tt.uncertaintyNs });
}

export function taiToTt(tai) {
  if (!(tai instanceof UTime) || tai.scale !== TimeScale.TAI) throw new TypeError('taiToTt requires a TAI UTime');
  return new UTime({ nsSinceJ2000: tai.nsSinceJ2000 + TT_MINUS_TAI_NS, scale: TimeScale.TT, frame: tai.frame, uncertaintyNs: tai.uncertaintyNs });
}

export function utcDateToCurrentTt(date = new Date()) {
  if (!(date instanceof Date) || Number.isNaN(date.getTime())) throw new TypeError('date must be a valid Date');
  const from = Date.parse(CURRENT_UTC_OFFSET_VALID_FROM);
  const through = Date.parse(CURRENT_UTC_OFFSET_VALID_THROUGH);
  if (date.getTime() < from || date.getTime() >= through) {
    throw new RangeError('current UTC conversion is intentionally bounded; refresh the IERS leap-second bulletin before using this date');
  }
  const posixElapsedMs = BigInt(date.getTime() - J2000_UTC_UNIX_MS);
  const leapSecondsAddedSinceJ2000 = CURRENT_TAI_MINUS_UTC_SECONDS - J2000_TAI_MINUS_UTC_SECONDS;
  const physicalElapsedNs = posixElapsedMs * 1_000_000n + leapSecondsAddedSinceJ2000 * NS_PER_SECOND;
  return new UTime({ nsSinceJ2000: physicalElapsedNs, scale: TimeScale.TT, frame: ReferenceFrame.GCRS, uncertaintyNs: 1_000_000n });
}

export function scaleEvidence() {
  return {
    ttTai: { status: 'validated', relation: 'TT = TAI + 32.184 s', source: 'BIPM / IAU' },
    currentUtcTai: { status: 'validated-for-window', relation: 'TAI - UTC = 37 s', window: [CURRENT_UTC_OFFSET_VALID_FROM, CURRENT_UTC_OFFSET_VALID_THROUGH], source: 'IERS Bulletin C 72' },
    tdbTcb: { status: 'not-implemented', reason: 'requires validated IAU/IERS relativistic transformations' },
  };
}
