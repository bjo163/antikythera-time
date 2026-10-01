import { NS_PER_SECOND } from './constants.js';
import { ReferenceFrame, TimeScale, UTime } from './utime.js';

export const TT_MINUS_TAI_NS = 32_184_000_000n;
export const J2000_TAI_MINUS_UTC_SECONDS = 32n;
export const CURRENT_TAI_MINUS_UTC_SECONDS = 37n;
export const LEAP_TABLE_VALID_FROM = '1972-01-01T00:00:00Z';
export const LEAP_TABLE_VALID_THROUGH = '2027-01-01T00:00:00Z';

export const TAI_MINUS_UTC_TABLE = Object.freeze([
  ['1972-01-01T00:00:00Z',10n],['1972-07-01T00:00:00Z',11n],
  ['1973-01-01T00:00:00Z',12n],['1974-01-01T00:00:00Z',13n],
  ['1975-01-01T00:00:00Z',14n],['1976-01-01T00:00:00Z',15n],
  ['1977-01-01T00:00:00Z',16n],['1978-01-01T00:00:00Z',17n],
  ['1979-01-01T00:00:00Z',18n],['1980-01-01T00:00:00Z',19n],
  ['1981-07-01T00:00:00Z',20n],['1982-07-01T00:00:00Z',21n],
  ['1983-07-01T00:00:00Z',22n],['1985-07-01T00:00:00Z',23n],
  ['1988-01-01T00:00:00Z',24n],['1990-01-01T00:00:00Z',25n],
  ['1991-01-01T00:00:00Z',26n],['1992-07-01T00:00:00Z',27n],
  ['1993-07-01T00:00:00Z',28n],['1994-07-01T00:00:00Z',29n],
  ['1996-01-01T00:00:00Z',30n],['1997-07-01T00:00:00Z',31n],
  ['1999-01-01T00:00:00Z',32n],['2006-01-01T00:00:00Z',33n],
  ['2009-01-01T00:00:00Z',34n],['2012-07-01T00:00:00Z',35n],
  ['2015-07-01T00:00:00Z',36n],['2017-01-01T00:00:00Z',37n],
]);

export const J2000_UTC_UNIX_MS = Date.parse('2000-01-01T11:58:55.816Z');

function assertDate(date) {
  if (!(date instanceof Date) || Number.isNaN(date.getTime())) throw new TypeError('date must be a valid Date');
}

export function taiMinusUtcAt(date) {
  assertDate(date);
  const ms = date.getTime();
  if (ms < Date.parse(LEAP_TABLE_VALID_FROM) || ms >= Date.parse(LEAP_TABLE_VALID_THROUGH)) {
    throw new RangeError('UTC conversion supported only in ['+LEAP_TABLE_VALID_FROM+', '+LEAP_TABLE_VALID_THROUGH+')');
  }
  let offset=10n;
  for (const [iso,value] of TAI_MINUS_UTC_TABLE) {
    if (ms >= Date.parse(iso)) offset=value; else break;
  }
  return offset;
}

export function ttToTai(tt) {
  if (!(tt instanceof UTime) || tt.scale !== TimeScale.TT) throw new TypeError('ttToTai requires a TT UTime');
  return new UTime({ nsSinceJ2000: tt.nsSinceJ2000 - TT_MINUS_TAI_NS, scale: TimeScale.TAI, frame: tt.frame, uncertaintyNs: tt.uncertaintyNs });
}

export function taiToTt(tai) {
  if (!(tai instanceof UTime) || tai.scale !== TimeScale.TAI) throw new TypeError('taiToTt requires a TAI UTime');
  return new UTime({ nsSinceJ2000: tai.nsSinceJ2000 + TT_MINUS_TAI_NS, scale: TimeScale.TT, frame: tai.frame, uncertaintyNs: tai.uncertaintyNs });
}

export function utcDateToTt(date = new Date()) {
  assertDate(date);
  const offset=taiMinusUtcAt(date);
  const posixElapsedMs=BigInt(date.getTime()-J2000_UTC_UNIX_MS);
  const physicalElapsedNs=posixElapsedMs*1_000_000n+(offset-J2000_TAI_MINUS_UTC_SECONDS)*NS_PER_SECOND;
  return new UTime({ nsSinceJ2000: physicalElapsedNs, scale: TimeScale.TT, frame: ReferenceFrame.GCRS, uncertaintyNs: 1_000_000n });
}

export const utcDateToCurrentTt = utcDateToTt;

export function scaleEvidence() {
  return {
    ttTai:{status:'validated',relation:'TT = TAI + 32.184 s',source:'BIPM / IAU'},
    utcTai:{status:'table-backed',validWindow:[LEAP_TABLE_VALID_FROM,LEAP_TABLE_VALID_THROUGH],currentOffsetSeconds:37,source:'IERS / BIPM leap-second history'},
    ttTcg:{status:'validated-against-reference-vectors',relation:'IAU 2000 B1.9',source:'IAU SOFA / IERS'},
    tdbTcb:{status:'validated-against-reference-vectors',relation:'IAU 2006 B3',source:'IAU SOFA / IERS'},
    ttTdb:{status:'canonical-with-explicit-dtr',reason:'automatic TDB-TT periodic model is intentionally not invented',source:'IAU SOFA / IERS'},
  };
}
