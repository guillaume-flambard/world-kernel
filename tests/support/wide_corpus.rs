//! The wide corpus: a bounded seeded generator, and an independent oracle over the full alphabet.
//!
//! Pre-registered in `docs/WIDE-CORPUS-PROTOCOL.md` before this file existed. The seed, the
//! distributions, the projection onto the baseline's model, the metric and the threshold are all in that
//! document, and none of them is chosen here.
//!
//! Three programs, and the separation between them is the point:
//!
//! - [`generate`] draws 96 cases from the declared distributions. It is bounded: a flat list of at most
//!   four conditions over six keys, no recursion, so a case cannot grow into something undecidable.
//! - [`oracle`] is a second implementation of the published semantics over plain values. It calls nothing
//!   in `transfer_core` and reads no expected value, so a planner bug and a matching oracle bug cannot
//!   cancel out.
//! - The baseline A is the recorded one, frozen by hash in `tests/wide_corpus.rs`, and is not extended
//!   here. A separation is only evidence if the thing being separated was not changed to separate it.

use std::collections::BTreeMap;

use world_kernel::experience::{
    ApplicabilityCondition, ApplicabilityCoverage, CapabilityState, ConditionKind, ContextSnapshot,
    ExperienceCapsule, ExperienceOutcome, Fact, FailureObservation, ProvenanceRecord,
};

/// The seed written in the protocol. The corpus is a function of that number and of nothing else.
///
/// Amendment 1 replaced it: run 0 drew from `SEED_RUN0` and produced one `expressible` case out of 96,
/// which left the primary metric unpowered. `SEED` is the amended draw, two declared strata in a fixed
/// order. The run-0 seed is kept so the defective run stays named rather than forgotten, and nothing
/// generates from it any more.
pub const SEED: u64 = 0x5EED_0002_0000_0002;
pub const SEED_RUN0: u64 = 0x5EED_0002_0000_0001;

/// 96 cases in two declared strata, stratum S first.
pub const CASES: usize = 96;
/// Stratum S, the subset both systems can express. Amendment 1: 32 of them, because the primary metric
/// is a count of disagreements on this subset and one case cannot carry a count of zero.
pub const STRATUM_SHARED: usize = 32;

const FACT_KEYS: usize = 6;
const CAPABILITY_KEYS: usize = 3;
const DEPENDENCY_KEYS: usize = 3;
const CONSTRAINT_KEYS: usize = 3;
const TEXT_VALUES: [&str; 4] = ["alpha", "bravo", "charlie", "delta"];

const ALL_KINDS: [ConditionKind; 10] = [
    ConditionKind::Equals,
    ConditionKind::NotEquals,
    ConditionKind::Present,
    ConditionKind::Absent,
    ConditionKind::Gte,
    ConditionKind::Lte,
    ConditionKind::MemberOf,
    ConditionKind::CapabilityAvailable,
    ConditionKind::CapabilityUnavailable,
    ConditionKind::PredicateRef,
];

/// The three kinds the baseline models. Stratum S draws only from these, which is what makes a stratum S
/// case one both systems can express.
const SHARED_KINDS: [ConditionKind; 3] = [
    ConditionKind::Equals,
    ConditionKind::Gte,
    ConditionKind::CapabilityAvailable,
];

/// Which declared stratum a case came from. Recorded in the artifact, because a number that does not say
/// which stratum produced it cannot be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stratum {
    /// The subset both systems implement.
    Shared,
    /// The full declared distributions.
    Full,
}

impl Stratum {
    pub fn as_str(self) -> &'static str {
        match self {
            Stratum::Shared => "S_shared_subset",
            Stratum::Full => "R_full_distributions",
        }
    }
}

/// What a case is, for the reading of any disagreement. The protocol fixes these three before the run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// A models everything the case uses.
    Expressible,
    /// The case uses a condition kind, a value shape or an order A's model cannot represent.
    ProjectionLossy,
    /// The case exercises something A does not implement at all: the coverage claim, recorded history,
    /// the out-of-band adaptation channel, or an undeclared target constraint.
    CapabilityAbsentInA,
}

impl Shape {
    pub fn as_str(self) -> &'static str {
        match self {
            Shape::Expressible => "expressible",
            Shape::ProjectionLossy => "projection_lossy",
            Shape::CapabilityAbsentInA => "capability_absent_in_a",
        }
    }
}

