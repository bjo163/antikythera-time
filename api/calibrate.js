import {
  calibrateWithHoldout,
  julianDateToUtcDate,
  parseHorizonsObserverRows,
} from '../src/index.js';

export default async function handler(req, res) {
  try {
    const startYear = 2000;
    const stopYear = 2027;
    const cutoffUtc = '2013-01-01T00:00:00Z';

    const params = new URLSearchParams({
      format: 'text', COMMAND: "'301'", OBJ_DATA: "'NO'", MAKE_EPHEM: "'YES'",
      EPHEM_TYPE: "'OBSERVER'", CENTER: "'500@399'", START_TIME: `'${startYear}-01-01'`,
      STOP_TIME: `'${stopYear}-01-01'`, STEP_SIZE: "'30 d'", TIME_TYPE: "'UT'",
      CAL_FORMAT: "'JD'", QUANTITIES: "'10,24'", CSV_FORMAT: "'YES'",
    });

    const response = await fetch('https://ssd.jpl.nasa.gov/api/horizons.api?' + params, {
      headers: { 'User-Agent': 'antikythera-time-v0.5' },
    });
    const text = await response.text();

    if (!response.ok) {
      return res.status(502).json({ error: 'JPL Horizons request failed', status: response.status });
    }

    const rows = parseHorizonsObserverRows(text)
      .filter(r => r.jd && r.illuminatedPercent != null && r.phaseAngleDeg != null);

    if (rows.length < 100) {
      return res.status(502).json({ error: 'Insufficient parseable JPL rows', rowCount: rows.length });
    }

    const references = rows.map(r => ({
      date: julianDateToUtcDate(r.jd),
      illuminatedPercent: r.illuminatedPercent,
      phaseAngleDeg: r.phaseAngleDeg,
    }));

    const result = calibrateWithHoldout(references, cutoffUtc);

    res.setHeader('Cache-Control', 's-maxage=86400, stale-while-revalidate=604800');
    return res.status(200).json({
      version: '0.5-alpha',
      source: 'NASA/JPL Horizons',
      sourceWindow: [`${startYear}-01-01`, `${stopYear}-01-01`],
      sampling: '30 days',
      target: 'Moon (301)',
      center: 'Earth geocenter',
      ...result,
      scientificRule: 'A calibration is accepted only if the untouched holdout score improves.',
    });
  } catch (error) {
    return res.status(500).json({ error: error instanceof Error ? error.message : 'unknown error' });
  }
}
