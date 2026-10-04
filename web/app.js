import init,{mabims_id_2026_pass,diyanet_1978_site_pass,mtime_version} from "./pkg/mtime_wasm.js";

const REPO="bjo163/antikythera-time";
const wanted=[
  "m-time-ci",
  "m-time-compatibility",
  "m-time-topocentric-matrix",
  "m-time-topocentric-multiyear",
  "m-time-source-ingestion",
  "m-time-historical-falsification",
  "m-time-geospatial-mainland",
  "m-time-wellington-fajr",
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
  evaluate();
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
