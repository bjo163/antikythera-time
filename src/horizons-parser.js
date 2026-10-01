export function parseHorizonsObserverRows(text) {
  if (typeof text !== 'string') throw new TypeError('text must be string');
  const start = text.indexOf('$$SOE');
  const end = text.indexOf('$$EOE');
  if (start < 0 || end <= start) return [];
  return text.slice(start + 5, end).trim().split('\n').map(line=>line.trim()).filter(Boolean).map(raw => {
    const parts = raw.split(',').map(x=>x.trim());
    const numeric = parts.filter(x=>x !== '').map(x=>Number(x)).filter(Number.isFinite);
    if (numeric.length < 3) return { raw, jd: null, illuminatedPercent: null, phaseAngleDeg: null };
    return {
      raw,
      jd: numeric[0],
      illuminatedPercent: numeric[numeric.length - 2],
      phaseAngleDeg: numeric[numeric.length - 1],
    };
  });
}

export function julianDateToUtcDate(jd) {
  if (!Number.isFinite(jd)) throw new TypeError('jd must be finite');
  return new Date((jd - 2_440_587.5) * 86_400_000);
}
