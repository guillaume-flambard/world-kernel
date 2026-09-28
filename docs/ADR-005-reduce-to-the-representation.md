# ADR 005: M5 is reduced to the portable representation

Date: 2026-09-27
Status: accepted
Relates to: [ADR-003](ADR-003-measure-before-building.md), [ADR-004](ADR-004-the-facet-advantage-scales.md), [M5-PROTOCOL.md](M5-PROTOCOL.md), [IMPACT-CONTRACT.md](IMPACT-CONTRACT.md), [TRANSFER-CONTRACT.md](TRANSFER-CONTRACT.md)

## What the gate scored

Tranche 1 of M5 built a closed corpus of 30 cases across the brief's four families, an independent
oracle, and a competent baseline A with no Kernel dependency. The recorded result is in
[experiments/transfer-benchmark](../experiments/transfer-benchmark/RESULTS.md).

| | |
|---|---|
| Decisions | tie on all 30 cases |
| `falseDirectTransfer` | 0 for the Kernel, 0 for baseline A |
| `falseIncompatibility` | 0 for the Kernel, 0 for baseline A, 6 for the deliberately weaker `baselineWithoutDeclaredParameters` |
| `kernelSurface` | 1007 lines: `src/experience.rs` 323 + `src/transfer.rs` 684 |
| `baselineA` | 515 lines, an over-estimate because that file also holds the fixtures and the oracle |
| Ratio | 1.96 |
| Gate | three met, two not met, two unreachable |

Condition 5 of the brief's seven is *"the competent baseline does not give the same result at lower
complexity"*. It is not met. The brief's section 2 says the Kernel *"must be reduced accordingly"* when
that happens, and `docs/SPEC.md` records the consequence as: M5 is to be reduced to the smallest useful
portable experience representation.

## What the measurement actually said

It said one thing, and it is worth being precise about it because the last time this repository reduced
something it read more into a measurement than was there.

**The decision procedure did not earn its lines.** `plan_transfer` and the machinery around it decide,
in 684 lines, what a competent application gate decides in a file of 515 lines that also contains its
competitor's fixtures and its competitor's oracle. On this corpus the extra 492 lines bought no case.
That is the trigger, and it fires.

**Nothing was measured about the representation itself.** The corpus hands a capsule to the planner.
`experiments/transfer-benchmark` says so in its own `notMeasured` list: *"no retrieval quality
measurement: a capsule is handed to the planner, so the discovery problem is not exercised at all"*.
Whether a portable format is worth anything is not established by a run in which the format was never
the thing under test.

So this ADR does not claim the representation measured well. It claims the opposite about the planner
and keeps the representation for a different reason, stated in the next section.

## Why the representation is what is kept

Three reasons, in the order they carry weight.

**It is the milestone's named subject.** The milestone is called transferable experience and the brief's
own phrase is *portable experience representation*. A reduction is asked to name what it reduces to; the
representation is the only candidate in the code that is that thing.

**It is the one thing baseline A did not have, by construction.** Baseline A competed by defining its
own `ACapsule`, `ACondition` and `AValue` inside `tests/support/transfer_fixture.rs`. It can imitate a
*procedure* line for line, which is exactly what it did and exactly why it tied. It cannot supply a
*format* two independent systems agree on, because a format is not a procedure and agreement is not
something one side can produce. The measurement showed the procedure was not worth buying; it did not
show the format was not worth having, and it could not have.

**It is where the errors the milestone exists to prevent are expressed.** `Fact` refuses to collapse
`Absent` into `Unknown`. `CapabilityState` keeps `Unsupported` apart from `NotObserved`.
`ApplicabilityCoverage` is a claim the capsule carries and cannot upgrade for itself. Those three are
the reason the milestone's primary red test passes: a retrieval system that cannot tell those states
apart finds the deceptive-similarity case with high confidence and transfers anyway. The types make the
error inexpressible rather than merely discouraged, and a type is the cheapest place to keep a rule.

## The cut

`src/transfer.rs` leaves the shipped surface of the crate. `pub mod transfer` is removed from
`src/lib.rs`, and nothing in `src/` depends on the planner.

The file is **moved to `tests/support/transfer_core.rs`, not deleted**, for the reason `b925462` already
recorded when it applied ADR-003: the reversal case is a measurement, and deleting the thing that
produces it would leave the claim unanswerable. Re-running the comparison stays one command.

