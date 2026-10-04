import init,{mabims_id_2026_pass,diyanet_1978_site_pass,mtime_version,mtime_clock_json,antikythera_state_json} from "./pkg/mtime_wasm.js";

const REPO="bjo163/antikythera-time";
const wanted=[
  "m-time-ci",
  "m-time-compatibility",
  "m-time-antikythera-calibration",
  "m-time-mclock",
  "m-time-topocentric-matrix",
  "m-time-topocentric-multiyear",
  "m-time-source-ingestion",
  "m-time-historical-falsification",
  "m-time-geospatial-mainland",
  "m-time-wellington-fajr",
  "m-time-wellington-seasonal-oracle",
  "m-time-pages"
];

function cls(conclusion,status){
  if(conclusion==="success") return "success";
  if(conclusion && conclusion!=="success") return "failure";
  if(status==="in_progress"||status==="queued"||status==="waiting") return "running";
  return "";
}

function paint(el,pass,label){
  el.textContent=pass?label+" PASS":label+" FAIL";
  el.className=pass?"ok":"bad";
}

async function loadBuildInfo(){
  try{
    const r=await fetch("./build-info.json",{cache:"no-store"});
    if(!r.ok) throw new Error("build info unavailable");
    const x=await r.json();
    document.getElementById("commitSha").textContent=(x.commit||"").slice(0,12);
    if(x.version) document.getElementById("releaseTag").textContent="v"+x.version;
  }catch(e){ console.warn(e); }
}

async function loadHistoryCorpus(){
  try{
    const r=await fetch("./data/hijri/historical-falsification-v0.11.json",{cache:"no-store"});
    if(!r.ok) throw new Error("historical corpus unavailable");
    const x=await r.json();
    const real=(x.cases||[]).filter(c=>c.kind==="REAL");
    const counts={REPRODUCED:0,FALSIFIED:0,INCOMPLETE:0};
    for(const c of x.cases||[]) counts[c.expected_replay_verdict]=(counts[c.expected_replay_verdict]||0)+1;
    document.getElementById("historyRealCases").textContent=real.length+" cases";
    document.getElementById("historyScope").textContent=(x.scope?.jurisdictions?.length||0)+" jurisdictions · "+(x.scope?.civil_years?.join("–")||"multi-year");
    document.getElementById("historyVerdicts").textContent=
      counts.REPRODUCED+" reproduced · "+counts.FALSIFIED+" falsified controls · "+counts.INCOMPLETE+" incomplete";
  }catch(e){ console.warn(e); }
}

async function loadGithub(){
  try{
    const [releaseRes,runsRes]=await Promise.all([
      fetch(`https://api.github.com/repos/${REPO}/releases/latest`,{headers:{"Accept":"application/vnd.github+json"}}),
      fetch(`https://api.github.com/repos/${REPO}/actions/runs?branch=main&per_page=100`,{headers:{"Accept":"application/vnd.github+json"}})
    ]);
    if(releaseRes.ok){
      const release=await releaseRes.json();
      document.getElementById("releaseTag").textContent=release.tag_name;
    }
    if(!runsRes.ok) throw new Error("actions API unavailable");
    const payload=await runsRes.json();
    const latest=new Map();
    for(const run of payload.workflow_runs||[]){
      if(wanted.includes(run.name) && !latest.has(run.name)) latest.set(run.name,run);
    }
    const box=document.getElementById("workflowList");
    box.innerHTML="";
    for(const name of wanted){
      const run=latest.get(name);
      const line=document.createElement("div");
      line.className="workflow";
      if(!run){
        line.innerHTML=`<span><i class="dot"></i>${name}</span><span class="muted">no run</span>`;
      }else{
        const state=run.conclusion||run.status;
        const dot=cls(run.conclusion,run.status);
        line.innerHTML=`<span><i class="dot ${dot}"></i>${name}</span><a href="${run.html_url}" target="_blank" rel="noreferrer">${state}</a>`;
      }
      box.appendChild(line);
    }
  }catch(e){
    console.warn(e);
    document.getElementById("workflowList").innerHTML='<span class="warn">Live GitHub API unavailable; deployed metrics above remain valid for this build.</span>';
  }
}

function angleDiff(a,b){
  return Math.abs((((a-b)+180)%360+360)%360-180);
}

function refreshMClock(){
  const packet=JSON.parse(mtime_clock_json(Date.now()/1000));
  const historical=JSON.parse(antikythera_state_json(packet.tt_jd,false));
  document.getElementById("machineProfile").textContent=packet.profile_id;
  document.getElementById("machineSun").textContent=packet.solar_longitude_deg.toFixed(6)+"°";
  document.getElementById("machineMoon").textContent=packet.lunar_longitude_deg.toFixed(6)+"°";
  document.getElementById("machinePhase").textContent=packet.lunar_phase_deg.toFixed(6)+"°";
  document.getElementById("machineMetonic").textContent=(packet.metonic_phase*100).toFixed(6)+"%";
  document.getElementById("machineSaros").textContent=(packet.saros_phase*100).toFixed(6)+"%";
  document.getElementById("machineLinear").textContent=packet.linear_si_nanoseconds_from_j2000_tt+" ns";
  document.getElementById("machineResidual").textContent=
    angleDiff(packet.lunar_longitude_deg,historical.lunar_longitude_deg).toFixed(6)+"°";
}

async function boot(){
  await loadBuildInfo();
  await init();
  document.getElementById("wasmVersion").textContent=mtime_version();

  function evaluate(){
    const a=Number(document.getElementById("alt").value);
    const e=Number(document.getElementById("elong").value);
    const m=mabims_id_2026_pass(a,e);
    const d=diyanet_1978_site_pass(a,e);
    paint(document.getElementById("mabimsResult"),m,"CRITERION");
    paint(document.getElementById("diyanetResult"),d,"SITE");
    const diff=document.getElementById("diffResult");
    if(m===d){
      diff.textContent="SAME PROFILE OUTCOME AT THIS SITE";
      diff.className="ok";
    }else{
      diff.textContent="CRITERION DIVERGENCE — SAME INPUT SKY";
      diff.className="warn";
    }
  }

  document.getElementById("eval").addEventListener("click",evaluate);
  document.getElementById("refreshClock").addEventListener("click",refreshMClock);
  evaluate();
  refreshMClock();
  loadHistoryCorpus();
  loadGithub();
}
boot().catch(e=>{
  console.error(e);
  for(const id of ["mabimsResult","diyanetResult","diffResult"]){
    const el=document.getElementById(id);
    el.textContent="WASM load failed";
    el.className="bad";
  }
});
