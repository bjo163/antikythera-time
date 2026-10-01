// Evaluates published Besselian polynomial elements.
// This does not generate Besselian elements from raw Sun/Moon ephemerides.
// NASA/GSFC remains the source/reference for the element sets.

function poly(coeff,t){let s=0,p=1;for(const c of coeff){s+=c*p;p*=t;}return s;}

export class BesselianElementSet {
  constructor({id,t0TdtHours,jdGreatestTt,deltaTSeconds,gamma,magnitude,type,saros,coefficients,tanF1,tanF2,source}){
    this.id=id;this.t0TdtHours=t0TdtHours;this.jdGreatestTt=jdGreatestTt;this.deltaTSeconds=deltaTSeconds;
    this.gamma=gamma;this.magnitude=magnitude;this.type=type;this.saros=saros;this.coefficients=coefficients;
    this.tanF1=tanF1;this.tanF2=tanF2;this.source=source;Object.freeze(this);
  }
}

export const NASA_2026_02_17=Object.freeze(new BesselianElementSet({
  id:'NASA-2026-02-17-A',t0TdtHours:12,jdGreatestTt:2461089.00900,deltaTSeconds:75.1,gamma:-0.9743,magnitude:0.9630,type:'ANNULAR',saros:121,
  coefficients:{
    x:[0.3219540,0.4827224,-0.0000314,-0.0000064],
    y:[-0.9269710,0.2355394,0.0001169,-0.0000033],
    d:[-11.8793001,0.0140490,0.0000020],
    l1:[0.5577200,-0.0001181,-0.0000111],
    l2:[0.0115240,-0.0001175,-0.0000111],
    mu:[356.514404,15.001980,0],
  },
  tanF1:0.0047321,tanF2:0.0047085,
  source:'NASA GSFC Besselian Elements, Annular Solar Eclipse 2026-02-17',
}));

export function evaluateBesselian(set,tHours){
  if(!(set instanceof BesselianElementSet))throw new TypeError('BesselianElementSet required');
  if(!Number.isFinite(tHours))throw new TypeError('finite tHours required');
  const dt=tHours-set.t0TdtHours,c=set.coefficients;
  const x=poly(c.x,dt),y=poly(c.y,dt),d=poly(c.d,dt),l1=poly(c.l1,dt),l2=poly(c.l2,dt),mu=((poly(c.mu,dt)%360)+360)%360;
  return {id:set.id,tHours,dtHours:dt,x,y,dDeg:d,l1,l2,muDeg:mu,axisDistanceEarthRadii:Math.hypot(x,y),source:set.source};
}

export function evaluateAtGreatestApprox(set){
  // Greatest instant converted to TDT hour on the date by fractional JD.
  const frac=((set.jdGreatestTt+0.5)%1+1)%1;
  const hour=frac*24;
  return evaluateBesselian(set,hour);
}

export function besselianReferenceSelfTest(){
  const e=evaluateBesselian(NASA_2026_02_17,12);
  const pass=Math.abs(e.x-0.3219540)<1e-12&&Math.abs(e.y+0.9269710)<1e-12&&Math.abs(e.muDeg-356.514404)<1e-9;
  return {source:NASA_2026_02_17.source,pass,evaluatedAtT0:e,boundary:'Published-element evaluator, not independent element generation.'};
}
