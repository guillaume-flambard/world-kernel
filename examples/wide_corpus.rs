//! ADR-005 reversal condition 2: does a wider corpus separate the competent baseline from the Kernel?
//!
//! ```bash
//! cargo run --example wide_corpus
//! cargo run --example wide_corpus -- --render-only
//! ```
//!
//! Pre-registered in `docs/WIDE-CORPUS-PROTOCOL.md` before the generator existed: the seed, the
//! distributions, the projection onto the baseline's model, the metric, the threshold of 1, the
//! exclusions and the three readings the result can have.
//!
//! Three systems per case. **C** is `plan_transfer`, as ADR-005 left it. **A** is the recorded baseline,
//! frozen by hash and not extended: widening a corpus until a baseline that models two condition kinds
//! fails would be rigging, and it is the obvious way to get a dramatic result. **R** is a new independent
//! oracle over the full published alphabet, which is what turns a disagreement into an attributed one
//! rather than a count.
//!
//! The second command rebuilds `RESULTS.md` from the checked-in `results.json` without running a case.
use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::{Value, json};
use world_kernel::experience::ConditionKind;

#[path = "../tests/support/mod.rs"]
mod support;

use support::transfer_core::{DeclaredAdaptation, TransferStatus, plan_transfer};
use support::transfer_fixture::{ACapsule, ACondition, AValue, OracleStatus, a_gate, a_view};
use support::wide_corpus::{self, GeneratedCase, OracleStatus as PlainStatus, Shape};

const EXPERIMENTS: &str = "experiments/wide-corpus";
const RESULTS_SCHEMA: &str = "world-wide-corpus/v1";
const PROTOCOL: &str = "docs/WIDE-CORPUS-PROTOCOL.md";
/// The frozen baseline, restated here so the artifact states what it was measured against. `tests/
/// wide_corpus.rs` recomputes this hash and fails if the file is not this file.
const BASELINE_A_SHA256: &str = "bc09c10b81896d25e61d502ae3ead3bb71c6d706163beaca5e4f4a8177492ce3";
const KERNEL_PLANNER_SHA256: &str =
    "4533e8df4cb24718c3f4c4ed2df24ac4573ee447af1712a38a688245d3bc270b";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let render_only = std::env::args().any(|argument| argument == "--render-only");
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let results_path = root.join(EXPERIMENTS).join("results.json");
    let human_path = root.join(EXPERIMENTS).join("RESULTS.md");

    if render_only {
        let recorded: Value = serde_json::from_str(&std::fs::read_to_string(&results_path)?)?;
        std::fs::write(&human_path, render_human(&recorded))?;
        println!("wrote {}", human_path.display());
        return Ok(());
    }

    let recorded = build_results();
    std::fs::create_dir_all(results_path.parent().expect("results have a parent"))?;
    std::fs::write(
        &results_path,
        format!("{}\n", serde_json::to_string_pretty(&recorded)?),
    )?;
    std::fs::write(&human_path, render_human(&recorded))?;

    let summary = &recorded["summary"];
    println!(
        "{} cases, {} separations, {} of them on cases the baseline can express",
        summary["cases"], summary["separations"], summary["separationsOnExpressibleCases"],
    );
    println!(
        "outcome {}: {}",
        summary["outcome"], summary["outcomeReading"]
    );
    println!("wrote {}", results_path.display());
    println!("wrote {}", human_path.display());
    Ok(())
}

