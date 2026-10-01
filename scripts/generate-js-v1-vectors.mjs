#!/usr/bin/env node
import fs from 'node:fs/promises';
import { TimeScale, coordinateFromTwoPartJD, ttToTcgCoordinate, inferCosmicAge, getCosmologyPreset, materializePresetParameters, EclipseRecurrence, predictSaros, jplApproxHeliocentric } from '../src/index.js';

const tt=coordinateFromTwoPartJD(TimeScale.TT,2453750.5,0.892482639);
const tcg=ttToTcgCoordinate(tt);
const p=getCosmologyPreset('planck2018');
const age=inferCosmicAge(p.model,materializePresetParameters(p)).result.gyr;
const eclipse=predictSaros(new EclipseRecurrence({jdTt:2460409.263,saros:139,type:'TOTAL',source:'compat'}),1);
const mars=jplApproxHeliocentric('Mars',2451545.0);
const out={implementation:'javascript',ttToTcgD2:tcg.d2,planckAgeGyr:age,sarosJdTt:eclipse.jdTt,mars:{x:mars.x,y:mars.y,z:mars.z}};
await fs.mkdir('artifacts',{recursive:true});
await fs.writeFile('artifacts/v1-js.json',JSON.stringify(out,null,2));
console.log(JSON.stringify(out,null,2));
