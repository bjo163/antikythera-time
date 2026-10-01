import {
  BASELINE_SYNODIC_PERIOD_SECONDS,
  DEFAULT_LUNAR_MODEL_PARAMS,
  lunarModelAtTtWithParams,
} from './lunar-model.js';
import { utcDateToTt } from './scales.js';
import { evaluateReferences } from './validation.js';

export const CALIBRATION_LIMITS = Object.freeze({
  periodDeltaSeconds: 180,
  epochOffsetSeconds: 12 * 3600,
});

function relativeImprovementPct(baselineScore, candidateScore) {
  if (baselineScore <= 1e-15) return candidateScore <= baselineScore + 1e-15 ? 0 : -Infinity;
  return (baselineScore - candidateScore) / baselineScore * 100;
}

function candidateParams(periodDeltaSeconds, epochOffsetSeconds) {
  return {
    periodSeconds: BASELINE_SYNODIC_PERIOD_SECONDS + periodDeltaSeconds,
    epochOffsetSeconds,
  };
}

function prepareReferences(references) {
  return references.map(r => ({ ...r, tt: utcDateToTt(r.date) }));
}

function fastScore(prepared, params) {
  let illumSq = 0;
  let phaseSq = 0;
  for (const ref of prepared) {
    const model = lunarModelAtTtWithParams(ref.tt, params);
    const ie = model.illuminationPercent - ref.illuminatedPercent;
    const pe = model.phaseAngleDeg - ref.phaseAngleDeg;
    illumSq += ie * ie;
    phaseSq += pe * pe;
  }
  const n = prepared.length;
  return Math.sqrt(illumSq / n) / 100 + Math.sqrt(phaseSq / n) / 180;
}

function gridSearch(prepared, center, radius, step) {
  let best = null;
  for (let periodDelta = center.periodDeltaSeconds - radius.periodSeconds;
       periodDelta <= center.periodDeltaSeconds + radius.periodSeconds + 1e-12;
       periodDelta += step.periodSeconds) {
    if (Math.abs(periodDelta) > CALIBRATION_LIMITS.periodDeltaSeconds) continue;
    for (let epochOffset = center.epochOffsetSeconds - radius.epochSeconds;
         epochOffset <= center.epochOffsetSeconds + radius.epochSeconds + 1e-9;
         epochOffset += step.epochSeconds) {
      if (Math.abs(epochOffset) > CALIBRATION_LIMITS.epochOffsetSeconds) continue;
      const params = candidateParams(periodDelta, epochOffset);
      const score = fastScore(prepared, params);
      if (!best || score < best.score) best = { periodDeltaSeconds: periodDelta, epochOffsetSeconds: epochOffset, params, score };
    }
  }
  return best;
}

export function calibrateLunarModel(trainingReferences) {
  if (!Array.isArray(trainingReferences) || trainingReferences.length < 12) {
    throw new TypeError('trainingReferences must contain at least 12 samples');
  }
  const prepared = prepareReferences(trainingReferences);
  const baseline = evaluateReferences(trainingReferences, DEFAULT_LUNAR_MODEL_PARAMS);

  let best = gridSearch(
    prepared,
    { periodDeltaSeconds: 0, epochOffsetSeconds: 0 },
    { periodSeconds: 180, epochSeconds: 12 * 3600 },
    { periodSeconds: 15, epochSeconds: 1800 },
  );
  best = gridSearch(
    prepared,
    { periodDeltaSeconds: best.periodDeltaSeconds, epochOffsetSeconds: best.epochOffsetSeconds },
    { periodSeconds: 18, epochSeconds: 2400 },
    { periodSeconds: 2, epochSeconds: 240 },
  );
  best = gridSearch(
    prepared,
    { periodDeltaSeconds: best.periodDeltaSeconds, epochOffsetSeconds: best.epochOffsetSeconds },
    { periodSeconds: 2.5, epochSeconds: 300 },
    { periodSeconds: 0.25, epochSeconds: 30 },
  );

  const calibrated = evaluateReferences(trainingReferences, best.params);

  return {
    baseline,
    calibrated,
    params: best.params,
    delta: {
      periodSeconds: best.periodDeltaSeconds,
      epochOffsetSeconds: best.epochOffsetSeconds,
    },
    trainScoreImprovementPct: relativeImprovementPct(baseline.score, calibrated.score),
    bounds: CALIBRATION_LIMITS,
  };
}

export function splitReferencesByCutoff(references, cutoffUtc = '2013-01-01T00:00:00Z') {
  const cutoff = Date.parse(cutoffUtc);
  const training = [], holdout = [];
  for (const ref of references) (ref.date.getTime() < cutoff ? training : holdout).push(ref);
  if (!training.length || !holdout.length) throw new Error('cutoff must produce non-empty training and holdout sets');
  return { cutoffUtc, training, holdout };
}

export function calibrateWithHoldout(references, cutoffUtc = '2013-01-01T00:00:00Z') {
  const { training, holdout } = splitReferencesByCutoff(references, cutoffUtc);
  const calibration = calibrateLunarModel(training);
  const baselineHoldout = evaluateReferences(holdout, DEFAULT_LUNAR_MODEL_PARAMS);
  const calibratedHoldout = evaluateReferences(holdout, calibration.params);
  const holdoutImprovementPct = relativeImprovementPct(baselineHoldout.score, calibratedHoldout.score);

  return {
    cutoffUtc,
    trainingSamples: training.length,
    holdoutSamples: holdout.length,
    params: calibration.params,
    delta: calibration.delta,
    train: {
      baseline: calibration.baseline.summary,
      calibrated: calibration.calibrated.summary,
      scoreImprovementPct: calibration.trainScoreImprovementPct,
    },
    holdout: {
      baseline: baselineHoldout.summary,
      calibrated: calibratedHoldout.summary,
      scoreImprovementPct: holdoutImprovementPct,
      generalizes: holdoutImprovementPct > 0,
    },
    verdict: holdoutImprovementPct > 0 ? 'GENERALIZES_ON_HOLDOUT' : 'REJECT_CALIBRATION',
  };
}