pub struct GeneratedCase {
    pub id: String,
    pub stratum: Stratum,
    pub family: &'static str,
    pub capsule: ExperienceCapsule,
    pub target: ContextSnapshot,
    pub declared_adaptation_keys: Vec<String>,
    pub shape: Shape,
    /// What the generator drew, so a hole in the corpus is visible in the artifact rather than inferred.
    pub drawn: String,
}

/// A deterministic generator. A small xorshift, written here rather than added as a dependency: the point
/// of a seeded corpus is that it is reproducible from a number in a document, and a dependency would make
/// the number depend on a lockfile as well.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Uniform in `0..bound`. `bound` is always a small constant in this generator, so the modulo bias is
    /// bounded and does not vary with the seed.
    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }

    fn chance(&mut self, percent: u64) -> bool {
        self.next() % 100 < percent
    }

    fn pick<'a, T>(&mut self, options: &'a [T]) -> &'a T {
        &options[self.below(options.len())]
    }
}

/// Draw 96 cases from the two declared strata, stratum S first.
pub fn generate() -> Vec<GeneratedCase> {
    let mut rng = Rng(SEED);
    (0..CASES)
        .map(|index| {
            let stratum = if index < STRATUM_SHARED {
                Stratum::Shared
            } else {
                Stratum::Full
            };
            one_case(index, stratum, &mut rng)
        })
        .collect()
}

