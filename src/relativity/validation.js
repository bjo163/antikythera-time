import { TimeScale } from '../utime.js';
import { coordinateFromTwoPartJD } from './coordinate-time.js';
import {
  tcgToTtCoordinate,
  tcbToTdbCoordinate,
  tdbToTcbCoordinate,
  tdbToTtCoordinate,
  ttToTcgCoordinate,
  ttToTdbCoordinate,
} from './transforms.js';

// Reference vectors from the IAU SOFA validation program.
// Names/functions here deliberately do not use the reserved SOFA "iau" prefix.
export const RELATIVITY_REFERENCE_VECTORS = Object.freeze([
  {
    id: 'TT_TO_TCG',
    expectedD2: 0.8924900312508587113,
    toleranceDays: 1e-12,
    run() {
      return ttToTcgCoordinate(coordinateFromTwoPartJD(TimeScale.TT, 2453750.5, 0.892482639));
    },
  },
  {
    id: 'TCG_TO_TT',
    expectedD2: 0.8928551387488816828,
    toleranceDays: 1e-12,
    run() {
      return tcgToTtCoordinate(coordinateFromTwoPartJD(TimeScale.TCG, 2453750.5, 0.892862531));
    },
  },
  {
    id: 'TCB_TO_TDB',
    expectedD2: 0.8928551362746343397,
    toleranceDays: 1e-12,
    run() {
      return tcbToTdbCoordinate(coordinateFromTwoPartJD(TimeScale.TCB, 2453750.5, 0.893019599));
    },
  },
  {
    id: 'TDB_TO_TCB',
    expectedD2: 0.8930195997253656716,
    toleranceDays: 1e-12,
    run() {
      return tdbToTcbCoordinate(coordinateFromTwoPartJD(TimeScale.TDB, 2453750.5, 0.892855137));
    },
  },
  {
    id: 'TDB_TO_TT_WITH_DTR',
    expectedD2: 0.8928551393263888889,
    toleranceDays: 1e-12,
    run() {
      return tdbToTtCoordinate(
        coordinateFromTwoPartJD(TimeScale.TDB, 2453750.5, 0.892855137),
        -0.000201,
      );
    },
  },
  {
    id: 'TT_TO_TDB_WITH_DTR',
    expectedD2: 0.8928551366736111111,
    toleranceDays: 1e-12,
    run() {
      return ttToTdbCoordinate(
        coordinateFromTwoPartJD(TimeScale.TT, 2453750.5, 0.892855139),
        -0.000201,
      );
    },
  },
]);

export function runRelativityReferenceVectors() {
  const results = RELATIVITY_REFERENCE_VECTORS.map(vector => {
    const actual = vector.run();
    const errorDays = actual.d2 - vector.expectedD2;
    return {
      id: vector.id,
      actualD2: actual.d2,
      expectedD2: vector.expectedD2,
      errorDays,
      errorSeconds: errorDays * 86400,
      toleranceDays: vector.toleranceDays,
      pass: Math.abs(errorDays) <= vector.toleranceDays,
    };
  });

  return {
    source: 'IAU SOFA validation reference vectors',
    tests: results,
    passCount: results.filter(r => r.pass).length,
    total: results.length,
    allPass: results.every(r => r.pass),
  };
}
