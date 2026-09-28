# Wide corpus: does a wider corpus separate A from C?

Pre-registered in [WIDE-CORPUS-PROTOCOL](../../docs/WIDE-CORPUS-PROTOCOL.md).

Reversal condition, applied literally: "at least one case where C and A return a different TransferStatus, over a 96-case corpus"

96 cases from seed `0x5EED000200000002`. **Separations: 26.** On expressible cases: **0**.

**Outcome `"separation_is_scope"`.** "A and C separate only where the baseline's model cannot carry the case, so the recorded tie was scoped to a narrow corpus. The planner is not vindicated by this, and the reduction is not re-opened on it alone"

## What the corpus actually covers

Bounded: "a flat list of at most four conditions over six keys, no recursion, so a case cannot grow into something undecidable".

| Dimension | Distribution |
|---|---|
| condition kinds drawn | absent 10, capability_available 37, capability_unavailable 20, equals 41, gte 49, lte 16, member_of 19, not_equals 11, predicate_ref 16, present 14 |
| target fact states | absent 99, known 298, undeclared 83, unknown 96 |
| coverage claims | closed_declared 79, opaque 8, partial_declared 9 |
| case shapes | capability_absent_in_a 50, expressible 33, projection_lossy 13 |

Exclusions, expected 0 and actual 0. "every shape the generator can produce has a defined expectation, so no case needed to be dropped to be decidable"

Strata: R_full_distributions {"capability_absent_in_a":50,"expressible":1,"projection_lossy":13}, S_shared_subset {"expressible":32}.

The decisive stratum's own coverage, so a zero on it cannot be a second power failure: a powered subset that never drew an absent fact, an unknown fact or a capability nobody checked would make a count of zero on it a second power failure, so the decisive stratum's own coverage is reported

- condition kinds: R_full_distributions [absent 10, capability_available 10, capability_unavailable 20, equals 20, gte 21, lte 16, member_of 19, not_equals 11, predicate_ref 16, present 14]; S_shared_subset [capability_available 27, equals 21, gte 28]
- target fact states: R_full_distributions [absent 67, known 200, undeclared 58, unknown 59]; S_shared_subset [absent 32, known 98, undeclared 25, unknown 37]
- target capability states: R_full_distributions [available 46, not_observed 57, unavailable 35, unsupported 54]; S_shared_subset [available 16, not_observed 32, unavailable 23, unsupported 25]

## The three systems

| Id | What | Frozen |
|---|---|---|
| A | the competent baseline (`tests/support/transfer_fixture.rs`) | yes |
| C | the transfer planner (`tests/support/transfer_core.rs`) | yes |
| R | a new independent oracle over the full published alphabet (`tests/support/wide_corpus.rs`) | no |

## Separations, and who the independent oracle sides with

26 of them, attributed: R_with_C 26.

