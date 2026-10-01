import { lunarModelAtUtcWithParams, DEFAULT_LUNAR_MODEL_PARAMS } from './lunar-model.js';

function mean(values) { return values.reduce((a,b)=>a+b,0) / values.length; }
function rms(values) { return Math.sqrt(mean(values.map(v=>v*v))); }

export function compareLunarReference({ date, illuminatedPercent, phaseAngleDeg, params = DEFAULT_LUNAR_MODEL_PARAMS }) {
  const model = lunarModelAtUtcWithParams(date, params);
  const illuminationError = model.illuminationPercent - illuminatedPercent;
  const phaseAngleError = model.phaseAngleDeg - phaseAngleDeg;
  return {
    utc: date.toISOString(),
    model,
    reference: { illuminatedPercent, phaseAngleDeg },
    error: {
      illuminationPoints: illuminationError,
      absIlluminationPoints: Math.abs(illuminationError),
      phaseAngleDeg: phaseAngleError,
      absPhaseAngleDeg: Math.abs(phaseAngleError),
    },
  };
}

export function summarizeValidation(samples) {
  if (!Array.isArray(samples) || samples.length === 0) throw new TypeError('samples must be a non-empty array');
  const ie = samples.map(s=>s.error.illuminationPoints);
  const pe = samples.map(s=>s.error.phaseAngleDeg);
  return {
    sampleCount: samples.length,
    illumination: {
      meanErrorPoints: mean(ie),
      maePoints: mean(ie.map(Math.abs)),
      rmsePoints: rms(ie),
      maxAbsPoints: Math.max(...ie.map(Math.abs)),
    },
    phaseAngle: {
      meanErrorDeg: mean(pe),
      maeDeg: mean(pe.map(Math.abs)),
      rmseDeg: rms(pe),
      maxAbsDeg: Math.max(...pe.map(Math.abs)),
    },
  };
}

export function validationScore(summary) {
  if (!summary?.illumination || !summary?.phaseAngle) throw new TypeError('summary must be a validation summary');
  return summary.illumination.rmsePoints / 100 + summary.phaseAngle.rmseDeg / 180;
}

export function evaluateReferences(references, params = DEFAULT_LUNAR_MODEL_PARAMS) {
  const samples = references.map(r => compareLunarReference({ ...r, params }));
  const summary = summarizeValidation(samples);
  return { params, summary, score: validationScore(summary), samples };
}
