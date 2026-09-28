//! ADR-005 reversal condition 2, held to four things: the baseline stays frozen, the corpus is
//! stratified so its primary metric can be measured, the decisive subset is actually exercised, and the
//! checked-in result is what a fresh run produces.
//!
//! The first is the one that decides whether the whole experiment means anything. A separation between a
//! planner and a baseline only separates the two as they stood. Widening the corpus until a baseline that
//! models three condition kinds fails would be rigging, and it is the easiest way to get a dramatic
//! number, so the freeze is a hash rather than a promise.

mod support;

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::Value;
use support::wide_corpus::{CASES, SEED, STRATUM_SHARED, Shape, generate};
use world_kernel::digest_of_bytes;

const EXPERIMENTS: &str = "experiments/wide-corpus";

/// The frozen files, as `docs/WIDE-CORPUS-PROTOCOL.md` records them. The protocol states the hash before
/// the corpus was generated, so this is a check against a pre-registered value and not a snapshot taken
/// after the fact.
const BASELINE_A_SHA256: &str = "bc09c10b81896d25e61d502ae3ead3bb71c6d706163beaca5e4f4a8177492ce3";
const BASELINE_A_LINES: usize = 515;
const KERNEL_PLANNER_SHA256: &str =
    "4533e8df4cb24718c3f4c4ed2df24ac4573ee447af1712a38a688245d3bc270b";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn recorded() -> Value {
    let text = std::fs::read_to_string(root().join(EXPERIMENTS).join("results.json"))
        .expect("the result is checked in");
    serde_json::from_str(&text).expect("the result is valid JSON")
}

/// The baseline is frozen, and so is the planner it is compared against.
///
/// If this fails, the run that produced the checked-in result was measured against different code, and
/// every number in the artifact describes a comparison that no longer exists in this repository.
#[test]
fn both_systems_are_the_files_the_protocol_froze() {
    for (path, expected, lines) in [
        (
            "tests/support/transfer_fixture.rs",
            BASELINE_A_SHA256,
            Some(BASELINE_A_LINES),
        ),
        (
            "tests/support/transfer_core.rs",
            KERNEL_PLANNER_SHA256,
            Some(684),
        ),
    ] {
        let text = std::fs::read_to_string(root().join(path)).expect("the file is checked in");
        assert_eq!(
            digest_of_bytes(text.as_bytes()),
            format!("sha256:{expected}"),
            "{path} is not the frozen file. The pre-registration says the separation is only evidence if \
             the thing being separated was not changed to separate it, and extending the baseline with the \
             kinds it does not model would change exactly what is being compared"
        );
        if let Some(lines) = lines {
            assert_eq!(
                text.lines().count(),
                lines,
                "{path} is not the recorded length, so the M5 line count in \
                 experiments/transfer-benchmark no longer describes it"
            );
        }
    }
}

/// The corpus is a function of the declared seed and nothing else, and it has the two declared strata.
///
/// A corpus that drifted from its seed, or that quietly lost the stratum carrying the primary metric,
/// would produce a number that cannot be reproduced by a reader who has the protocol.
#[test]
fn the_corpus_is_the_declared_shape() {
    let first = generate();
    let second = generate();
    assert_eq!(first.len(), CASES, "96 cases, as the protocol declares");
    assert_eq!(
        format!(
            "{:?}",
            first
                .iter()
                .map(|case| case.drawn.clone())
                .collect::<Vec<_>>()
        ),
        format!(
            "{:?}",
            second
                .iter()
                .map(|case| case.drawn.clone())
                .collect::<Vec<_>>()
        ),
        "the same seed has to draw the same corpus twice, or the artifact is not reproducible"
    );

    let shared = first
        .iter()
        .filter(|case| case.stratum == support::wide_corpus::Stratum::Shared)
        .count();
    assert_eq!(
        shared, STRATUM_SHARED,
        "stratum S is the declared {STRATUM_SHARED} cases both systems can express"
    );
    for case in first
        .iter()
        .filter(|case| case.stratum == support::wide_corpus::Stratum::Shared)
    {
        assert_eq!(
            case.shape,
            Shape::Expressible,
            "{}: a stratum S case has to be one both systems can express, or the stratum does not say what \
             it says",
            case.id
        );
        assert!(
            case.drawn.contains("integer"),
            "{}: a text value is a projection the baseline cannot carry",
            case.id
        );
    }
}