fn build_results() -> Value {
    let cases = wide_corpus::generate();
    let mut rows: Vec<Value> = Vec::new();

    // Coverage of the generated corpus, so a hole in it is visible rather than inferred. A distribution
    // that never drew a shape would hide a disagreement instead of finding one.
    let mut kind_coverage: BTreeMap<&str, usize> = BTreeMap::new();
    let mut fact_state_coverage: BTreeMap<&str, usize> = BTreeMap::new();
    let mut coverage_claim_coverage: BTreeMap<&str, usize> = BTreeMap::new();
    let mut shape_counts: BTreeMap<&str, usize> = BTreeMap::new();
    let mut shape_counts_by_stratum: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();
    // Per-stratum coverage, because a powered subset that never drew an absent fact, an unknown fact or a
    // capability nobody checked would make a count of zero a second power failure wearing a different hat.
    let mut kind_coverage_by_stratum: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();
    let mut fact_state_by_stratum: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();
    let mut capability_state_by_stratum: BTreeMap<&str, BTreeMap<&str, usize>> = BTreeMap::new();
    let mut separations_by_shape: BTreeMap<&str, usize> = BTreeMap::new();
    let mut attribution: BTreeMap<&str, usize> = BTreeMap::new();
    let mut disagreements: Vec<Value> = Vec::new();

    for case in &cases {
        let adaptations: Vec<DeclaredAdaptation> = case
            .declared_adaptation_keys
            .iter()
            .map(|key| DeclaredAdaptation {
                key: key.clone(),
                detail: format!("the target domain declares it can bridge {key}"),
            })
            .collect();

        let plan = plan_transfer(&case.capsule, &case.target, &adaptations);
        let c = status_name(plan.status);

        let (a, a_detail) = baseline_decision(case);
        let (r, r_detail) = oracle_decision(case);

        for condition in &case.capsule.applicability_conditions {
            *kind_coverage.entry(kind_name(condition.kind)).or_insert(0) += 1;
            *kind_coverage_by_stratum
                .entry(case.stratum.as_str())
                .or_default()
                .entry(kind_name(condition.kind))
                .or_insert(0) += 1;
        }
        for slot in 0..6 {
            let key = format!("k{slot}");
            let state = match case.target.facts.get(&key) {
                Some(world_kernel::experience::Fact::Known(_)) => "known",
                Some(world_kernel::experience::Fact::Absent) => "absent",
                Some(world_kernel::experience::Fact::Unknown) => "unknown",
                None => "undeclared",
            };
            *fact_state_coverage.entry(state).or_insert(0) += 1;
            *fact_state_by_stratum
                .entry(case.stratum.as_str())
                .or_default()
                .entry(state)
                .or_insert(0) += 1;
        }
        for slot in 0..3 {
            let key = format!("c{slot}");
            let state = match case.target.capabilities.get(&key) {
                Some(world_kernel::experience::CapabilityState::Available) => "available",
                Some(world_kernel::experience::CapabilityState::Unavailable) => "unavailable",
                Some(world_kernel::experience::CapabilityState::Unsupported) => "unsupported",
                Some(world_kernel::experience::CapabilityState::NotObserved) => "not_observed",
                None => "undeclared",
            };
            *capability_state_by_stratum
                .entry(case.stratum.as_str())
                .or_default()
                .entry(state)
                .or_insert(0) += 1;
        }
        *coverage_claim_coverage.entry(claim_name(case)).or_insert(0) += 1;
        *shape_counts.entry(case.shape.as_str()).or_insert(0) += 1;
        *shape_counts_by_stratum
            .entry(case.stratum.as_str())
            .or_default()
            .entry(case.shape.as_str())
            .or_insert(0) += 1;

        let separates = c != a;
        if separates {
            *separations_by_shape.entry(case.shape.as_str()).or_insert(0) += 1;
            let with = if r == c {
                "R_with_C"
            } else if r == a {
                "R_with_A"
            } else {
                "R_with_neither"
            };
            *attribution.entry(with).or_insert(0) += 1;
            disagreements.push(json!({
                "id": case.id,
                "shape": case.shape.as_str(),
                "drawn": case.drawn,
                "C": c,
                "A": a,
                "R": r,
                "attribution": with,
                "planFindings": {"different": plan.delta.different.iter().map(|d| d.key.clone()).collect::<Vec<_>>(),
                                 "unknown": plan.delta.unknown.iter().map(|u| u.key.clone()).collect::<Vec<_>>(),
                                 "unavailable": plan.delta.unavailable.iter().map(|u| u.key.clone()).collect::<Vec<_>>()},
                "baselineFindings": a_detail,
                "oracleFindings": r_detail,
            }));
        }

        rows.push(json!({
            "id": case.id,
            "stratum": case.stratum.as_str(),
            "shape": case.shape.as_str(),
            "drawn": case.drawn,
            "C": c,
            "A": a,
            "R": r,
            "separates": separates,
            "C_matches_R": c == r,
            "A_matches_R": a == r,
        }));
    }

    let separations = disagreements.len();
    let expressible_cases = shape_counts
        .get(Shape::Expressible.as_str())
        .copied()
        .unwrap_or(0);
    let on_expressible = separations_by_shape
        .get(Shape::Expressible.as_str())
        .copied()
        .unwrap_or(0);
    // The amendment's own condition: the decisive subset has to be powered, or a count of zero on it is a
    // power failure rather than a pass. 32 is the stratum the amendment declares.
    let powered = expressible_cases >= 32;
    // The three readings, decided by the numbers rather than chosen after them.
    let (outcome, reading) = if !powered {
        (
            "unpowered",
            "the shared subset is still too small for a count of zero to mean anything, so the primary \
             metric was not measured and no reading is claimed",
        )
    } else if separations == 0 {
        (
            "no_separation",
            "the tie survives 96 cases drawn over the whole published alphabet, so the second condition \
             does not fire",
        )
    } else if on_expressible == 0 {
        (
            "separation_is_scope",
            "A and C separate only where the baseline's model cannot carry the case, so the recorded \
             tie was scoped to a narrow corpus. The planner is not vindicated by this, and the reduction \
             is not re-opened on it alone",
        )
    } else {
        (
            "separation_on_expressible_cases",
            "A and C implement the same subset and do not agree on it, which is a real disagreement \
             between two competent implementations rather than an artefact of what the corpus covered",
        )
    };

    let c_matches_r = rows
        .iter()
        .filter(|row| row["C_matches_R"] == json!(true))
        .count();
    let a_matches_r = rows
        .iter()
        .filter(|row| row["A_matches_R"] == json!(true))
        .count();

    json!({
        "schema": RESULTS_SCHEMA,
        "protocol": PROTOCOL,
        "reversalCondition": "at least one case where C and A return a different TransferStatus, over a 96-case corpus",
        "systems": {
            "C": {"name": "the transfer planner", "path": "tests/support/transfer_core.rs", "frozen": true, "sha256": KERNEL_PLANNER_SHA256},
            "A": {"name": "the competent baseline", "path": "tests/support/transfer_fixture.rs", "frozen": true, "notExtended": "a separation is only evidence if the thing being separated was not changed to separate it", "sha256": BASELINE_A_SHA256},
            "R": {"name": "a new independent oracle over the full published alphabet", "path": "tests/support/wide_corpus.rs", "frozen": false, "independence": "plain values, calls nothing in transfer_core, reads no expected value"},
        },
        "corpus": {
            "cases": cases.len(),
            "seed": format!("0x{:016X}", wide_corpus::SEED),
            "strata": {
                "S_shared_subset": wide_corpus::STRATUM_SHARED,
                "R_full_distributions": cases.len() - wide_corpus::STRATUM_SHARED,
                "drawn": shape_counts_by_stratum,
            },
            "bounded": "a flat list of at most four conditions over six keys, no recursion, so a case cannot grow into something undecidable",
            "conditionKindCoverage": kind_coverage,
            "targetFactStateCoverage": fact_state_coverage,
            "coverageClaimCoverage": coverage_claim_coverage,
            "shapeCounts": shape_counts,
            "coverageByStratum": {
                "note": "a powered subset that never drew an absent fact, an unknown fact or a capability nobody checked would make a count of zero on it a second power failure, so the decisive stratum's own coverage is reported",
                "conditionKinds": kind_coverage_by_stratum,
                "targetFactStates": fact_state_by_stratum,
                "targetCapabilityStates": capability_state_by_stratum,
            },
            "exclusions": {"expected": 0, "actual": 0, "note": "every shape the generator can produce has a defined expectation, so no case needed to be dropped to be decidable"},
        },
        "runZero": {
            "what": "the first run, kept verbatim as results-run0-defect.json and RESULTS-run0-defect.md",
            "why": "it drew 96 cases and produced one expressible case, so the primary metric was unpowered. Amendment 1 in the protocol stratified the corpus. The numbers below are transcribed from that run, whose generator no longer exists, which is why the run is kept as a file rather than as prose.",
            "seed": format!("0x{:016X}", wide_corpus::SEED_RUN0),
            "cases": 96,
            "shapeCounts": {"expressible": 1, "projection_lossy": 21, "capability_absent_in_a": 74},
            "separations": 44,
            "separationsOnExpressibleCases": 0,
            "attribution": {"R_with_C": 44},
            "verdict": "defect. The zero on the expressible subset was a power failure and is not read as a pass.",
        },
        "summary": {
            "cases": cases.len(),
            "expressibleCases": expressible_cases,
            "primaryMetricPowered": powered,
            "separations": separations,
            "separationsOnExpressibleCases": on_expressible,
            "separationsByShape": separations_by_shape,
            "attribution": attribution,
            "C_matches_R": c_matches_r,
            "A_matches_R": a_matches_r,
            "outcome": outcome,
            "outcomeReading": reading,
        },
        "disagreements": disagreements,
        "cases": rows,
        "notMeasured": [
            "retrieval: a capsule is handed to both systems, so the discovery problem is still not exercised",
            "a competent baseline extended with the eight condition kinds A does not model, which is the question this run deliberately leaves open rather than answering in the direction that flatters either side",
            "no real consumer and no real transfer: the corpus is generated from declared distributions, not sampled from anything",
            "no B or C baseline using UNI, so no assurance-integration comparison exists",
        ],
    })
}

