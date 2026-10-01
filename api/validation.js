import { compareLunarReference, julianDateToUtcDate, parseHorizonsObserverRows, summarizeValidation } from '../src/index.js';

export default async function handler(req, res) {
  try {
    const year = Number(req.query?.year ?? 2026);
    if (!Number.isInteger(year) || year < 2000 || year > 2026) {
      return res.status(400).json({ error: 'year must be an integer from 2000 through 2026' });
    }
    const params = new URLSearchParams({
      format: 'text', COMMAND: "'301'", OBJ_DATA: "'NO'", MAKE_EPHEM: "'YES'",
      EPHEM_TYPE: "'OBSERVER'", CENTER: "'500@399'", START_TIME: "'"+year+"-01-01'",
      STOP_TIME: "'"+(year+1)+"-01-01'", STEP_SIZE: "'1 mo'", TIME_TYPE: "'UT'",
      CAL_FORMAT: "'JD'", QUANTITIES: "'10,24'", CSV_FORMAT: "'YES'",
    });
    const response = await fetch('https://ssd.jpl.nasa.gov/api/horizons.api?'+params, { headers: { 'User-Agent': 'antikythera-time-v0.4' } });
    const text = await response.text();
    if (!response.ok) return res.status(502).json({ error: 'JPL Horizons request failed', status: response.status });
    const rows = parseHorizonsObserverRows(text).filter(r => r.jd && r.illuminatedPercent != null && r.phaseAngleDeg != null);
    if (!rows.length) return res.status(502).json({ error: 'No parseable JPL rows', excerpt: text.slice(0, 800) });
    const samples = rows.map(r => compareLunarReference({
      date: julianDateToUtcDate(r.jd), illuminatedPercent: r.illuminatedPercent, phaseAngleDeg: r.phaseAngleDeg,
    }));
    res.setHeader('Cache-Control','s-maxage=3600, stale-while-revalidate=86400');
    return res.status(200).json({
      version: '0.4-alpha', year, source: 'NASA/JPL Horizons', target: 'Moon (301)', center: 'Earth geocenter',
      model: 'mean synodic month calibrated to NASA/GSFC New Moon 2000-01-06 18:14 UT',
      summary: summarizeValidation(samples), samples,
      interpretation: 'Errors quantify this simple cycle model against JPL; non-zero error is expected from lunar orbital perturbations.',
    });
  } catch (error) {
    return res.status(500).json({ error: error instanceof Error ? error.message : 'unknown error' });
  }
}