| Case | Shape | C | A | R | Attribution | Kinds |
|---|---|---|---|---|---|---|
| wide-34 | capability_absent_in_a | additional_evidence_required | incompatible | additional_evidence_required | R_with_C | stratum R_full_distributions: 2 condition(s) [capability_unavailable, predicate_ref], coverage PartialDeclared, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-37 | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | R_with_C | stratum R_full_distributions: 4 condition(s) [lte, absent, equals, member_of], coverage Opaque, adaptation keys 0, recorded failure 1, extra constraint 1, values integer |
| wide-38 | capability_absent_in_a | additional_evidence_required | incompatible | additional_evidence_required | R_with_C | stratum R_full_distributions: 3 condition(s) [equals, present, capability_unavailable], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 1, values integer |
| wide-40 | capability_absent_in_a | incompatible | additional_evidence_required | incompatible | R_with_C | stratum R_full_distributions: 3 condition(s) [absent, lte, lte], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 0, values text |
| wide-42 | projection_lossy | additional_evidence_required | incompatible | additional_evidence_required | R_with_C | stratum R_full_distributions: 3 condition(s) [member_of, present, equals], coverage ClosedDeclared, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-46 | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | R_with_C | stratum R_full_distributions: 1 condition(s) [predicate_ref], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 1, values text |
| wide-47 | capability_absent_in_a | additional_evidence_required | incompatible | additional_evidence_required | R_with_C | stratum R_full_distributions: 3 condition(s) [predicate_ref, predicate_ref, capability_unavailable], coverage ClosedDeclared, adaptation keys 1, recorded failure 0, extra constraint 1, values text |
| wide-48 | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | R_with_C | stratum R_full_distributions: 1 condition(s) [member_of], coverage Opaque, adaptation keys 0, recorded failure 1, extra constraint 0, values integer |
| wide-51 | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | R_with_C | stratum R_full_distributions: 3 condition(s) [equals, predicate_ref, equals], coverage Opaque, adaptation keys 0, recorded failure 1, extra constraint 0, values integer |
| wide-52 | capability_absent_in_a | insufficient_information | adaptation_required | insufficient_information | R_with_C | stratum R_full_distributions: 4 condition(s) [equals, lte, capability_unavailable, lte], coverage Opaque, adaptation keys 2, recorded failure 1, extra constraint 0, values integer |
| wide-53 | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | R_with_C | stratum R_full_distributions: 4 condition(s) [absent, gte, predicate_ref, absent], coverage Opaque, adaptation keys 0, recorded failure 1, extra constraint 1, values integer |
| wide-55 | capability_absent_in_a | adaptation_required | incompatible | adaptation_required | R_with_C | stratum R_full_distributions: 2 condition(s) [lte, gte], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 1, values integer |
| wide-64 | capability_absent_in_a | incompatible | additional_evidence_required | incompatible | R_with_C | stratum R_full_distributions: 1 condition(s) [equals], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 1, values text |
| wide-66 | capability_absent_in_a | additional_evidence_required | insufficient_information | additional_evidence_required | R_with_C | stratum R_full_distributions: 1 condition(s) [capability_unavailable], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 0, values text |
| wide-70 | capability_absent_in_a | insufficient_information | additional_evidence_required | insufficient_information | R_with_C | stratum R_full_distributions: 1 condition(s) [capability_available], coverage Opaque, adaptation keys 0, recorded failure 0, extra constraint 1, values integer |
| wide-71 | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | R_with_C | stratum R_full_distributions: 4 condition(s) [member_of, present, member_of, present], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 1, values text |
| wide-72 | capability_absent_in_a | additional_evidence_required | incompatible | additional_evidence_required | R_with_C | stratum R_full_distributions: 2 condition(s) [capability_available, not_equals], coverage ClosedDeclared, adaptation keys 0, recorded failure 0, extra constraint 1, values integer |
| wide-74 | projection_lossy | additional_evidence_required | incompatible | additional_evidence_required | R_with_C | stratum R_full_distributions: 3 condition(s) [member_of, not_equals, not_equals], coverage ClosedDeclared, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-79 | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | R_with_C | stratum R_full_distributions: 2 condition(s) [capability_available, lte], coverage PartialDeclared, adaptation keys 2, recorded failure 1, extra constraint 0, values integer |
| wide-81 | capability_absent_in_a | directly_reusable | adaptation_required | directly_reusable | R_with_C | stratum R_full_distributions: 1 condition(s) [not_equals], coverage ClosedDeclared, adaptation keys 2, recorded failure 0, extra constraint 0, values integer |
| wide-84 | capability_absent_in_a | incompatible | additional_evidence_required | incompatible | R_with_C | stratum R_full_distributions: 2 condition(s) [absent, gte], coverage ClosedDeclared, adaptation keys 1, recorded failure 0, extra constraint 1, values text |
| wide-87 | capability_absent_in_a | insufficient_information | additional_evidence_required | insufficient_information | R_with_C | stratum R_full_distributions: 2 condition(s) [not_equals, capability_available], coverage Opaque, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-89 | capability_absent_in_a | adaptation_required | insufficient_information | adaptation_required | R_with_C | stratum R_full_distributions: 1 condition(s) [capability_unavailable], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 0, values integer |
| wide-90 | capability_absent_in_a | directly_reusable | incompatible | directly_reusable | R_with_C | stratum R_full_distributions: 1 condition(s) [lte], coverage PartialDeclared, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-93 | capability_absent_in_a | insufficient_information | adaptation_required | insufficient_information | R_with_C | stratum R_full_distributions: 2 condition(s) [equals, gte], coverage Opaque, adaptation keys 2, recorded failure 0, extra constraint 1, values integer |
| wide-95 | capability_absent_in_a | additional_evidence_required | incompatible | additional_evidence_required | R_with_C | stratum R_full_distributions: 1 condition(s) [predicate_ref], coverage ClosedDeclared, adaptation keys 0, recorded failure 0, extra constraint 1, values integer |

