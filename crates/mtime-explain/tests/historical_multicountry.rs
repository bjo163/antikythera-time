use std::collections::{BTreeMap, BTreeSet};

use mtime_explain::{
    audit_source_backed_historical_case, PublishedAuthorityAction, PublishedCriterionStatus,
    ReplayVerdict, SourceBackedHistoricalCase,
};
use serde_json::Value;

fn criterion(value: &str) -> PublishedCriterionStatus {
    match value {
        "MET" => PublishedCriterionStatus::Met,
        "NOT_MET" => PublishedCriterionStatus::NotMet,
        "UNKNOWN" => PublishedCriterionStatus::Unknown,
        other => panic!("unknown criterion status: {other}"),
    }
}

fn authority(value: &str) -> PublishedAuthorityAction {
    match value {
        "BEGIN_NEW_MONTH_NEXT_DAY" => PublishedAuthorityAction::BeginNewMonthNextDay,
        "COMPLETE_CURRENT_MONTH_TO_30_DAYS" => {
            PublishedAuthorityAction::CompleteCurrentMonthTo30Days
        }
        other => panic!("unknown authority action: {other}"),
    }
}

fn verdict_name(verdict: ReplayVerdict) -> &'static str {
    match verdict {
        ReplayVerdict::Reproduced => "REPRODUCED",
        ReplayVerdict::Falsified => "FALSIFIED",
        ReplayVerdict::Incomplete => "INCOMPLETE",
    }
}

fn corpus() -> Value {
    serde_json::from_str(include_str!(
        "../../../data/hijri/historical-falsification-v0.11.json"
    ))
    .expect("valid v0.11 historical corpus JSON")
}

#[test]
fn source_backed_multicountry_corpus_matches_expected_verdicts() {
    let data = corpus();
    let cases = data["cases"].as_array().expect("cases array");
    let mut counts = BTreeMap::<&str, usize>::new();
    let mut jurisdictions = BTreeSet::new();
    let mut years = BTreeSet::new();
    let mut real_cases = 0usize;

    for row in cases {
        let id = row["id"].as_str().unwrap();
        let jurisdiction = row["jurisdiction"].as_str().unwrap();
        let event = row["event"].as_str().unwrap();
        let source_url = row["source_url"].as_str().unwrap();
        let expected = row["expected_replay_verdict"].as_str().unwrap();
        let audit = audit_source_backed_historical_case(&SourceBackedHistoricalCase {
            id,
            jurisdiction,
            event,
            source_url,
            criterion_status: criterion(row["criterion_status"].as_str().unwrap()),
            authority_action: authority(row["authority_action"].as_str().unwrap()),
        });

        assert_eq!(
            verdict_name(audit.verdict),
            expected,
            "unexpected verdict for {id}: {:?}",
            audit.reasons
        );
        *counts.entry(expected).or_default() += 1;

        if row["kind"].as_str() == Some("REAL") {
            real_cases += 1;
            jurisdictions.insert(jurisdiction);
            years.insert(row["civil_year"].as_u64().unwrap());
            assert!(
                !source_url.contains("example.invalid"),
                "real case must have an official source"
            );
        }
    }

    assert_eq!(real_cases, 21);
    assert_eq!(jurisdictions, BTreeSet::from(["ID", "MY", "SG"]));
    assert_eq!(years, BTreeSet::from([2024, 2025, 2026]));
    assert_eq!(counts.get("REPRODUCED"), Some(&12));
    assert_eq!(counts.get("FALSIFIED"), Some(&2));
    assert_eq!(counts.get("INCOMPLETE"), Some(&10));

    println!("real_cases={real_cases}");
    println!("jurisdictions={}", jurisdictions.len());
    println!("civil_years={}", years.len());
    println!("reproduced={}", counts["REPRODUCED"]);
    println!("falsified_controls={}", counts["FALSIFIED"]);
    println!("incomplete={}", counts["INCOMPLETE"]);
}

#[test]
fn documented_cross_jurisdiction_date_divergences_are_preserved() {
    let data = corpus();
    let cases = data["cases"].as_array().unwrap();
    let by_id = cases
        .iter()
        .filter_map(|row| row["id"].as_str().map(|id| (id, row)))
        .collect::<BTreeMap<_, _>>();

    for check in data["divergence_checks"].as_array().unwrap() {
        let a = by_id[check["a"].as_str().unwrap()];
        let b = by_id[check["b"].as_str().unwrap()];
        let da = a["official_month_start"].as_str().unwrap();
        let db = b["official_month_start"].as_str().unwrap();
        assert_ne!(da, db, "expected date divergence was collapsed");

        let different_evidence = a["criterion_status"] != b["criterion_status"]
            || a["observation_status"] != b["observation_status"]
            || a["jurisdiction"] != b["jurisdiction"];
        assert!(different_evidence, "date divergence lacks represented causal layers");

        println!(
            "divergence={} {}:{} vs {}:{}",
            check["event"].as_str().unwrap(),
            a["jurisdiction"].as_str().unwrap(),
            da,
            b["jurisdiction"].as_str().unwrap(),
            db
        );
    }
}

#[test]
fn unknown_published_criterion_never_becomes_a_pass() {
    let data = corpus();
    for row in data["cases"].as_array().unwrap() {
        if row["criterion_status"].as_str() != Some("UNKNOWN") {
            continue;
        }
        let audit = audit_source_backed_historical_case(&SourceBackedHistoricalCase {
            id: row["id"].as_str().unwrap(),
            jurisdiction: row["jurisdiction"].as_str().unwrap(),
            event: row["event"].as_str().unwrap(),
            source_url: row["source_url"].as_str().unwrap(),
            criterion_status: PublishedCriterionStatus::Unknown,
            authority_action: authority(row["authority_action"].as_str().unwrap()),
        });
        assert_eq!(audit.verdict, ReplayVerdict::Incomplete);
    }
}