fn one_case(index: usize, stratum: Stratum, rng: &mut Rng) -> GeneratedCase {
    let id = format!("wide-{index:02}");
    // Text-valued cases exist because `a_view` maps a non-integer to unknown, and that mapping is A's
    // recorded behaviour rather than a choice made here. Stratum S is the shared subset, so it stays
    // integer: a text value is a projection A cannot carry.
    let text = stratum == Stratum::Full && rng.chance(25);
    let kinds_pool: &[ConditionKind] = match stratum {
        Stratum::Shared => &SHARED_KINDS,
        Stratum::Full => &ALL_KINDS,
    };
    let condition_count = 1 + rng.below(4);

    let mut conditions: Vec<ApplicabilityCondition> = Vec::new();
    let mut used_keys: Vec<String> = Vec::new();
    let mut kinds: Vec<&'static str> = Vec::new();
    for position in 0..condition_count {
        let kind = *rng.pick(kinds_pool);
        // Distinct keys where the draw allows it, so two conditions do not collapse onto one finding.
        let key = loop {
            let candidate = format!("k{}", (position + rng.below(FACT_KEYS)) % FACT_KEYS);
            if !used_keys.contains(&candidate) {
                used_keys.push(candidate.clone());
                break candidate;
            }
            if used_keys.len() == FACT_KEYS {
                break candidate;
            }
        };
        let adaptable = rng.chance(25);
        let condition = build_condition(&key, kind, text, adaptable, rng);
        kinds.push(kind_name(kind));
        conditions.push(condition);
    }

    // The target. Every key is drawn, whether or not a condition needs it, so a case can carry a fact
    // nobody asked about.
    let mut target_values: BTreeMap<String, Option<PlainValue>> = BTreeMap::new();
    let mut target = ContextSnapshot::new(&format!("world:{id}-tgt"), 1);
    for slot in 0..FACT_KEYS {
        let key = format!("k{slot}");
        let roll = rng.below(100);
        let value = if roll < 55 {
            Some(if text {
                PlainValue::Text((*rng.pick(&TEXT_VALUES)).to_owned())
            } else {
                PlainValue::Int(1 + rng.below(1000) as i64)
            })
        } else if roll < 70 {
            Some(PlainValue::Absent)
        } else if roll < 85 {
            Some(PlainValue::Unknown)
        } else {
            None
        };
        target_values.insert(key.clone(), value.clone());
        target = match value {
            Some(PlainValue::Int(number)) => {
                target.with_fact(&key, Fact::Known(serde_json::json!(number)))
            }
            Some(PlainValue::Text(text)) => {
                target.with_fact(&key, Fact::Known(serde_json::json!(text)))
            }
            Some(PlainValue::Absent) => target.with_fact(&key, Fact::Absent),
            Some(PlainValue::Unknown) => target.with_fact(&key, Fact::Unknown),
            None => target,
        };
    }
    for slot in 0..CAPABILITY_KEYS {
        let key = format!("c{slot}");
        target = target.with_capability(&key, draw_capability(rng));
    }

    // Coverage is the claim the capsule makes about its own conditions, and it is drawn here rather than
    // forced, because a corpus where every capsule claims closed coverage never tests the claim. Stratum
    // S fixes it, because the claim is one of the four things that put a case outside the shared subset.
    let coverage = if stratum == Stratum::Shared || rng.chance(70) {
        ApplicabilityCoverage::ClosedDeclared
    } else if rng.chance(50) {
        ApplicabilityCoverage::PartialDeclared
    } else {
        ApplicabilityCoverage::Opaque
    };

    // The out-of-band adaptation channel. A has no type for it and receives only the keys.
    let declared_adaptation_keys: Vec<String> = if stratum == Stratum::Shared || rng.chance(75) {
        Vec::new()
    } else {
        let count = 1 + rng.below(2);
        (0..count)
            .map(|_| used_keys[rng.below(used_keys.len())].clone())
            .collect()
    };

    // Recorded history. A has no notion of it. The entry is only emitted where the target establishes a
    // known fact for the key, so a failure that can never be relevant is not generated as a case.
    let known_failures: Vec<FailureObservation> = if stratum != Stratum::Shared && rng.chance(25) {
        let candidates: Vec<(&String, &PlainValue)> = target_values
            .iter()
            .filter_map(|(key, value)| value.as_ref().map(|value| (key, value)))
            .collect();
        if candidates.is_empty() {
            Vec::new()
        } else {
            let (key, value) = candidates[rng.below(candidates.len())];
            // Half the time the recorded condition is the one the target establishes, which is the case
            // that has to resurface.
            let recorded = if rng.chance(50) {
                value.clone().to_json()
            } else if matches!(value, PlainValue::Int(_)) {
                serde_json::json!(9_999)
            } else {
                serde_json::json!("never-recorded")
            };
            vec![FailureObservation {
                id: format!("fail/{id}"),
                condition_key: key.clone(),
                condition_value: recorded,
                detail: "a previous attempt failed under this exact condition".to_owned(),
            }]
        }
    } else {
        Vec::new()
    };

    let dependencies: Vec<String> = (0..rng.below(3)).map(|slot| format!("d{slot}")).collect();

    // A constraint the target holds and the capsule never declared.
    let extra_constraints: Vec<String> = if stratum != Stratum::Shared && rng.chance(30) {
        let key = format!("x{}", rng.below(CONSTRAINT_KEYS));
        target
            .constraints
            .insert(key.clone(), serde_json::json!("set-by-the-target"));
        vec![key]
    } else {
        Vec::new()
    };

    let mut capsule = ExperienceCapsule::new(
        &format!("exp/{id}"),
        1,
        {
            let mut source = ContextSnapshot::new(&format!("world:{id}-src"), 1);
            for (key, value) in &target_values {
                if let Some(PlainValue::Int(number)) = value {
                    source = source.with_fact(key, Fact::Known(serde_json::json!(number)));
                } else if let Some(PlainValue::Text(text)) = value {
                    source = source.with_fact(key, Fact::Known(serde_json::json!(text)));
                }
            }
            source.coverage = coverage;
            source
        },
        world_kernel::experience::Intervention::new(
            "a packaged attempt",
            "apply the packaged attempt in the target",
        ),
        conditions,
        ExperienceOutcome {
            verdict: "accepted".to_owned(),
            detail: "in the source".to_owned(),
            limits: vec!["established in the source only".to_owned()],
        },
        ProvenanceRecord {
            author: "generator".to_owned(),
            activity: "wide-corpus".to_owned(),
            source_revision: 1,
            used_refs: Vec::new(),
        },
    );
    for key in &dependencies {
        capsule = capsule.with_dependency(key);
    }
    for failure in known_failures {
        capsule = capsule.with_failure(failure);
    }

    let shape = classify(
        &capsule.applicability_conditions,
        coverage,
        !declared_adaptation_keys.is_empty(),
        !extra_constraints.is_empty(),
        !capsule.known_failures.is_empty(),
        text,
    );

    let drawn = format!(
        "stratum {}: {} condition(s) [{}], coverage {:?}, adaptation keys {}, recorded failure {}, extra \
         constraint {}, values {}",
        stratum.as_str(),
        capsule.applicability_conditions.len(),
        kinds.join(", "),
        coverage,
        declared_adaptation_keys.len(),
        usize::from(!capsule.known_failures.is_empty()),
        usize::from(!extra_constraints.is_empty()),
        if text { "text" } else { "integer" },
    );

    GeneratedCase {
        id,
        stratum,
        family: family_of(shape),
        capsule,
        target,
        declared_adaptation_keys,
        shape,
        drawn,
    }
}