fn status_name(status: TransferStatus) -> &'static str {
    match status {
        TransferStatus::DirectlyReusable => "directly_reusable",
        TransferStatus::AdaptationRequired => "adaptation_required",
        TransferStatus::AdditionalEvidenceRequired => "additional_evidence_required",
        TransferStatus::Incompatible => "incompatible",
        TransferStatus::InsufficientInformation => "insufficient_information",
    }
}

fn kind_name(kind: ConditionKind) -> &'static str {
    match kind {
        ConditionKind::Equals => "equals",
        ConditionKind::NotEquals => "not_equals",
        ConditionKind::Present => "present",
        ConditionKind::Absent => "absent",
        ConditionKind::Gte => "gte",
        ConditionKind::Lte => "lte",
        ConditionKind::MemberOf => "member_of",
        ConditionKind::CapabilityAvailable => "capability_available",
        ConditionKind::CapabilityUnavailable => "capability_unavailable",
        ConditionKind::PredicateRef => "predicate_ref",
    }
}

fn claim_name(case: &GeneratedCase) -> &'static str {
    use world_kernel::experience::ApplicabilityCoverage;
    match case.capsule.source_context.coverage {
        ApplicabilityCoverage::ClosedDeclared => "closed_declared",
        ApplicabilityCoverage::PartialDeclared => "partial_declared",
        ApplicabilityCoverage::Opaque => "opaque",
    }
}

