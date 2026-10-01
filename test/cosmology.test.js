import test from 'node:test';
import assert from 'node:assert/strict';
import {
  CosmicAgeEstimate,
  MPC_METERS,
  adaptiveSimpson,
  cplDarkEnergyRelativeDensity,
  expansionE2,
  evaluatePosteriorChain,
  getCosmologyPreset,
  hubbleKmSmpcToSI,
  inferCosmicAge,
  integrationStability,
  listCosmologyPresets,
  materializePresetParameters,
  normalizeCosmologyParameters,
  omegaRadiationFromH0,
  parseCosmologyAgeQuery,
  parameterSensitivity,
  phaseAgeDegeneracy,
  propagateCovariance,
  propagateIndependentPreset,
  rejectAntikytheraBigBangSemanticMisuse,
  summarizeSamples,
  validateEinsteinDeSitter,
  validateFlatMatterLambda,
  validateH0Dimensions,
  validatePlanckSanity,
} from '../src/cosmology/index.js';

test('Mpc conversion is explicit SI scale',()=>{
  assert.ok(MPC_METERS>3.0856e22&&MPC_METERS<3.0858e22);
  assert.ok(Math.abs(hubbleKmSmpcToSI(70)-2.2685e-18)<2e-22);
});

test('adaptive Simpson integrates polynomial with exposed error',()=>{
  const r=adaptiveSimpson(x=>x**4,0,1,{absoluteTolerance:1e-12,relativeTolerance:1e-12});
  assert.ok(Math.abs(r.value-0.2)<1e-12);
  assert.ok(r.errorEstimate<1e-10);
  assert.ok(r.evaluations>3);
});

test('radiation density is finite and small today',()=>{
  const r=omegaRadiationFromH0(67.36);
  assert.ok(r>8e-5&&r<1.1e-4);
});

test('flat model warns instead of silently renormalizing inconsistent input',()=>{
  const n=normalizeCosmologyParameters('flat-lcdm',{H0:70,omegaR:0,omegaM:0.3,omegaLambda:0.8},{flatnessTolerance:1e-6});
  assert.equal(n.parameters.omegaLambda,0.8);
  assert.ok(n.warnings.length===1);
  assert.throws(()=>normalizeCosmologyParameters('flat-lcdm',{H0:70,omegaR:0,omegaM:0.3,omegaLambda:0.8},{flatnessTolerance:1e-6,consistencyMode:'reject'}));
});

test('curved LCDM supports explicit curvature',()=>{
  const n=normalizeCosmologyParameters('lcdm',{H0:68,omegaR:0,omegaM:0.3,omegaLambda:0.69,omegaK:0.01});
  assert.equal(n.parameters.omegaK,0.01);
  assert.ok(expansionE2(1,n)>0);
});

test('curved LCDM only derives curvature when explicitly requested',()=>{
  assert.throws(()=>normalizeCosmologyParameters('lcdm',{H0:68,omegaR:0,omegaM:0.3,omegaLambda:0.69}));
  const n=normalizeCosmologyParameters('lcdm',{H0:68,omegaR:0,omegaM:0.3,omegaLambda:0.69},{deriveOmegaK:true});
  assert.ok(Math.abs(n.parameters.omegaK-0.01)<1e-15);
});

test('CPL density evolution is normalized at a=1 and equation is stable',()=>{
  assert.equal(cplDarkEnergyRelativeDensity(1,-0.8,-0.6),1);
  assert.ok(cplDarkEnergyRelativeDensity(0.5,-0.8,-0.6)>0);
});

test('w0wa with w0=-1 wa=0 matches LambdaCDM background',()=>{
  const lcdm=normalizeCosmologyParameters('flat-lcdm',{H0:70,omegaR:0,omegaM:0.3,omegaLambda:0.7});
  const cpl=normalizeCosmologyParameters('w0wa-cdm',{H0:70,omegaR:0,omegaM:0.3,omegaDE:0.7,w0:-1,wa:0});
  for(const a of [0.01,0.1,0.5,1]) assert.ok(Math.abs(expansionE2(a,lcdm)-expansionE2(a,cpl))<1e-10);
});