fn build_condition(
    key: &str,
    kind: ConditionKind,
    text: bool,
    adaptable: bool,
    rng: &mut Rng,
) -> ApplicabilityCondition {
    let mut value = || {
        if text {
            serde_json::Value::String((*rng.pick(&TEXT_VALUES)).to_owned())
        } else {
            serde_json::json!(1 + rng.below(1000) as i64)
        }
    };
    let make = |expected: Option<serde_json::Value>| ApplicabilityCondition {
        key: key.to_owned(),
        kind,
        expected,
        adaptable,
    };
    match kind {
        ConditionKind::Equals
        | ConditionKind::NotEquals
        | ConditionKind::Gte
        | ConditionKind::Lte => make(Some(value())),
        ConditionKind::MemberOf => {
            let mut members: Vec<serde_json::Value> = Vec::new();
            for _ in 0..3 {
                let member = value();
                if !members.contains(&member) {
                    members.push(member);
                }
            }
            make(Some(serde_json::Value::Array(members)))
        }
        ConditionKind::Present
        | ConditionKind::Absent
        | ConditionKind::CapabilityAvailable
        | ConditionKind::CapabilityUnavailable
        | ConditionKind::PredicateRef => make(None),
    }
}

fn draw_capability(rng: &mut Rng) -> CapabilityState {
    match rng.below(4) {
        0 => CapabilityState::Available,
        1 => CapabilityState::Unavailable,
        2 => CapabilityState::Unsupported,
        _ => CapabilityState::NotObserved,
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

/// The classification, exactly as the protocol fixes it. A `coverage` the baseline never reads, a
/// recorded failure it has no field for, an out-of-band adaptation key it cannot type, or an undeclared
/// target constraint it does not look at, all put a case in `capability_absent_in_a` whatever else it
/// uses, because those are the things A does not implement rather than the things its model cannot
/// express.
pub fn classify(
    conditions: &[ApplicabilityCondition],
    coverage: ApplicabilityCoverage,
    has_declared_adaptation: bool,
    has_extra_constraint: bool,
    has_recorded_failure: bool,
    text: bool,
) -> Shape {
    if coverage != ApplicabilityCoverage::ClosedDeclared
        || has_declared_adaptation
        || has_extra_constraint
        || has_recorded_failure
    {
        return Shape::CapabilityAbsentInA;
    }
    let expressible = conditions.iter().all(|condition| {
        !text
            && matches!(
                condition.kind,
                ConditionKind::Equals | ConditionKind::Gte | ConditionKind::CapabilityAvailable
            )
    });
    if expressible {
        Shape::Expressible
    } else {
        Shape::ProjectionLossy
    }
}

fn family_of(shape: Shape) -> &'static str {
    match shape {
        Shape::Expressible => "expressible",
        Shape::ProjectionLossy => "projection_lossy",
        Shape::CapabilityAbsentInA => "capability_absent_in_a",
    }
}

// ---------------------------------------------------------------------------------------------
// The independent oracle. Plain values, no Kernel types, no call into `transfer_core`.
// ---------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlainValue {
    Int(i64),
    Text(String),
    Absent,
    Unknown,
}

