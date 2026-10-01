use mtime_core::{Provenance, QualityClass};
use mtime_cosmology::{infer_age, planck_2018_flat_reference};
use mtime_explain::{explain_difference, TemporalResolution};
use mtime_hijri::{computed_action, CalendarProfile, HijriAstronomicalState, GeometrySemantics};

fn main() {
    let astronomy = HijriAstronomicalState {
        conjunction_jd_tt: None,
        sunset_jd_ut1: None,
        moon_altitude_topocentric_deg: 3.1,
        elongation_geocentric_deg: 6.5,
        geometry_semantics: GeometrySemantics::mabims_required(),
        moon_age_hours: Some(18.0),
        moon_lag_minutes: Some(35.0),
        site_id: "DEMO-JAKARTA".into(),
        ephemeris_source: "demo fixture — replace with validated ephemeris".into(),
        quality: QualityClass::Approximate,
    };

    let pa = CalendarProfile::mabims_indonesia_2026();
    let ra = pa.evaluate(&astronomy);

    let mut pb = CalendarProfile::mabims_indonesia_2026();
    pb.id = "DEMO_STRICT_PROFILE";
    pb.clauses[0].threshold = 5.0;
    let rb = pb.evaluate(&astronomy);

    let a = TemporalResolution {
        id: "A".into(),
        astronomy: astronomy.clone(),
        computed_action: computed_action(&ra),
        criterion: ra,
        observations: vec![],
        observation_summary: None,
        authority: None,
    };
    let b = TemporalResolution {
        id: "B".into(),
        astronomy,
        computed_action: computed_action(&rb),
        criterion: rb,
        observations: vec![],
        observation_summary: None,
        authority: None,
    };

    let diff = explain_difference(&a, &b);
    let age = infer_age(
        planck_2018_flat_reference(),
        1e-9,
        Provenance::new("Planck 2018 reference parameters"),
    )
    .expect("cosmology demo");

    println!("M-Time v0.1 demo");
    println!("MABIMS computed action: {:?}", a.computed_action);
    println!("ExplainDifference: {}", diff.explanation);
    println!(
        "Planck-like cosmic age inference: {:.6} Gyr [model-dependent]",
        age.gyr
    );
}
