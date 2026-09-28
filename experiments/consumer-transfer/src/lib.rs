//! An external consumer of the published experience capsule.
//!
//! This crate is not part of the Kernel and does not live in `tests/`. It is the consumer that
//! [ADR-005](../../docs/ADR-005-reduce-to-the-representation.md) claims exists: something outside the test
//! harness that reads a published capsule and decides what transfers.
//!
//! What it is allowed to read is fixed in [CONSUMER-CHECK-PROTOCOL](../../docs/CONSUMER-CHECK-PROTOCOL.md):
//! the published representation, the wire schema, the transfer contract, and sections 1 to 5 of that
//! protocol. Every rule below carries the section it implements, so a reader can check competence without
//! reading the procedure it is being compared against.
//!
//! It is a consumer, not a reimplementation of a planner. Where the contract states a rule it follows it,
//! and where the protocol resolves a silence in the contract it follows that. It never reads a file, never
//! sees a case name, and has no access to any other module of the Kernel.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use world_kernel::experience::{
    ApplicabilityCondition, CapabilityState, ConditionKind, ContextSnapshot, ExperienceCapsule,
    Fact,
};

/// The five outcomes, named and spelled as `docs/TRANSFER-CONTRACT.md` names them (protocol 5.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReuseStatus {
    DirectlyReusable,
    AdaptationRequired,
    AdditionalEvidenceRequired,
    Incompatible,
    InsufficientInformation,
}

/// A transformation the target domain declares it can perform about a known difference.
///
/// The Kernel publishes no such type. The capsule carries `adaptable` per condition, and a target-domain
/// adaptation is passed beside the plan by the caller. That channel is out of band, which protocol 5.3
/// records as a gap in the published surface and case 16 measures.
#[derive(Debug, Clone, Deserialize)]
pub struct DeclaredAdaptation {
    pub key: String,
    #[serde(default)]
    pub detail: String,
}

/// What this consumer concludes, and what it owes. The lists are for diagnosis (protocol 5.5); the status
/// is the decision.
#[derive(Debug, Clone, Serialize)]
pub struct Decision {
    pub status: ReuseStatus,
    pub different_keys: Vec<String>,
    pub unknown_keys: Vec<String>,
    /// Every obligation, as `verb:key` or a sentence, in the order the rules produce them.
    pub obligations: Vec<String>,
}

/// Decide what of `capsule_json` transfers into `target_json`.
///
/// The three arguments are the whole input. There is no fourth argument a caller could pass an answer in,
/// and no way for this function to know which case it is looking at.
pub fn decide(
    capsule_json: &str,
    target_json: &str,
    declared_adaptations_json: &str,
) -> Result<Decision, String> {
    let capsule: ExperienceCapsule =
        serde_json::from_str(capsule_json).map_err(|error| format!("capsule: {error}"))?;
    let target: ContextSnapshot =
        serde_json::from_str(target_json).map_err(|error| format!("target: {error}"))?;
    let declared: Vec<DeclaredAdaptation> = serde_json::from_str(declared_adaptations_json)
        .map_err(|error| format!("declared adaptations: {error}"))?;

    let mut different: Vec<String> = Vec::new();
    let mut unknown: Vec<String> = Vec::new();

    for condition in &capsule.applicability_conditions {
        match resolve(condition, &target) {
            Finding::Matches => {}
            Finding::Differs => different.push(condition.key.clone()),
            Finding::NotEstablished => unknown.push(condition.key.clone()),
        }
    }

    // Protocol 5.3. A difference is bridgeable when the author declared the key adaptable or the target
    // domain declared an adaptation for it. Never inferred from the value, and an adaptation nobody
    // declared is not available however obvious it looks.
    let adaptations: BTreeSet<&str> = declared.iter().map(|entry| entry.key.as_str()).collect();
    let bridgeable = |key: &str| -> bool {
        capsule
            .applicability_conditions
            .iter()
            .any(|condition| condition.key == key && condition.adaptable)
            || adaptations.contains(key)
    };

    // Protocol 5.2 rule 2. A prior failure is relevant when the target establishes the condition it was
    // recorded under. It is not relevant because the experience looks similar, so a capsule whose
    // conditions all match can still refuse a clean reuse.
    let repeated_failure = capsule.known_failures.iter().any(|failure| {
        matches!(
            target.facts.get(&failure.condition_key),
            Some(Fact::Known(value)) if value == &failure.condition_value
        )
    });

    let uncovered = different.iter().any(|key| !bridgeable(key));

    // Protocol 5.2, in the order it fixes. A known violation outranks a known difference, and a known
    // difference outranks a condition nobody has looked at.
    let status = if capsule.applicability_conditions.is_empty()
        || capsule.source_context.coverage
            == world_kernel::experience::ApplicabilityCoverage::Opaque
    {
        ReuseStatus::InsufficientInformation
    } else if repeated_failure {
        ReuseStatus::AdaptationRequired
    } else if uncovered {
        ReuseStatus::Incompatible
    } else if !different.is_empty() {
        ReuseStatus::AdaptationRequired
    } else if !unknown.is_empty() {
        ReuseStatus::AdditionalEvidenceRequired
    } else {
        ReuseStatus::DirectlyReusable
    };

    let mut obligations: Vec<String> = Vec::new();
    // Protocol 5.4. Every known difference owes an adaptation, including a bridgeable one: being
    // re-parameterisable is not the same as already done.
    for key in &different {
        obligations.push(format!("adapt:{key}"));
    }
    if repeated_failure {
        obligations.push("review:repeated-prior-failure".to_owned());
    }
    // Every condition nobody established owes an observation in the target.
    for key in &unknown {
        obligations.push(format!("observe:{key}"));
    }
    // A key the source consumed owes an observation in the target even when every condition holds.
    for key in &capsule.dependencies {
        obligations.push(format!("observe:{key}"));
    }
    // A constraint the target holds and the capsule never declared is recorded, because the source had no
    // reason to know about it. It is not a failure.
    for key in target.constraints.keys() {
        if !capsule
            .applicability_conditions
            .iter()
            .any(|condition| &condition.key == key)
        {
            obligations.push(format!("undeclared-target-constraint:{key}"));
        }
    }
    // Protocol 5.4. Assurance is owed in the target and nothing the source recorded is inherited.
    obligations.push(format!("assure:{}", target.context_id));

    Ok(Decision {
        status,
        different_keys: different,
        unknown_keys: unknown,
        obligations,
    })
}

