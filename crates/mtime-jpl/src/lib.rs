#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HorizonsMoonObserverSample {
    pub azimuth_deg: f64,
    pub elevation_deg: f64,
    pub solar_elongation_deg: f64,
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

/// Parser contract for Horizons observer queries that request only quantities 4 and 23,
/// with CSV_FORMAT=YES and ANG_FORMAT=DEG.
///
/// The calendar field and lead/trail marker are non-numeric, so the final three numeric
/// CSV fields are azimuth, elevation, and Sun-observer-target elongation.
pub fn parse_moon_observer_4_23_row(row: &str) -> Result<HorizonsMoonObserverSample, HorizonsParseError> {
    let numeric = row
        .split(',')
        .filter_map(|x| x.trim().parse::<f64>().ok())
        .collect::<Vec<_>>();
    if numeric.len() < 3 {
        return Err(HorizonsParseError::TooFewNumericFields);
    }
    let n = numeric.len();
    let sample = HorizonsMoonObserverSample {
        azimuth_deg: numeric[n - 3],
        elevation_deg: numeric[n - 2],
        solar_elongation_deg: numeric[n - 1],
    };
    if ![
        sample.azimuth_deg,
        sample.elevation_deg,
        sample.solar_elongation_deg,
    ]
    .iter()
    .all(|x| x.is_finite())
    {
        return Err(HorizonsParseError::NonFinite);
    }
    Ok(sample)
}

pub fn parse_single_moon_observer_4_23(text: &str) -> Result<HorizonsMoonObserverSample, HorizonsParseError> {
    let rows = extract_soe_rows(text)?;
    parse_moon_observer_4_23_row(rows[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_takes_final_three_numeric_csv_fields() {
        let row = "2026-Mar-19 10:00, , , 274.1234, 2.9876, 6.0123, /T,";
        let x = parse_moon_observer_4_23_row(row).unwrap();
        assert_eq!(x.azimuth_deg, 274.1234);
        assert_eq!(x.elevation_deg, 2.9876);
        assert_eq!(x.solar_elongation_deg, 6.0123);
    }

    #[test]
    fn extracts_only_ephemeris_rows() {
        let text = "header\n$$SOE\n row-a\n row-b\n$$EOE\nfooter";
        assert_eq!(extract_soe_rows(text).unwrap(), vec!["row-a", "row-b"]);
    }
}
