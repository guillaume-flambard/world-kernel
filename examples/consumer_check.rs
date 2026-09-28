//! Check 1 of [ADR-005](../../docs/ADR-005-reduce-to-the-representation.md): can a consumer outside
//! `tests/` reach the right transfer decision from the published representation alone?
//!
//! ```bash
//! cargo run --example consumer_check
//! cargo run --example consumer_check -- --render-only
//! ```
//!
//! Three independent measurements per case, declared before the consumer existed in
//! `docs/CONSUMER-CHECK-PROTOCOL.md`:
//!
//! - `expected`, from `experiments/consumer-transfer/expectations.json`, read off the contract;
//! - `consumer`, from the `consumer-transfer` crate, which lives outside `tests/` and may read the
//!   representation, the schema, the contract and sections 1 to 5 of the protocol, and nothing else;
//! - `plan`, from `tests/support/transfer_core.rs`, the procedure ADR-005 moved out of `src/`.
//!
//! All three read the same case file, so the two decisions differ only by the procedure. The reversal
//! condition is literal and is applied by the runner, not chosen after the numbers:
//! `consumer != expected AND plan == expected`.
//!
//! The second command rebuilds `RESULTS.md` from the checked-in `results.json` without measuring
//! anything. The runner performs no external effect and is allowed to report a negative result.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::{Value, json};
use world_kernel::experience::{ContextSnapshot, ExperienceCapsule};

#[path = "../tests/support/mod.rs"]
mod support;

use support::consumer_check::classify;
use support::transfer_core::{TransferStatus, plan_transfer};

const EXPERIMENTS: &str = "experiments/consumer-transfer";
const RESULTS_SCHEMA: &str = "world-consumer-check/v1";
const PROTOCOL: &str = "docs/CONSUMER-CHECK-PROTOCOL.md";

/// One pre-registered case. `deny_unknown_fields` so a case file cannot carry an answer the harness
/// would then pass on.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    id: String,
    capsule: ExperienceCapsule,
    target: ContextSnapshot,
    #[serde(default)]
    declared_adaptations: Vec<DeclaredAdaptation>,
}

/// A target-domain adaptation, as a case file carries it.
///
/// This is a third spelling of the same two fields, and that is the finding rather than an accident: the
/// planner has one, the consumer has to write its own because the Kernel publishes none, and the case file
/// needs one to parse it at all. `tests/support/transfer_core.rs` is left exactly as ADR-005 left it,
/// because its line count is the recorded number the M5 gate scored.
#[derive(Debug, Clone, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct DeclaredAdaptation {
    key: String,
    detail: String,
}

impl DeclaredAdaptation {
    fn to_planner(&self) -> support::transfer_core::DeclaredAdaptation {
        support::transfer_core::DeclaredAdaptation {
            key: self.key.clone(),
            detail: self.detail.clone(),
        }
    }
}

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

    let recorded = build_results(&root)?;
    std::fs::write(
        &results_path,
        format!("{}\n", serde_json::to_string_pretty(&recorded)?),
    )?;
    std::fs::write(&human_path, render_human(&recorded))?;

    let summary = &recorded["summary"];
    println!(
        "{} cases, consumer matches on {}, planner matches on {}",
        summary["cases"], summary["consumerMatchesExpected"], summary["planMatchesExpected"],
    );
    println!(
        "outcome {}: {}",
        summary["outcome"], summary["outcomeReading"]
    );
    println!("wrote {}", results_path.display());
    println!("wrote {}", human_path.display());
    Ok(())
}