/// The decisive subset has to be exercised, or a count of zero on it is a second power failure.
///
/// This is the check that would have caught run 0 in the corpus rather than in a paragraph about it: 96
/// cases, one of them expressible, and a reported zero.
#[test]
fn the_decisive_subset_is_powered_and_exercised() {
    let cases = generate();
    let expressible: Vec<_> = cases
        .iter()
        .filter(|case| case.shape == Shape::Expressible)
        .collect();
    assert!(
        expressible.len() >= STRATUM_SHARED,
        "the primary metric counts disagreements on the expressible subset, so it needs the declared {} \
         cases and not {}: {}",
        STRATUM_SHARED,
        expressible.len(),
        expressible.len()
    );

    // Every fact state and every capability state, in the subset the zero is claimed on. A subset that
    // only ever saw a known fact and an available capability would agree for the wrong reason.
    let mut fact_states: BTreeMap<&str, usize> = BTreeMap::new();
    let mut capability_states: BTreeMap<&str, usize> = BTreeMap::new();
    for case in &expressible {
        for value in case.target.facts.values() {
            let state = match value {
                world_kernel::experience::Fact::Known(_) => "known",
                world_kernel::experience::Fact::Absent => "absent",
                world_kernel::experience::Fact::Unknown => "unknown",
            };
            *fact_states.entry(state).or_insert(0) += 1;
        }
        for state in case.target.capabilities.values() {
            let name = match state {
                world_kernel::experience::CapabilityState::Available => "available",
                world_kernel::experience::CapabilityState::Unavailable => "unavailable",
                world_kernel::experience::CapabilityState::Unsupported => "unsupported",
                world_kernel::experience::CapabilityState::NotObserved => "not_observed",
            };
            *capability_states.entry(name).or_insert(0) += 1;
        }
    }
    for state in ["known", "absent", "unknown"] {
        assert!(
            fact_states.get(state).copied().unwrap_or(0) > 0,
            "the expressible subset never drew a {state} fact, so the zero on it is not the measurement it \
             claims to be"
        );
    }
    for state in ["available", "unavailable", "unsupported", "not_observed"] {
        assert!(
            capability_states.get(state).copied().unwrap_or(0) > 0,
            "the expressible subset never drew a {state} capability, so a gate that cannot tell them apart \
             would still score zero on it"
        );
    }
}

/// The checked-in result is what a fresh run produced, and its verdict is derived from its own numbers.
#[test]
fn the_checked_in_result_is_a_fresh_measurement() {
    let recorded = recorded();
    let summary = &recorded["summary"];
    let cases = recorded["cases"]
        .as_array()
        .expect("the run records its cases");

    assert_eq!(summary["cases"], serde_json::json!(CASES));
    assert_eq!(
        cases.len(),
        CASES,
        "every generated case is measured and recorded"
    );

    let disagreements = recorded["disagreements"].as_array().expect("a list");
    assert_eq!(
        summary["separations"],
        serde_json::json!(disagreements.len()),
        "the count and the list cannot disagree"
    );
    assert_eq!(
        summary["primaryMetricPowered"],
        serde_json::json!(
            summary["expressibleCases"].as_u64().unwrap_or(0) >= STRATUM_SHARED as u64
        ),
        "powered is a function of how many expressible cases were drawn, not of the outcome"
    );

    // The verdict is the protocol's table, computed from the numbers.
    let separations = summary["separations"].as_u64().unwrap_or(0);
    let on_expressible = summary["separationsOnExpressibleCases"]
        .as_u64()
        .unwrap_or(0);
    let powered = summary["primaryMetricPowered"] == serde_json::json!(true);
    let expected_outcome = if !powered {
        "unpowered"
    } else if separations == 0 {
        "no_separation"
    } else if on_expressible == 0 {
        "separation_is_scope"
    } else {
        "separation_on_expressible_cases"
    };
    assert_eq!(
        summary["outcome"],
        serde_json::json!(expected_outcome),
        "the recorded outcome has to be the one the pre-registered derivation gives"
    );

    // Every row agrees with the three systems it records.
    for row in cases {
        let (c, a, r) = (
            row["C"].as_str().expect("C"),
            row["A"].as_str().expect("A"),
            row["R"].as_str().expect("R"),
        );
        assert_eq!(
            row["separates"],
            serde_json::json!(c != a),
            "{}: a separation is C and A differing, and nothing else",
            row["id"]
        );
        assert_eq!(row["C_matches_R"], serde_json::json!(c == r));
        assert_eq!(row["A_matches_R"], serde_json::json!(a == r));
    }
    for row in disagreements {
        let (c, a, r) = (
            row["C"].as_str().expect("C"),
            row["A"].as_str().expect("A"),
            row["R"].as_str().expect("R"),
        );
        let expected = if r == c {
            "R_with_C"
        } else if r == a {
            "R_with_A"
        } else {
            "R_with_neither"
        };
        assert_eq!(
            row["attribution"],
            serde_json::json!(expected),
            "{}: the attribution is derived from the three statuses, so a disagreement is never left \
             unattributed",
            row["id"]
        );
    }
}

