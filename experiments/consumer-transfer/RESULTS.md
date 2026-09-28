# Consumer check result

Pre-registered in [CONSUMER-CHECK-PROTOCOL](../../docs/CONSUMER-CHECK-PROTOCOL.md). Outcome **"A"**.

Reversal condition, applied literally: `"consumer(capsule) != expected AND plan_transfer(case) == expected"`

16 cases. The consumer matched the pre-registered expectation on **16**, the planner on **16**.

**Reading.** "the representation carried every pre-registered decision on its own, outside the test harness, and nothing is reintroduced".

| Case | Expected | Consumer | Planner | Consumer right | Planner right | Contested | Reversal |
|---|---|---|---|---|---|---|---|
| case-01 | directly_reusable | directly_reusable | directly_reusable | yes | yes | no | no |
| case-02 | adaptation_required | adaptation_required | adaptation_required | yes | yes | no | no |
| case-03 | incompatible | incompatible | incompatible | yes | yes | no | no |
| case-04 | additional_evidence_required | additional_evidence_required | additional_evidence_required | yes | yes | no | no |
| case-05 | additional_evidence_required | additional_evidence_required | additional_evidence_required | yes | yes | no | no |
| case-06 | incompatible | incompatible | incompatible | yes | yes | no | no |
| case-07 | additional_evidence_required | additional_evidence_required | additional_evidence_required | yes | yes | no | no |
| case-08 | incompatible | incompatible | incompatible | yes | yes | no | no |
| case-09 | insufficient_information | insufficient_information | insufficient_information | yes | yes | no | no |
| case-10 | insufficient_information | insufficient_information | insufficient_information | yes | yes | no | no |
| case-11 | adaptation_required | adaptation_required | adaptation_required | yes | yes | no | no |
| case-12 | adaptation_required | adaptation_required | adaptation_required | yes | yes | no | no |
| case-13 | additional_evidence_required | additional_evidence_required | additional_evidence_required | yes | yes | no | no |
| case-14 | directly_reusable | directly_reusable | directly_reusable | yes | yes | no | no |
| case-15 | additional_evidence_required | additional_evidence_required | additional_evidence_required | yes | yes | yes | no |
| case-16 | adaptation_required | adaptation_required | adaptation_required | yes | yes | yes | no |

## Where the two disagreed

None. On all 16 cases the two procedures reached the same status.

## Obligations, which a status alone does not carry

A status is a decision, and a decision that names nothing is one nobody can act on. The contract's
rules 3 and 4 are about obligations rather than about statuses, so a consumer could agree on every
status in the table above and still drop every obligation a status implies. The four the two
implementations are compared on, and why each one is in the contract:

- an observation for every key nobody established, because not knowing is work owed;
- an observation for every key the source consumed, even under a direct reuse, because a reuse
still has to be re-observed in the target;
- a target constraint the capsule never declared, recorded rather than dropped;
- an assurance bound to the target, which is the only assurance in the plan and is never a source
one.

| Case | Consumer | Planner | Agree |
|---|---|---|---|
| case-01 | ledger.decimal_places, world:ledger-target-a | ledger.decimal_places, world:ledger-target-a | yes |
| case-02 | retention.days, world:ledger-target-b | retention.days, world:ledger-target-b | yes |
| case-03 | ledger.currency, world:ledger-target-c | ledger.currency, world:ledger-target-c | yes |
| case-04 | retention.days, world:ledger-target-d | retention.days, world:ledger-target-d | yes |
| case-05 | retention.days, world:ledger-target-e | retention.days, world:ledger-target-e | yes |
| case-06 | ledger.decimal_places, world:ledger-target-f | ledger.decimal_places, world:ledger-target-f | yes |
| case-07 | export.object_storage, world:ledger-target-g | export.object_storage, world:ledger-target-g | yes |
| case-08 | export.object_storage, world:ledger-target-h | export.object_storage, world:ledger-target-h | yes |
| case-09 | world:ledger-target-i | world:ledger-target-i | yes |
| case-10 | world:ledger-target-j | world:ledger-target-j | yes |
| case-11 | review:repeated-prior-failure, world:ledger-target-k | ledger.decimal_places, world:ledger-target-k | no |
| case-12 | export.window_hours, retention.days, world:ledger-target-l | export.window_hours, retention.days, world:ledger-target-l | yes |
| case-13 | export.window_hours, world:ledger-target-m | export.window_hours, world:ledger-target-m | yes |
| case-14 | world:ledger-target-n | world:ledger-target-n | yes |
| case-15 | ledger.tax_rules_version, world:ledger-target-o | world:ledger-target-o | no |
| case-16 | ledger.currency, world:ledger-target-p | ledger.currency, world:ledger-target-p | yes |

### The two obligation disagreements, read

The statuses agreed everywhere, so by the pre-registered condition neither of these is a
reversal. Both are recorded anyway, because an obligation nobody can act on is the failure mode
a status table hides.

- **case-11**: the consumer owes review:repeated-prior-failure and the planner owes ledger.decimal_places.
- **case-15**: the consumer owes ledger.tax_rules_version and the planner owes none.

On case 11 the capsule carries `known_failures[0].condition_key`, and both sides reached the
same status from it. The planner names the key it has to look at again; the consumer owes a
generic "a prior failure was recorded under these conditions" that a target cannot be checked
against. The information was present in the representation and the consumer did not use it, so
this is a shortcoming of the consumer and not a gap in what ADR-005 kept.

On case 15 the reverse. The condition is a `predicate_ref`, which is not evaluable by either
side. The planner files it as unevaluable and owes nothing, and the consumer owes an
observation of `ledger.tax_rules_version`. The consumer's obligation is the more actionable of
the two, and whether the planner's silence on an unevaluable condition is correct is a question
about the contract, not about the representation. It is not a reversal and it is not a defence
of either side.

## What this result is worth

The threat section of the protocol was written before the run and is not revised by it. The same
session read the planner before writing the pre-registration and then wrote the consumer, so a pass
is necessary and not sufficient, and a second consumer written by somebody who has not read `tests/`
is the check that would make it worth more.

A reversal would have been strong evidence. Nothing here establishes that the representation is
valuable; it establishes only what it did or did not decide on its own.

The comparison is known to be able to fail: `the_comparison_detects_a_wrong_consumer` runs a
consumer that keeps no unknown state, reads an unobserved key as agreement and forgets the recorded
failures, and the pre-registered condition catches it on the nine cases whose status turns on a
state it cannot represent. It agrees on case 07, case 15 and case 16 while still being wrong, which
is recorded in the test rather than counted as evidence.