fn build_results(root: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let expectations_text =
        std::fs::read_to_string(root.join(EXPERIMENTS).join("expectations.json"))?;
    let expectations: Value = serde_json::from_str(&expectations_text)?;
    let declared: Vec<Value> = expectations["cases"]
        .as_array()
        .expect("the expectations carry a case list")
        .clone();

    let case_paths = sorted_case_paths(root)?;
    assert_eq!(
        case_paths.len(),
        declared.len(),
        "there is one case file per declared case, and no case file is unmeasured"
    );

    let mut rows: Vec<Value> = Vec::new();
    for (path, declaration) in case_paths.iter().zip(&declared) {
        rows.push(measure(root, path, declaration)?);
    }

    let ids_where = |field: &str, is_true: bool| -> Vec<String> {
        rows.iter()
            .filter(|row| row[field] == json!(is_true))
            .map(|row| row["id"].as_str().unwrap_or_default().to_owned())
            .collect()
    };
    let matched = |field: &str| -> usize { ids_where(field, true).len() };

    let consumer_misses = ids_where("consumerMatchesExpected", false);
    let planner_misses = ids_where("planMatchesExpected", false);
    let reversal_cases = ids_where("reversal", true);
    let contested_reversals: Vec<String> = reversal_cases
        .iter()
        .filter(|id| {
            rows.iter()
                .any(|row| row["id"] == json!(id) && row["contested"] == json!(true))
        })
        .cloned()
        .collect();
    let uncontested_reversals: Vec<String> = reversal_cases
        .iter()
        .filter(|id| {
            rows.iter()
                .any(|row| row["id"] == json!(id) && row["contested"] == json!(false))
        })
        .cloned()
        .collect();
    let both_wrong = ids_where("bothMissExpected", true);
    let planner_only_wrong = ids_where("plannerMissesExpectedConsumerAgrees", true);

    // A reversal outranks everything, because it is the one result that puts a procedure back in `src/`.
    let (outcome, reading) = if !reversal_cases.is_empty() {
        (
            "B",
            "the consumer is wrong where the planner is right, so the missing information is located before anything is restored",
        )
    } else if !both_wrong.is_empty() {
        (
            "C",
            "a case the system as a whole gets wrong, which argues for no side of the reduction",
        )
    } else if !planner_only_wrong.is_empty() {
        (
            "D",
            "the representation was enough and the procedure was also wrong, which is an architecture finding rather than a defence of either",
        )
    } else {
        (
            "A",
            "the representation carried every pre-registered decision on its own, outside the test harness, and nothing is reintroduced",
        )
    };

    Ok(json!({
        "schema": RESULTS_SCHEMA,
        "protocol": PROTOCOL,
        "reversalCondition": "consumer(capsule) != expected AND plan_transfer(case) == expected",
        "measurements": {
            "expected": "experiments/consumer-transfer/expectations.json, declared in the protocol before the consumer was written",
            "consumer": "the consumer-transfer crate, outside tests/, reading the representation, the schema, the contract and protocol sections 1 to 5",
            "plan": "tests/support/transfer_core.rs, the procedure ADR-005 moved out of src/",
            "sharedInput": "one case file per case; the consumer receives the re-serialised published capsule, the re-serialised published target and the declared adaptations, and nothing else",
        },
        "summary": {
            "cases": rows.len(),
            "consumerMatchesExpected": matched("consumerMatchesExpected"),
            "planMatchesExpected": matched("planMatchesExpected"),
            "reversalObserved": !reversal_cases.is_empty(),
            "outcome": outcome,
            "outcomeReading": reading,
            "reversalCases": reversal_cases,
            "contestedReversalCases": contested_reversals,
            "reversalCasesOutsideContested": uncontested_reversals,
            "casesWhereTheConsumerMisses": consumer_misses,
            "casesWhereThePlannerMisses": planner_misses,
            "casesWhereBothMiss": both_wrong,
        },
        "cases": rows,
    }))
}