Against the independent oracle, C matched on 96 of 96 cases and A matched on 70.

## Every case

The disagreements are above. This is the rest, so a reader can check the counts against the run rather than taking them.

| Case | Stratum | Shape | C | A | R | Separates |
|---|---|---|---|---|---|---|
| wide-00 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-01 | S_shared_subset | expressible | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-02 | S_shared_subset | expressible | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-03 | S_shared_subset | expressible | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-04 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-05 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-06 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-07 | S_shared_subset | expressible | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-08 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-09 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-10 | S_shared_subset | expressible | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-11 | S_shared_subset | expressible | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-12 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-13 | S_shared_subset | expressible | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-14 | S_shared_subset | expressible | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-15 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-16 | S_shared_subset | expressible | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-17 | S_shared_subset | expressible | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-18 | S_shared_subset | expressible | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-19 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-20 | S_shared_subset | expressible | adaptation_required | adaptation_required | adaptation_required | no |
| wide-21 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-22 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-23 | S_shared_subset | expressible | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-24 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-25 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-26 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-27 | S_shared_subset | expressible | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-28 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-29 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-30 | S_shared_subset | expressible | incompatible | incompatible | incompatible | no |
| wide-31 | S_shared_subset | expressible | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-32 | R_full_distributions | projection_lossy | incompatible | incompatible | incompatible | no |
| wide-33 | R_full_distributions | capability_absent_in_a | adaptation_required | adaptation_required | adaptation_required | no |
| wide-34 | R_full_distributions | capability_absent_in_a | additional_evidence_required | incompatible | additional_evidence_required | yes |
| wide-35 | R_full_distributions | projection_lossy | adaptation_required | adaptation_required | adaptation_required | no |
| wide-36 | R_full_distributions | capability_absent_in_a | incompatible | incompatible | incompatible | no |
| wide-37 | R_full_distributions | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | yes |
| wide-38 | R_full_distributions | capability_absent_in_a | additional_evidence_required | incompatible | additional_evidence_required | yes |
| wide-39 | R_full_distributions | capability_absent_in_a | incompatible | incompatible | incompatible | no |
| wide-40 | R_full_distributions | capability_absent_in_a | incompatible | additional_evidence_required | incompatible | yes |
| wide-41 | R_full_distributions | projection_lossy | incompatible | incompatible | incompatible | no |
| wide-42 | R_full_distributions | projection_lossy | additional_evidence_required | incompatible | additional_evidence_required | yes |
| wide-43 | R_full_distributions | projection_lossy | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-44 | R_full_distributions | capability_absent_in_a | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-45 | R_full_distributions | capability_absent_in_a | incompatible | incompatible | incompatible | no |
| wide-46 | R_full_distributions | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | yes |
| wide-47 | R_full_distributions | capability_absent_in_a | additional_evidence_required | incompatible | additional_evidence_required | yes |
| wide-48 | R_full_distributions | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | yes |
| wide-49 | R_full_distributions | projection_lossy | incompatible | incompatible | incompatible | no |
| wide-50 | R_full_distributions | capability_absent_in_a | incompatible | incompatible | incompatible | no |
| wide-51 | R_full_distributions | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | yes |
| wide-52 | R_full_distributions | capability_absent_in_a | insufficient_information | adaptation_required | insufficient_information | yes |
| wide-53 | R_full_distributions | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | yes |
| wide-54 | R_full_distributions | expressible | adaptation_required | adaptation_required | adaptation_required | no |
| wide-55 | R_full_distributions | capability_absent_in_a | adaptation_required | incompatible | adaptation_required | yes |
| wide-56 | R_full_distributions | projection_lossy | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-57 | R_full_distributions | projection_lossy | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-58 | R_full_distributions | capability_absent_in_a | incompatible | incompatible | incompatible | no |
| wide-59 | R_full_distributions | capability_absent_in_a | adaptation_required | adaptation_required | adaptation_required | no |
| wide-60 | R_full_distributions | capability_absent_in_a | directly_reusable | directly_reusable | directly_reusable | no |
| wide-61 | R_full_distributions | capability_absent_in_a | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-62 | R_full_distributions | projection_lossy | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-63 | R_full_distributions | capability_absent_in_a | adaptation_required | adaptation_required | adaptation_required | no |
| wide-64 | R_full_distributions | capability_absent_in_a | incompatible | additional_evidence_required | incompatible | yes |
| wide-65 | R_full_distributions | capability_absent_in_a | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-66 | R_full_distributions | capability_absent_in_a | additional_evidence_required | insufficient_information | additional_evidence_required | yes |
| wide-67 | R_full_distributions | capability_absent_in_a | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-68 | R_full_distributions | capability_absent_in_a | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-69 | R_full_distributions | capability_absent_in_a | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-70 | R_full_distributions | capability_absent_in_a | insufficient_information | additional_evidence_required | insufficient_information | yes |
| wide-71 | R_full_distributions | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | yes |
| wide-72 | R_full_distributions | capability_absent_in_a | additional_evidence_required | incompatible | additional_evidence_required | yes |
| wide-73 | R_full_distributions | capability_absent_in_a | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-74 | R_full_distributions | projection_lossy | additional_evidence_required | incompatible | additional_evidence_required | yes |
| wide-75 | R_full_distributions | capability_absent_in_a | incompatible | incompatible | incompatible | no |
| wide-76 | R_full_distributions | capability_absent_in_a | incompatible | incompatible | incompatible | no |
| wide-77 | R_full_distributions | capability_absent_in_a | incompatible | incompatible | incompatible | no |
| wide-78 | R_full_distributions | projection_lossy | incompatible | incompatible | incompatible | no |
| wide-79 | R_full_distributions | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | yes |
| wide-80 | R_full_distributions | capability_absent_in_a | incompatible | incompatible | incompatible | no |
| wide-81 | R_full_distributions | capability_absent_in_a | directly_reusable | adaptation_required | directly_reusable | yes |
| wide-82 | R_full_distributions | capability_absent_in_a | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-83 | R_full_distributions | capability_absent_in_a | incompatible | incompatible | incompatible | no |
| wide-84 | R_full_distributions | capability_absent_in_a | incompatible | additional_evidence_required | incompatible | yes |
| wide-85 | R_full_distributions | capability_absent_in_a | incompatible | incompatible | incompatible | no |
| wide-86 | R_full_distributions | capability_absent_in_a | incompatible | incompatible | incompatible | no |
| wide-87 | R_full_distributions | capability_absent_in_a | insufficient_information | additional_evidence_required | insufficient_information | yes |
| wide-88 | R_full_distributions | projection_lossy | adaptation_required | adaptation_required | adaptation_required | no |
| wide-89 | R_full_distributions | capability_absent_in_a | adaptation_required | insufficient_information | adaptation_required | yes |
| wide-90 | R_full_distributions | capability_absent_in_a | directly_reusable | incompatible | directly_reusable | yes |
| wide-91 | R_full_distributions | capability_absent_in_a | additional_evidence_required | additional_evidence_required | additional_evidence_required | no |
| wide-92 | R_full_distributions | capability_absent_in_a | incompatible | incompatible | incompatible | no |
| wide-93 | R_full_distributions | capability_absent_in_a | insufficient_information | adaptation_required | insufficient_information | yes |
| wide-94 | R_full_distributions | projection_lossy | incompatible | incompatible | incompatible | no |
| wide-95 | R_full_distributions | capability_absent_in_a | additional_evidence_required | incompatible | additional_evidence_required | yes |

## Not measured

- retrieval: a capsule is handed to both systems, so the discovery problem is still not exercised
- a competent baseline extended with the eight condition kinds A does not model, which is the question this run deliberately leaves open rather than answering in the direction that flatters either side
- no real consumer and no real transfer: the corpus is generated from declared distributions, not sampled from anything
- no B or C baseline using UNI, so no assurance-integration comparison exists
