import { TimeScale } from '../utime.js';
import { CoordinateTime } from './coordinate-time.js';

// NASA TP-2022-0014814 gives the compact engineering approximation
// TDB ≈ TT + 0.001658 sin(M_E) + 0.000014 sin(2 M_E) seconds,
// M_E = 357.53 + 0.9856003 (JD_TT - 2451545.0) degrees.
// More complete SOFA/ERFA Dtdb models remain the higher-accuracy reference.
//
// This provider is intentionally labelled APPROXIMATE_GEOCENTRIC rather than
// standards-grade. The uncertainty below is a conservative engineering
// allowance and is benchmarked in CI against ERFA over a declared interval.

export const NASA_SIMPLE_DTR_MODEL = Object.freeze({
  id: 'NASA_TP_2022_SIMPLE_TDB_TT',
  status: 'APPROXIMATE_GEOCENTRIC',
  reference: 'NASA/TP-2022-0014814',
  declaredModelUncertaintySeconds: 0.0001,
  validBenchmarkWindow: ['1900-01-01','2100-01-01'],
});

function ttJulianDate(tt) {
  if (!(tt instanceof CoordinateTime) || tt.scale !== TimeScale.TT) {
    throw new TypeError('automatic dtr provider requires CoordinateTime(TT)');
  }
  return tt.d1 + tt.d2;
}

export function nasaSimpleDtr(tt) {
  const jd = ttJulianDate(tt);
  const meanAnomalyDeg = 357.53 + 0.9856003 * (jd - 2451545.0);
  const g = meanAnomalyDeg * Math.PI / 180;
  const seconds = 0.001658 * Math.sin(g) + 0.000014 * Math.sin(2*g);
  return {
    seconds,
    uncertaintySeconds: NASA_SIMPLE_DTR_MODEL.declaredModelUncertaintySeconds,
    provenance: NASA_SIMPLE_DTR_MODEL.id,
    status: NASA_SIMPLE_DTR_MODEL.status,
    model: NASA_SIMPLE_DTR_MODEL,
  };
}
