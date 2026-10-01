import init,{mabims_id_2026_pass,mtime_version} from "./pkg/mtime_wasm.js";
const result=document.getElementById("result");
async function boot(){await init();function evaluate(){const a=Number(document.getElementById("alt").value),e=Number(document.getElementById("elong").value);const pass=mabims_id_2026_pass(a,e);result.textContent=pass?"CRITERION PASS":"CRITERION FAIL";result.className=pass?"ok":"no";}document.getElementById("eval").addEventListener("click",evaluate);evaluate();console.log("M-Time WASM",mtime_version());}
boot().catch(e=>{result.textContent="WASM load failed";console.error(e);});