/// The defective first run is kept, and the record of it matches the file.
///
/// A defect that is corrected and then deleted is a defect that will be made again, because the next
/// person cannot see that the composition once produced one expressible case in ninety-six.
#[test]
fn the_defective_run_is_kept_and_the_record_matches_it() {
    let path = root().join(EXPERIMENTS).join("results-run0-defect.json");
    assert!(
        path.exists(),
        "run 0 is a recorded run. It is kept under a name that says it was defective, and it is not \
         deleted because the amendment replaced its generator"
    );
    let run_zero: Value = serde_json::from_str(
        &std::fs::read_to_string(&path).expect("the defective run is checked in"),
    )
    .expect("the defective run is valid JSON");

    let recorded = recorded();
    let stated = &recorded["runZero"];
    assert_eq!(
        stated["separations"], run_zero["summary"]["separations"],
        "the run-zero numbers in the artifact are transcribed from the file, and this is what keeps the \
         transcription honest"
    );
    assert_eq!(
        stated["separationsOnExpressibleCases"],
        run_zero["summary"]["separationsOnExpressibleCases"],
        "including the zero that was a power failure rather than a pass"
    );
    assert_eq!(stated["shapeCounts"], run_zero["corpus"]["shapeCounts"]);
    assert_eq!(
        stated["attribution"], run_zero["summary"]["attribution"],
        "run 0 attributed all of its separations the same way, and that is worth keeping next to the \
         amendment"
    );
    assert_ne!(
        run_zero["corpus"]["seed"], recorded["corpus"]["seed"],
        "the two runs are drawn from different seeds, which is what Amendment 1 declares"
    );
    assert_eq!(format!("0x{SEED:016X}"), recorded["corpus"]["seed"]);
}

/// The rendered result must not claim more than the numbers say.
#[test]
fn the_rendered_result_agrees_with_the_json() {
    let recorded = recorded();
    let rendered = std::fs::read_to_string(root().join(EXPERIMENTS).join("RESULTS.md"))
        .expect("the rendered result is checked in");
    let summary = &recorded["summary"];

    for row in recorded["cases"].as_array().expect("cases") {
        assert!(
            rendered.contains(row["id"].as_str().expect("an id")),
            "{} is measured and has to be visible in RESULTS.md",
            row["id"]
        );
    }
    // The counts the document leads with have to be the counts in the json, checked as the exact sentence
    // rather than as a number that might appear somewhere else in the file.
    let expected_headline = format!(
        "**Separations: {}.** On expressible cases: **{}**.",
        summary["separations"], summary["separationsOnExpressibleCases"],
    );
    assert!(
        rendered.contains(&expected_headline),
        "RESULTS.md must carry the sentence \"{expected_headline}\", because that is the number the \
         reversal condition turns on"
    );
    let expected_outcome = format!("**Outcome `{}`.**", summary["outcome"]);
    assert!(
        rendered.contains(&expected_outcome),
        "RESULTS.md must name the recorded outcome, and it has to be the derived one"
    );
    if summary["separations"] == serde_json::json!(0) {
        assert!(
            rendered.contains("None. C and A returned the same status on every case."),
            "a run with no separation says so, rather than leaving an empty table to imply it"
        );
    }
    if !rendered.contains("None. C and A returned the same status on every case.") {
        assert!(
            summary["separations"] != serde_json::json!(0),
            "the document claims no separation while the result records one"
        );
    }
}
