#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HorizonsTopocentricMoonSample {
    pub azimuth_deg: f64,
    pub elevation_deg: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HorizonsGeocentricElongationSample {
    pub solar_elongation_deg: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HorizonsHilalReference {
    pub moon_altitude_topocentric_deg: f64,
    pub elongation_geocentric_deg: f64,
    pub topocentric_azimuth_deg: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HorizonsParseError {
    MissingDataRow,
    TooFewNumericFields,
    NonFinite,
}

pub fn extract_soe_rows(text: &str) -> Result<Vec<&str>, HorizonsParseError> {
    let start = text.find("$$SOE").ok_or(HorizonsParseError::MissingDataRow)? + 5;
    let rest = &text[start..];
    let end = rest.find("$$EOE").ok_or(HorizonsParseError::MissingDataRow)?;
    let rows = rest[..end]
        .lines()
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();
    if rows.is_empty() {
        return Err(HorizonsParseError::MissingDataRow);
    }
    Ok(rows)
}

fn numeric_csv_fields(row: &str) -> Vec<f64> {
    row.split(',')
        .filter_map(|x| x.trim().parse::<f64>().ok())
        .collect()
}

/// Parse Horizons quantity #4 only (apparent topocentric AZ/EL) in CSV+DEG output.
pub fn parse_topocentric_moon_quantity_4(
    row: &str,
) -> Result<HorizonsTopocentricMoonSample, HorizonsParseError> {
    let numeric = numeric_csv_fields(row);
    if numeric.len() < 2 {
        return Err(HorizonsParseError::TooFewNumericFields);
    }
    let n = numeric.len();
    let sample = HorizonsTopocentricMoonSample {
        azimuth_deg: numeric[n - 2],
        elevation_deg: numeric[n - 1],
    };
    if ![sample.azimuth_deg, sample.elevation_deg]
        .iter()
        .all(|x| x.is_finite())
    {
        return Err(HorizonsParseError::NonFinite);
    }
    Ok(sample)
}

/// Parse Horizons quantity #23 only from a geocentric observer query.
/// Quantity #23 is Sun-Observer-Target apparent solar elongation.
pub fn parse_geocentric_elongation_quantity_23(
    row: &str,
) -> Result<HorizonsGeocentricElongationSample, HorizonsParseError> {
    let numeric = numeric_csv_fields(row);
    let Some(value) = numeric.last().copied() else {
        return Err(HorizonsParseError::TooFewNumericFields);
    };
    if !value.is_finite() {
        return Err(HorizonsParseError::NonFinite);
    }
    Ok(HorizonsGeocentricElongationSample {
        solar_elongation_deg: value,
    })
}

pub fn parse_single_topocentric_moon_4(
    text: &str,
) -> Result<HorizonsTopocentricMoonSample, HorizonsParseError> {
    let rows = extract_soe_rows(text)?;
    parse_topocentric_moon_quantity_4(rows[0])
}

pub fn parse_single_geocentric_elongation_23(
    text: &str,
) -> Result<HorizonsGeocentricElongationSample, HorizonsParseError> {
    let rows = extract_soe_rows(text)?;
    parse_geocentric_elongation_quantity_23(rows[0])
}

#[must_use]
pub fn combine_hilal_reference(
    topo: HorizonsTopocentricMoonSample,
    geo: HorizonsGeocentricElongationSample,
) -> HorizonsHilalReference {
    HorizonsHilalReference {
        moon_altitude_topocentric_deg: topo.elevation_deg,
        elongation_geocentric_deg: geo.solar_elongation_deg,
        topocentric_azimuth_deg: topo.azimuth_deg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_topocentric_az_el_without_relabeling_elongation() {
        let row = "2026-Mar-19 10:00, , , 274.1234, 2.9876,";
        let x = parse_topocentric_moon_quantity_4(row).unwrap();
        assert_eq!(x.azimuth_deg, 274.1234);
        assert_eq!(x.elevation_deg, 2.9876);
    }

    #[test]
    fn parses_geocentric_elongation_separately() {
        let row = "2026-Mar-19 10:00, , , 6.0123, /T,";
        let x = parse_geocentric_elongation_quantity_23(row).unwrap();
        assert_eq!(x.solar_elongation_deg, 6.0123);
    }

    #[test]
    fn combines_only_after_observer_semantics_are_explicit() {
        let h = combine_hilal_reference(
            HorizonsTopocentricMoonSample {
                azimuth_deg: 270.0,
                elevation_deg: 3.1,
            },
            HorizonsGeocentricElongationSample {
                solar_elongation_deg: 6.5,
            },
        );
        assert_eq!(h.moon_altitude_topocentric_deg, 3.1);
        assert_eq!(h.elongation_geocentric_deg, 6.5);
    }

    #[test]
    fn extracts_only_ephemeris_rows() {
        let text = "header\n$$SOE\n row-a\n row-b\n$$EOE\nfooter";
        assert_eq!(extract_soe_rows(text).unwrap(), vec!["row-a", "row-b"]);
    }
}
