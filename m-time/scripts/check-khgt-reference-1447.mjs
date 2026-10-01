import fs from 'node:fs/promises';
const r=JSON.parse(await fs.readFile(process.argv[2]??'/tmp/khgt-reference.json','utf8'));
const jd=(iso)=>Date.parse(iso)/86400000+2440587.5;
const expected={
  conjunction_jd_utc:jd('2026-03-19T01:23:28Z'),
  published_sunset_jd_utc:jd('2026-03-19T15:34:04Z'),
  geocentric_altitude_deg:6+9/60+9/3600,
  elongation_deg:8+5/60+24/3600,
};
const errors={
  conjunction_seconds:Math.abs(r.conjunction_jd_utc-expected.conjunction_jd_utc)*86400,
  published_timestamp_seconds:Math.abs(r.published_sunset_jd_utc-expected.published_sunset_jd_utc)*86400,
  geometry_altitude_deg:Math.abs(r.published_time_geocentric_altitude_deg-expected.geocentric_altitude_deg),
  geometry_elongation_deg:Math.abs(r.published_time_elongation_deg-expected.elongation_deg),
  sunset_convention_difference_seconds:(r.mtime_sunset_jd_utc-expected.published_sunset_jd_utc)*86400,
};
const gates={conjunction_seconds:60,published_timestamp_seconds:0.1,geometry_altitude_deg:0.1,geometry_elongation_deg:0.1};
const pass=Object.keys(gates).every(k=>Math.abs(errors[k])<=gates[k]);
const report={
 source:'Muhammadiyah KHGT Syawal 1447 H published calculation',
 computed:r,expected,errors,gates,pass,
 sunsetConventionStatus:'DIAGNOSTIC_ONLY_NOT_TUNED',
 note:'M-Time sunset model is not forced to equal the KHGT published sunset; lunar geometry is compared at KHGT published timestamp.'
};
console.log(JSON.stringify(report,null,2));
await fs.mkdir('artifacts',{recursive:true});await fs.writeFile('artifacts/khgt-reference-1447.json',JSON.stringify(report,null,2));
if(!pass)process.exit(1);
