// IERS Earth Orientation Parameters (EOP) support.
// Source family: IERS Bulletin A / finals.all (IAU2000).
// Numerical fields are observations/predictions supplied by IERS, not U-Time models.

export function utcDateToMjd(date){
  if(!(date instanceof Date)||Number.isNaN(date.getTime())) throw new TypeError('valid Date required');
  return date.getTime()/86400000 + 40587;
}

export function parseIersFinals2000A(text){
  if(typeof text!=='string') throw new TypeError('text must be string');
  const rows=[];
  for(const raw of text.split(/\r?\n/)){
    const s=raw.trim();
    if(!s||s.startsWith('#')) continue;
    const p=s.split(/\s+/);
    // Typical finals.all IAU2000:
    // yy mm dd mjd flag xp sx yp sy flag ut1 sut1 lod slod ...
    if(p.length<13) continue;
    const year2=Number(p[0]),month=Number(p[1]),day=Number(p[2]),mjd=Number(p[3]);
    if(!Number.isFinite(mjd)||!Number.isInteger(month)||month<1||month>12) continue;
    let i=4;
    const poleFlag=/^[IP]$/.test(p[i])?p[i++]:null;
    const xp=Number(p[i++]),sigmaXp=Number(p[i++]),yp=Number(p[i++]),sigmaYp=Number(p[i++]);
    const ut1Flag=/^[IP]$/.test(p[i])?p[i++]:null;
    const ut1MinusUtc=Number(p[i++]),sigmaUt1=Number(p[i++]);
    const lod=Number(p[i++]),sigmaLod=Number(p[i++]);
    if(![xp,yp,ut1MinusUtc].every(Number.isFinite)) continue;
    const year=year2<70?2000+year2:1900+year2;
    rows.push({
      date:`${year.toString().padStart(4,'0')}-${String(month).padStart(2,'0')}-${String(day).padStart(2,'0')}`,
      mjd,year,month,day,poleFlag,ut1Flag,
      xpArcsec:xp,ypArcsec:yp,ut1MinusUtcSeconds:ut1MinusUtc,
      lodMilliseconds:Number.isFinite(lod)?lod:null,
      uncertainty:{xpArcsec:Number.isFinite(sigmaXp)?sigmaXp:null,ypArcsec:Number.isFinite(sigmaYp)?sigmaYp:null,ut1Seconds:Number.isFinite(sigmaUt1)?sigmaUt1:null,lodMilliseconds:Number.isFinite(sigmaLod)?sigmaLod:null},
      evidence:(poleFlag==='I'&&ut1Flag==='I')?'OBSERVED':'PREDICTED',
      provenance:'IERS finals.all IAU2000',
    });
  }
  return rows.sort((a,b)=>a.mjd-b.mjd);
}

function lerp(a,b,t){return a+(b-a)*t;}
export function interpolateEop(rows,mjd){
  if(!Array.isArray(rows)||rows.length<2) throw new TypeError('at least two EOP rows required');
  if(!Number.isFinite(mjd)) throw new TypeError('finite MJD required');
  let lo=0,hi=rows.length-1;
  if(mjd<rows[0].mjd||mjd>rows[hi].mjd) throw new RangeError('requested MJD outside EOP coverage');
  while(hi-lo>1){const mid=(lo+hi)>>1;if(rows[mid].mjd<=mjd)lo=mid;else hi=mid;}
  const a=rows[lo],b=rows[hi];
  if(mjd===a.mjd) return {...a,interpolated:false};
  const t=(mjd-a.mjd)/(b.mjd-a.mjd);
  return {
    date:null,mjd,
    xpArcsec:lerp(a.xpArcsec,b.xpArcsec,t),
    ypArcsec:lerp(a.ypArcsec,b.ypArcsec,t),
    ut1MinusUtcSeconds:lerp(a.ut1MinusUtcSeconds,b.ut1MinusUtcSeconds,t),
    lodMilliseconds:a.lodMilliseconds!=null&&b.lodMilliseconds!=null?lerp(a.lodMilliseconds,b.lodMilliseconds,t):null,
    uncertainty:{
      xpArcsec:Math.max(a.uncertainty.xpArcsec??0,b.uncertainty.xpArcsec??0),
      ypArcsec:Math.max(a.uncertainty.ypArcsec??0,b.uncertainty.ypArcsec??0),
      ut1Seconds:Math.max(a.uncertainty.ut1Seconds??0,b.uncertainty.ut1Seconds??0),
      lodMilliseconds:Math.max(a.uncertainty.lodMilliseconds??0,b.uncertainty.lodMilliseconds??0),
    },
    evidence:(a.evidence==='OBSERVED'&&b.evidence==='OBSERVED')?'INTERPOLATED_OBSERVED':'INTERPOLATED_PREDICTED',
    provenance:'linear interpolation of IERS finals.all IAU2000',
    interpolated:true,
  };
}

export function eopForUtcDate(rows,date){return interpolateEop(rows,utcDateToMjd(date));}

export function ut1DateFromUtc(date,eop){
  if(!(date instanceof Date)||Number.isNaN(date.getTime())) throw new TypeError('valid UTC Date required');
  if(!eop||!Number.isFinite(eop.ut1MinusUtcSeconds)) throw new TypeError('EOP UT1-UTC required');
  return new Date(date.getTime()+eop.ut1MinusUtcSeconds*1000);
}
