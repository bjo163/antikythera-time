import fs from 'node:fs/promises';
const provider=JSON.parse(await fs.readFile(process.argv[2]??'/tmp/provider.json','utf8'));
const base='https://ssd.jpl.nasa.gov/api/horizons.api';
async function query(params){
 const q=new URLSearchParams(params);let last;
 for(let i=1;i<=4;i++){try{const r=await fetch(base+'?'+q,{headers:{'User-Agent':'m-time-hilal-reference/0.1'}});if(!r.ok)throw new Error('HTTP '+r.status);const t=await r.text();if(t.includes('ERROR'))throw new Error(t.slice(0,1000));return t}catch(e){last=e;if(i<4)await new Promise(r=>setTimeout(r,i*2000));}}
 throw last;
}
function row(text){
 const a=text.indexOf('$$SOE'),b=text.indexOf('$$EOE');if(a<0||b<a)throw new Error('Horizons data block missing');
 const line=text.slice(a+5,b).split(/\r?\n/).map(x=>x.trim()).find(Boolean);
 if(!line)throw new Error('Horizons row missing');
 return line.split(',').map(x=>x.trim());
}
function nums(fields){return fields.map(Number).filter(Number.isFinite)}
const common={format:'text',COMMAND:"'301'",OBJ_DATA:"'NO'",MAKE_EPHEM:"'YES'",EPHEM_TYPE:"'OBSERVER'",TLIST:"'2461118.95'",TLIST_TYPE:"'JD'",TIME_TYPE:"'UT'",CAL_FORMAT:"'JD'",CSV_FORMAT:"'YES'",ANG_FORMAT:"'DEG'"};
const top=await query({...common,CENTER:"'coord@399'",COORD_TYPE:"'GEODETIC'",SITE_COORD:"'106.8,-6.2,0.01'",APPARENT:"'AIRLESS'",QUANTITIES:"'4'"});
const geo=await query({...common,CENTER:"'500@399'",QUANTITIES:"'23'"});
const topFields=row(top),geoFields=row(geo);
console.error('TOP_ROW',JSON.stringify(topFields));
console.error('GEO_ROW',JSON.stringify(geoFields));
const tn=nums(topFields),gn=nums(geoFields);
if(tn.length<3||gn.length<2)throw new Error('unexpected numeric Horizons columns\nTOP '+row(top).join('|')+'\nGEO '+row(geo).join('|'));
const jplAltitude=tn.at(-1); // quantity 4 => azimuth, elevation; final numeric is elevation
const jplElongation=gn.at(-1); // quantity 23 => elongation plus non-numeric lead/trail code
const altitudeError=Math.abs(provider.altitude_deg-jplAltitude);
const elongationError=Math.abs(provider.elongation_deg-jplElongation);
const thresholdDeg=0.1;
const report={provider,jpl:{altitude_deg:jplAltitude,elongation_deg:jplElongation},error_deg:{altitude:altitudeError,elongation:elongationError},threshold_deg:thresholdDeg,pass:altitudeError<=thresholdDeg&&elongationError<=thresholdDeg,source:'NASA/JPL Horizons'};
console.log(JSON.stringify(report,null,2));
await fs.mkdir('artifacts',{recursive:true});await fs.writeFile('artifacts/jpl-hilal-benchmark.json',JSON.stringify(report,null,2));
if(!report.pass)process.exit(1);
