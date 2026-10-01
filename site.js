import { METONIC_CYCLE,SAROS_CYCLE,lunarModelAtUtc,taiMinusUtcAt,utcDateToTt } from './src/index.js';
const $=id=>document.getElementById(id);
for(let y=2026;y>=2000;y--){const o=document.createElement('option');o.value=y;o.textContent=y;$('yearSelect').append(o);}
function render(){
 try{
  const now=new Date(),t=utcDateToTt(now),moon=lunarModelAtUtc(now);
  $('utc').textContent=now.toISOString();$('utime').textContent=t.nsSinceJ2000.toLocaleString()+' ns';
  $('jd').textContent=t.julianDateTT().toFixed(8);$('taiUtc').textContent=taiMinusUtcAt(now)+' s';
  for(const [c,b,l] of [[METONIC_CYCLE,'metonicBar','metonicPhase'],[SAROS_CYCLE,'sarosBar','sarosPhase']]){
   const p=c.positionAt(t);$(b).style.width=(p.phase*100).toFixed(2)+'%';$(l).textContent=(p.phase*100).toFixed(6)+'% through cycle';
  }
  $('moonName').textContent=moon.phaseName;$('moonAge').textContent=moon.ageDays.toFixed(3)+' d';
  $('moonIllum').textContent=moon.illuminationPercent.toFixed(3)+'%';$('moonAngle').textContent=moon.phaseAngleDeg.toFixed(3)+'°';
 }catch(e){$('utime').textContent='validation blocked: '+e.message;}
}
render();setInterval(render,1000);
$('validateBtn').addEventListener('click',async()=>{
 const year=$('yearSelect').value,status=$('validationStatus');
 status.className='result';status.textContent='Querying JPL Horizons for '+year+'…';$('metrics').classList.add('hidden');$('errorBars').innerHTML='';$('sampleTable').innerHTML='';
 try{
  const r=await fetch('/api/validation?year='+year),d=await r.json();if(!r.ok)throw new Error(d.error||('HTTP '+r.status));
  status.innerHTML='<b>'+d.source+'</b> · Moon, Earth geocenter · '+d.summary.sampleCount+' samples<br><small>'+d.interpretation+'</small>';
  $('sampleCount').textContent=d.summary.sampleCount;$('illumMae').textContent=d.summary.illumination.maePoints.toFixed(3)+' pp';
  $('illumMax').textContent=d.summary.illumination.maxAbsPoints.toFixed(3)+' pp';$('phaseMae').textContent=d.summary.phaseAngle.maeDeg.toFixed(3)+'°';$('metrics').classList.remove('hidden');
  const max=Math.max(...d.samples.map(s=>s.error.absIlluminationPoints),1);
  $('errorBars').innerHTML=d.samples.map(s=>'<div class="bar" title="'+s.utc+'"><i style="height:'+Math.max(4,s.error.absIlluminationPoints/max*100)+'%"></i><small>'+s.utc.slice(5,7)+'</small></div>').join('');
  $('sampleTable').innerHTML='<table><thead><tr><th>UTC</th><th>Model</th><th>JPL</th><th>|error|</th></tr></thead><tbody>'+d.samples.map(s=>'<tr><td>'+s.utc.slice(0,10)+'</td><td>'+s.model.illuminationPercent.toFixed(2)+'%</td><td>'+s.reference.illuminatedPercent.toFixed(2)+'%</td><td>'+s.error.absIlluminationPoints.toFixed(2)+' pp</td></tr>').join('')+'</tbody></table>';
 }catch(e){status.className='result error';status.textContent='Validation unavailable: '+e.message;}
});