# Consumer check: can the representation alone carry transfer?

Status: pre-registered, then executed on 2026-09-28 without a reversal
Last verified: 2026-09-28. This file was written before the consumer existed and is not revised by the run.
Result: [experiments/consumer-transfer/RESULTS.md](../experiments/consumer-transfer/RESULTS.md).

## What this is

This is the first of the two reversal conditions
[ADR-005](ADR-005-reduce-to-the-representation.md) names, executed. It has no milestone number and it
funds nothing. Nothing here is authorised to grow the Kernel.

The claim ADR-005 rests on is that the capsule representation is the portable part and the planner is not.
The cheapest way to attack that claim is to be the consumer: take the published artifacts, decide what
transfers, and compare against the planner that was moved out of the crate.

## The reversal condition, literally

A case reverses the reduction when **both** hold:

```text
consumer(capsule) != expected
AND
plan_transfer(case) == expected
```

One such case is the finding. It says an operative piece of information was lost when the procedure left
the shipped surface, which is what ADR-005 claims did not happen.

The condition is not "the consumer was awkward". A consumer that is careless fails everywhere, including
the cases the planner also fails, and that is information about the consumer, not about the reduction.

## Three measurements, one input

Every case is measured three ways, and the three are independent of each other:

| Measurement | Where it comes from |
|---|---|
| `expected` | `experiments/consumer-transfer/expectations.json`, declared in section 6 below, before the consumer was written |
| `consumer` | the `consumer-transfer` crate, written against the published artifacts only |
| `plan` | `tests/support/transfer_core.rs`, the procedure ADR-005 moved out of `src/` |

All three read the same case file, so the only thing that differs between the two decisions is the
procedure. The expectations are a third derivation: they are read off the contract's rules, not off either
implementation.

## What the consumer may read

Allowed, in full:

- `src/experience.rs`, the published representation
- `schemas/experience-capsule-v0.experimental.schema.json`, the wire format
- `docs/TRANSFER-CONTRACT.md`, the rules written for a consumer
- this file, **sections 1 to 5 only**. Section 6 holds the expectations and is not readable by the consumer
- the case files, for their input format only

Forbidden, and the build plus a test enforce it rather than a promise:

- `tests/**`, which is where the planner lives
- `examples/**`
- `src/impact`, `src/branch`, `src/continuation`, `src/kernel.rs`, `src/model.rs`
- `expectations.json`, `results.json` and this file's section 6
- any other module of this repository

`the_consumer_is_external_and_uninformed` in `tests/consumer_check.rs` fails the build if the consumer
source imports anything but `world_kernel::experience`, reads the filesystem, contains a case id, or
contains the word `expected`. The consumer's entry point takes three arguments, the capsule text, the
target text and the declared adaptations, so it cannot receive an answer even by accident.

## The threat this experiment cannot remove

Stated before the run, because it decides how the result may be read:

- one consumer, so a pass is necessary and not sufficient
- **the same session read the planner before writing this pre-registration, and then wrote the consumer.**
  There is no independent author available in the environment this ran in. This is the known way to get a
  rigged experiment and it is not repaired by good intentions. A second consumer written by somebody who has
  not read `tests/` is the one check that would make a pass mean more than it currently does.
- sixteen hand-written cases, so the corpus is not diverse
- no real second domain, so the slices 7 and 8 of the brief stay unreachable

A **failure** is therefore strong evidence, and a **pass** is weak evidence. A consumer that is written to
fail would produce a reversal, which is why the consumer is written to the contract, cites the section it
implements on every rule, and is audited against the planner afterwards. The audit and its result are in
`RESULTS.md`.

## Section 5: the semantics the consumer is held to

Section 5 is part of the pre-registration. `docs/TRANSFER-CONTRACT.md` states the rules and the vocabulary
but leaves some of them open. A consumer cannot be held to a reading the contract never states, and a
disagreement that is only a disagreement about an unstated reading is a gap in the document, not in the
representation. So the readings are fixed here, in advance, and the first four are direct consequences of
sentences the contract already contains.

### 5.1 Condition evaluation, from the contract's own table

- A capability condition is answered from the target's capability map and never from a fact of the same
  name. A capability nobody checked is not observed, and not observed is not unavailable.
- A fact the target does not declare, and a fact the target declares as unknown, are both unknown. They are
  two findings, and neither is a match.
- A fact declared absent is an answer. It satisfies an `absent` condition and it violates an `equals`,
  a `gte`, a `lte`, a `not_equals` and a `member_of` one. It is never unknown.
- `gte` and `lte` compare numbers. A value that is not a number does not satisfy them, and is a difference
  rather than a crash.
- A condition whose kind is `predicate_ref` names a predicate that no published consumer holds, so it
  cannot be evaluated whatever the target says. It is neither a match nor a violation.

### 5.2 The five statuses, and the precedence between them, from the contract's tables

The contract's status table fixes what each status means. It does not say which finding outranks which when
more than one is present, so this is fixed here:

1. The capsule declares no applicability condition, or declares its own coverage as `opaque`, and no other
   finding is available, so the status is `insufficient_information`. A capsule that will not say how
   complete it is cannot license a direct reuse.
2. A prior recorded failure whose condition the target establishes right now is `adaptation_required`. It
   outranks everything below it, because every condition can hold and the history can still say the
   intervention is unsafe here. This is the case that separates a status from a diff.
3. A known difference on a condition that nothing declares bridgeable is `incompatible`.
4. Any remaining known difference is `adaptation_required`.
5. Otherwise, if any condition is unknown or unevaluable, `additional_evidence_required`.
6. Otherwise `directly_reusable`.