impl PlainValue {
    fn to_json(&self) -> serde_json::Value {
        match self {
            PlainValue::Int(number) => serde_json::json!(number),
            PlainValue::Text(text) => serde_json::json!(text),
            PlainValue::Absent => serde_json::Value::Null,
            PlainValue::Unknown => serde_json::Value::Null,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlainState {
    Available,
    Unavailable,
    Unsupported,
    NotObserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlainKind {
    Equals,
    NotEquals,
    Present,
    Absent,
    Gte,
    Lte,
    MemberOf,
    CapabilityAvailable,
    CapabilityUnavailable,
    PredicateRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OracleStatus {
    DirectlyReusable,
    AdaptationRequired,
    AdditionalEvidenceRequired,
    Incompatible,
    InsufficientInformation,
}

impl OracleStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            OracleStatus::DirectlyReusable => "directly_reusable",
            OracleStatus::AdaptationRequired => "adaptation_required",
            OracleStatus::AdditionalEvidenceRequired => "additional_evidence_required",
            OracleStatus::Incompatible => "incompatible",
            OracleStatus::InsufficientInformation => "insufficient_information",
        }
    }
}

/// The status plus the keys each bucket found, so a disagreement can be attributed rather than merely
/// counted.
#[derive(Debug, Clone)]
pub struct OracleOutcome {
    pub status: OracleStatus,
    pub different: Vec<String>,
    pub unknown: Vec<String>,
}

/// The declared semantics, sections 5.1 to 5.2 of `docs/CONSUMER-CHECK-PROTOCOL.md`, written a second
/// time over plain values. It shares no code with the planner on purpose.
#[allow(clippy::too_many_arguments)]
pub fn oracle(
    conditions: &[(String, PlainKind, Option<serde_json::Value>, bool)],
    facts: &BTreeMap<String, PlainValue>,
    capabilities: &BTreeMap<String, PlainState>,
    opaque: bool,
    adaptation_keys: &[String],
    recorded_failures: &[(String, serde_json::Value)],
) -> OracleOutcome {
    let mut different: Vec<String> = Vec::new();
    let mut unknown: Vec<String> = Vec::new();

    for (key, kind, expected, _) in conditions {
        if *kind == PlainKind::PredicateRef {
            // A predicate nobody here holds cannot be resolved whatever the target says.
            unknown.push(key.clone());
            continue;
        }
        if matches!(
            kind,
            PlainKind::CapabilityAvailable | PlainKind::CapabilityUnavailable
        ) {
            match capabilities.get(key) {
                Some(PlainState::Available) => {
                    if *kind == PlainKind::CapabilityAvailable {
                        continue;
                    }
                    different.push(key.clone());
                }
                Some(PlainState::Unavailable) | Some(PlainState::Unsupported) => {
                    if *kind == PlainKind::CapabilityUnavailable {
                        continue;
                    }
                    different.push(key.clone());
                }
                // Nobody checked. Not observed is not unavailable, and it is not a match either.
                Some(PlainState::NotObserved) | None => unknown.push(key.clone()),
            }
            continue;
        }

        let Some(value) = facts.get(key) else {
            // The target never declared this key, which is a second finding from an unknown key and
            // reaches the same status.
            unknown.push(key.clone());
            continue;
        };
        // An established absence is an answer. It satisfies `absent` and it violates every other kind,
        // including `present`, and reading it as "we do not know" would turn a known world into a missing
        // observation.
        let value = match value {
            PlainValue::Absent => {
                if *kind == PlainKind::Absent {
                    continue;
                }
                different.push(key.clone());
                continue;
            }
            PlainValue::Unknown => {
                unknown.push(key.clone());
                continue;
            }
            PlainValue::Int(number) => serde_json::json!(number),
            PlainValue::Text(text) => serde_json::json!(text),
        };
        let required = expected.clone().unwrap_or(serde_json::Value::Null);
        let holds = match kind {
            PlainKind::Equals => value == required,
            PlainKind::NotEquals => value != required,
            PlainKind::Present => true,
            PlainKind::Absent => false,
            // A value that is not a number does not satisfy an order. It is a difference, not a crash and
            // not a match.
            PlainKind::Gte => match (value.as_i64(), required.as_i64()) {
                (Some(actual), Some(bound)) => actual >= bound,
                _ => false,
            },
            PlainKind::Lte => match (value.as_i64(), required.as_i64()) {
                (Some(actual), Some(bound)) => actual <= bound,
                _ => false,
            },
            PlainKind::MemberOf => match required.as_array() {
                Some(members) => members.contains(&value),
                None => false,
            },
            PlainKind::PredicateRef
            | PlainKind::CapabilityAvailable
            | PlainKind::CapabilityUnavailable => false,
        };
        if !holds {
            different.push(key.clone());
        }
    }

    let bridgeable = |key: &str| -> bool {
        conditions
            .iter()
            .any(|(condition_key, _, _, adaptable)| condition_key == key && *adaptable)
            || adaptation_keys.iter().any(|entry| entry == key)
    };
    let uncovered = different.iter().any(|key| !bridgeable(key));

    // A failure the target establishes right now, whatever the conditions say.
    let repeated = recorded_failures.iter().any(|(key, value)| {
        matches!(facts.get(key), Some(PlainValue::Int(number)) if serde_json::json!(number) == *value)
            || matches!(facts.get(key), Some(PlainValue::Text(text)) if serde_json::json!(text) == *value)
    });

    let status = if conditions.is_empty() || opaque {
        OracleStatus::InsufficientInformation
    } else if repeated {
        OracleStatus::AdaptationRequired
    } else if uncovered {
        OracleStatus::Incompatible
    } else if !different.is_empty() {
        OracleStatus::AdaptationRequired
    } else if !unknown.is_empty() {
        OracleStatus::AdditionalEvidenceRequired
    } else {
        OracleStatus::DirectlyReusable
    };

    OracleOutcome {
        status,
        different,
        unknown,
    }
}

/// One declared condition, as the oracle sees it.
pub type PlainCondition = (String, PlainKind, Option<serde_json::Value>, bool);
pub type PlainFacts = BTreeMap<String, PlainValue>;
pub type PlainCapabilities = BTreeMap<String, PlainState>;
pub type PlainFailures = Vec<(String, serde_json::Value)>;

/// Everything the oracle takes, in one value rather than a six-element tuple, so the call site reads as
/// the argument list it is.
pub struct PlainCase {
    pub conditions: Vec<PlainCondition>,
    pub facts: PlainFacts,
    pub capabilities: PlainCapabilities,
    /// Whether the capsule declares its own coverage opaque.
    pub opaque: bool,
    pub adaptation_keys: Vec<String>,
    pub recorded_failures: PlainFailures,
}

/// Translate a generated case into the plain values the oracle takes. A structural translation, not a
/// second specification: the rules live in [`oracle`] and nowhere else.
pub fn to_plain(case: &GeneratedCase) -> PlainCase {
    let conditions = case
        .capsule
        .applicability_conditions
        .iter()
        .map(|condition| {
            (
                condition.key.clone(),
                match condition.kind {
                    ConditionKind::Equals => PlainKind::Equals,
                    ConditionKind::NotEquals => PlainKind::NotEquals,
                    ConditionKind::Present => PlainKind::Present,
                    ConditionKind::Absent => PlainKind::Absent,
                    ConditionKind::Gte => PlainKind::Gte,
                    ConditionKind::Lte => PlainKind::Lte,
                    ConditionKind::MemberOf => PlainKind::MemberOf,
                    ConditionKind::CapabilityAvailable => PlainKind::CapabilityAvailable,
                    ConditionKind::CapabilityUnavailable => PlainKind::CapabilityUnavailable,
                    ConditionKind::PredicateRef => PlainKind::PredicateRef,
                },
                condition.expected.clone(),
                condition.adaptable,
            )
        })
        .collect();

    let facts = case
        .target
        .facts
        .iter()
        .map(|(key, fact)| {
            (
                key.clone(),
                match fact {
                    Fact::Known(value) => match value {
                        serde_json::Value::String(text) => PlainValue::Text(text.clone()),
                        other => PlainValue::Int(other.as_i64().unwrap_or_default()),
                    },
                    Fact::Absent => PlainValue::Absent,
                    Fact::Unknown => PlainValue::Unknown,
                },
            )
        })
        .collect();

    let capabilities = case
        .target
        .capabilities
        .iter()
        .map(|(key, state)| {
            (
                key.clone(),
                match state {
                    CapabilityState::Available => PlainState::Available,
                    CapabilityState::Unavailable => PlainState::Unavailable,
                    CapabilityState::Unsupported => PlainState::Unsupported,
                    CapabilityState::NotObserved => PlainState::NotObserved,
                },
            )
        })
        .collect();

    let failures = case
        .capsule
        .known_failures
        .iter()
        .map(|failure| {
            (
                failure.condition_key.clone(),
                failure.condition_value.clone(),
            )
        })
        .collect();

    PlainCase {
        conditions,
        facts,
        capabilities,
        opaque: case.capsule.source_context.coverage == ApplicabilityCoverage::Opaque,
        adaptation_keys: case.declared_adaptation_keys.clone(),
        recorded_failures: failures,
    }
}