| Surface | Before | After |
|---|---|---|
| M5 shipped (`src/experience.rs` + `src/transfer.rs`) | 1007 | 323 |
| Change | | −684 lines, −67.9% |
| Against baseline A (515) | 1.96× | 0.63× |

After the reduction the shipped M5 surface is smaller than the baseline that beat it. That is the shape
the trigger asks for: not a Kernel that does the same thing slightly better, but a Kernel that no longer
contains the thing the baseline showed an application can do for itself.

The move touches the import path and nothing else: `crate::experience` becomes `world_kernel::experience`,
because the representation stays in the crate and the procedure no longer does. The measured logic is
carried byte for byte. `TransferAttempt`, `TransferOutcome` and the five `ConditionKind` variants nothing
constructs are carried across as they were, because a move that also rewrites the measured implementation
would no longer be the measured implementation. They are recorded here as known dead surface inside test
support rather than silently dropped.

**Not reduced:** `src/kernel.rs`, `src/model.rs`, `src/adapters.rs`, `src/continuation.rs` (M2),
`src/impact` (M3), `src/branch.rs` (M4). Each of those has its own measurement and its own decision
record, and none of them is covered by this trigger.

## The rules cannot travel in a comment

`b925462` learned this and wrote it down: *"A move alone would have destroyed the rules, because ...
they existed nowhere except inside the module that was measured."* The same is true here. If the Kernel
no longer enforces transfer rules, the consumer who reimplements transfer, which is precisely what this
ADR says a consumer should do, has nothing to be held to.

So the rules move into [TRANSFER-CONTRACT.md](TRANSFER-CONTRACT.md): the five statuses and no boolean;
unknown is never absent and not-observed is never unavailable; a source assurance never becomes a target
assurance; a plan is bound to both revisions; declared coverage is read as a claim and never as a grant.
`the_transfer_contract_and_the_moved_implementation_agree` in `tests/transfer_contract.rs` binds the
document back to the code in both directions, over `TransferStatus`, `UnknownReason` and
`InstantiationError` in `tests/support/transfer_core.rs` and `ApplicabilityCoverage` in
`src/experience.rs`, the same way `the_contract_document_and_the_measured_codes_agree` binds
`IMPACT-CONTRACT.md` to `src/impact`.

The test lives in a file of its own rather than in `tests/transfer_plan.rs` so that adding it does not
rewrite `lineCounts.test` in the recorded artifact. This commit changes no number in
`experiments/transfer-benchmark/`.

A contract document is not shipped surface. It does not count against the line budget this ADR reduces.

## The recorded number survives the move

`tests/transfer_artifact.rs` reads `experiments/transfer-benchmark/results.json` and requires
`baselineA < kernelSurface` and a ratio under 2.0, because the gate was scored on the surface that was
measured. That score is not rewritten.

`examples/transfer_benchmark.rs` therefore counts the mechanism where it now lives: `kernelSurface`
becomes `src/experience.rs` + `tests/support/transfer_core.rs`, which is still 1007. Re-running the
example reproduces the recorded artifact instead of quietly re-scoring the gate with a smaller number.
The field means the lines of the mechanism that were compared, which did not change because a file
moved.

## The reversal case, stated cheaply

ADR-004 exists because ADR-003 wrote a reversal case, called it cheap, and then deferred it until after
the decision had been built. The lesson is that a reversal case nobody is obliged to run is not a
reversal case. This one names the commands.

This reduction is wrong if either of these is observed:

1. **A consumer reads a capsule and reaches a wrong decision without the evaluator.** The whole argument
   for keeping the representation rather than the planner is that the representation is the portable
   part. One consumer outside `tests/` that consumes a capsule and gets transfer wrong, where
   `plan_transfer` got it right, puts the planner back in `src/`. No such consumer existed in this
   workspace when this ADR was written: slices 7 and 8 of the brief are unreachable from here and are
   recorded as not attempted.
2. **A wider corpus separates the two systems.** The brief's 96-case corpus, bounded generative trees
   and the B and C baselines are excluded from tranche 1. If any of them produces a case where A and C
   disagree, the tie was an artefact of 30 hand-designed cases and the comparison must be re-run before
   the reduction stands.

Both checks are an import and a command. Neither needs a milestone, and neither is deferred behind one.