test('negative H0 / densities and NaN are rejected',()=>{
  assert.throws(()=>inferCosmicAge('flat-lcdm',{H0:-1,omegaM:0.3,omegaLambda:0.7}));
  assert.throws(()=>inferCosmicAge('flat-lcdm',{H0:70,omegaM:-0.3,omegaLambda:1.3}));
  assert.throws(()=>inferCosmicAge('flat-lcdm',{H0:NaN,omegaM:0.3,omegaLambda:0.7}));
});

test('Einstein-de Sitter numerical age matches 2/(3H0)',()=>{
  assert.equal(validateEinsteinDeSitter({H0:70,tolerance:1e-9}).pass,true);
});

test('flat matter+Lambda numerical age matches analytic solution',()=>{
  assert.equal(validateFlatMatterLambda({H0:70,omegaM:0.3,tolerance:1e-9}).pass,true);
});

test('Planck preset produces age close to published Planck model-dependent age',()=>{
  const v=validatePlanckSanity({toleranceGyr:0.05});
  assert.equal(v.pass,true);
  assert.ok(v.computedGyr>13.7&&v.computedGyr<13.9);
});

test('integration converges across tolerances',()=>{
  const p=materializePresetParameters(getCosmologyPreset('planck2018'));
  assert.equal(integrationStability('flat-lcdm',p).pass,true);
});

test('CosmicAgeEstimate is semantically separate from UTime',()=>{
  const e=inferCosmicAge('flat-lcdm',{H0:70,omegaR:0,omegaM:0.3,omegaLambda:0.7});
  assert.ok(e instanceof CosmicAgeEstimate);
  assert.equal(e.status,'MODEL_DEPENDENT_INFERENCE');
  assert.equal(e.scientificBoundary.isUTimeTimestamp,false);
  assert.equal(e.scientificBoundary.isAbsoluteCosmicClock,false);
  assert.equal('nsSinceJ2000' in e,false);
});

test('CosmicAgeEstimate JSON preserves scientific boundary',()=>{
  const e=inferCosmicAge('flat-lcdm',{H0:70,omegaR:0,omegaM:0.3,omegaLambda:0.7});
  const j=JSON.parse(JSON.stringify(e));
  assert.equal(j.quantity,'age_of_universe');
  assert.equal(j.scientificBoundary.antikytheraDirectlyDeterminesCosmicAge,false);
});

test('phase recurrence cannot identify absolute cycle count',()=>{
  const d=phaseAgeDegeneracy(10,0.25,1,100);
  assert.equal(d.samePhaseModuloPeriod,true);
  assert.equal(d.absoluteAgeDetermined,false);
  assert.notEqual(d.tA,d.tB);
});

test('semantic misuse claiming Antikythera determines Big Bang date is rejected',()=>{
  assert.throws(()=>rejectAntikytheraBigBangSemanticMisuse('Antikythera determines Big Bang date'),/SEMANTIC_BOUNDARY/);
  assert.equal(rejectAntikytheraBigBangSemanticMisuse('Antikythera tracks lunar cycles'),true);
});

test('query parser validates explicit API parameters and rejects preset mixing',()=>{
  const q=parseCosmologyAgeQuery({model:'flat-lcdm',H0:'67.4',OmegaM:'0.315',OmegaLambda:'0.685'});
  assert.equal(q.parameters.H0,67.4);
  assert.equal(q.parameters.omegaM,0.315);
  assert.throws(()=>parseCosmologyAgeQuery({preset:'planck2018',H0:'70'}));
  assert.throws(()=>parseCosmologyAgeQuery({model:'flat-lcdm',H0:'70'}));
});

