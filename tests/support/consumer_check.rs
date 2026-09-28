//! The check-1 comparison, and a control that proves the comparison can fail.
//!
//! Two things live here rather than in the runner, so that the runner and the test measure the same code.
//!
//! - [`classify`] is the reversal condition, applied literally: a case reverses when the consumer is wrong
//!   and the planner is right. It is the only place that condition is written down.
//! - [`naive_decision`] is a deliberately hurried consumer. It is not a strawman, it is the failure mode the
//!   milestone exists to catch: it treats an unobserved key as agreement, treats an established absence as
//!   satisfying a required value, bridges every difference, forgets the recorded failures and ignores the
//!   coverage claim. A reader should find each of those choices defensible in isolation.
//!
//! Without the control, sixteen correct answers from a consumer prove that the consumer is correct and
//! nothing about whether the measurement would have noticed if it were not.

use world_kernel::experience::{
    CapabilityState, ConditionKind, ContextSnapshot, ExperienceCapsule, Fact,
};

/// What the three measurements say about one case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Verdict {
    pub consumer_matches: bool,
    pub plan_matches: bool,
    /// The pre-registered reversal condition, and nothing else.
    pub reversal: bool,
    pub both_miss: bool,
    pub planner_misses_consumer_agrees: bool,
}

pub fn classify(expected: &str, consumer: &str, plan: &str) -> Verdict {
    let consumer_matches = consumer == expected;
    let plan_matches = plan == expected;
    Verdict {
        consumer_matches,
        plan_matches,
        reversal: !consumer_matches && plan_matches,
        both_miss: !consumer_matches && !plan_matches,
        planner_misses_consumer_agrees: consumer_matches && !plan_matches,
    }
}

/// The hurried consumer. Returns a status in the contract's vocabulary.
///
/// It keeps no unknown state at all. That is the whole shape of it: every finding it can report is a
/// difference, so a condition nobody observed is silently a match and the only way to reach
/// `additional_evidence_required` is a capability nobody checked that the target happens to declare.
pub fn naive_decision(capsule: &ExperienceCapsule, target: &ContextSnapshot) -> &'static str {
    let mut differences = 0usize;
    let mut unchecked_capability = 0usize;

    for condition in &capsule.applicability_conditions {
        // Reads as a list of checks with one shared shape, which is exactly how the mistake gets made.
        if matches!(
            condition.kind,
            ConditionKind::PredicateRef
                | ConditionKind::CapabilityAvailable
                | ConditionKind::CapabilityUnavailable
        ) {
            let state = target.capabilities.get(&condition.key);
            let satisfied = match condition.kind {
                ConditionKind::CapabilityAvailable => {
                    matches!(state, Some(CapabilityState::Available))
                }
                // A capability nobody checked reads as "nothing to complain about".
                _ => !matches!(state, Some(CapabilityState::Available)),
            };
            if !satisfied {
                differences += 1;
            }
            if matches!(state, Some(CapabilityState::NotObserved) | None) {
                unchecked_capability += 1;
            }
            continue;
        }

        match target.facts.get(&condition.key) {
            // A key the target never declared, and a fact nobody has looked at, are read as agreement.
            None | Some(Fact::Unknown) => {}
            // An established absence is read as satisfying whatever was asked for.
            Some(Fact::Absent) => {}
            Some(Fact::Known(value)) => {
                let holds = match condition.kind {
                    ConditionKind::Equals => {
                        value
                            == condition
                                .expected
                                .as_ref()
                                .unwrap_or(&serde_json::Value::Null)
                    }
                    ConditionKind::NotEquals => {
                        value
                            != condition
                                .expected
                                .as_ref()
                                .unwrap_or(&serde_json::Value::Null)
                    }
                    ConditionKind::Present => true,
                    ConditionKind::Absent => false,
                    ConditionKind::Gte => value
                        .as_i64()
                        .zip(
                            condition
                                .expected
                                .as_ref()
                                .and_then(serde_json::Value::as_i64),
                        )
                        .is_some_and(|(actual, bound)| actual >= bound),
                    ConditionKind::Lte => value
                        .as_i64()
                        .zip(
                            condition
                                .expected
                                .as_ref()
                                .and_then(serde_json::Value::as_i64),
                        )
                        .is_some_and(|(actual, bound)| actual <= bound),
                    ConditionKind::MemberOf => condition
                        .expected
                        .as_ref()
                        .and_then(serde_json::Value::as_array)
                        .is_some_and(|members| members.contains(value)),
                    // A predicate reference reads as satisfied, because nothing here can disprove it.
                    ConditionKind::PredicateRef
                    | ConditionKind::CapabilityAvailable
                    | ConditionKind::CapabilityUnavailable => true,
                };
                if !holds {
                    differences += 1;
                }
            }
        }
    }

    if unchecked_capability > 0 {
        // The only unknown the hurried consumer can still report is a capability the target itself says it
        // never checked, and even that it treats as a reason to go and look rather than a reason to stop.
        return "additional_evidence_required";
    }
    if differences > 0 {
        // Every difference is bridgeable, because re-parameterising anything is what the author meant.
        return "adaptation_required";
    }
    // A capsule that will not say how complete it is, and a capsule that says nothing at all, are read as
    // "no objection", and a recorded failure is read as history rather than as a reason to look again.
    let _ = (capsule.source_context.coverage, &capsule.known_failures);
    "directly_reusable"
}
