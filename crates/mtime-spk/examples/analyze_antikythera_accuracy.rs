use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use mtime_antikythera::{
    shortest_angle_deg, wrap_deg, AntikytheraMachine, AntikytheraState,
    ANOMALISTIC_MONTH, J2000_JD_TT, MEAN_SYNODIC_MONTH,
};
use mtime_astro::{icrf_vector_to_mean_ecliptic_of_date, Body};
use mtime_core::{CoordinateTime, Tt};
use mtime_spk::SpkEphemeris;
use mtime_timescales::{tt_to_tdb, DtrProvider, NasaSimpleDtr};

const START_YEAR: i32 = 1900;
const END_YEAR: i32 = 2100;
const PHASE_BINS: usize = 24;

#[derive(Debug, Clone, Copy)]
struct ReferenceState {
    sun_deg: f64,
    moon_deg: f64,
    phase_deg: f64,
}

#[derive(Debug, Clone, Copy)]
struct Residuals {
    absolute_sun_deg: f64,
    absolute_moon_deg: f64,
    phase_deg: f64,
    dynamic_sun_deg: f64,
    dynamic_moon_deg: f64,
}

#[derive(Default)]
struct Stats {
    values: Vec<f64>,
}

impl Stats {
    fn push(&mut self, value: f64) {
        assert!(value.is_finite());
        self.values.push(value);
    }

    fn count(&self) -> usize {
        self.values.len()
    }

    fn percentile(&self, q: f64) -> f64 {
        assert!(!self.values.is_empty());
        let mut values = self.values.clone();
        values.sort_by(f64::total_cmp);
        let rank = ((values.len() - 1) as f64 * q).round() as usize;
        values[rank]
    }

    fn mean(&self) -> f64 {
        self.values.iter().sum::<f64>() / self.values.len() as f64
    }

    fn max(&self) -> f64 {
        self.values
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
    }
}

#[derive(Default)]
struct MetricSet {
    historical_absolute_sun: Stats,
    historical_absolute_moon: Stats,
    historical_phase: Stats,
    historical_dynamic_sun: Stats,
    historical_dynamic_moon: Stats,
    digital_absolute_sun: Stats,
    digital_absolute_moon: Stats,
    digital_phase: Stats,
    digital_dynamic_sun: Stats,
    digital_dynamic_moon: Stats,
}

impl MetricSet {
    fn push(&mut self, historical: Residuals, digital: Residuals) {
        self.historical_absolute_sun.push(historical.absolute_sun_deg);
        self.historical_absolute_moon.push(historical.absolute_moon_deg);
        self.historical_phase.push(historical.phase_deg);
        self.historical_dynamic_sun.push(historical.dynamic_sun_deg);
        self.historical_dynamic_moon.push(historical.dynamic_moon_deg);
        self.digital_absolute_sun.push(digital.absolute_sun_deg);
        self.digital_absolute_moon.push(digital.absolute_moon_deg);
        self.digital_phase.push(digital.phase_deg);
        self.digital_dynamic_sun.push(digital.dynamic_sun_deg);
        self.digital_dynamic_moon.push(digital.dynamic_moon_deg);
    }