Precedence rule 3 before 4 before 5: a known violation outranks a known difference, and a known difference
outranks an unobserved condition. The reason is that the first is a statement about the world and the last
is a statement about what we have not looked at yet.

`directly_reusable` is about the experience structure. It never asserts that the source's evidence or
assurance became the target's.

### 5.3 What makes a difference bridgeable, and what a consumer may use

A difference is bridgeable when the author of the condition set `adaptable` on it, or when the target domain
declares an adaptation that covers that key. It is never inferred from the value, and an adaptation nobody
declared is not available however obvious it looks.

**The second channel is not published.** The capsule carries `adaptable` per condition. A target-domain
adaptation is passed beside the plan by the caller, and the type that carries it lives in the moved planner,
not in `src/`. Section 5.3 is therefore only implementable by a consumer that reads an out-of-band
declaration, which is exactly what case 16 measures. This is recorded here as a known gap in the published
surface, before the run, and not discovered from a result.

### 5.4 Obligations, which a status alone does not carry

- Every known difference owes an adaptation, including a bridgeable one. Being re-parameterisable does not
  mean already done.
- Every unknown or unevaluable condition owes an observation in the target.
- Every key the source consumed as a dependency owes an observation in the target, even when every condition
  holds. A direct reuse is still work to redo.
- A target constraint the capsule never declared is recorded, because the source had no reason to know
  about it. It is not a failure and it does not by itself change the status.
- An assurance obligation is always bound to the target. Nothing a source recorded is inherited, and a
  target-side value built from an experience starts with no assurance at all.

### 5.5 The reported form

A consumer reports one status, using the snake_case spelling of the variant name in the contract's table,
plus the keys it found different and the keys it found unknown. Those two lists are recorded for diagnosis
and are not part of the reversal condition.

## Section 6: the cases and their expectations

Sixteen cases, `experiments/consumer-transfer/cases/case-01.json` to `case-16.json`. The ids are neutral on
purpose: a case called `clean-reuse` would hand the consumer the answer in its filename.

The domain is a nightly ledger export, which is not the arithmetic world of `tests/support/transfer_fixture.rs`
and not Sarah, not IntentLane and not a real budget. Expectations are read off sections 5.1 to 5.4.

| Case | Situation | Expected | Read from |
|---|---|---|---|
| 01 | three facts and one capability hold, coverage closed | `directly_reusable` | 5.1, 5.2 rule 6 |
| 02 | one fact differs, the author marked that key adaptable | `adaptation_required` | 5.2 rule 4, 5.3 |
| 03 | one required fact differs, nothing declares it bridgeable | `incompatible` | 5.2 rule 3 |
| 04 | the target never declares the key a condition needs | `additional_evidence_required` | 5.1 |
| 05 | the target declares that key unknown | `additional_evidence_required` | 5.1 |
| 06 | the target declares that key absent, and the condition requires a value | `incompatible` | 5.1, 5.2 rule 3 |
| 07 | the required capability was never checked in the target | `additional_evidence_required` | 5.1 |
| 08 | the required capability is known not to exist | `incompatible` | 5.1, 5.2 rule 3 |
| 09 | every condition holds but the capsule declares its coverage opaque | `insufficient_information` | 5.2 rule 1 |
| 10 | the capsule declares no condition at all | `insufficient_information` | 5.2 rule 1 |
| 11 | every condition holds and a recorded failure's condition is present again | `adaptation_required` | 5.2 rule 2 |
| 12 | one bridgeable difference and one undeclared key | `adaptation_required` | 5.2 rule 4 outranks 5 |
| 13 | one condition holds and one is unknown | `additional_evidence_required` | 5.2 rule 5 |
| 14 | an `absent` condition, and the target established absence | `directly_reusable` | 5.1 |
| 15 | a `predicate_ref` condition, and the target holds the key it names | `additional_evidence_required` | 5.1, 5.2 rule 5 |
| 16 | a required fact differs, and the target domain declares an adaptation for that key | `adaptation_required` | 5.3 |

Cases 15 and 16 are marked contested in `expectations.json`, with the reason recorded there. They are the
two places where the contract is silent or the published surface is incomplete, so a consumer may answer
defensibly and differently. They are still scored by the literal condition, and the result is reported both
ways: the literal count, and the count with the contested cases removed.

## What each result means, fixed in advance

| Result | Reading | What happens next |
|---|---|---|
| consumer == expected on every case | A | nothing is reintroduced. ADR-005 is strengthened by an observation made outside `tests/`, and check 2 runs. |
| consumer != expected somewhere while plan == expected | B | locate the missing information first. A missing field or a missing statement is fixed in the representation or in the contract. A procedural layer comes back into `src/` only if the information was present all along and the consumer had to compute it. |
| consumer != expected and plan != expected | C | no reversal. The case is uncovered by the system as a whole, which is a different finding and does not argue for the planner. |
| consumer == expected and plan != expected | D | the reduction is sufficient and the procedure was also wrong. This is the outcome the brief does not have a name for, and it is recorded as an architecture finding: the capability attributed to the Kernel belongs to the representation. |

A contested disagreement is reported as B and is then diagnosed as either a representation gap, a contract
gap or an unfair case. It is not silently dropped, and it is not silently counted as clean.

## Reproduction

```bash
cargo run --example consumer_check
cargo test --test consumer_check
```

`cargo run --example consumer_check` writes `experiments/consumer-transfer/results.json` and
`RESULTS.md`. `cargo test --test consumer_check` fails if the checked-in result does not match a fresh
measurement, and fails if the consumer has stopped being external and uninformed.
