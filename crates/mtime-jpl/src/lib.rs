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

/// Quantity #4 has the same azimuth/elevation layout for any observer target.
/// This generic alias is used by Sun-specific worship-time oracle checks.
pub fn parse_single_topocentric_quantity_4(
    text: &str,
) -> Result<HorizonsTopocentricMoonSample, HorizonsParseError> {
    parse_single_topocentric_moon_4(text)
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


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HorizonsGeometricVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn parse_geometric_vector_row(row: &str) -> Result<HorizonsGeometricVector, HorizonsParseError> {
    let numeric = numeric_csv_fields(row);
    if numeric.len() < 3 {
        return Err(HorizonsParseError::TooFewNumericFields);
    }
    let n = numeric.len();
    let v = HorizonsGeometricVector {
        x: numeric[n - 3],
        y: numeric[n - 2],
        z: numeric[n - 1],
    };
    if ![v.x, v.y, v.z].iter().all(|x| x.is_finite()) {
        return Err(HorizonsParseError::NonFinite);
    }
    Ok(v)
}

pub fn parse_single_geometric_vector(text: &str) -> Result<HorizonsGeometricVector, HorizonsParseError> {
    let rows = extract_soe_rows(text)?;
    parse_geometric_vector_row(rows[0])
}

pub fn geometric_center_to_center_elongation_deg(
    sun_from_earth: HorizonsGeometricVector,
    moon_from_earth: HorizonsGeometricVector,
) -> Result<f64, HorizonsParseError> {
    let dot = sun_from_earth.x * moon_from_earth.x
        + sun_from_earth.y * moon_from_earth.y
        + sun_from_earth.z * moon_from_earth.z;
    let rs = (sun_from_earth.x.powi(2) + sun_from_earth.y.powi(2) + sun_from_earth.z.powi(2)).sqrt();
    let rm = (moon_from_earth.x.powi(2) + moon_from_earth.y.powi(2) + moon_from_earth.z.powi(2)).sqrt();
    if !dot.is_finite() || !rs.is_finite() || !rm.is_finite() || rs == 0.0 || rm == 0.0 {
        return Err(HorizonsParseError::NonFinite);
    }
    Ok((dot / (rs * rm)).clamp(-1.0, 1.0).acos().to_degrees())
}

pub fn combine_hilal_reference_from_vectors(
    topo: HorizonsTopocentricMoonSample,
    sun_from_earth: HorizonsGeometricVector,
    moon_from_earth: HorizonsGeometricVector,
) -> Result<HorizonsHilalReference, HorizonsParseError> {
    Ok(HorizonsHilalReference {
        moon_altitude_topocentric_deg: topo.elevation_deg,
        elongation_geocentric_deg: geometric_center_to_center_elongation_deg(
            sun_from_earth,
            moon_from_earth,
        )?,
        topocentric_azimuth_deg: topo.azimuth_deg,
    })
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
    fn geometric_vector_angle_is_center_to_center() {
        let a = HorizonsGeometricVector { x: 1.0, y: 0.0, z: 0.0 };
        let b = HorizonsGeometricVector { x: 0.0, y: 1.0, z: 0.0 };
        assert!((geometric_center_to_center_elongation_deg(a, b).unwrap() - 90.0).abs() < 1e-12);
    }

    #[test]
    fn combines_topocentric_altitude_with_geometric_geocentric_elongation() {
        let topo = HorizonsTopocentricMoonSample { azimuth_deg: 270.0, elevation_deg: 3.1 };
        let sun = HorizonsGeometricVector { x: 1.0, y: 0.0, z: 0.0 };
        let moon = HorizonsGeometricVector { x: 0.9935718557, y: 0.1132032138, z: 0.0 };
        let h = combine_hilal_reference_from_vectors(topo, sun, moon).unwrap();
        assert_eq!(h.moon_altitude_topocentric_deg, 3.1);
        assert!((h.elongation_geocentric_deg - 6.5).abs() < 0.01);
    }

    #[test]
    fn extracts_only_ephemeris_rows() {
        let text = "header\n$$SOE\n row-a\n row-b\n$$EOE\nfooter";
        assert_eq!(extract_soe_rows(text).unwrap(), vec!["row-a", "row-b"]);
    }
}
