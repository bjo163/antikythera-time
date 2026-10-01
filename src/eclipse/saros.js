export const NASA_SAROS_DAYS = 6585.3223;
export const NASA_EXELIGMOS_DAYS = NASA_SAROS_DAYS * 3;

export class EclipseRecurrence {
  constructor({jdTt,saros,type='UNKNOWN',source='unknown'}) {
    if(!Number.isFinite(jdTt)) throw new TypeError('jdTt must be finite');
    if(!Number.isInteger(saros)) throw new TypeError('saros must be integer');
    this.jdTt=jdTt;
    this.saros=saros;
    this.type=type;
    this.source=source;
    Object.freeze(this);
  }
}

export function predictSaros(seed,cycles=1) {
  if(!(seed instanceof EclipseRecurrence)) throw new TypeError('seed must be EclipseRecurrence');
  if(!Number.isInteger(cycles)) throw new TypeError('cycles must be integer');
  return new EclipseRecurrence({
    jdTt: seed.jdTt + cycles*NASA_SAROS_DAYS,
    saros: seed.saros,
    type: seed.type,
    source: `Saros recurrence from ${seed.source}`,
  });
}

export function predictExeligmos(seed,cycles=1) {
  if(!Number.isInteger(cycles)) throw new TypeError('cycles must be integer');
  return predictSaros(seed,3*cycles);
}

export function eclipseTimingResidualMinutes(predicted,reference) {
  if(!(predicted instanceof EclipseRecurrence)||!(reference instanceof EclipseRecurrence)) throw new TypeError('predicted/reference must be EclipseRecurrence');
  return (predicted.jdTt-reference.jdTt)*1440;
}

export const NASA_SOLAR_SAROS_139_REFERENCE=Object.freeze([
  new EclipseRecurrence({jdTt:2460409.26300,saros:139,type:'TOTAL',source:'NASA GSFC 2024-04-08'}),
  new EclipseRecurrence({jdTt:2466994.595481,saros:139,type:'TOTAL',source:'NASA GSFC 2042-04-20'}),
  new EclipseRecurrence({jdTt:2473579.92400,saros:139,type:'TOTAL',source:'NASA GSFC 2060-04-30'}),
]);

export function validateSaros139() {
  const seed=NASA_SOLAR_SAROS_139_REFERENCE[0];
  const comparisons=NASA_SOLAR_SAROS_139_REFERENCE.slice(1).map((reference,i)=>{
    const predicted=predictSaros(seed,i+1);
    const residualMinutes=eclipseTimingResidualMinutes(predicted,reference);
    return {cycles:i+1,predictedJdTt:predicted.jdTt,referenceJdTt:reference.jdTt,residualMinutes,absResidualMinutes:Math.abs(residualMinutes),pass:Math.abs(residualMinutes)<30};
  });
  return {
    model:'constant-Saros recurrence',
    periodDays:NASA_SAROS_DAYS,
    source:'NASA GSFC eclipse periodicity + Saros 139 reference events',
    comparisons,
    allPass:comparisons.every(x=>x.pass),
    boundary:'Recurrence-family predictor only; not a Besselian eclipse geometry solver.',
  };
}
