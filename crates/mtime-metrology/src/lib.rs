#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PpsSample {
    pub elapsed_seconds: f64,
    pub offset_nanoseconds: f64,
    pub temperature_c: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetrologySummary {
    pub samples: usize,
    pub mean_offset_ns: f64,
    pub jitter_stddev_ns: f64,
    pub max_abs_offset_ns: f64,
    pub fitted_drift_ppm: f64,
}

pub fn summarize(samples: &[PpsSample]) -> Option<MetrologySummary> {
    if samples.len() < 2
        || samples.iter().any(|s| !s.elapsed_seconds.is_finite() || !s.offset_nanoseconds.is_finite())
    {
        return None;
    }
    let n = samples.len() as f64;
    let mean_y = samples.iter().map(|s| s.offset_nanoseconds).sum::<f64>() / n;
    let mean_x = samples.iter().map(|s| s.elapsed_seconds).sum::<f64>() / n;
    let var = samples
        .iter()
        .map(|s| (s.offset_nanoseconds - mean_y).powi(2))
        .sum::<f64>()
        / n;
    let cov = samples
        .iter()
        .map(|s| (s.elapsed_seconds - mean_x) * (s.offset_nanoseconds - mean_y))
        .sum::<f64>();
    let xx = samples
        .iter()
        .map(|s| (s.elapsed_seconds - mean_x).powi(2))
        .sum::<f64>();
    let slope_ns_per_second = if xx == 0.0 { 0.0 } else { cov / xx };
    let drift_ppm = slope_ns_per_second / 1000.0;
    Some(MetrologySummary {
        samples: samples.len(),
        mean_offset_ns: mean_y,
        jitter_stddev_ns: var.sqrt(),
        max_abs_offset_ns: samples
            .iter()
            .map(|s| s.offset_nanoseconds.abs())
            .fold(0.0, f64::max),
        fitted_drift_ppm: drift_ppm,
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
        || points.iter().any(|p| !p.elapsed_seconds.is_finite() || !p.absolute_error_seconds.is_finite() || p.absolute_error_seconds < 0.0)
    {
        return None;
    }
    Some(points.iter().map(|p| p.absolute_error_seconds).fold(0.0, f64::max))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_offset_growth_recovers_frequency_error() {
        let samples = (0..100)
            .map(|i| PpsSample {
                elapsed_seconds: i as f64,
                offset_nanoseconds: 2_000.0 * i as f64,
                temperature_c: Some(25.0),
            })
            .collect::<Vec<_>>();
        let s = summarize(&samples).unwrap();
        assert!((s.fitted_drift_ppm - 2.0).abs() < 1e-12);
    }

    #[test]
    fn holdover_reports_worst_measured_error() {
        assert_eq!(
            worst_holdover_error(&[
                HoldoverPoint { elapsed_seconds: 3600.0, absolute_error_seconds: 0.01 },
                HoldoverPoint { elapsed_seconds: 86400.0, absolute_error_seconds: 0.20 },
            ]),
            Some(0.20)
        );
    }
}
