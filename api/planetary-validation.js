import { jplApproxHeliocentric, vectorErrorAu } from '../src/planetary/index.js';

function parseVector(text){
  const start=text.indexOf('$$SOE'),end=text.indexOf('$$EOE');
  if(start<0||end<=start) throw new Error('Horizons vector block missing');
  const block=text.slice(start+5,end);
  const nums=[...block.matchAll(/(?:X|Y|Z)\s*=\s*([+-]?[0-9.]+(?:E[+-]?\d+)?)/gi)].map(m=>Number(m[1]));
  if(nums.length<3) throw new Error('Horizons XYZ parse failed');
  return {x:nums[0],y:nums[1],z:nums[2]};
}

export default async function handler(req,res){
  try{
    const planet=String(req.query?.planet??'Mars');
    const jd=Number(req.query?.jd??2461315.5);
    const model=jplApproxHeliocentric(planet,jd);
    const params=new URLSearchParams({
      format:'text',COMMAND:`'${model.horizonsId}'`,OBJ_DATA:"'NO'",MAKE_EPHEM:"'YES'",
      EPHEM_TYPE:"'VECTORS'",CENTER:"'500@10'",TLIST:`'${jd}'`,TLIST_TYPE:"'JD'",
      TIME_TYPE:"'TDB'",REF_PLANE:"'ECLIPTIC'",OUT_UNITS:"'AU-D'",VEC_TABLE:"'1'",
    });
    const response=await fetch('https://ssd.jpl.nasa.gov/api/horizons.api?'+params,{headers:{'User-Agent':'antikythera-time-v0.10'}});
    const text=await response.text();if(!response.ok)throw new Error('Horizons HTTP '+response.status);
    const horizons=parseVector(text),errorAu=vectorErrorAu(model,horizons);
    return res.status(200).json({version:'0.10-alpha',planet,jdTdb:jd,model,horizons,errorAu,errorKm:errorAu*149597870.7,boundary:'JPL approximate elements are lower accuracy; Horizons remains the high-precision external reference.'});
  }catch(error){return res.status(400).json({error:error instanceof Error?error.message:'unknown error'});}
}