/// Feed the frozen baseline the same capsule and target, through the projection the protocol fixes.
///
/// The projection is where a separation can come from, so it is stated once here and nowhere else: a
/// capability condition becomes a capability condition, an order becomes the one order A models, and
/// everything else becomes an `equals` on 0. A is given the declared adaptation keys and nothing else,
/// because it has no type for anything else.
fn baseline_decision(case: &GeneratedCase) -> (&'static str, Value) {
    let (facts, capabilities) = a_view(&case.target);
    let capsule = ACapsule {
        id: case.capsule.id.clone(),
        filter_keys: case.capsule.source_context.facts.keys().cloned().collect(),
        conditions: case
            .capsule
            .applicability_conditions
            .iter()
            .filter(|condition| {
                !matches!(
                    condition.kind,
                    ConditionKind::CapabilityAvailable | ConditionKind::CapabilityUnavailable
                )
            })
            .map(|condition| ACondition {
                key: condition.key.clone(),
                kind: match condition.kind {
                    ConditionKind::Gte | ConditionKind::Lte => "gte",
                    _ => "equals",
                },
                value: condition
                    .expected
                    .as_ref()
                    .and_then(serde_json::Value::as_i64)
                    .unwrap_or_default(),
                adaptable: condition.adaptable,
            })
            .collect(),
        capability_conditions: case
            .capsule
            .applicability_conditions
            .iter()
            .filter(|condition| condition.kind == ConditionKind::CapabilityAvailable)
            .map(|condition| condition.key.clone())
            .collect(),
    };
    let adaptation_keys: Vec<&str> = case
        .declared_adaptation_keys
        .iter()
        .map(String::as_str)
        .collect();
    let outcome = a_gate(&capsule, &facts, &capabilities, &adaptation_keys);
    let name = match outcome.status {
        OracleStatus::DirectlyReusable => "directly_reusable",
        OracleStatus::AdaptationRequired => "adaptation_required",
        OracleStatus::AdditionalEvidenceRequired => "additional_evidence_required",
        OracleStatus::Incompatible => "incompatible",
        OracleStatus::InsufficientInformation => "insufficient_information",
    };
    let detail = json!({
        "differingKeys": outcome.differing_keys,
        "unknownKeys": outcome.unknown_keys,
        "uncoveredKeys": outcome.adaptations_needed,
    });
    // The baseline's own view of the facts, kept so a disagreement can be read rather than counted.
    let _ = |value: AValue| value;
    (name, detail)
}