test('reference presets remain dataset/model specific',()=>{
  const ids=listCosmologyPresets().map(x=>x.id);
  assert.ok(ids.includes('planck2018'));
  assert.ok(ids.includes('desi-dr2-lcdm-cmb'));
  assert.ok(ids.includes('desi-dr2-w0wa-cmb-desy5'));
});

test('sample summary returns requested credible intervals',()=>{
  const s=summarizeSamples([1,2,3,4,5]);
  assert.equal(s.median,3);
  assert.ok(s.p16<s.p84);
  assert.ok(s.p2_5<s.p97_5);
});

test('independent Monte Carlo is deterministic with seeded PRNG',()=>{
  const a=propagateIndependentPreset('planck2018',{samples:80,seed:42,integration:{absoluteTolerance:1e-8,relativeTolerance:1e-8}});
  const b=propagateIndependentPreset('planck2018',{samples:80,seed:42,integration:{absoluteTolerance:1e-8,relativeTolerance:1e-8}});
  assert.deepEqual(a.ageGyr,b.ageGyr);
  assert.ok(a.ageGyr.standardDeviation>0);
});

test('covariance-aware sampler runs deterministically',()=>{
  const complete=p=>({H0:p.H0,omegaR:0,omegaM:p.omegaM,omegaLambda:1-p.omegaM});
  const a=propagateCovariance({model:'flat-lcdm',means:{H0:70,omegaM:0.3},parameterNames:['H0','omegaM'],covariance:[[0.25,0],[0,0.000025]],samples:60,seed:7,completeParameters:complete,integration:{absoluteTolerance:1e-8,relativeTolerance:1e-8}});
  const b=propagateCovariance({model:'flat-lcdm',means:{H0:70,omegaM:0.3},parameterNames:['H0','omegaM'],covariance:[[0.25,0],[0,0.000025]],samples:60,seed:7,completeParameters:complete,integration:{absoluteTolerance:1e-8,relativeTolerance:1e-8}});
  assert.deepEqual(a.ageGyr,b.ageGyr);
});

test('H0 sensitivity has correct direction: higher H0 lowers age at fixed dimensionless expansion',()=>{
  const a=inferCosmicAge('flat-lcdm',{H0:67,omegaR:0,omegaM:0.3,omegaLambda:0.7});
  const b=inferCosmicAge('flat-lcdm',{H0:70,omegaR:0,omegaM:0.3,omegaLambda:0.7});
  assert.ok(b.result.gyr<a.result.gyr);
});

test('dimension diagnostic returns plausible Hubble time',()=>{
  const d=validateH0Dimensions(70);
  assert.equal(d.pass,true);
  assert.ok(d.hubbleTimeSeconds>4e17&&d.hubbleTimeSeconds<5e17);
});


test('derived negative dark-energy density is rejected',()=>{
  assert.throws(()=>normalizeCosmologyParameters('flat-lcdm',{H0:70,omegaM:1.2}),/omegaLambda/);
});

test('parameter sensitivity diagnostic changes all requested cosmology dimensions',()=>{
  const s=parameterSensitivity();
  assert.ok(s.H0.high<s.H0.low);
  assert.ok(s.omegaM.high<s.omegaM.low);
  assert.notEqual(s.omegaLambda.low,s.omegaLambda.high);
  assert.notEqual(s.w0.minusOne,s.w0.lessNegative);
  assert.notEqual(s.wa.zero,s.wa.negative);
});

test('posterior-chain age evaluation supports weights and stays model-dependent',()=>{
  const chain=[
    {H0:67.2,omegaR:0,omegaM:0.31,omegaLambda:0.69,w:1},
    {H0:67.4,omegaR:0,omegaM:0.315,omegaLambda:0.685,w:3},
    {H0:67.6,omegaR:0,omegaM:0.32,omegaLambda:0.68,w:1},
  ];
  const r=evaluatePosteriorChain(chain,{model:'flat-lcdm',weightKey:'w'});
  assert.equal(r.weighted,true);
  assert.equal(r.ageGyr.count,3);
  assert.ok(r.ageGyr.p16<r.ageGyr.p84);
});
