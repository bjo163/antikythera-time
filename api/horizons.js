export default async function handler(req, res) {
  try {
    const rawDate = typeof req.query?.date === 'string' ? req.query.date : new Date().toISOString();
    const instant = new Date(rawDate);
    if (Number.isNaN(instant.getTime())) return res.status(400).json({ error: 'invalid date' });
    const isoMinute = instant.toISOString().slice(0,16).replace('T',' ');
    const params = new URLSearchParams({
      format: 'text',
      COMMAND: "'301'",
      OBJ_DATA: "'NO'",
      MAKE_EPHEM: "'YES'",
      EPHEM_TYPE: "'OBSERVER'",
      CENTER: "'500@399'",
      TLIST: `'${isoMinute}'`,
      TLIST_TYPE: "'CAL'",
      TIME_TYPE: "'UT'",
      QUANTITIES: "'10,24'",
      CSV_FORMAT: "'YES'",
      TIME_DIGITS: "'MINUTES'",
    });
    const url = `https://ssd.jpl.nasa.gov/api/horizons.api?${params}`;
    const response = await fetch(url, { headers: { 'User-Agent': 'antikythera-time-v0.3' } });
    const text = await response.text();
    if (!response.ok) return res.status(502).json({ error: 'JPL Horizons request failed', status: response.status });
    const start = text.indexOf('$$SOE');
    const end = text.indexOf('$$EOE');
    const rows = start >= 0 && end > start ? text.slice(start + 5, end).trim().split('\n').map(s=>s.trim()).filter(Boolean) : [];
    const firstRow = rows[0] ?? null;
    let illuminatedPercent = null;
    let phaseAngleDeg = null;
    if (firstRow) {
      const parts = firstRow.split(',').map(x=>x.trim()).filter(Boolean);
      const nums = parts.map(x=>Number(x)).filter(Number.isFinite);
      if (nums.length >= 2) { illuminatedPercent = nums[nums.length-2]; phaseAngleDeg = nums[nums.length-1]; }
    }
    res.setHeader('Cache-Control','s-maxage=300, stale-while-revalidate=600');
    return res.status(200).json({ source:'NASA/JPL Horizons', target:'Moon (301)', center:'Earth geocenter', requestedUtc:instant.toISOString(), rawRow:firstRow, illuminatedPercent, phaseAngleDeg, note:'Live external reference; parsing is intentionally narrow and should be regression-tested against JPL format changes.' });
  } catch (error) {
    return res.status(500).json({ error: error instanceof Error ? error.message : 'unknown error' });
  }
}