enum Finding {
    Matches,
    Differs,
    NotEstablished,
}

/// Resolve one declared condition in the target (protocol 5.1).
fn resolve(condition: &ApplicabilityCondition, target: &ContextSnapshot) -> Finding {
    // A predicate reference names a predicate this consumer does not hold, so it cannot be resolved
    // whatever the target says. It is neither a match nor a violation, it is a thing still to establish.
    if condition.kind == ConditionKind::PredicateRef {
        return Finding::NotEstablished;
    }

    // A capability condition is answered from the capability map and never from a fact of the same name.
    if matches!(
        condition.kind,
        ConditionKind::CapabilityAvailable | ConditionKind::CapabilityUnavailable
    ) {
        return match target.capabilities.get(&condition.key) {
            Some(CapabilityState::Available) => match condition.kind {
                ConditionKind::CapabilityAvailable => Finding::Matches,
                _ => Finding::Differs,
            },
            // Known not to exist here, in either spelling.
            Some(CapabilityState::Unavailable) | Some(CapabilityState::Unsupported) => {
                match condition.kind {
                    ConditionKind::CapabilityAvailable => Finding::Differs,
                    _ => Finding::Matches,
                }
            }
            // Nobody checked. Not observed is not unavailable, and it is not a match either.
            Some(CapabilityState::NotObserved) | None => Finding::NotEstablished,
        };
    }

    let Some(fact) = target.facts.get(&condition.key) else {
        // The target never declared this key. An undeclared key and an unknown key are two findings.
        return Finding::NotEstablished;
    };
    let value = match fact {
        Fact::Known(value) => value,
        // Nobody has looked. Not the same as absent.
        Fact::Unknown => return Finding::NotEstablished,
        // Established absence is an answer. It satisfies an `absent` condition and violates a required
        // value, and reading it as "we do not know" would turn a known world into a missing observation.
        Fact::Absent => {
            return match condition.kind {
                ConditionKind::Absent => Finding::Matches,
                _ => Finding::Differs,
            };
        }
    };

    let required = condition
        .expected
        .clone()
        .unwrap_or(serde_json::Value::Null);
    let holds = match condition.kind {
        ConditionKind::Equals => *value == required,
        ConditionKind::NotEquals => *value != required,
        ConditionKind::Present => true,
        ConditionKind::Absent => false,
        // A value that is not a number does not satisfy an order, and is a difference rather than a crash.
        ConditionKind::Gte => number(value)
            .is_some_and(|actual| number(&required).is_some_and(|bound| actual >= bound)),
        ConditionKind::Lte => number(value)
            .is_some_and(|actual| number(&required).is_some_and(|bound| actual <= bound)),
        ConditionKind::MemberOf => required
            .as_array()
            .is_some_and(|members| members.contains(value)),
        ConditionKind::PredicateRef
        | ConditionKind::CapabilityAvailable
        | ConditionKind::CapabilityUnavailable => false,
    };
    if holds {
        Finding::Matches
    } else {
        Finding::Differs
    }
}

fn number(value: &serde_json::Value) -> Option<i64> {
    value.as_i64()
}