/// Measure one case three ways and record the diagnosis the protocol's branch B asks for.
fn measure(
    root: &Path,
    path: &Path,
    declaration: &Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(path)?;
    let case: Case = serde_json::from_str(&text)?;

    let expected = declaration["expected_status"]
        .as_str()
        .expect("every case declares an expectation");
    assert_eq!(
        case.id,
        declaration["id"].as_str().unwrap_or_default(),
        "a case file and its declaration share an id"
    );

    // The consumer gets the published objects, and nothing else. Its entry point takes three arguments,
    // so it cannot be handed an answer and it cannot tell which case it is looking at.
    let decision = consumer_transfer::decide(
        &serde_json::to_string(&case.capsule)?,
        &serde_json::to_string(&case.target)?,
        &serde_json::to_string(&case.declared_adaptations)?,
    )?;
    let consumer: Value = serde_json::to_value(&decision)?;
    let consumer_status = consumer["status"]
        .as_str()
        .expect("the consumer reports a status")
        .to_owned();

    let plan = plan_transfer(
        &case.capsule,
        &case.target,
        &case
            .declared_adaptations
            .iter()
            .map(DeclaredAdaptation::to_planner)
            .collect::<Vec<_>>(),
    );

    let plan_status = match plan.status {
        TransferStatus::DirectlyReusable => "directly_reusable",
        TransferStatus::AdaptationRequired => "adaptation_required",
        TransferStatus::AdditionalEvidenceRequired => "additional_evidence_required",
        TransferStatus::Incompatible => "incompatible",
        TransferStatus::InsufficientInformation => "insufficient_information",
    }
    .to_owned();

    let plan_different: Vec<String> = plan
        .delta
        .different
        .iter()
        .map(|difference| difference.key.clone())
        .collect();
    let plan_unknown: Vec<String> = plan
        .delta
        .unknown
        .iter()
        .map(|finding| finding.key.clone())
        .chain(
            plan.delta
                .unavailable
                .iter()
                .map(|finding| finding.key.clone()),
        )
        .collect();
    let consumer_different: Vec<String> = consumer["different_keys"]
        .as_array()
        .map(|keys| {
            keys.iter()
                .filter_map(|key| key.as_str())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();
    let consumer_unknown: Vec<String> = consumer["unknown_keys"]
        .as_array()
        .map(|keys| {
            keys.iter()
                .filter_map(|key| key.as_str())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();

    // Branch B asks which information was lost. For every finding only the planner reported, say whether
    // the input carried the key at all: a key the target never declared cannot be read by anybody, while a
    // key the target did declare was available to a consumer that chose not to use it.
    let plan_only: Vec<String> = plan_different
        .iter()
        .chain(plan_unknown.iter())
        .filter(|key| !consumer_different.contains(key) && !consumer_unknown.contains(key))
        .cloned()
        .collect();
    let target_declares = |key: &str| -> Value {
        json!({
            "key": key,
            "inTargetFacts": case.target.facts.contains_key(key),
            "inTargetCapabilities": case.target.capabilities.contains_key(key),
            "declaredAsACondition": case.capsule.applicability_conditions.iter().any(|condition| condition.key == key),
        })
    };

    // The pre-registered condition, applied by the one function that writes it down.
    let verdict = classify(expected, &consumer_status, &plan_status);

    Ok(json!({
        "id": case.id,
        "file": path.strip_prefix(root).expect("a case path is under the root").to_string_lossy(),
        "expected": expected,
        "consumer": consumer_status,
        "plan": plan_status,
        "consumerMatchesExpected": verdict.consumer_matches,
        "planMatchesExpected": verdict.plan_matches,
        "reversal": verdict.reversal,
        "bothMissExpected": verdict.both_miss,
        "plannerMissesExpectedConsumerAgrees": verdict.planner_misses_consumer_agrees,
        "contested": declaration["contested"] == json!(true),
        "contestedReason": declaration["contestedReason"].clone(),
        "findings": {
            "planDifferent": plan_different,
            "planUnknown": plan_unknown,
            "consumerDifferent": consumer_different,
            "consumerUnknown": consumer_unknown,
            "findingsOnlyThePlannerReported": plan_only,
            "inputCarriesThoseKeys": plan_only.iter().map(|key| target_declares(key)).collect::<Vec<_>>(),
        },
        "obligations": {
            "consumer": consumer["obligations"].clone(),
            "plan": {
                "adaptations": plan.required_adaptations.iter().map(|obligation| obligation.key.clone()).collect::<Vec<_>>(),
                "observations": plan.required_observations.iter().map(|obligation| obligation.key.clone()).collect::<Vec<_>>(),
                "assurancesBoundTo": plan.required_assurances.iter().map(|obligation| obligation.bound_to.clone()).collect::<Vec<_>>(),
            },
        },
        "capsuleSignals": {
            "coverage": case.capsule.source_context.coverage,
            "knownFailureKeys": case.capsule.known_failures.iter().map(|failure| failure.condition_key.clone()).collect::<Vec<_>>(),
            "adaptableKeys": case.capsule.applicability_conditions.iter().filter(|condition| condition.adaptable).map(|condition| condition.key.clone()).collect::<Vec<_>>(),
            "dependencies": case.capsule.dependencies,
            "declaredAdaptationKeys": case.declared_adaptations.iter().map(|entry| entry.key.clone()).collect::<Vec<_>>(),
            "targetExtraConstraints": case.target.constraints.keys().cloned().collect::<Vec<_>>(),
        },
    }))
}

fn sorted_case_paths(root: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(root.join(EXPERIMENTS).join("cases"))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .collect();
    paths.sort();
    Ok(paths)
}

fn render_human(recorded: &Value) -> String {
    let summary = &recorded["summary"];
    let mut out = String::new();
    out.push_str("# Consumer check result\n\n");
    out.push_str(&format!(
        "Pre-registered in [{}](../../{}). Outcome **{}**.\n\n",
        "CONSUMER-CHECK-PROTOCOL", PROTOCOL, summary["outcome"]
    ));
    out.push_str(&format!(
        "Reversal condition, applied literally: `{}`\n\n",
        recorded["reversalCondition"]
    ));
    out.push_str(&format!(
        "{} cases. The consumer matched the pre-registered expectation on **{}**, the planner on **{}**.\n\n",
        summary["cases"],
        summary["consumerMatchesExpected"],
        summary["planMatchesExpected"]
    ));
    out.push_str(&format!("**Reading.** {}.\n\n", summary["outcomeReading"]));

    out.push_str("| Case | Expected | Consumer | Planner | Consumer right | Planner right | Contested | Reversal |\n");
    out.push_str("|---|---|---|---|---|---|---|---|\n");
    for row in recorded["cases"].as_array().into_iter().flatten() {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            row["id"].as_str().unwrap_or_default(),
            row["expected"].as_str().unwrap_or_default(),
            row["consumer"].as_str().unwrap_or_default(),
            row["plan"].as_str().unwrap_or_default(),
            mark(&row["consumerMatchesExpected"]),
            mark(&row["planMatchesExpected"]),
            mark(&row["contested"]),
            mark(&row["reversal"]),
        ));
    }
    out.push('\n');

    out.push_str("## Where the two disagreed\n\n");
    let mut any = false;
    for row in recorded["cases"].as_array().into_iter().flatten() {
        if row["consumer"] == row["plan"] {
            continue;
        }
        any = true;
        out.push_str(&format!(
            "- **{}**: consumer `{}`, planner `{}`, expected `{}`. Planner-only findings: {:?}. Input carries: {:?}.\n",
            row["id"].as_str().unwrap_or_default(),
            row["consumer"].as_str().unwrap_or_default(),
            row["plan"].as_str().unwrap_or_default(),
            row["expected"].as_str().unwrap_or_default(),
            row["findings"]["findingsOnlyThePlannerReported"],
            row["findings"]["inputCarriesThoseKeys"],
        ));
    }
    if !any {
        out.push_str("None. On all 16 cases the two procedures reached the same status.\n");
    }
    out.push('\n');

    out.push_str("## Obligations, which a status alone does not carry\n\n");
    out.push_str(
        "A status is a decision, and a decision that names nothing is one nobody can act on. The contract's\n\
         rules 3 and 4 are about obligations rather than about statuses, so a consumer could agree on every\n\
         status in the table above and still drop every obligation a status implies. The four the two\n\
         implementations are compared on, and why each one is in the contract:\n\n\
         - an observation for every key nobody established, because not knowing is work owed;\n\
         - an observation for every key the source consumed, even under a direct reuse, because a reuse\n\
           still has to be re-observed in the target;\n\
         - a target constraint the capsule never declared, recorded rather than dropped;\n\
         - an assurance bound to the target, which is the only assurance in the plan and is never a source\n\
           one.\n\n",
    );
    out.push_str("| Case | Consumer | Planner | Agree |\n|---|---|---|---|\n");
    let mut obligation_disagreements: Vec<(String, Vec<String>, Vec<String>)> = Vec::new();
    for row in recorded["cases"].as_array().into_iter().flatten() {
        let consumer: BTreeSet<String> = obligation_keys(&row["obligations"]["consumer"]);
        let planner: BTreeSet<String> = obligation_keys(&row["obligations"]["plan"]);
        let agrees = consumer == planner;
        if !agrees {
            obligation_disagreements.push((
                row["id"].as_str().unwrap_or_default().to_owned(),
                consumer.difference(&planner).cloned().collect(),
                planner.difference(&consumer).cloned().collect(),
            ));
        }
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            row["id"].as_str().unwrap_or_default(),
            render_keys(&consumer),
            render_keys(&planner),
            if agrees { "yes" } else { "no" },
        ));
    }
    out.push('\n');

    if !obligation_disagreements.is_empty() {
        out.push_str("### The two obligation disagreements, read\n\n");
        out.push_str(
            "The statuses agreed everywhere, so by the pre-registered condition neither of these is a\n\
             reversal. Both are recorded anyway, because an obligation nobody can act on is the failure mode\n\
             a status table hides.\n\n",
        );
        for (id, only_consumer, only_planner) in &obligation_disagreements {
            out.push_str(&format!(
                "- **{id}**: the consumer owes {} and the planner owes {}.\n",
                render_keys(&BTreeSet::from_iter(only_consumer.iter().cloned())),
                render_keys(&BTreeSet::from_iter(only_planner.iter().cloned())),
            ));
        }
        out.push_str(
            "\nOn case 11 the capsule carries `known_failures[0].condition_key`, and both sides reached the\n\
             same status from it. The planner names the key it has to look at again; the consumer owes a\n\
             generic \"a prior failure was recorded under these conditions\" that a target cannot be checked\n\
             against. The information was present in the representation and the consumer did not use it, so\n\
             this is a shortcoming of the consumer and not a gap in what ADR-005 kept.\n\n\
             On case 15 the reverse. The condition is a `predicate_ref`, which is not evaluable by either\n\
             side. The planner files it as unevaluable and owes nothing, and the consumer owes an\n\
             observation of `ledger.tax_rules_version`. The consumer's obligation is the more actionable of\n\
             the two, and whether the planner's silence on an unevaluable condition is correct is a question\n\
             about the contract, not about the representation. It is not a reversal and it is not a defence\n\
             of either side.\n",
        );
    }

    out.push('\n');
    out.push_str("## What this result is worth\n\n");
    out.push_str(
        "The threat section of the protocol was written before the run and is not revised by it. The same\n\
         session read the planner before writing the pre-registration and then wrote the consumer, so a pass\n\
         is necessary and not sufficient, and a second consumer written by somebody who has not read `tests/`\n\
         is the check that would make it worth more.\n\n",
    );
    out.push_str(
        "A reversal would have been strong evidence. Nothing here establishes that the representation is\n\
         valuable; it establishes only what it did or did not decide on its own.\n\n",
    );
    out.push_str(
        "The comparison is known to be able to fail: `the_comparison_detects_a_wrong_consumer` runs a\n\
         consumer that keeps no unknown state, reads an unobserved key as agreement and forgets the recorded\n\
         failures, and the pre-registered condition catches it on the nine cases whose status turns on a\n\
         state it cannot represent. It agrees on case 07, case 15 and case 16 while still being wrong, which\n\
         is recorded in the test rather than counted as evidence.\n",
    );
    out
}

