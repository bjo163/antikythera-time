import { METONIC_CYCLE, SAROS_CYCLE, utcDateToCurrentTt } from './src/index.js';
const $ = (id) => document.getElementById(id);
function renderNow() {
  try {
    const now = new Date();
    const t = utcDateToCurrentTt(now);
    $('utc').textContent = now.toISOString();
    $('utime').textContent = `${t.nsSinceJ2000.toLocaleString()} ns`;
    $('jd').textContent = t.julianDateTT().toFixed(8);
    for (const [cycle, barId, labelId] of [[METONIC_CYCLE,'metonicBar','metonicPhase'],[SAROS_CYCLE,'sarosBar','sarosPhase']]) {
      const p = cycle.positionAt(t);
      $(barId).style.width = `${(p.phase*100).toFixed(3)}%`;
      $(labelId).textContent = `${(p.phase*100).toFixed(6)}% through cycle`;
    }
  } catch (e) { $('utime').textContent = `validation blocked: ${e.message}`; }
}
renderNow(); setInterval(renderNow, 1000);
$('jplBtn').addEventListener('click', async () => {
  const status = $('jplStatus'); status.className='result'; status.textContent='Querying JPL Horizons…';
  try {
    const r = await fetch(`/api/horizons?date=${encodeURIComponent(new Date().toISOString())}`);
    const data = await r.json();
    if (!r.ok) throw new Error(data.error || `HTTP ${r.status}`);
    const illum = data.illuminatedPercent == null ? 'parse pending' : `${data.illuminatedPercent}%`;
    const phase = data.phaseAngleDeg == null ? 'parse pending' : `${data.phaseAngleDeg}°`;
    status.innerHTML = `<b>${data.source}</b><br>Illuminated: ${illum}<br>Phase angle: ${phase}<br><span class="mono small">${data.rawRow ?? 'No ephemeris row parsed'}</span>`;
  } catch (e) { status.className='result error'; status.textContent=`JPL validation unavailable: ${e.message}`; }
});
