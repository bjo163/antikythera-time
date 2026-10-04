use std::{env, fs, process};

use mtime_metrology::{
    parse_measurement_csv, summarize, MeasurementIdentity, MetrologySummary,
};

const REPORT_SCHEMA: &str = "mtime-mclock-metrology-suite-1";

struct DatasetReport {
    label: &'static str,
    source_csv: String,
    minimum_duration_seconds: f64,
    summary: MetrologySummary,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("mtime-metrology: {error}");
        process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 8 || args[1] != "suite" {
        return Err(format!(
            "usage: {} suite <analysis-git-sha> <pps.csv> <holdover-1h.csv> <holdover-6h.csv> <holdover-24h.csv> <holdover-72h.csv>",
            args.first().map_or("mtime-metrology", String::as_str)
        ));
    }

    let analysis_git_sha = args[2].trim().to_ascii_lowercase();
    if analysis_git_sha.len() != 40
        || !analysis_git_sha
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err("analysis-git-sha must be 40 lowercase hexadecimal characters".to_owned());
    }

    let specifications = [
        ("pps_latency_jitter", 0.0_f64, args[3].as_str()),
        ("holdover_1h", 3600.0_f64, args[4].as_str()),
        ("holdover_6h", 21600.0_f64, args[5].as_str()),
        ("holdover_24h", 86400.0_f64, args[6].as_str()),
        ("holdover_72h", 259200.0_f64, args[7].as_str()),
    ];

    let mut identity: Option<MeasurementIdentity> = None;
    let mut reports = Vec::new();

    for (label, minimum_duration_seconds, path) in specifications {
        let csv = fs::read_to_string(path)
            .map_err(|error| format!("cannot read {path}: {error}"))?;
        let series =
            parse_measurement_csv(&csv).map_err(|error| format!("{label}: {error}"))?;

        if let Some(expected) = &identity {
            if expected != &series.identity {
                return Err(format!(
                    "{label}: device/reference/firmware identity differs from previous datasets"
                ));
            }
        } else {
            identity = Some(series.identity.clone());
        }

        let summary = summarize(&series.samples)
            .ok_or_else(|| format!("{label}: cannot compute metrology summary"))?;
        if summary.duration_seconds < minimum_duration_seconds {
            return Err(format!(
                "{label}: duration {:.6}s is below required {:.6}s",
                summary.duration_seconds, minimum_duration_seconds
            ));
        }

        reports.push(DatasetReport {
            label,
            source_csv: path.to_owned(),
            minimum_duration_seconds,
            summary,
        });
    }

    let identity = identity.ok_or_else(|| "no measurement datasets supplied".to_owned())?;
    print!("{}", render_report(&analysis_git_sha, &identity, &reports));
    Ok(())
}