fn oracle_decision(case: &GeneratedCase) -> (&'static str, Value) {
    let plain = wide_corpus::to_plain(case);
    let outcome: wide_corpus::OracleOutcome = wide_corpus::oracle(
        &plain.conditions,
        &plain.facts,
        &plain.capabilities,
        plain.opaque,
        &plain.adaptation_keys,
        &plain.recorded_failures,
    );
    let status: PlainStatus = outcome.status;
    (
        status.as_str(),
        json!({"different": outcome.different, "unknown": outcome.unknown}),
    )
}

fn render_human(recorded: &Value) -> String {
    let summary = &recorded["summary"];
    let corpus = &recorded["corpus"];
    let mut out = String::new();
    out.push_str("# Wide corpus: does a wider corpus separate A from C?\n\n");
    out.push_str(&format!(
        "Pre-registered in [WIDE-CORPUS-PROTOCOL](../../{}).\n\n",
        PROTOCOL
    ));
    out.push_str(&format!(
        "Reversal condition, applied literally: {}\n\n",
        recorded["reversalCondition"]
    ));
    out.push_str(&format!(
        "{} cases from seed `{}`. **Separations: {}.** On expressible cases: **{}**.\n\n",
        corpus["cases"],
        corpus["seed"].as_str().unwrap_or(""),
        summary["separations"],
        summary["separationsOnExpressibleCases"],
    ));
    out.push_str(&format!(
        "**Outcome `{}`.** {}\n\n",
        summary["outcome"], summary["outcomeReading"]
    ));

    out.push_str("## What the corpus actually covers\n\n");
    out.push_str(&format!(
        "Bounded: {}.\n\n| Dimension | Distribution |\n|---|---|\n",
        corpus["bounded"]
    ));
    out.push_str(&format!(
        "| condition kinds drawn | {} |\n| target fact states | {} |\n| coverage claims | {} |\n| case shapes | {} |\n\n",
        render_map(&corpus["conditionKindCoverage"]),
        render_map(&corpus["targetFactStateCoverage"]),
        render_map(&corpus["coverageClaimCoverage"]),
        render_map(&corpus["shapeCounts"]),
    ));
    out.push_str(&format!(
        "Exclusions, expected {} and actual {}. {}\n\n",
        corpus["exclusions"]["expected"],
        corpus["exclusions"]["actual"],
        corpus["exclusions"]["note"],
    ));
    out.push_str(&format!(
        "Strata: {}.\n\nThe decisive stratum's own coverage, so a zero on it cannot be a second power failure: {}\n\n",
        render_map(&corpus["strata"]["drawn"]),
        corpus["coverageByStratum"]["note"].as_str().unwrap_or(""),
    ));
    for (field, label) in [
        ("conditionKinds", "condition kinds"),
        ("targetFactStates", "target fact states"),
        ("targetCapabilityStates", "target capability states"),
    ] {
        out.push_str(&format!(
            "- {label}: {}\n",
            render_nested(&corpus["coverageByStratum"][field])
        ));
    }
    out.push('\n');

    out.push_str("## The three systems\n\n| Id | What | Frozen |\n|---|---|---|\n");
    for (id, system) in recorded["systems"].as_object().into_iter().flatten() {
        out.push_str(&format!(
            "| {} | {} (`{}`) | {} |\n",
            id,
            system["name"].as_str().unwrap_or(""),
            system["path"].as_str().unwrap_or(""),
            if system["frozen"] == json!(true) {
                "yes"
            } else {
                "no"
            },
        ));
    }
    out.push('\n');

    out.push_str("## Separations, and who the independent oracle sides with\n\n");
    let disagreements = recorded["disagreements"]
        .as_array()
        .map(Vec::len)
        .unwrap_or(0);
    if disagreements == 0 {
        out.push_str("None. C and A returned the same status on every case.\n\n");
    } else {
        out.push_str(&format!(
            "{} of them, attributed: {}.\n\n| Case | Shape | C | A | R | Attribution | Kinds |\n|---|---|---|---|---|---|---|\n",
            disagreements,
            render_map(&summary["attribution"]),
        ));
        for row in recorded["disagreements"].as_array().into_iter().flatten() {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                row["id"].as_str().unwrap_or(""),
                row["shape"].as_str().unwrap_or(""),
                row["C"].as_str().unwrap_or(""),
                row["A"].as_str().unwrap_or(""),
                row["R"].as_str().unwrap_or(""),
                row["attribution"].as_str().unwrap_or(""),
                row["drawn"].as_str().unwrap_or(""),
            ));
        }
        out.push('\n');
    }

    out.push_str(&format!(
        "Against the independent oracle, C matched on {} of {} cases and A matched on {}.\n\n",
        summary["C_matches_R"], summary["cases"], summary["A_matches_R"],
    ));

    out.push_str("## Every case\n\n");
    out.push_str("The disagreements are above. This is the rest, so a reader can check the counts against the run rather than taking them.\n\n");
    out.push_str(
        "| Case | Stratum | Shape | C | A | R | Separates |\n|---|---|---|---|---|---|---|\n",
    );
    for row in recorded["cases"].as_array().into_iter().flatten() {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            row["id"].as_str().unwrap_or(""),
            row["stratum"].as_str().unwrap_or(""),
            row["shape"].as_str().unwrap_or(""),
            row["C"].as_str().unwrap_or(""),
            row["A"].as_str().unwrap_or(""),
            row["R"].as_str().unwrap_or(""),
            mark(&row["separates"]),
        ));
    }
    out.push('\n');

    out.push_str("## Not measured\n\n");
    for item in recorded["notMeasured"].as_array().into_iter().flatten() {
        out.push_str(&format!("- {}\n", item.as_str().unwrap_or("")));
    }
    out
}

fn mark(value: &Value) -> &'static str {
    if value == &json!(true) { "yes" } else { "no" }
}

fn render_map(value: &Value) -> String {
    let mut parts: Vec<String> = value
        .as_object()
        .into_iter()
        .flatten()
        .map(|(key, count)| format!("{key} {count}"))
        .collect();
    parts.sort();
    if parts.is_empty() {
        return "none".to_owned();
    }
    parts.join(", ")
}

fn render_nested(value: &Value) -> String {
    let mut parts: Vec<String> = value
        .as_object()
        .into_iter()
        .flatten()
        .map(|(stratum, inner)| format!("{stratum} [{}]", render_map(inner)))
        .collect();
    parts.sort();
    if parts.is_empty() {
        return "none".to_owned();
    }
    parts.join("; ")
}
