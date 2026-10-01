import { runRelativityReferenceVectors } from '../src/relativity/index.js';

export default async function handler(req, res) {
  const result = runRelativityReferenceVectors();
  return res.status(result.allPass ? 200 : 500).json({
    version: '0.8-alpha',
    layer: 'Relativistic Time Core',
    ...result,
    boundary: {
      automaticTdbMinusTtModel: false,
      reason: 'TT<->TDB requires an explicit ephemeris/model-dependent dtr provider; no fake periodic approximation is injected.',
    },
  });
}