/// The keys one side owes work on, as a comparable set.
///
/// The two sides record their obligations in different shapes, because they are two programs: the consumer
/// writes one flat list of `verb:key` strings, the planner writes three lists of typed obligations. Only
/// the key has to match, not the sentence or the list it came from, or the comparison would be measuring
/// the two formats rather than the two decisions.
fn obligation_keys(obligations: &Value) -> BTreeSet<String> {
    let lists: Vec<&Value> = match obligations {
        // The consumer: a flat list of strings.
        Value::Array(_) => vec![obligations],
        // The planner: adaptations, observations, and the targets the assurances are bound to.
        Value::Object(_) => ["adaptations", "observations", "assurancesBoundTo"]
            .iter()
            .filter_map(|field| obligations.get(field))
            .collect(),
        _ => Vec::new(),
    };
    let mut keys: BTreeSet<String> = BTreeSet::new();
    for list in lists {
        for value in list.as_array().into_iter().flatten() {
            let text = value.as_str().unwrap_or_default().trim();
            // The consumer writes `verb:key` and the planner's three lists contribute the key alone. Only
            // the key has to match, not the sentence around it, or the comparison would be measuring the two
            // formats rather than the two decisions. The verbs are fixed, so stripping them is mechanical
            // and a verb nobody knows about is left in place rather than silently half-stripped.
            let key = [
                "adapt:",
                "observe:",
                "assure:",
                "undeclared-target-constraint:",
            ]
            .iter()
            .find_map(|verb| text.strip_prefix(verb))
            .unwrap_or(text)
            .to_owned();
            if !key.is_empty() {
                keys.insert(key);
            }
        }
    }
    keys
}

fn render_keys(keys: &BTreeSet<String>) -> String {
    if keys.is_empty() {
        return "none".to_owned();
    }
    keys.iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(", ")
}

fn mark(value: &Value) -> &'static str {
    if value == &json!(true) { "yes" } else { "no" }
}
