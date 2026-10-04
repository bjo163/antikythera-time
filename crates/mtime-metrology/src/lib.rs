pub const MEASUREMENT_CSV_HEADER: &str =
    "elapsed_seconds,offset_nanoseconds,temperature_c,reference_id,device_id,firmware_sha";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PpsSample {
    pub elapsed_seconds: f64,
    pub offset_nanoseconds: f64,
    pub temperature_c: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementIdentity {
    pub reference_id: String,
    pub device_id: String,
    pub firmware_sha: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementSeries {
    pub identity: MeasurementIdentity,
    pub samples: Vec<PpsSample>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetrologySummary {
    pub samples: usize,
    pub elapsed_start_seconds: f64,
    pub elapsed_end_seconds: f64,
    pub duration_seconds: f64,
    pub mean_offset_ns: f64,
    pub offset_stddev_ns: f64,
    pub jitter_stddev_ns: f64,
    pub fit_residual_stddev_ns: f64,
    pub max_abs_offset_ns: f64,
    pub fitted_intercept_ns: f64,
    pub fitted_drift_ppm: f64,
    pub temperature_min_c: Option<f64>,
    pub temperature_max_c: Option<f64>,
}

pub fn parse_measurement_csv(input: &str) -> Result<MeasurementSeries, String> {
    let mut lines = input.lines();
    let header = lines
        .next()
        .ok_or_else(|| "measurement CSV is empty".to_owned())?
        .trim_end_matches('\r');

    if header != MEASUREMENT_CSV_HEADER {
        return Err(format!(
            "measurement CSV header mismatch: expected {MEASUREMENT_CSV_HEADER}"
        ));
    }

    let mut identity: Option<MeasurementIdentity> = None;
    let mut samples = Vec::new();
    let mut previous_elapsed: Option<f64> = None;

    for (index, raw_line) in lines.enumerate() {
        let line_number = index + 2;
        let line = raw_line.trim_end_matches('\r');
        if line.trim().is_empty() {
            return Err(format!("blank measurement row at line {line_number}"));
        }

        let columns = line.split(',').collect::<Vec<_>>();
        if columns.len() != 6 {
            return Err(format!(
                "measurement row {line_number} has {} columns; expected 6",
                columns.len()
            ));
        }

        let elapsed_seconds = parse_finite(columns[0], "elapsed_seconds", line_number)?;
        let offset_nanoseconds = parse_finite(columns[1], "offset_nanoseconds", line_number)?;
        if elapsed_seconds < 0.0 {
            return Err(format!(
                "elapsed_seconds must be >= 0 at line {line_number}"
            ));
        }
        if let Some(previous) = previous_elapsed {
            if elapsed_seconds < previous {
                return Err(format!(
                    "elapsed_seconds must be non-decreasing at line {line_number}"
                ));
            }
        }
        previous_elapsed = Some(elapsed_seconds);

        let temperature_c = if columns[2].trim().is_empty() {
            None
        } else {
            Some(parse_finite(columns[2], "temperature_c", line_number)?)
        };

        let row_identity = MeasurementIdentity {
            reference_id: require_field(columns[3], "reference_id", line_number)?,
            device_id: require_field(columns[4], "device_id", line_number)?,
            firmware_sha: require_field(columns[5], "firmware_sha", line_number)?,
        };

        match &identity {
            Some(expected) if expected != &row_identity => {
                return Err(format!(
                    "measurement identity changed within CSV at line {line_number}"
                ));
            }
            None => identity = Some(row_identity),
            _ => {}
        }

        samples.push(PpsSample {
            elapsed_seconds,
            offset_nanoseconds,
            temperature_c,
        });
    }

    if samples.len() < 2 {
        return Err("measurement CSV must contain at least two rows".to_owned());
    }

    Ok(MeasurementSeries {
        identity: identity.expect("two rows imply an identity"),
        samples,
    })
}

fn parse_finite(raw: &str, field: &str, line_number: usize) -> Result<f64, String> {
    let value = raw.trim().parse::<f64>().map_err(|_| {
        format!("invalid {field} numeric value at line {line_number}")
    })?;
    if !value.is_finite() {
        return Err(format!("{field} must be finite at line {line_number}"));
    }
    Ok(value)
}

fn require_field(raw: &str, field: &str, line_number: usize) -> Result<String, String> {
    let value = raw.trim();
    if value.is_empty() {
        return Err(format!("{field} must be non-empty at line {line_number}"));
    }
    Ok(value.to_owned())
}

pub fn summarize(samples: &[PpsSample]) -> Option<MetrologySummary> {
    if samples.len() < 2
        || samples.iter().any(|sample| {
            !sample.elapsed_seconds.is_finite()
                || sample.elapsed_seconds < 0.0
                || !sample.offset_nanoseconds.is_finite()
                || sample
                    .temperature_c
                    .is_some_and(|temperature| !temperature.is_finite())
        })
        || samples
            .windows(2)
            .any(|pair| pair[1].elapsed_seconds < pair[0].elapsed_seconds)
    {
        return None;
    }

    let n = samples.len() as f64;
    let mean_y = samples
        .iter()
        .map(|sample| sample.offset_nanoseconds)
        .sum::<f64>()
        / n;
    let mean_x = samples
        .iter()
        .map(|sample| sample.elapsed_seconds)
        .sum::<f64>()
        / n;

    let xx = samples
        .iter()
        .map(|sample| (sample.elapsed_seconds - mean_x).powi(2))
        .sum::<f64>();
    if xx <= f64::EPSILON {
        return None;
    }

    let cov = samples
        .iter()
        .map(|sample| {
            (sample.elapsed_seconds - mean_x) * (sample.offset_nanoseconds - mean_y)
        })
        .sum::<f64>();
    let slope_ns_per_second = cov / xx;
    let intercept_ns = mean_y - slope_ns_per_second * mean_x;

    let offset_variance = samples
        .iter()
        .map(|sample| (sample.offset_nanoseconds - mean_y).powi(2))
        .sum::<f64>()
        / n;

    let residual_variance = samples
        .iter()
        .map(|sample| {
            let fitted = intercept_ns + slope_ns_per_second * sample.elapsed_seconds;
            (sample.offset_nanoseconds - fitted).powi(2)
        })
        .sum::<f64>()
        / n;

    let mut temperature_min_c: Option<f64> = None;
    let mut temperature_max_c: Option<f64> = None;
    for temperature in samples.iter().filter_map(|sample| sample.temperature_c) {
        temperature_min_c = Some(
            temperature_min_c.map_or(temperature, |current| current.min(temperature)),
        );
        temperature_max_c = Some(
            temperature_max_c.map_or(temperature, |current| current.max(temperature)),
        );
    }

    let elapsed_start_seconds = samples.first()?.elapsed_seconds;
    let elapsed_end_seconds = samples.last()?.elapsed_seconds;
    let fit_residual_stddev_ns = residual_variance.sqrt();

    Some(MetrologySummary {
        samples: samples.len(),
        elapsed_start_seconds,
        elapsed_end_seconds,
        duration_seconds: elapsed_end_seconds - elapsed_start_seconds,
        mean_offset_ns: mean_y,
        offset_stddev_ns: offset_variance.sqrt(),
        jitter_stddev_ns: fit_residual_stddev_ns,
        fit_residual_stddev_ns,
        max_abs_offset_ns: samples
            .iter()
            .map(|sample| sample.offset_nanoseconds.abs())
            .fold(0.0, f64::max),
        fitted_intercept_ns: intercept_ns,
        fitted_drift_ppm: slope_ns_per_second / 1000.0,
        temperature_min_c,
        temperature_max_c,
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HoldoverPoint {
    pub elapsed_seconds: f64,
    pub absolute_error_seconds: f64,
}

#[must_use]
pub fn worst_holdover_error(points: &[HoldoverPoint]) -> Option<f64> {
    if points.is_empty()
        || points.iter().any(|point| {
            !point.elapsed_seconds.is_finite()
                || !point.absolute_error_seconds.is_finite()
                || point.absolute_error_seconds < 0.0
        })
    {
        return None;
    }
    Some(
        points
            .iter()
            .map(|point| point.absolute_error_seconds)
            .fold(0.0, f64::max),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_offset_growth_recovers_frequency_error_without_fake_jitter() {
        let samples = (0..100)
            .map(|i| PpsSample {
                elapsed_seconds: i as f64,
                offset_nanoseconds: 2_000.0 * i as f64,
                temperature_c: Some(25.0),
            })
            .collect::<Vec<_>>();
        let summary = summarize(&samples).unwrap();
        assert!((summary.fitted_drift_ppm - 2.0).abs() < 1e-12);
        assert!(summary.fit_residual_stddev_ns < 1e-9);
        assert!(summary.offset_stddev_ns > 0.0);
        assert_eq!(summary.duration_seconds, 99.0);
    }

    #[test]
    fn parser_rejects_identity_change() {
        let csv = concat!(
            "elapsed_seconds,offset_nanoseconds,temperature_c,reference_id,device_id,firmware_sha\n",
            "0,1,25,REF,DEVICE,SHA\n",
            "1,2,25,REF,OTHER,SHA\n",
        );
        let error = parse_measurement_csv(csv).unwrap_err();
        assert!(error.contains("identity changed"));
    }

    #[test]
    fn parser_rejects_non_monotonic_elapsed_time() {
        let csv = concat!(
            "elapsed_seconds,offset_nanoseconds,temperature_c,reference_id,device_id,firmware_sha\n",
            "1,1,25,REF,DEVICE,SHA\n",
            "0,2,25,REF,DEVICE,SHA\n",
        );
        let error = parse_measurement_csv(csv).unwrap_err();
        assert!(error.contains("non-decreasing"));
    }

    #[test]
    fn holdover_reports_worst_measured_error() {
        assert_eq!(
            worst_holdover_error(&[
                HoldoverPoint {
                    elapsed_seconds: 3600.0,
                    absolute_error_seconds: 0.01,
                },
                HoldoverPoint {
                    elapsed_seconds: 86400.0,
                    absolute_error_seconds: 0.20,
                },
            ]),
            Some(0.20)
        );
    }
}
