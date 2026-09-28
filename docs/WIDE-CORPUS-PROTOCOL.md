# Wide corpus: does a wider corpus separate A from C?

Status: pre-registered, run once, amended once, not yet run on the amended corpus
Last verified: run 0 executed and found a generator defect, kept as
`experiments/wide-corpus/RESULTS-run0-defect.md`. Amendment 1 is below and has not been run.

## What this is

The second of the two reversal conditions
[ADR-005](ADR-005-reduce-to-the-representation.md) attaches to the transfer reduction:

> A wider corpus separates the two systems. The brief's 96-case corpus, bounded generative trees and
> the B and C baselines are excluded from tranche 1. If any of them produces a case where A and C
> disagree, the tie was an artefact of 30 hand-designed cases and the comparison must be re-run before
> the reduction stands.

The first condition was executed on 2026-09-28 and did not fire, recorded in
[experiments/consumer-transfer](../experiments/consumer-transfer/RESULTS.md). This is the second. It
has no milestone number and it funds nothing.

## What the recorded 30 cases actually cover

Stated before the run, because it is the whole reason a wider corpus might separate the two systems and
because a reader should be able to check it. `tests/transfer_benchmark.rs` builds 30 cases from five
loops, and reading the generator:

- every case declares **one** condition, except the deceptive and unknown families which declare two;
- the condition kinds used are `equals`, `gte` and `capability_available`. That is **3 of the 10**
  `ConditionKind` variants the representation publishes;
- every fact is an integer. No string, no boolean, no `absent` fact, no explicitly `unknown` fact;
- every capability is `available` or `unsupported`. `unavailable` and `not_observed` never appear;
- `coverage` is forced to `closed_declared` by the case constructor, so `opaque` never appears;
- `adaptation_keys` is empty on all 30, so the out-of-band adaptation channel is never exercised;
- `known_failures` is never populated, so no case carries recorded history;
- the source and target contexts always agree on every key but the one that decides the case.

So the recorded tie is a tie on a corpus that exercises a fraction of the published alphabet. That is
not yet a finding. It is the reason to run this.

## The three systems, and which of them is frozen

| Id | What it is | Frozen? |
|---|---|---|
| **C** | `plan_transfer` in `tests/support/transfer_core.rs`, as ADR-005 left it | yes, sha256 `4533e8df4cb24718c3f4c4ed2df24ac4573ee447af1712a38a688245d3bc270b`, 684 lines |
| **A** | the competent baseline, `a_gate` and `a_view` in `tests/support/transfer_fixture.rs` | **yes, and not to be touched**: sha256 `bc09c10b81896d25e61d502ae3ead3bb71c6d706163beaca5e4f4a8177492ce3`, 515 lines |
| **R** | a new independent oracle over the full published alphabet | no, it is written for this run |

**A is frozen and is not extended, at all, ever, in this run.** This is the single most important line in
this document. A separation is only evidence if the thing being separated was not changed to make it
separate. Widening the corpus until a baseline that only models two condition kinds fails would be
rigging, and it is the obvious way to get a dramatic result. A competent baseline *could* grow the other
eight kinds, exactly as it grew `adaptable`, and a result that says "A cannot express this" is a statement
about what the 30-case corpus covered, not a permanent ranking. `tests/wide_corpus.rs` asserts A's hash,
so a well-meaning edit fails the build.

R is a separate program over plain values, the same discipline the tranche-1 oracle used: it calls
nothing in `transfer_core` and reads no expected value, so a planner bug and a matching oracle bug
cannot cancel out.

## The corpus, declared before it is generated

96 cases from one seed, `0x5EED_0002_0000_0001`, through a bounded generator. "Bounded" is literal: the
generator draws from the distributions below and cannot recurse, and every case is a flat list of at most
4 conditions over at most 6 keys. There is no tree to grow, so a case cannot become unrepresentable.

| Dimension | Distribution, fixed here |
|---|---|
| conditions per capsule | 1 to 4, uniform |
| condition kind | uniform over all 10 published `ConditionKind` variants |
| numeric expected value | an integer in 1..=1000 |
| string expected value | one of 4 fixed strings, for the string-valued `equals` cases |
| `adaptable` | true on 25% of conditions |
| target fact state | `Known` 55%, `Absent` 15%, `Unknown` 15%, key undeclared 15% |
| target capability state | uniform over `Available`, `Unavailable`, `Unsupported`, `NotObserved` |
| capsule coverage | `ClosedDeclared` 70%, `PartialDeclared` 15%, `Opaque` 15% |
| declared adaptations | none on 75% of cases, otherwise 1 to 2 keys drawn from the case's own condition keys |
| recorded failure | on 25% of cases, one `known_failures` entry whose key and value match the target half the time |
| dependencies | 0 to 2 keys |
| extra target constraints | none on 70% of cases, otherwise 1 undeclared key |
| fact values | integers, and strings on 25% of cases, because `a_view` maps a non-integer to unknown |

