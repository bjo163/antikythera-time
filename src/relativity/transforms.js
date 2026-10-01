import { ReferenceFrame, TimeScale } from '../utime.js';
import {
  REL_DAY_SECONDS,
  REL_JD_MJD0,
  REL_LB,
  REL_LG,
  REL_MJD_1977,
  REL_TDB0_SECONDS,
  REL_TT_MINUS_TAI_SECONDS,
} from './constants.js';
import { CoordinateTime, coordinateFromTwoPartJD } from './coordinate-time.js';

function requireCoordinate(value, scale) {
  if (!(value instanceof CoordinateTime) || value.scale !== scale) {
    throw new TypeError(`expected CoordinateTime(${scale})`);
  }
}

function make(source, scale, frame, d1, d2, extra = {}) {
  return coordinateFromTwoPartJD(scale, d1, d2, {
    frame,
    uncertaintySeconds: Math.hypot(
      source.uncertaintySeconds ?? 0,
      extra.uncertaintySeconds ?? 0,
    ),
    provenance: extra.provenance ?? source.provenance,
  });
}

function dtrInfo(dtr) {
  if (typeof dtr === 'number') {
    if (!Number.isFinite(dtr)) throw new TypeError('dtr must be finite');
    return { seconds: dtr, uncertaintySeconds: 0, provenance: 'caller-supplied TDB-TT' };
  }
  if (!dtr || !Number.isFinite(dtr.seconds)) {
    throw new TypeError('TT<->TDB requires dtr = TDB-TT in seconds');
  }
  const uncertaintySeconds = dtr.uncertaintySeconds ?? 0;
  if (!Number.isFinite(uncertaintySeconds) || uncertaintySeconds < 0) {
    throw new RangeError('dtr uncertaintySeconds must be finite and non-negative');
  }
  return {
    seconds: dtr.seconds,
    uncertaintySeconds,
    provenance: dtr.provenance ?? 'caller-supplied TDB-TT',
  };
}

// IAU 2000 Resolution B1.9 / SOFA canonical TT -> TCG relation.
export function ttToTcgCoordinate(tt) {
  requireCoordinate(tt, TimeScale.TT);
  const t77t = REL_MJD_1977 + REL_TT_MINUS_TAI_SECONDS / REL_DAY_SECONDS;
  const rate = REL_LG / (1 - REL_LG);
  let d1, d2;
  if (Math.abs(tt.d1) > Math.abs(tt.d2)) {
    d1 = tt.d1;
    d2 = tt.d2 + ((tt.d1 - REL_JD_MJD0) + (tt.d2 - t77t)) * rate;
  } else {
    d1 = tt.d1 + ((tt.d2 - REL_JD_MJD0) + (tt.d1 - t77t)) * rate;
    d2 = tt.d2;
  }
  return make(tt, TimeScale.TCG, ReferenceFrame.GCRS, d1, d2, {
    provenance: 'IAU 2000 B1.9 canonical TT->TCG',
  });
}

// IAU 2000 Resolution B1.9 / SOFA canonical TCG -> TT relation.
export function tcgToTtCoordinate(tcg) {
  requireCoordinate(tcg, TimeScale.TCG);
  const t77t = REL_MJD_1977 + REL_TT_MINUS_TAI_SECONDS / REL_DAY_SECONDS;
  let d1, d2;
  if (Math.abs(tcg.d1) > Math.abs(tcg.d2)) {
    d1 = tcg.d1;
    d2 = tcg.d2 - ((tcg.d1 - REL_JD_MJD0) + (tcg.d2 - t77t)) * REL_LG;
  } else {
    d1 = tcg.d1 - ((tcg.d2 - REL_JD_MJD0) + (tcg.d1 - t77t)) * REL_LG;
    d2 = tcg.d2;
  }
  return make(tcg, TimeScale.TT, ReferenceFrame.GCRS, d1, d2, {
    provenance: 'IAU 2000 B1.9 canonical TCG->TT',
  });
}

