import { NS_PER_DAY } from './constants.js';
import { calibrateLunarModel, splitReferencesByCutoff } from './calibration.js';
import { lunarModelAtUtcWithParams } from './lunar-model.js';
import { summarizeValidation, validationScore } from './validation.js';
import { utcDateToTt } from './scales.js';

export const ANOMALISTIC_MONTH_DAYS = 27.554551;
export const DRACONIC_MONTH_DAYS = 27.212220;
export const SAROS_RELATION = Object.freeze({
  synodicMonths: 223,
  anomalisticMonths: 239,
  draconicMonths: 242,
});

function clamp(x, lo, hi) { return Math.max(lo, Math.min(hi, x)); }

function elapsedTtDays(date) {
  const tt = utcDateToTt(date);
  return Number(tt.nsSinceJ2000) / Number(NS_PER_DAY);
}

export function physicalResidualBasis(date) {
  const t = elapsedTtDays(date);
  const a = 2 * Math.PI * t / ANOMALISTIC_MONTH_DAYS;
  const d = 2 * Math.PI * t / DRACONIC_MONTH_DAYS;
  return [Math.sin(a), Math.cos(a), Math.sin(d), Math.cos(d)];
}

function solveLinearSystem(matrix, vector) {
  const n = vector.length;
  const a = matrix.map((row, i) => [...row, vector[i]]);
  for (let col = 0; col < n; col++) {
    let pivot = col;
    for (let r = col + 1; r < n; r++) if (Math.abs(a[r][col]) > Math.abs(a[pivot][col])) pivot = r;
    if (Math.abs(a[pivot][col]) < 1e-12) throw new Error('residual basis is singular');
    [a[col], a[pivot]] = [a[pivot], a[col]];
    const p = a[col][col];
    for (let c = col; c <= n; c++) a[col][c] /= p;
    for (let r = 0; r < n; r++) {
      if (r === col) continue;
      const f = a[r][col];
      for (let c = col; c <= n; c++) a[r][c] -= f * a[col][c];
    }
  }
  return a.map(row => row[n]);
}

function fitCoefficients(rows, key, ridge = 1e-6) {
  const n = 4;
  const xtx = Array.from({ length: n }, () => Array(n).fill(0));
  const xty = Array(n).fill(0);
  for (const row of rows) {
    const x = physicalResidualBasis(row.date);
    const y = row[key];
    for (let i = 0; i < n; i++) {
      xty[i] += x[i] * y;
      for (let j = 0; j < n; j++) xtx[i][j] += x[i] * x[j];
    }
  }
  for (let i = 0; i < n; i++) xtx[i][i] += ridge;
  return solveLinearSystem(xtx, xty);
}

function harmonicMeta(coeffs) {
  const pair = (sin, cos) => ({
    sin,
    cos,
    amplitude: Math.hypot(sin, cos),
    phaseRad: Math.atan2(cos, sin),
  });
  return {
    anomalistic: pair(coeffs[0], coeffs[1]),
    draconic: pair(coeffs[2], coeffs[3]),
  };
}

export function fitPhysicalResidualModel(references, lunarParams) {
  if (!Array.isArray(references) || references.length < 12) throw new TypeError('at least 12 references are required');
  const rows = references.map(ref => {
    const base = lunarModelAtUtcWithParams(ref.date, lunarParams);
    return {
      date: ref.date,
      illuminationResidual: ref.illuminatedPercent - base.illuminationPercent,
      phaseResidual: ref.phaseAngleDeg - base.phaseAngleDeg,
    };
  });
  const illuminationCoefficients = fitCoefficients(rows, 'illuminationResidual');
  const phaseCoefficients = fitCoefficients(rows, 'phaseResidual');
  return {
    cycles: {
      anomalisticDays: ANOMALISTIC_MONTH_DAYS,
      draconicDays: DRACONIC_MONTH_DAYS,
    },
    illuminationCoefficients,
    phaseCoefficients,
    illuminationHarmonics: harmonicMeta(illuminationCoefficients),
    phaseHarmonics: harmonicMeta(phaseCoefficients),
  };
}

function dot(a, b) { return a.reduce((sum, v, i) => sum + v * b[i], 0); }

