export const ANTIKYTHERA_EVIDENCE_LEVEL=Object.freeze({
  SURVIVING:'SURVIVING_EVIDENCE',
  STRONGLY_INDICATED:'STRONGLY_INDICATED_RECONSTRUCTION',
  RECONSTRUCTED:'RECONSTRUCTED_MODEL',
  HYPOTHETICAL:'HYPOTHETICAL',
});

export const ANTIKYTHERA_DIGITAL_TWIN=Object.freeze({
  version:'2.0-research-manifest',
  principle:'cycle -> ratio -> mechanical state -> astronomical display',
  components:[
    {id:'rear-metonic',label:'Metonic dial, 235 lunar months / 19 years',status:'SURVIVING_EVIDENCE',source:'Freeth et al. 2008'},
    {id:'rear-saros',label:'Saros dial, 223 lunar months / eclipse glyphs',status:'SURVIVING_EVIDENCE',source:'Freeth et al. 2006/2008'},
    {id:'lunar-anomaly',label:'Pin-and-slot lunar anomaly mechanism',status:'SURVIVING_EVIDENCE',source:'Freeth et al. 2006'},
    {id:'moon-phase',label:'Moon position/phase mechanism',status:'SURVIVING_EVIDENCE',source:'published fragment reconstruction'},
    {id:'games',label:'Panhellenic Games/Olympiad dial',status:'SURVIVING_EVIDENCE',source:'Freeth et al. 2008'},
    {id:'planet-inscriptions',label:'Five classical planets described in front-cosmos inscriptions',status:'SURVIVING_EVIDENCE',source:'X-ray CT inscriptions / Freeth et al. 2021'},
    {id:'venus-mercury-2021',label:'Venus/Mercury front gear trains',status:'STRONGLY_INDICATED_RECONSTRUCTION',source:'Freeth et al. 2021'},
    {id:'superior-planets-2021',label:'Mars/Jupiter/Saturn front gear trains',status:'RECONSTRUCTED_MODEL',source:'Freeth et al. 2021; no surviving direct gearing evidence'},
    {id:'dragon-hand',label:'Lunar-node Dragon Hand',status:'HYPOTHETICAL',source:'Freeth et al. 2021 reconstruction'},
  ],
  boundary:'Manifest reports evidence status; it does not claim one unique historically proven front-gear reconstruction.',
});

export function antikytheraComponent(id){
  const item=ANTIKYTHERA_DIGITAL_TWIN.components.find(x=>x.id===id);
  if(!item)throw new RangeError('unknown Antikythera component');
  return item;
}

export function antikytheraEvidenceSummary(){
  return ANTIKYTHERA_DIGITAL_TWIN.components.reduce((acc,x)=>{acc[x.status]=(acc[x.status]??0)+1;return acc;},{});
}
