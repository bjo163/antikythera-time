// Implementation of JPL Solar System Dynamics "Approximate Positions of the Planets"
// Table 1, valid 1800 AD through 2050 AD.
// Reference: https://ssd.jpl.nasa.gov/planets/approx_pos.html
//
// JPL explicitly labels these lower-accuracy formulae. High precision work
// should use Horizons / integrated ephemerides instead.

const D2R=Math.PI/180;
const ELEMENTS=Object.freeze({
  Mercury:{id:199,a:[0.38709927,0.00000037],e:[0.20563593,0.00001906],I:[7.00497902,-0.00594749],L:[252.25032350,149472.67411175],peri:[77.45779628,0.16047689],node:[48.33076593,-0.12534081]},
  Venus:{id:299,a:[0.72333566,0.00000390],e:[0.00677672,-0.00004107],I:[3.39467605,-0.00078890],L:[181.97909950,58517.81538729],peri:[131.60246718,0.00268329],node:[76.67984255,-0.27769418]},
  Earth:{id:399,a:[1.00000261,0.00000562],e:[0.01671123,-0.00004392],I:[-0.00001531,-0.01294668],L:[100.46457166,35999.37244981],peri:[102.93768193,0.32327364],node:[0,0]},
  Mars:{id:499,a:[1.52371034,0.00001847],e:[0.09339410,0.00007882],I:[1.84969142,-0.00813131],L:[-4.55343205,19140.30268499],peri:[-23.94362959,0.44441088],node:[49.55953891,-0.29257343]},
  Jupiter:{id:599,a:[5.20288700,-0.00011607],e:[0.04838624,-0.00013253],I:[1.30439695,-0.00183714],L:[34.39644051,3034.74612775],peri:[14.72847983,0.21252668],node:[100.47390909,0.20469106]},
  Saturn:{id:699,a:[9.53667594,-0.00125060],e:[0.05386179,-0.00050991],I:[2.48599187,0.00193609],L:[49.95424423,1222.49362201],peri:[92.59887831,-0.41897216],node:[113.66242448,-0.28867794]},
});

export const JPL_APPROX_PLANETS=Object.freeze(Object.keys(ELEMENTS));
export const JPL_APPROX_VALID_JD=Object.freeze([2378496.5,2469807.5]); // 1800-01-01 .. 2050-01-01 approximately.

function element(pair,T){return pair[0]+pair[1]*T;}
function wrap180(x){x=((x+180)%360+360)%360-180;return x;}
function solveKeplerDeg(M,e){
  const eStar=180/Math.PI*e;
  let E=M+eStar*Math.sin(M*D2R);
  for(let i=0;i<20;i++){
    const dM=M-(E-eStar*Math.sin(E*D2R));
    const dE=dM/(1-e*Math.cos(E*D2R));
    E+=dE;
    if(Math.abs(dE)<=1e-10) break;
  }
  return E;
}

export function jplApproxHeliocentric(name,jdTdb){
  const p=ELEMENTS[name];
  if(!p) throw new RangeError('supported planets: '+JPL_APPROX_PLANETS.join(', '));
  if(!Number.isFinite(jdTdb)) throw new TypeError('jdTdb must be finite');
  if(jdTdb<JPL_APPROX_VALID_JD[0]||jdTdb>JPL_APPROX_VALID_JD[1]) throw new RangeError('JPL approximate Table 1 model is restricted to 1800-2050');

  const T=(jdTdb-2451545.0)/36525;
  const a=element(p.a,T),e=element(p.e,T),I=element(p.I,T),L=element(p.L,T),peri=element(p.peri,T),node=element(p.node,T);
  const omega=peri-node;
  const M=wrap180(L-peri);
  const E=solveKeplerDeg(M,e);
  const xPrime=a*(Math.cos(E*D2R)-e);
  const yPrime=a*Math.sqrt(1-e*e)*Math.sin(E*D2R);
  const w=omega*D2R,O=node*D2R,inc=I*D2R;
  const cw=Math.cos(w),sw=Math.sin(w),cO=Math.cos(O),sO=Math.sin(O),cI=Math.cos(inc),sI=Math.sin(inc);
  const x=(cw*cO-sw*sO*cI)*xPrime+(-sw*cO-cw*sO*cI)*yPrime;
  const y=(cw*sO+sw*cO*cI)*xPrime+(-sw*sO+cw*cO*cI)*yPrime;
  const z=(sw*sI)*xPrime+(cw*sI)*yPrime;
  const radius=Math.hypot(x,y,z);
  const longitude=((Math.atan2(y,x)/D2R)%360+360)%360;
  const latitude=Math.asin(z/radius)/D2R;
  return {planet:name,horizonsId:p.id,jdTdb,x,y,z,radiusAu:radius,longitudeDeg:longitude,latitudeDeg:latitude,frame:'J2000 mean ecliptic/equinox',status:'JPL_APPROXIMATE_1800_2050'};
}

export function jplApproxGeocentric(name,jdTdb){
  if(name==='Earth') return {planet:'Earth',jdTdb,x:0,y:0,z:0,radiusAu:0,status:'observer-origin'};
  const p=jplApproxHeliocentric(name,jdTdb),earth=jplApproxHeliocentric('Earth',jdTdb);
  const x=p.x-earth.x,y=p.y-earth.y,z=p.z-earth.z,r=Math.hypot(x,y,z);
  return {...p,x,y,z,radiusAu:r,longitudeDeg:((Math.atan2(y,x)/D2R)%360+360)%360,latitudeDeg:Math.asin(z/r)/D2R,origin:'Earth-Moon barycenter approximation'};
}

export function vectorErrorAu(a,b){return Math.hypot(a.x-b.x,a.y-b.y,a.z-b.z);}