export function applyPhysicalResidual(date, baseModel, residualModel) {
  const basis = physicalResidualBasis(date);
  const illuminationCorrection = dot(basis, residualModel.illuminationCoefficients);
  const phaseCorrection = dot(basis, residualModel.phaseCoefficients);
  return {
    ...baseModel,
    illuminationPercent: clamp(baseModel.illuminationPercent + illuminationCorrection, 0, 100),
    phaseAngleDeg: clamp(baseModel.phaseAngleDeg + phaseCorrection, 0, 180),
    residualCorrection: { illuminationPoints: illuminationCorrection, phaseAngleDeg: phaseCorrection },
    residualModel: 'fixed-anomalistic+draconic-harmonics',
  };
}

export function evaluateResidualModel(references, lunarParams, residualModel) {
  const samples = references.map(ref => {
    const base = lunarModelAtUtcWithParams(ref.date, lunarParams);
    const model = applyPhysicalResidual(ref.date, base, residualModel);
    const illuminationError = model.illuminationPercent - ref.illuminatedPercent;
    const phaseAngleError = model.phaseAngleDeg - ref.phaseAngleDeg;
    return {
      utc: ref.date.toISOString(),
      model,
      reference: { illuminatedPercent: ref.illuminatedPercent, phaseAngleDeg: ref.phaseAngleDeg },
      error: {
        illuminationPoints: illuminationError,
        absIlluminationPoints: Math.abs(illuminationError),
        phaseAngleDeg: phaseAngleError,
        absPhaseAngleDeg: Math.abs(phaseAngleError),
      },
    };
  });
  const summary = summarizeValidation(samples);
  return { summary, score: validationScore(summary), samples };
}

function relativeImprovementPct(before, after) {
  if (before <= 1e-15) return after <= before + 1e-15 ? 0 : -Infinity;
  return (before - after) / before * 100;
}

export function residualEngineWithHoldout(references, cutoffUtc = '2013-01-01T00:00:00Z') {
  const { training, holdout } = splitReferencesByCutoff(references, cutoffUtc);
  const calibration = calibrateLunarModel(training);
  const residualModel = fitPhysicalResidualModel(training, calibration.params);

  const trainBefore = calibration.calibrated;
  const trainAfter = evaluateResidualModel(training, calibration.params, residualModel);

  const holdoutBeforeSamples = holdout.map(ref => {
    const model = lunarModelAtUtcWithParams(ref.date, calibration.params);
    const illuminationError = model.illuminationPercent - ref.illuminatedPercent;
    const phaseAngleError = model.phaseAngleDeg - ref.phaseAngleDeg;
    return {
      utc: ref.date.toISOString(), model,
      reference: { illuminatedPercent: ref.illuminatedPercent, phaseAngleDeg: ref.phaseAngleDeg },
      error: {
        illuminationPoints: illuminationError,
        absIlluminationPoints: Math.abs(illuminationError),
        phaseAngleDeg: phaseAngleError,
        absPhaseAngleDeg: Math.abs(phaseAngleError),
      },
    };
  });
  const holdoutBeforeSummary = summarizeValidation(holdoutBeforeSamples);
  const holdoutBeforeScore = validationScore(holdoutBeforeSummary);
  const holdoutAfter = evaluateResidualModel(holdout, calibration.params, residualModel);

  const trainImprove = relativeImprovementPct(trainBefore.score, trainAfter.score);
  const holdoutImprove = relativeImprovementPct(holdoutBeforeScore, holdoutAfter.score);
  const accepted = trainImprove > 0 && holdoutImprove > 0;

  return {
    cutoffUtc,
    trainingSamples: training.length,
    holdoutSamples: holdout.length,
    lunarParams: calibration.params,
    residualModel,
    train: {
      before: trainBefore.summary,
      after: trainAfter.summary,
      scoreImprovementPct: trainImprove,
    },
    holdout: {
      before: holdoutBeforeSummary,
      after: holdoutAfter.summary,
      scoreImprovementPct: holdoutImprove,
      generalizes: accepted,
    },
    verdict: accepted ? 'RESIDUAL_MODEL_GENERALIZES' : 'REJECT_RESIDUAL_MODEL',
  };
}

export function sarosCycleDiagnostics(synodicDays = 29.530589) {
  const syn = SAROS_RELATION.synodicMonths * synodicDays;
  const ano = SAROS_RELATION.anomalisticMonths * ANOMALISTIC_MONTH_DAYS;
  const dra = SAROS_RELATION.draconicMonths * DRACONIC_MONTH_DAYS;
  return {
    days: { synodic: syn, anomalistic: ano, draconic: dra },
    deltaHours: {
      anomalisticMinusSynodic: (ano - syn) * 24,
      draconicMinusSynodic: (dra - syn) * 24,
    },
  };
}
