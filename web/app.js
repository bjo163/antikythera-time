import init,{mabims_id_2026_pass,diyanet_1978_site_pass,mtime_version} from "./pkg/mtime_wasm.js";

const mabims=document.getElementById("mabimsResult");
const diyanet=document.getElementById("diyanetResult");
const diff=document.getElementById("diffResult");

function paint(el,pass,label){
  el.textContent=pass?label+" PASS":label+" FAIL";
  el.className=pass?"ok":"no";
}

async function boot(){
  await init();
  function evaluate(){
    const a=Number(document.getElementById("alt").value);
    const e=Number(document.getElementById("elong").value);
    const m=mabims_id_2026_pass(a,e);
    const d=diyanet_1978_site_pass(a,e);
    paint(mabims,m,"CRITERION");
    paint(diyanet,d,"SITE");
    if(m===d){
      diff.textContent="SAME PROFILE OUTCOME AT THIS SITE";
      diff.className="ok";
    }else{
      diff.textContent="CRITERION DIVERGENCE — SAME INPUT SKY";
      diff.className="note";
    }
  }
  document.getElementById("eval").addEventListener("click",evaluate);
  evaluate();
  console.log("M-Time WASM",mtime_version());
}
boot().catch(e=>{
  mabims.textContent="WASM load failed";
  diyanet.textContent="WASM load failed";
  diff.textContent="WASM load failed";
  console.error(e);
});
