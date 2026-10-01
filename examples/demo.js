import {
  SECONDS_PER_DAY,
  METONIC_CYCLE,
  SAROS_CYCLE,
  UTime,
} from '../src/index.js';

const start = UTime.j2000TT();
const oneDayLater = start.plusSeconds(SECONDS_PER_DAY);

console.log(JSON.stringify(oneDayLater, null, 2));
console.log('JD(TT):', oneDayLater.julianDateTT());
console.log('Metonic:', METONIC_CYCLE.positionAt(oneDayLater));
console.log('Saros:', SAROS_CYCLE.positionAt(oneDayLater));
