import { validateSaros139 } from '../src/eclipse/index.js';
export default async function handler(req,res){const r=validateSaros139();return res.status(r.allPass?200:500).json({version:'0.9-alpha',...r});}
