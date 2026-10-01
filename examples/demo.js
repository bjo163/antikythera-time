import { METONIC_CYCLE, SAROS_CYCLE, utcDateToCurrentTt } from '../src/index.js';
const now = utcDateToCurrentTt(new Date());
console.log(JSON.stringify(now, null, 2));
console.log('JD(TT):', now.julianDateTT());
console.log('Metonic:', METONIC_CYCLE.positionAt(now));
console.log('Saros:', SAROS_CYCLE.positionAt(now));