    fn named(&self) -> [(&'static str, &Stats); 10] {
        [
            ("historical_absolute_sun_deg", &self.historical_absolute_sun),
            ("historical_absolute_moon_deg", &self.historical_absolute_moon),
            ("historical_phase_deg", &self.historical_phase),
            ("historical_dynamic_sun_deg", &self.historical_dynamic_sun),
            ("historical_dynamic_moon_deg", &self.historical_dynamic_moon),
            ("digital_absolute_sun_deg", &self.digital_absolute_sun),
            ("digital_absolute_moon_deg", &self.digital_absolute_moon),
            ("digital_phase_deg", &self.digital_phase),
            ("digital_dynamic_sun_deg", &self.digital_dynamic_sun),
            ("digital_dynamic_moon_deg", &self.digital_dynamic_moon),
        ]
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let spk_path = args
        .next()
        .expect("usage: analyze_antikythera_accuracy <de440s.bsp> <output-dir>");
    let output_dir = args.next().expect("output directory");
    fs::create_dir_all(&output_dir).expect("create output directory");

    let bytes = fs::read(spk_path).expect("read DE440");
    let eph = SpkEphemeris::from_bytes(&bytes).expect("parse DE440");
    let historical = AntikytheraMachine::historical();
    let digital = AntikytheraMachine::digital();

    let reference_base = reference_state(&eph, J2000_JD_TT);
    let historical_base = historical.state_at_tt(J2000_JD_TT).unwrap();
    let digital_base = digital.state_at_tt(J2000_JD_TT).unwrap();

    let monthly_path = Path::new(&output_dir).join("monthly-1900-2100.csv");
    let targeted_path = Path::new(&output_dir).join("targeted-phases.csv");
    let phase_bins_path = Path::new(&output_dir).join("phase-bins.csv");
    let summary_path = Path::new(&output_dir).join("summary.csv");

    let mut monthly = csv_writer(&monthly_path);
    writeln!(
        monthly,
        "year,month,jd_tt,jpl_sun_deg,jpl_moon_deg,jpl_phase_deg,historical_abs_sun_deg,historical_abs_moon_deg,historical_phase_deg,historical_dyn_sun_deg,historical_dyn_moon_deg,digital_abs_sun_deg,digital_abs_moon_deg,digital_phase_deg,digital_dyn_sun_deg,digital_dyn_moon_deg,synodic_phase,anomalistic_phase,draconic_phase"
    )
    .unwrap();

    let mut metrics = MetricSet::default();
    let mut synodic_bins = empty_bins();
    let mut anomalistic_bins = empty_bins();
    let mut draconic_bins = empty_bins();

    let mut monthly_count = 0usize;
    for year in START_YEAR..=END_YEAR {
        for month in 1..=12 {
            let jd_tt = gregorian_to_jd(year, month, 1, 12.0);
            let reference = reference_state(&eph, jd_tt);
            let h = historical.state_at_tt(jd_tt).unwrap();
            let d = digital.state_at_tt(jd_tt).unwrap();
            let hr = residuals(reference, reference_base, h, historical_base);
            let dr = residuals(reference, reference_base, d, digital_base);
            metrics.push(hr, dr);

            add_to_bin(&mut synodic_bins, d.synodic_phase, dr.dynamic_moon_deg);
            add_to_bin(
                &mut anomalistic_bins,
                d.anomalistic_phase,
                dr.dynamic_moon_deg,
            );
            add_to_bin(&mut draconic_bins, d.draconic_phase, dr.dynamic_moon_deg);

            writeln!(
                monthly,
                "{year},{month},{jd_tt:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.12},{:.12},{:.12}",
                reference.sun_deg,
                reference.moon_deg,
                reference.phase_deg,
                hr.absolute_sun_deg,
                hr.absolute_moon_deg,
                hr.phase_deg,
                hr.dynamic_sun_deg,
                hr.dynamic_moon_deg,
                dr.absolute_sun_deg,
                dr.absolute_moon_deg,
                dr.phase_deg,
                dr.dynamic_sun_deg,
                dr.dynamic_moon_deg,
                d.synodic_phase,
                d.anomalistic_phase,
                d.draconic_phase,
            )
            .unwrap();
            monthly_count += 1;
        }
    }

    let start_jd = gregorian_to_jd(START_YEAR, 1, 1, 0.0);
    let end_jd = gregorian_to_jd(END_YEAR + 1, 1, 1, 0.0);
    let mut targeted = csv_writer(&targeted_path);
    writeln!(
        targeted,
        "category,target_phase,jd_tt,jpl_phase_deg,digital_synodic_phase,digital_anomalistic_phase,digital_draconic_phase,digital_abs_sun_deg,digital_abs_moon_deg,digital_phase_error_deg,digital_dyn_sun_deg,digital_dyn_moon_deg"
    )
    .unwrap();

    let mut targeted_count = 0usize;
    targeted_count += emit_periodic_targets(
        &eph,
        &digital,
        digital_base,
        reference_base,
        &mut targeted,
        "lunar_quarter",
        MEAN_SYNODIC_MONTH.period_days,
        297.850_192_1 / 360.0,
        &[0.0, 0.25, 0.5, 0.75],
        start_jd,
        end_jd,
    );
    targeted_count += emit_periodic_targets(
        &eph,
        &digital,
        digital_base,
        reference_base,
        &mut targeted,
        "lunar_perigee_apogee",
        ANOMALISTIC_MONTH.period_days,
        134.963_396_4 / 360.0,
        &[0.0, 0.5],
        start_jd,
        end_jd,
    );
    targeted_count += emit_solar_anomaly_targets(
        &eph,
        &digital,
        digital_base,
        reference_base,
        &mut targeted,
        start_jd,
        end_jd,
    );

    let mut phase_bins = csv_writer(&phase_bins_path);
    writeln!(
        phase_bins,
        "dimension,bin_index,phase_start,phase_end,count,mean_dynamic_moon_error_deg,p95_dynamic_moon_error_deg,max_dynamic_moon_error_deg"
    )
    .unwrap();
    write_bins(&mut phase_bins, "synodic", &synodic_bins);
    write_bins(&mut phase_bins, "anomalistic", &anomalistic_bins);
    write_bins(&mut phase_bins, "draconic", &draconic_bins);

    let mut summary = csv_writer(&summary_path);
    writeln!(summary, "metric,count,mean,p50,p95,p99,max").unwrap();
    for (name, stats) in metrics.named() {
        writeln!(
            summary,
            "{name},{},{:.9},{:.9},{:.9},{:.9},{:.9}",
            stats.count(),
            stats.mean(),
            stats.percentile(0.50),
            stats.percentile(0.95),
            stats.percentile(0.99),
            stats.max(),
        )
        .unwrap();
    }

    println!("===== M6 ANTYKITHERA ACCURACY PROGRAM I =====");
    println!("range={START_YEAR}-{END_YEAR}");
    println!("monthly_epochs={monthly_count}");
    println!("targeted_epochs={targeted_count}");
    for (name, stats) in metrics.named() {
        println!("{name}_p50={:.9}", stats.percentile(0.50));
        println!("{name}_p95={:.9}", stats.percentile(0.95));
        println!("{name}_p99={:.9}", stats.percentile(0.99));
        println!("{name}_max={:.9}", stats.max());
    }
    println!("monthly_csv={}", monthly_path.display());
    println!("targeted_csv={}", targeted_path.display());
    println!("phase_bins_csv={}", phase_bins_path.display());
    println!("summary_csv={}", summary_path.display());
    print_worst_bin("synodic", &synodic_bins);
    print_worst_bin("anomalistic", &anomalistic_bins);
    print_worst_bin("draconic", &draconic_bins);

    assert_eq!(monthly_count, ((END_YEAR - START_YEAR + 1) * 12) as usize);
    assert!(targeted_count > 10_000);
    assert_eq!(metrics.digital_dynamic_moon.count(), monthly_count);
}

fn csv_writer(path: &Path) -> BufWriter<File> {
    BufWriter::new(File::create(path).expect("create CSV"))
}

fn empty_bins() -> Vec<Stats> {
    (0..PHASE_BINS).map(|_| Stats::default()).collect()
}

fn add_to_bin(bins: &mut [Stats], phase: f64, value: f64) {
    let index = ((phase.rem_euclid(1.0) * bins.len() as f64).floor() as usize).min(bins.len() - 1);
    bins[index].push(value);
}

fn print_worst_bin(dimension: &str, bins: &[Stats]) {
    let (index, stats) = bins
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.max().total_cmp(&b.max()))
        .expect("phase bins");
    let start = index as f64 / bins.len() as f64;
    let end = (index + 1) as f64 / bins.len() as f64;
    println!("{dimension}_worst_bin_index={index}");
    println!("{dimension}_worst_bin_phase_start={start:.12}");
    println!("{dimension}_worst_bin_phase_end={end:.12}");
    println!("{dimension}_worst_bin_count={}", stats.count());
    println!("{dimension}_worst_bin_mean_deg={:.9}", stats.mean());
    println!("{dimension}_worst_bin_p95_deg={:.9}", stats.percentile(0.95));
    println!("{dimension}_worst_bin_max_deg={:.9}", stats.max());
}

fn write_bins(writer: &mut impl Write, dimension: &str, bins: &[Stats]) {
    for (index, stats) in bins.iter().enumerate() {
        let start = index as f64 / bins.len() as f64;
        let end = (index + 1) as f64 / bins.len() as f64;
        writeln!(
            writer,
            "{dimension},{index},{start:.12},{end:.12},{},{:.9},{:.9},{:.9}",
            stats.count(),
            stats.mean(),
            stats.percentile(0.95),
            stats.max(),
        )
        .unwrap();
    }
}

fn emit_periodic_targets(
    eph: &SpkEphemeris<'_>,
    machine: &AntikytheraMachine,
    machine_base: AntikytheraState,
    reference_base: ReferenceState,
    writer: &mut impl Write,
    category: &str,
    period_days: f64,
    epoch_phase: f64,
    targets: &[f64],
    start_jd: f64,
    end_jd: f64,
) -> usize {
    let mut count = 0usize;
    for &target in targets {
        let seed_elapsed = (target - epoch_phase).rem_euclid(1.0) * period_days;
        let mut k = ((start_jd - (J2000_JD_TT + seed_elapsed)) / period_days).floor() as i64 - 1;
        loop {
            let jd_tt = J2000_JD_TT + seed_elapsed + k as f64 * period_days;
            k += 1;
            if jd_tt < start_jd {
                continue;
            }
            if jd_tt >= end_jd {
                break;
            }
            write_target_row(
                eph,
                machine,
                machine_base,
                reference_base,
                writer,
                category,
                target,
                jd_tt,
            );
            count += 1;
        }
    }
    count
}

fn emit_solar_anomaly_targets(
    eph: &SpkEphemeris<'_>,
    machine: &AntikytheraMachine,
    machine_base: AntikytheraState,
    reference_base: ReferenceState,
    writer: &mut impl Write,
    start_jd: f64,
    end_jd: f64,
) -> usize {
    const SOLAR_ANOMALY_RATE_DEG_PER_DAY: f64 = 0.985_600_28;
    const SOLAR_ANOMALY_EPOCH_DEG: f64 = 357.529_11;
    let period_days = 360.0 / SOLAR_ANOMALY_RATE_DEG_PER_DAY;
    emit_periodic_targets(
        eph,
        machine,
        machine_base,
        reference_base,
        writer,
        "solar_perihelion_aphelion",
        period_days,
        SOLAR_ANOMALY_EPOCH_DEG / 360.0,
        &[0.0, 0.5],
        start_jd,
        end_jd,
    )
}

fn write_target_row(
    eph: &SpkEphemeris<'_>,
    machine: &AntikytheraMachine,
    machine_base: AntikytheraState,
    reference_base: ReferenceState,
    writer: &mut impl Write,
    category: &str,
    target: f64,
    jd_tt: f64,
) {
    let reference = reference_state(eph, jd_tt);
    let state = machine.state_at_tt(jd_tt).unwrap();
    let r = residuals(reference, reference_base, state, machine_base);
    writeln!(
        writer,
        "{category},{target:.6},{jd_tt:.9},{:.9},{:.12},{:.12},{:.12},{:.9},{:.9},{:.9},{:.9},{:.9}",
        reference.phase_deg,
        state.synodic_phase,
        state.anomalistic_phase,
        state.draconic_phase,
        r.absolute_sun_deg,
        r.absolute_moon_deg,
        r.phase_deg,
        r.dynamic_sun_deg,
        r.dynamic_moon_deg,
    )
    .unwrap();
}

fn residuals(
    reference: ReferenceState,
    reference_base: ReferenceState,
    model: AntikytheraState,
    model_base: AntikytheraState,
) -> Residuals {
    Residuals {
        absolute_sun_deg: shortest_angle_deg(model.solar_longitude.angle_deg, reference.sun_deg),
        absolute_moon_deg: shortest_angle_deg(model.lunar_longitude.angle_deg, reference.moon_deg),
        phase_deg: shortest_angle_deg(model.lunar_phase.angle_deg, reference.phase_deg),
        dynamic_sun_deg: shortest_angle_deg(
            wrap_deg(model.solar_longitude.angle_deg - model_base.solar_longitude.angle_deg),
            wrap_deg(reference.sun_deg - reference_base.sun_deg),
        ),
        dynamic_moon_deg: shortest_angle_deg(
            wrap_deg(model.lunar_longitude.angle_deg - model_base.lunar_longitude.angle_deg),
            wrap_deg(reference.moon_deg - reference_base.moon_deg),
        ),
    }
}

fn reference_state(eph: &SpkEphemeris<'_>, jd_tt: f64) -> ReferenceState {
    let tt = CoordinateTime::<Tt>::new(jd_tt, 0.0, 0.0).unwrap();
    let dtr = NasaSimpleDtr.dtr(&tt).unwrap();
    let tdb = tt_to_tdb(&tt, dtr).unwrap();
    let p = tdb.jd_parts();
    let tt_parts = tt.jd_parts();
    let sun = icrf_vector_to_mean_ecliptic_of_date(
        eph.geocentric_vector_km(Body::Sun, (p.d1, p.d2)).unwrap(),
        (tt_parts.d1, tt_parts.d2),
    )
    .unwrap();
    let moon = icrf_vector_to_mean_ecliptic_of_date(
        eph.geocentric_vector_km(Body::Moon, (p.d1, p.d2)).unwrap(),
        (tt_parts.d1, tt_parts.d2),
    )
    .unwrap();
    ReferenceState {
        sun_deg: sun.longitude_deg,
        moon_deg: moon.longitude_deg,
        phase_deg: wrap_deg(moon.longitude_deg - sun.longitude_deg),
    }
}

fn gregorian_to_jd(year: i32, month: u32, day: u32, hour: f64) -> f64 {
    let (y, m) = if month <= 2 {
        (year - 1, month as i32 + 12)
    } else {
        (year, month as i32)
    };
    let a = (y as f64 / 100.0).floor() as i32;
    let b = 2 - a + a / 4;
    (365.25 * (y + 4_716) as f64).floor()
        + (30.6001 * (m + 1) as f64).floor()
        + day as f64
        + b as f64
        - 1_524.5
        + hour / 24.0
}