The generator is deterministic and the seed is in this file, so the corpus is a function of this document
and not of the run.

## The projection, declared, because it is where a separation comes from

A is fed the same capsule and the same target. How the published alphabet is projected onto A's model is
fixed here, and it is the recorded tranche-1 projection, extended mechanically:

- a `capability_available` condition becomes an A capability condition, with the target's capability state
  as one of A's four strings. A models this fully, including `not_observed` as unknown;
- a `gte` or `lte` condition on an integer becomes an A `gte` condition, which is the only order A models;
- every other kind becomes an A `equals` condition on the integer 0, which is what the tranche-1
  benchmark did for any kind it did not model;
- `a_view` maps a non-integer `Known` value to `AValue::Unknown`, which is A's recorded behaviour and is
  not changed here;
- A is given the declared adaptation keys and nothing else. It has no concept of `coverage`, of
  `known_failures`, of `dependencies`, of `PredicateRef`, or of an undeclared target constraint.

A case is then one of three kinds, and the kind decides what a disagreement means:

| Kind | Definition |
|---|---|
| `expressible` | every condition kind is `equals`-on-integer, `gte` or `capability_available`; no string fact; `coverage` closed; no recorded failure; no declared adaptation; no extra target constraint |
| `projection_lossy` | the case uses a condition kind, a fact shape or a value A's model cannot represent |
| `capability_absent_in_a` | the case exercises something A does not implement at all: the coverage claim, recorded history, the out-of-band adaptation channel, or an undeclared target constraint |

The two sub-counts below are the reason for this table, and they are fixed before the run:

- **separations on `expressible` cases** is the strong finding. A and C both implement that subset, so a
  disagreement there is a real separation or a bug in one of them, and it cannot be blamed on the
  projection.
- **separations on `projection_lossy` or `capability_absent_in_a` cases** is the expected finding, and it
  says the recorded tie was scoped to a narrow corpus. It is still a real observation about the recorded
  comparison, and it is still not a verdict on A.

## The metric and the threshold, fixed

**Primary metric:** the number of cases where C and A return a different `TransferStatus`. The reversal
condition fires at **1**. There is no significance threshold and no sample-size argument: the condition as
ADR-005 states it is "if any of them produces a case where A and C disagree", and a pre-registered
threshold is a place to hide a result.

**Attribution metric:** for every disagreement, R's status. `R_with_C` means the oracle sides with the
planner, `R_with_A` means it sides with the baseline, `R_with_neither` means both implementations differ
from the independent reading. A run where R sides with neither is a finding about the corpus, not about
either system.

**Also recorded, and not a gate:** the obligation sets C and A produce on the `expressible` cases, and the
count of cases where the planner matches R but A does not, grouped by the condition kind involved.

## Exclusions, fixed

- **Expected exclusions: zero.** Every shape the generator can produce has a defined expectation in the
  semantics below, including `predicate_ref`, so no case needs to be dropped to be decidable.
- If the generator produces a case that is not decidable, that is a defect in the generator and it is
  fixed in the generator, the corpus is re-derived, and the defective run is not kept as a result.
- No case is dropped after the run for producing an inconvenient status. The only permitted exclusion is a
  generation defect, and it is reported with the seed and the case id if it ever happens.

## The semantics the oracle implements, fixed before it is written

The same readings as sections 5.1 to 5.4 of
[CONSUMER-CHECK-PROTOCOL](CONSUMER-CHECK-PROTOCOL.md), which are already pre-registered and already
measured. Reusing them is deliberate: a second specification for the same published types would be a
second thing to get wrong, and the corpus is meant to test the two systems against one reading, not
against a new one.

In outline: a capability condition is answered from the capability map only; an undeclared key and an
`unknown` key are unknown and are not a match; an `absent` fact satisfies `absent` and violates every
required value; a non-numeric value does not satisfy an order; `predicate_ref` is unevaluable; the
precedence is coverage or no condition, then a recurrent recorded failure, then an uncovered difference,
then any difference, then an unknown, then a clean reuse; a difference is bridgeable when the author
declared the key adaptable or a declared adaptation covers it.

## What each result means, fixed in advance

| Result | Reading | What happens next |
|---|---|---|
| zero separations | the tie survives a 96-case corpus over the full alphabet | ADR-005's second condition does not fire; the reduction stands and both conditions are then observed |
| separations only on `projection_lossy` or `capability_absent_in_a` | the recorded tie was scoped to a narrow corpus, and the corpus, not the ranking, is what changed | the recorded comparison is re-stated with its scope, and R's attribution says which side was right on each case. **The planner is not thereby vindicated** and the reduction is not re-opened on this alone |
| any separation on an `expressible` case | A and C implement the same subset and do not agree on it | this is the finding that matters: a real disagreement between two competent implementations of the same rules, attributed by R, and the comparison has to be re-run before the reduction stands |

