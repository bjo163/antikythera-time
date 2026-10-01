function assertFiniteNumber(name, value) {
  if (!Number.isFinite(value)) throw new TypeError(`${name} must be finite`);
}

export function adaptiveSimpson(f, a, b, {
  absoluteTolerance = 1e-11,
  relativeTolerance = 1e-11,
  maxDepth = 28,
  maxEvaluations = 250_000,
} = {}) {
  if (typeof f !== 'function') throw new TypeError('f must be a function');
  assertFiniteNumber('a', a); assertFiniteNumber('b', b);
  if (!(b > a)) throw new RangeError('b must be greater than a');
  if (!(absoluteTolerance > 0) || !(relativeTolerance >= 0)) throw new RangeError('invalid integration tolerances');
  if (!Number.isInteger(maxDepth) || maxDepth < 1) throw new RangeError('maxDepth must be a positive integer');
  if (!Number.isInteger(maxEvaluations) || maxEvaluations < 16) throw new RangeError('maxEvaluations too small');

  let evaluations = 0;
  const evalF = (x) => {
    if (++evaluations > maxEvaluations) throw new Error('integration exceeded maxEvaluations');
    const y = f(x);
    if (!Number.isFinite(y)) throw new Error(`integrand is not finite at x=${x}`);
    return y;
  };
  const simpson = (x0,x1,f0,fm,f1)=>(x1-x0)*(f0+4*fm+f1)/6;

  const m=(a+b)/2, fa=evalF(a), fm=evalF(m), fb=evalF(b);
  const whole=simpson(a,b,fa,fm,fb);
  const rootTolerance=Math.max(absoluteTolerance,relativeTolerance*Math.abs(whole));

  function recurse(x0,x1,f0,fmid,f1,coarse,tolerance,depth){
    const mid=(x0+x1)/2, leftMid=(x0+mid)/2, rightMid=(mid+x1)/2;
    const flm=evalF(leftMid), frm=evalF(rightMid);
    const left=simpson(x0,mid,f0,flm,fmid), right=simpson(mid,x1,fmid,frm,f1);
    const refined=left+right, delta=refined-coarse, errorEstimate=Math.abs(delta)/15;
    if(errorEstimate<=tolerance) return {value:refined+delta/15,errorEstimate,depthUsed:maxDepth-depth};
    if(depth<=0) return {value:refined+delta/15,errorEstimate,depthUsed:maxDepth,maxDepthReached:true};
    const l=recurse(x0,mid,f0,flm,fmid,left,tolerance/2,depth-1);
    const r=recurse(mid,x1,fmid,frm,f1,right,tolerance/2,depth-1);
    return {value:l.value+r.value,errorEstimate:l.errorEstimate+r.errorEstimate,depthUsed:Math.max(l.depthUsed,r.depthUsed),maxDepthReached:Boolean(l.maxDepthReached||r.maxDepthReached)};
  }

  const result=recurse(a,b,fa,fm,fb,whole,rootTolerance,maxDepth);
  return {
    value:result.value,errorEstimate:result.errorEstimate,evaluations,
    converged:!result.maxDepthReached&&result.errorEstimate<=rootTolerance,
    maxDepthReached:Boolean(result.maxDepthReached),
    tolerances:{absoluteTolerance,relativeTolerance,maxDepth,maxEvaluations},
  };
}
