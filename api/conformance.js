import { checkV2Record } from '../src/v2/index.js';
export default async function handler(req,res){
  try{
    if(req.method!=='POST')return res.status(405).json({error:'POST required'});
    const body=typeof req.body==='string'?JSON.parse(req.body):req.body;
    const records=Array.isArray(body)?body:[body];
    const results=records.map((record,index)=>({index,...checkV2Record(record)}));
    return res.status(results.every(x=>x.pass)?200:422).json({version:'2.0.0',specStatus:'STANDARDIZATION_CANDIDATE',results,allPass:results.every(x=>x.pass)});
  }catch(error){return res.status(400).json({error:error instanceof Error?error.message:'unknown error'});}
}