In the second row the honest conclusion is narrow and worth stating plainly: the extra lines bought the
eight condition kinds A never modelled, the coverage claim, recorded history and the out-of-band
adaptation channel. That is real capability, and on this corpus it is also capability a competent
baseline can add in a fraction of the lines. Both halves of that sentence belong in the record.

## Amendment 1, 2026-09-28: the decisive subset was starved, and is given a declared stratum

Written after the first run and **before** the amended run, and before any separation count on a powered
expressible subset is known. This is a composition change and nothing else: the metric, the threshold of
1, the projection, the three readings and the semantics are all unchanged, because none of them was
defective.

**What run 0 found, and why it is not a result.** The clause above about per-kind coverage existing so a
hole in the corpus is visible did its job. The first run drew 96 cases and produced:

| Shape | Cases |
|---|---|
| `capability_absent_in_a` | 74 |
| `projection_lossy` | 21 |
| **`expressible`** | **1** |

44 separations, every one of them on a case the baseline's model cannot carry, and the independent oracle
siding with the planner in all 44. That is a real statement about the baseline. But `separations on
expressible cases`, which this document names as the strong finding and the reason for the three-way
shape table, was measured on **one case**. A count of zero disagreements on one case is not a measurement
of anything, and reporting it as the strong row would be reading a power failure as a pass.

**Why the subset was starved, from the declared distributions.** The `expressible` definition is a
conjunction of five things: an integer value space, every condition kind inside a 3-subset of 10, a closed
coverage claim, no recorded failure, no declared adaptation, and no undeclared target constraint. The
declared rates make the last four likely to fail at least once per case, and drawing kinds uniformly over
ten makes the third fail fast as the condition count rises. The generator was not biased; the composition
was simply not stratified, so the one stratum the experiment was designed around almost never appeared.

**The amendment.** The corpus becomes 96 cases in two declared strata, drawn from one new seed,
`0x5EED_0002_0000_0002`, stratum S first:

| Stratum | Cases | What is drawn |
|---|---|---|
| **S, the shared subset** | 32 | integer values; condition kinds drawn only from `equals`, `gte`, `capability_available`; coverage always `closed_declared`; never a recorded failure, a declared adaptation or an undeclared target constraint. Everything else, meaning the condition count, the target fact and capability states and the dependencies, is drawn from the table above unchanged. |
| **R, the full distributions** | 64 | the table above exactly as written, nothing changed. |

Stratum S is the subset both systems implement, drawn from the same declared distributions for everything
that does not define expressibility. Declaring it is the fix for a corpus that could not measure its own
primary metric; it is not a licence to pick cases that agree, because the reading for stratum S was
already fixed above and applies unchanged: **any separation on an expressible case is the strong row, and
the strong row says the reduction has to be re-run.**

**What this does not do.**

- It does not change the threshold, the metric, the projection, the classification, the semantics or the
  three readings. A disagreement found on stratum S has exactly the weight it would have had in run 0.
- It does not extend the baseline. A stays frozen, and the honest half of the finding stays true: the
  eight kinds A cannot model, the coverage claim, recorded history and the out-of-band adaptation channel
  are capability a competent baseline can add, and the run still does not show that adding them would tie.
- It does not delete run 0. Run 0 is a recorded run and it is kept verbatim as
  `experiments/wide-corpus/results-run0-defect.json` and `RESULTS-run0-defect.md`, under a name that says
  what it is. Its generator is the one this amendment replaces, so it cannot be reproduced by the current
  code, which is exactly why it is preserved as a file rather than as a paragraph of numbers.
- It does not license a third amendment. If the amended run's expressive subset is still unpowered, that
  is reported as unpowered.

## Threats this cannot remove, stated before the run

- The same session read the planner, the baseline and the tranche-1 corpus, and it also writes R. R's
  independence is from the planner, which is the property the oracle discipline exists for. Its
  independence from *me* is not a property anything here can establish.
- The corpus is generated from a distribution I chose. It is not a sample of real transfers, and a
  distribution that never produces a given shape would hide a disagreement rather than find one. The
  distributions above are therefore written out in full, and the per-kind coverage of the generated
  corpus is reported so a hole in it is visible.
- A is frozen, which protects the separation from being manufactured, and also means this run cannot
  show that a competent baseline *would* tie once extended. That question is left open rather than
  answered in the direction that flatters either side.

## Reproduction

```bash
cargo run --example wide_corpus
cargo run --example wide_corpus -- --render-only
cargo test --test wide_corpus
```

The first command writes `experiments/wide-corpus/results.json` and `RESULTS.md`. The test fails if the
checked-in result is not what a fresh run produces, and fails if baseline A is no longer the frozen file.
