import fs from 'node:fs/promises';
const cases=JSON.parse(await fs.readFile(process.argv[2]??'/tmp/provider-grid.json','utf8'));
const base='https://ssd.jpl.nasa.gov/api/horizons.api';
async function query(params){
 const q=new URLSearchParams(params);let last;
 for(let i=1;i<=4;i++){try{const r=await fetch(base+'?'+q,{headers:{'User-Agent':'m-time-hilal-corpus/0.1'}});if(!r.ok)throw new Error('HTTP '+r.status);const t=await r.text();if(t.includes('ERROR'))throw new Error(t.slice(0,1000));return t}catch(e){last=e;if(i<4)await new Promise(r=>setTimeout(r,i*1500));}}
 throw last;
}
function fields(text){const a=text.indexOf('$$SOE'),b=text.indexOf('$$EOE');if(a<0||b<a)throw new Error('Horizons block missing');const line=text.slice(a+5,b).split(/\r?\n/).map(x=>x.trim()).find(Boolean);if(!line)throw new Error('row missing');return line.split(',').map(x=>x.trim())}
const threshold=0.1,results=[];
for(const p of cases){
 const common={format:'text',COMMAND:"'301'",OBJ_DATA:"'NO'",MAKE_EPHEM:"'YES'",EPHEM_TYPE:"'OBSERVER'",TLIST:`'${p.jd_utc}'`,TLIST_TYPE:"'JD'",TIME_TYPE:"'UT'",CAL_FORMAT:"'JD'",CSV_FORMAT:"'YES'",ANG_FORMAT:"'DEG'"};
 const top=fields(await query({...common,CENTER:"'coord@399'",COORD_TYPE:"'GEODETIC'",SITE_COORD:`'${p.lon},${p.lat},${p.height_m/1000}'`,APPARENT:"'AIRLESS'",QUANTITIES:"'4'"}));
 const geo=fields(await query({...common,CENTER:"'500@399'",QUANTITIES:"'23'"}));
 const jplAlt=Number(top[4]),jplElong=Number(geo[3]);
 const altErr=Math.abs(p.altitude_deg-jplAlt),elongErr=Math.abs(p.elongation_deg-jplElong);
 results.push({id:p.id,jd_utc:p.jd_utc,provider:{altitude_deg:p.altitude_deg,elongation_deg:p.elongation_deg},jpl:{altitude_deg:jplAlt,elongation_deg:jplElong},error_deg:{altitude:altErr,elongation:elongErr},pass:altErr<=threshold&&elongErr<=threshold});
 console.error(p.id,'alt err',altErr,'elong err',elongErr);
}
const maxAlt=Math.max(...results.map(x=>x.error_deg.altitude)),maxElong=Math.max(...results.map(x=>x.error_deg.elongation));
const report={source:'NASA/JPL Horizons',threshold_deg:threshold,sample_count:results.length,max_error_deg:{altitude:maxAlt,elongation:maxElong},results,allPass:results.every(x=>x.pass)};
await fs.mkdir('artifacts',{recursive:true});await fs.writeFile('artifacts/jpl-hilal-corpus.json',JSON.stringify(report,null,2));console.log(JSON.stringify(report,null,2));if(!report.allPass)process.exit(1);
