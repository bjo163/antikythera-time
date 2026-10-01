const D2R=Math.PI/180;
const WGS84_A_KM=6378.137;
const WGS84_F=1/298.257223563;
const WGS84_E2=WGS84_F*(2-WGS84_F);

export function makeEarthObserver({longitudeDeg,latitudeDeg,heightMeters=0,name=null}){
  if(!Number.isFinite(longitudeDeg)||!Number.isFinite(latitudeDeg)||!Number.isFinite(heightMeters)) throw new TypeError('finite observer coordinates required');
  if(latitudeDeg<-90||latitudeDeg>90) throw new RangeError('latitudeDeg must be [-90,90]');
  return Object.freeze({type:'earth-fixed-observer',longitudeDeg,latitudeDeg,heightMeters,name,datum:'WGS84'});
}

// Returns the SOFA Dtdb observer geometry parameters:
// elong = east longitude [rad], u = distance from spin axis [km],
// v = north distance from equatorial plane [km].
export function observerDtdbGeometry(observer){
  if(!observer||observer.type!=='earth-fixed-observer') throw new TypeError('earth observer required');
  const lon=observer.longitudeDeg*D2R,lat=observer.latitudeDeg*D2R,h=observer.heightMeters/1000;
  const sin=Math.sin(lat),cos=Math.cos(lat);
  const N=WGS84_A_KM/Math.sqrt(1-WGS84_E2*sin*sin);
  const u=(N+h)*cos;
  const v=(N*(1-WGS84_E2)+h)*sin;
  return {elongRad:lon,uKm:u,vKm:v,datum:'WGS84',provenance:'WGS84 geodetic->geocentric geometry'};
}

export function makeSpacecraftObserver({id,frame='BCRS',positionKm,velocityKmS,epoch}){
  if(!id||!Array.isArray(positionKm)||positionKm.length!==3||!Array.isArray(velocityKmS)||velocityKmS.length!==3) throw new TypeError('spacecraft id, positionKm[3], velocityKmS[3] required');
  if(![...positionKm,...velocityKmS].every(Number.isFinite)) throw new TypeError('spacecraft state must be finite');
  return Object.freeze({type:'spacecraft-observer',id,frame,positionKm:[...positionKm],velocityKmS:[...velocityKmS],epoch});
}

export function weakFieldProperTimeRate({potentialM2S2,velocityMS}){
  const C=299792458;
  if(!Number.isFinite(potentialM2S2)||!Number.isFinite(velocityMS)) throw new TypeError('finite potential and velocity required');
  return 1 + potentialM2S2/(C*C) - velocityMS*velocityMS/(2*C*C);
}