// IAU 2006 Resolution B3 / SOFA canonical TCB -> TDB linear relation.
export function tcbToTdbCoordinate(tcb) {
  requireCoordinate(tcb, TimeScale.TCB);
  const t77td = REL_JD_MJD0 + REL_MJD_1977;
  const t77tf = REL_TT_MINUS_TAI_SECONDS / REL_DAY_SECONDS;
  const tdb0 = REL_TDB0_SECONDS / REL_DAY_SECONDS;
  let d1, d2;

  if (Math.abs(tcb.d1) > Math.abs(tcb.d2)) {
    const d = tcb.d1 - t77td;
    d1 = tcb.d1;
    d2 = tcb.d2 + tdb0 - (d + (tcb.d2 - t77tf)) * REL_LB;
  } else {
    const d = tcb.d2 - t77td;
    d1 = tcb.d1 + tdb0 - (d + (tcb.d1 - t77tf)) * REL_LB;
    d2 = tcb.d2;
  }

  return make(tcb, TimeScale.TDB, ReferenceFrame.BCRS, d1, d2, {
    provenance: 'IAU 2006 B3 canonical TCB->TDB',
  });
}

// IAU 2006 Resolution B3 / SOFA canonical TDB -> TCB linear relation.
export function tdbToTcbCoordinate(tdb) {
  requireCoordinate(tdb, TimeScale.TDB);
  const t77td = REL_JD_MJD0 + REL_MJD_1977;
  const t77tf = REL_TT_MINUS_TAI_SECONDS / REL_DAY_SECONDS;
  const tdb0 = REL_TDB0_SECONDS / REL_DAY_SECONDS;
  const rate = REL_LB / (1 - REL_LB);
  let d1, d2;

  if (Math.abs(tdb.d1) > Math.abs(tdb.d2)) {
    const d = t77td - tdb.d1;
    const f = tdb.d2 - tdb0;
    d1 = tdb.d1;
    d2 = f - (d - (f - t77tf)) * rate;
  } else {
    const d = t77td - tdb.d2;
    const f = tdb.d1 - tdb0;
    d1 = f - (d - (f - t77tf)) * rate;
    d2 = tdb.d2;
  }

  return make(tdb, TimeScale.TCB, ReferenceFrame.BCRS, d1, d2, {
    provenance: 'IAU 2006 B3 canonical TDB->TCB',
  });
}

// Canonical TT -> TDB transformation once TDB-TT (dtr) is supplied.
// The dtr value is intentionally NOT invented here; SOFA documents it as
// ephemeris/model dependent and normally obtained from a time ephemeris or
// an implementation such as its Dtdb model.
export function ttToTdbCoordinate(tt, dtr) {
  requireCoordinate(tt, TimeScale.TT);
  const info = dtrInfo(dtr);
  const deltaDays = info.seconds / REL_DAY_SECONDS;
  let d1 = tt.d1, d2 = tt.d2;
  if (Math.abs(tt.d1) > Math.abs(tt.d2)) d2 += deltaDays;
  else d1 += deltaDays;

  return make(tt, TimeScale.TDB, ReferenceFrame.BCRS, d1, d2, {
    uncertaintySeconds: info.uncertaintySeconds,
    provenance: `canonical TT->TDB; dtr source: ${info.provenance}`,
  });
}

export function tdbToTtCoordinate(tdb, dtr) {
  requireCoordinate(tdb, TimeScale.TDB);
  const info = dtrInfo(dtr);
  const deltaDays = info.seconds / REL_DAY_SECONDS;
  let d1 = tdb.d1, d2 = tdb.d2;
  if (Math.abs(tdb.d1) > Math.abs(tdb.d2)) d2 -= deltaDays;
  else d1 -= deltaDays;

  return make(tdb, TimeScale.TT, ReferenceFrame.GCRS, d1, d2, {
    uncertaintySeconds: info.uncertaintySeconds,
    provenance: `canonical TDB->TT; dtr source: ${info.provenance}`,
  });
}

export function coordinateDeltaSeconds(a, b) {
  if (!(a instanceof CoordinateTime) || !(b instanceof CoordinateTime)) {
    throw new TypeError('coordinateDeltaSeconds requires CoordinateTime values');
  }
  return ((a.d1 - b.d1) + (a.d2 - b.d2)) * REL_DAY_SECONDS;
}
