import { parseHorizonsObserverRows } from '../src/index.js';

export default async function handler(req, res) {
  try {
    const rawDate=typeof req.query?.date==='string'?req.query.date:new Date().toISOString();
    const instant=new Date(rawDate);
    if (Number.isNaN(instant.getTime())) return res.status(400).json({error:'invalid date'});
    const jd=instant.getTime()/86_400_000+2_440_587.5;
    const params=new URLSearchParams({
      format:'text',COMMAND:"'301'",OBJ_DATA:"'NO'",MAKE_EPHEM:"'YES'",
      EPHEM_TYPE:"'OBSERVER'",CENTER:"'500@399'",TLIST:"'"+jd.toFixed(9)+"'",
      TLIST_TYPE:"'JD'",TIME_TYPE:"'UT'",CAL_FORMAT:"'JD'",QUANTITIES:"'10,24'",CSV_FORMAT:"'YES'",
    });
    const response=await fetch('https://ssd.jpl.nasa.gov/api/horizons.api?'+params,{headers:{'User-Agent':'antikythera-time-v0.4'}});
    const text=await response.text();
    if(!response.ok) return res.status(502).json({error:'JPL Horizons request failed',status:response.status});
    const row=parseHorizonsObserverRows(text)[0]??null;
    res.setHeader('Cache-Control','s-maxage=300, stale-while-revalidate=600');
    return res.status(200).json({source:'NASA/JPL Horizons',target:'Moon (301)',center:'Earth geocenter',requestedUtc:instant.toISOString(),...row});
  } catch(error) {
    return res.status(500).json({error:error instanceof Error?error.message:'unknown error'});
  }
}