fn render_report(
    analysis_git_sha: &str,
    identity: &MeasurementIdentity,
    reports: &[DatasetReport],
) -> String {
    let overall_max_abs_offset_ns = reports
        .iter()
        .map(|report| report.summary.max_abs_offset_ns)
        .fold(0.0, f64::max);
    let overall_max_abs_drift_ppm = reports
        .iter()
        .map(|report| report.summary.fitted_drift_ppm.abs())
        .fold(0.0, f64::max);

    let temperature_min_c = reports
        .iter()
        .filter_map(|report| report.summary.temperature_min_c)
        .reduce(f64::min);
    let temperature_max_c = reports
        .iter()
        .filter_map(|report| report.summary.temperature_max_c)
        .reduce(f64::max);

    let mut output = String::new();
    output.push_str("{\n");
    output.push_str(&format!(
        "  \"schema\": {},\n",
        json_string(REPORT_SCHEMA)
    ));
    output.push_str("  \"analysis_tool\": {\n");
    output.push_str("    \"crate\": \"mtime-metrology\",\n");
    output.push_str(&format!(
        "    \"version\": {},\n",
        json_string(env!("CARGO_PKG_VERSION"))
    ));
    output.push_str(&format!(
        "    \"git_sha\": {}\n",
        json_string(analysis_git_sha)
    ));
    output.push_str("  },\n");
    output.push_str("  \"identity\": {\n");
    output.push_str(&format!(
        "    \"reference_id\": {},\n",
        json_string(&identity.reference_id)
    ));
    output.push_str(&format!(
        "    \"device_id\": {},\n",
        json_string(&identity.device_id)
    ));
    output.push_str(&format!(
        "    \"firmware_sha\": {}\n",
        json_string(&identity.firmware_sha)
    ));
    output.push_str("  },\n");
    output.push_str("  \"method\": {\n");
    output.push_str(
        "    \"drift_fit\": \"ordinary_least_squares_offset_ns_vs_elapsed_seconds\",\n",
    );
    output.push_str(
        "    \"jitter\": \"population_standard_deviation_of_linear_fit_residual_ns\",\n",
    );
    output.push_str(
        "    \"offset_spread\": \"population_standard_deviation_of_raw_offset_ns\"\n",
    );
    output.push_str("  },\n");
    output.push_str("  \"datasets\": [\n");

    for (index, report) in reports.iter().enumerate() {
        output.push_str("    {\n");
        output.push_str(&format!(
            "      \"label\": {},\n",
            json_string(report.label)
        ));
        output.push_str(&format!(
            "      \"source_csv\": {},\n",
            json_string(&report.source_csv)
        ));
        output.push_str(&format!(
            "      \"minimum_duration_seconds\": {},\n",
            number(report.minimum_duration_seconds)
        ));
        output.push_str(&format!(
            "      \"samples\": {},\n",
            report.summary.samples
        ));
        output.push_str(&format!(
            "      \"elapsed_start_seconds\": {},\n",
            number(report.summary.elapsed_start_seconds)
        ));
        output.push_str(&format!(
            "      \"elapsed_end_seconds\": {},\n",
            number(report.summary.elapsed_end_seconds)
        ));
        output.push_str(&format!(
            "      \"duration_seconds\": {},\n",
            number(report.summary.duration_seconds)
        ));
        output.push_str(&format!(
            "      \"mean_offset_ns\": {},\n",
            number(report.summary.mean_offset_ns)
        ));
        output.push_str(&format!(
            "      \"offset_stddev_ns\": {},\n",
            number(report.summary.offset_stddev_ns)
        ));
        output.push_str(&format!(
            "      \"jitter_stddev_ns\": {},\n",
            number(report.summary.jitter_stddev_ns)
        ));
        output.push_str(&format!(
            "      \"fit_residual_stddev_ns\": {},\n",
            number(report.summary.fit_residual_stddev_ns)
        ));
        output.push_str(&format!(
            "      \"max_abs_offset_ns\": {},\n",
            number(report.summary.max_abs_offset_ns)
        ));
        output.push_str(&format!(
            "      \"fitted_intercept_ns\": {},\n",
            number(report.summary.fitted_intercept_ns)
        ));
        output.push_str(&format!(
            "      \"fitted_drift_ppm\": {},\n",
            number(report.summary.fitted_drift_ppm)
        ));
        output.push_str(&format!(
            "      \"temperature_c\": {}\n",
            temperature_json(
                report.summary.temperature_min_c,
                report.summary.temperature_max_c
            )
        ));
        output.push_str(if index + 1 == reports.len() {
            "    }\n"
        } else {
            "    },\n"
        });
    }

    output.push_str("  ],\n");
    output.push_str("  \"suite_summary\": {\n");
    output.push_str(&format!(
        "    \"overall_max_abs_offset_ns\": {},\n",
        number(overall_max_abs_offset_ns)
    ));
    output.push_str(&format!(
        "    \"overall_max_abs_drift_ppm\": {},\n",
        number(overall_max_abs_drift_ppm)
    ));
    output.push_str(&format!(
        "    \"temperature_c\": {}\n",
        temperature_json(temperature_min_c, temperature_max_c)
    ));
    output.push_str("  },\n");
    output.push_str(
        "  \"interpretation\": \"measurement_characterization_only_no_accuracy_class_assigned\"\n",
    );
    output.push_str("}\n");
    output
}

fn temperature_json(minimum: Option<f64>, maximum: Option<f64>) -> String {
    match (minimum, maximum) {
        (Some(minimum), Some(maximum)) => format!(
            "{{\"min_c\":{},\"max_c\":{}}}",
            number(minimum),
            number(maximum)
        ),
        _ => "null".to_owned(),
    }
}

fn number(value: f64) -> String {
    format!("{value:.12}")
}

fn json_string(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + 2);
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character if character.is_control() => {
                output.push_str(&format!("\\u{:04x}", character as u32));
            }
            character => output.push(character),
        }
    }
    output.push('"');
    output
}