### Condition 1 was executed on 2026-09-28, and did not fire

Pre-registered in [CONSUMER-CHECK-PROTOCOL](CONSUMER-CHECK-PROTOCOL.md), run by
`cargo run --example consumer_check`, recorded in
[experiments/consumer-transfer](experiments/consumer-transfer/RESULTS.md).

Sixteen cases in a domain the existing fixture does not use, each with an expectation declared from the
contract before the consumer was written, measured three ways. The consumer, in its own crate outside
`tests/`, reached the pre-registered decision on all sixteen, and so did `plan_transfer`. Zero reversals,
including the two cases marked contested in advance because the contract is silent or the published surface
is incomplete about them. The planner stays out of `src/`.

Two obligation disagreements were recorded rather than dropped, and neither is a reversal. On case 11 the
capsule carried the key and the consumer did not use it, which is a shortcoming of that consumer. On case
15 the consumer owes an observation the planner does not, which is a question about the contract's silence
on `predicate_ref` and not about the representation.

What this did not do is stronger than what it did. The same session read the planner before writing the
pre-registration and then wrote the consumer, so the pass is necessary and not sufficient. The check that
would change that is a second consumer written by somebody who has not read `tests/`. The comparison was
shown to be able to fail: a control consumer that keeps no unknown state and forgets the recorded failures
is caught on the nine cases whose status turns on a state it cannot represent.

### Condition 2 was executed on 2026-09-28, and it fired

Pre-registered in [WIDE-CORPUS-PROTOCOL](WIDE-CORPUS-PROTOCOL.md), run by
`cargo run --example wide_corpus`, recorded in
[experiments/wide-corpus](experiments/wide-corpus/RESULTS.md).

**It fired.** 26 of 96 cases return a different status from `plan_transfer` than from the competent
baseline, against a threshold of one. So the clause above holds as written: the tie was an artefact of 30
hand-designed cases, and the comparison has now been re-run.

**But the separations are entirely of one kind, and that is the finding.** Every one of the 26 sits on a
case the baseline's model cannot carry: a condition kind outside the three it implements, a string value
where it has integers, the capsule's coverage claim, recorded history, the out-of-band adaptation channel,
or an undeclared target constraint. On the 33 cases both systems can express, the two agree on every one.
A new independent oracle over the full published alphabet sides with the planner on all 26, and with the
planner on all 96.

**What this does and does not decide.** It decides that the recorded tie was scoped, and the recorded
comparison now says so. It does not vindicate the planner. A competent baseline can add the eight missing
kinds, the coverage claim, failure memory and the adaptation channel, and the recorded note already calls
the baseline's line count an over-estimate, so what the extra 492 lines buy here is capability a baseline
could buy more cheaply. This run deliberately cannot show that extending it would tie, because extending it
would change the thing being compared. That question is open, and it is a measurement rather than a
milestone.

**The composition was amended once, and the amendment is recorded.** The first run drew 96 cases and
produced one `expressible` case, which left the primary metric unpowered: a count of zero on one case is
not a measurement, and reading it as the strong row would have been reading a power failure as a pass.
The protocol's own clause about per-kind coverage existing so a hole is visible is what caught it.
Amendment 1 stratified the corpus into a declared 32-case shared subset and 64 cases from the full
distributions, with the metric, the threshold, the projection and the readings untouched. Run 0 is kept
verbatim as `experiments/wide-corpus/results-run0-defect.json`; its generator no longer exists, which is
why it is a file and not a paragraph.

**Both systems were frozen by hash before the corpus was generated.** The baseline is not extended at all,
because widening a corpus until a baseline that models three condition kinds fails would be rigging, and it
is the easiest way to get a dramatic number.

## What this does not do

It does not fund tranche 2. Moving code into test support is not integration, and a reduction is not a
result. The two gate conditions recorded as unreachable, the real IntentLane transfer and Kollio
consuming the same core, stay unreachable rather than becoming failures.

It does not touch M1's negative result, M2's cost gap, M3's coverage or M4's tranche-1 limit. Each of
those was measured against its own pre-registered gate and each keeps the score it was given.

It does not claim the representation was measured valuable. Nothing on this corpus measured that. The
representation is kept because it is the milestone's named subject and because the errors it prevents
are expressed in its types, and the first reversal condition above is the one that would show that
reasoning wrong.
