const saros=2460409.263+6585.3223;
const mabims=(a,e)=>a>=3.0&&e>=6.4;
const out={saros,mabimsBoundary:mabims(3,6.4),altFail:mabims(2.999,7),elongFail:mabims(4,6.399)};
console.log(JSON.stringify(out));
if(Math.abs(saros-2466994.5853)>1e-9||!out.mabimsBoundary||out.altFail||out.elongFail)process.exit(1);
