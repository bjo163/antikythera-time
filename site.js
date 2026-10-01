import { METONIC_CYCLE, SAROS_CYCLE, lunarModelAtUtc, taiMinusUtcAt, utcDateToTt } from './src/index.js';
const $ = (id) => document.getElementById(id);

for (let y=2026; y>=2000; y--) {
  const o=document.createElement('option'); o.value=y; o.textContent=y; $('yearSelect').append(o);
}

function renderNow() {
  try {
    const now = new Date();
    const t = utcDateToTt(now);
    $('utc').textContent = now.toISOString();
    $('utime').textContent = `${t.nsSinceJ2000.toLocaleString()} ns`;
    $('jd').textContent = t.julianDateTT().toFixed(8);
    $('taiUtc').textContent = `${taiMinusUtcAt(now)} s`;

    for (const [cycle, barId, labelId] of [[METONIC_CYCLE,'metonicBar','metonicPhase'],[SAROS_CYCLE,'sarosBar','sarosPhase']]) {
      const p = cycle.positionAt(t);
      $(barId).style.width = `${(p.phase*100).toFixed(3)}%`;
      $(labelId).textContent = `${(p.phase*100).toFixed(6)}% through cycle`;
    }

    const moon = lunarModelAtUtc(now);
    $('moonName').textContent = moon.phaseName;
    $('moonAge').textContent = `${moon.ageDays.toFixed(3)} d`;
    $('moonIllum').textContent = `${moon.illuminationPercent.toFixed(3)}%`;
    $('moonAngle').textContent = `${moon.phaseAngleDeg.toFixed(3)}°`;
    $('moonOrb').style.setProperty('--illum', `${moon.illuminationPercent}%`);
  } catch (e) {
    $('utime').textContent = `validation blocked: ${e.message}`;
  }
}

renderNow();
setInterval(renderNow, 1000);

$('validateBtn').addEventListener('click', async () => {
  const year = $('yearSelect').value;
  const status = $('validationStatus');
  status.className='result';
  status.textContent=`Querying JPL Horizons for ${year}…`;
  $('metrics').classList.add('hidden');
  $('errorBars').innerHTML='';
  $('sampleTable').innerHTML='';

  try {
    const r = await fetch(`/api/validation?year=${year}`);
    const data = await r.json();
    if (!r.ok) throw new Error(data.error || `HTTP ${r.status}`);

    status.innerHTML = `<b>${data.source}</b> · Moon, Earth geocenter · ${data.summary.sampleCount} samples<br><span class="small">${data.interpretation}</span>`;
    $('sampleCount').textContent=data.summary.sampleCount;
    $('illumMae').textContent=`${data.summary.illumination.maePoints.toFixed(3)} pp`;
    $('illumMax').textContent=`${data.summary.illumination.maxAbsPoints.toFixed(3)} pp`;
    $('phaseMae').textContent=`${data.summary.phaseAngle.maeDeg.toFixed(3)}°`;
    $('metrics').classList.remove('hidden');

    const max = Math.max(...data.samples.map(s=>s.error.absIlluminationPoints),1);
    $('errorBars').innerHTML = data.samples.map(s =>
      `<div class="errcol" title="${s.utc}: ${s.error.absIlluminationPoints.toFixed(3)} percentage points"><i style="height:${Math.max(4,s.error.absIlluminationPoints/max*100)}%"></i><span>${s.utc.slice(5,7)}</span></div>`
    ).join('');

    $('sampleTable').innerHTML = `<table><thead><tr><th>UTC</th><th>Model illum.</th><th>JPL illum.</th><th>|error|</th></tr></thead><tbody>${data.samples.map(s =>
      `<tr><td>${s.utc.slice(0,10)}</td><td>${s.model.illuminationPercent.toFixed(2)}%</td><td>${s.reference.illuminatedPercent.toFixed(2)}%</td><td>${s.error.absIlluminationPoints.toFixed(2)} pp</td></tr>`
    ).join('')}</tbody></table>`;
  } catch (e) {
    status.className='result error';
    status.textContent=`Validation unavailable: ${e.message}`;
  }
});

$('calibrateBtn').addEventListener('click', async () => {
  const status=$('calibrationStatus');
  const metrics=$('calibrationMetrics');
  const verdict=$('calibrationVerdict');

  status.className='result';
  status.textContent='Querying JPL 2000–2026 and fitting training window…';
  metrics.classList.add('hidden');
  verdict.classList.add('hidden');

  try {
    const r=await fetch('/api/calibrate');
    const d=await r.json();
    if(!r.ok) throw new Error(d.error || `HTTP ${r.status}`);

    status.innerHTML=`<b>${d.source}</b> · ${d.trainingSamples} training + ${d.holdoutSamples} holdout samples<br><span class="small">${d.scientificRule}</span>`;
    $('periodDelta').textContent=`${d.delta.periodSeconds>=0?'+':''}${d.delta.periodSeconds.toFixed(2)} s`;
    $('epochDelta').textContent=`${d.delta.epochOffsetSeconds>=0?'+':''}${d.delta.epochOffsetSeconds.toFixed(0)} s`;
    $('trainImprove').textContent=`${d.train.scoreImprovementPct.toFixed(2)}%`;
    $('holdoutImprove').textContent=`${d.holdout.scoreImprovementPct.toFixed(2)}%`;
    metrics.classList.remove('hidden');

    verdict.className=`verdict ${d.holdout.generalizes?'accept':'reject'}`;
    verdict.textContent=d.holdout.generalizes?'ACCEPT: GENERALIZES ON HOLDOUT':'REJECT: NO HOLDOUT IMPROVEMENT';
  } catch(e) {
    status.className='result error';
    status.textContent=`Calibration unavailable: ${e.message}`;
  }
});


$('residualBtn').addEventListener('click', async () => {
  const status=$('residualStatus');
  const metrics=$('residualMetrics');
  const verdict=$('residualVerdict');

  status.className='result';
  status.textContent='Querying JPL 2000–2026 and fitting fixed anomalistic/draconic residuals…';
  metrics.classList.add('hidden');
  verdict.classList.add('hidden');

  try {
    const r=await fetch('/api/residuals');
    const d=await r.json();
    if(!r.ok) throw new Error(d.error || `HTTP ${r.status}`);

    status.innerHTML=`<b>${d.source}</b> · ${d.trainingSamples} training + ${d.holdoutSamples} holdout samples<br><span class="small">${d.basisPolicy}</span>`;
    $('anomIllumAmp').textContent=`${d.residualModel.illuminationHarmonics.anomalistic.amplitude.toFixed(3)} pp`;
    $('dracIllumAmp').textContent=`${d.residualModel.illuminationHarmonics.draconic.amplitude.toFixed(3)} pp`;
    $('resTrainImprove').textContent=`${d.train.scoreImprovementPct.toFixed(2)}%`;
    $('resHoldoutImprove').textContent=`${d.holdout.scoreImprovementPct.toFixed(2)}%`;
    metrics.classList.remove('hidden');

    verdict.className=`verdict ${d.holdout.generalizes?'accept':'reject'}`;
    verdict.textContent=d.holdout.generalizes
      ? 'ACCEPT: FIXED-CYCLE RESIDUALS GENERALIZE'
      : 'REJECT: RESIDUAL CYCLES DO NOT IMPROVE HOLDOUT';
  } catch(e) {
    status.className='result error';
    status.textContent=`Residual validation unavailable: ${e.message}`;
  }
});
