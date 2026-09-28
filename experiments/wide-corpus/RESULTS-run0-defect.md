# Wide corpus: does a wider corpus separate A from C?

Pre-registered in [WIDE-CORPUS-PROTOCOL](../../docs/WIDE-CORPUS-PROTOCOL.md).

Reversal condition, applied literally: "at least one case where C and A return a different TransferStatus, over a 96-case corpus"

96 cases from seed `0x5EED000200000001`. **Separations: 44.** On expressible cases: **0**.

**Outcome `"separation_is_scope"`.** "A and C separate only where the baseline's model cannot carry the case, so the recorded tie was scoped to a narrow corpus. The planner is not vindicated by this, and the reduction is not re-opened on it alone"

## What the corpus actually covers

Bounded: "a flat list of at most four conditions over six keys, no recursion, so a case cannot grow into something undecidable".

| Dimension | Distribution |
|---|---|
| condition kinds drawn | absent 36, capability_available 22, capability_unavailable 27, equals 29, gte 22, lte 23, member_of 26, not_equals 17, predicate_ref 25, present 17 |
| target fact states | absent 88, known 334, undeclared 82, unknown 72 |
| coverage claims | closed_declared 62, opaque 13, partial_declared 21 |
| case shapes | capability_absent_in_a 74, expressible 1, projection_lossy 21 |

Exclusions, expected 0 and actual 0. "every shape the generator can produce has a defined expectation, so no case needed to be dropped to be decidable"

## The three systems

| Id | What | Frozen |
|---|---|---|
| A | the competent baseline (`tests/support/transfer_fixture.rs`) | yes |
| C | the transfer planner (`tests/support/transfer_core.rs`) | yes |
| R | a new independent oracle over the full published alphabet (`tests/support/wide_corpus.rs`) | no |

## Separations, and who the independent oracle sides with

44 of them, attributed: R_with_C 44.

| Case | Shape | C | A | R | Attribution | Kinds |
|---|---|---|---|---|---|---|
| wide-01 | projection_lossy | additional_evidence_required | incompatible | additional_evidence_required | R_with_C | 1 condition(s) [predicate_ref], coverage ClosedDeclared, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-03 | capability_absent_in_a | additional_evidence_required | incompatible | additional_evidence_required | R_with_C | 3 condition(s) [absent, equals, capability_unavailable], coverage ClosedDeclared, adaptation keys 1, recorded failure 0, extra constraint 1, values integer |
| wide-04 | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | R_with_C | 3 condition(s) [equals, gte, member_of], coverage Opaque, adaptation keys 0, recorded failure 0, extra constraint 1, values integer |
| wide-12 | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | R_with_C | 2 condition(s) [equals, lte], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 0, values integer |
| wide-14 | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | R_with_C | 3 condition(s) [member_of, absent, absent], coverage Opaque, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-15 | capability_absent_in_a | directly_reusable | incompatible | directly_reusable | R_with_C | 2 condition(s) [not_equals, gte], coverage ClosedDeclared, adaptation keys 0, recorded failure 0, extra constraint 1, values integer |
| wide-16 | capability_absent_in_a | adaptation_required | insufficient_information | adaptation_required | R_with_C | 1 condition(s) [capability_unavailable], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 0, values integer |
| wide-17 | capability_absent_in_a | insufficient_information | adaptation_required | insufficient_information | R_with_C | 2 condition(s) [not_equals, equals], coverage Opaque, adaptation keys 0, recorded failure 0, extra constraint 1, values integer |
| wide-19 | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | R_with_C | 2 condition(s) [absent, lte], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 0, values text |
| wide-20 | capability_absent_in_a | additional_evidence_required | incompatible | additional_evidence_required | R_with_C | 4 condition(s) [absent, gte, absent, absent], coverage ClosedDeclared, adaptation keys 2, recorded failure 0, extra constraint 0, values integer |
| wide-22 | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | R_with_C | 2 condition(s) [member_of, gte], coverage PartialDeclared, adaptation keys 0, recorded failure 1, extra constraint 1, values text |
| wide-23 | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | R_with_C | 4 condition(s) [absent, predicate_ref, capability_available, gte], coverage Opaque, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-24 | capability_absent_in_a | incompatible | additional_evidence_required | incompatible | R_with_C | 1 condition(s) [gte], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 0, values text |
| wide-26 | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | R_with_C | 4 condition(s) [capability_available, member_of, not_equals, member_of], coverage Opaque, adaptation keys 1, recorded failure 0, extra constraint 0, values integer |
| wide-29 | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | R_with_C | 3 condition(s) [gte, not_equals, lte], coverage Opaque, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-33 | capability_absent_in_a | insufficient_information | additional_evidence_required | insufficient_information | R_with_C | 1 condition(s) [lte], coverage Opaque, adaptation keys 0, recorded failure 0, extra constraint 0, values text |
| wide-34 | projection_lossy | incompatible | additional_evidence_required | incompatible | R_with_C | 4 condition(s) [absent, predicate_ref, lte, absent], coverage ClosedDeclared, adaptation keys 0, recorded failure 0, extra constraint 0, values text |
| wide-35 | capability_absent_in_a | incompatible | additional_evidence_required | incompatible | R_with_C | 4 condition(s) [member_of, member_of, lte, present], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 1, values text |
| wide-36 | capability_absent_in_a | adaptation_required | incompatible | adaptation_required | R_with_C | 3 condition(s) [equals, absent, predicate_ref], coverage ClosedDeclared, adaptation keys 0, recorded failure 0, extra constraint 1, values integer |
| wide-37 | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | R_with_C | 1 condition(s) [member_of], coverage Opaque, adaptation keys 0, recorded failure 0, extra constraint 1, values integer |
| wide-40 | capability_absent_in_a | additional_evidence_required | incompatible | additional_evidence_required | R_with_C | 3 condition(s) [lte, present, capability_available], coverage PartialDeclared, adaptation keys 2, recorded failure 0, extra constraint 0, values integer |
| wide-42 | projection_lossy | additional_evidence_required | incompatible | additional_evidence_required | R_with_C | 2 condition(s) [present, not_equals], coverage ClosedDeclared, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-44 | capability_absent_in_a | adaptation_required | incompatible | adaptation_required | R_with_C | 4 condition(s) [capability_unavailable, absent, predicate_ref, member_of], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 1, values integer |
| wide-46 | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | R_with_C | 1 condition(s) [equals], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 0, values text |
| wide-47 | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | R_with_C | 1 condition(s) [present], coverage Opaque, adaptation keys 0, recorded failure 1, extra constraint 0, values integer |
| wide-49 | capability_absent_in_a | insufficient_information | additional_evidence_required | insufficient_information | R_with_C | 1 condition(s) [capability_available], coverage Opaque, adaptation keys 0, recorded failure 0, extra constraint 1, values text |
| wide-50 | projection_lossy | adaptation_required | incompatible | adaptation_required | R_with_C | 4 condition(s) [member_of, lte, predicate_ref, capability_available], coverage ClosedDeclared, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-51 | capability_absent_in_a | insufficient_information | adaptation_required | insufficient_information | R_with_C | 1 condition(s) [lte], coverage Opaque, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-57 | capability_absent_in_a | additional_evidence_required | incompatible | additional_evidence_required | R_with_C | 3 condition(s) [capability_available, predicate_ref, capability_unavailable], coverage ClosedDeclared, adaptation keys 0, recorded failure 0, extra constraint 1, values integer |
| wide-61 | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | R_with_C | 1 condition(s) [gte], coverage ClosedDeclared, adaptation keys 1, recorded failure 0, extra constraint 0, values text |
| wide-62 | projection_lossy | adaptation_required | additional_evidence_required | adaptation_required | R_with_C | 3 condition(s) [present, capability_available, equals], coverage ClosedDeclared, adaptation keys 0, recorded failure 0, extra constraint 0, values text |
| wide-63 | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | R_with_C | 4 condition(s) [capability_unavailable, not_equals, lte, predicate_ref], coverage PartialDeclared, adaptation keys 0, recorded failure 1, extra constraint 0, values text |
| wide-65 | capability_absent_in_a | adaptation_required | incompatible | adaptation_required | R_with_C | 4 condition(s) [capability_unavailable, absent, predicate_ref, not_equals], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 0, values integer |
| wide-66 | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | R_with_C | 3 condition(s) [gte, lte, equals], coverage ClosedDeclared, adaptation keys 1, recorded failure 0, extra constraint 0, values text |
| wide-68 | capability_absent_in_a | insufficient_information | incompatible | insufficient_information | R_with_C | 1 condition(s) [predicate_ref], coverage Opaque, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-76 | capability_absent_in_a | additional_evidence_required | adaptation_required | additional_evidence_required | R_with_C | 3 condition(s) [predicate_ref, gte, present], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 0, values integer |
| wide-77 | projection_lossy | adaptation_required | additional_evidence_required | adaptation_required | R_with_C | 2 condition(s) [capability_available, absent], coverage ClosedDeclared, adaptation keys 0, recorded failure 0, extra constraint 0, values text |
| wide-79 | capability_absent_in_a | adaptation_required | incompatible | adaptation_required | R_with_C | 3 condition(s) [member_of, lte, predicate_ref], coverage PartialDeclared, adaptation keys 2, recorded failure 0, extra constraint 0, values integer |
| wide-80 | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | R_with_C | 4 condition(s) [absent, absent, capability_unavailable, capability_unavailable], coverage ClosedDeclared, adaptation keys 2, recorded failure 1, extra constraint 0, values text |
| wide-81 | projection_lossy | additional_evidence_required | incompatible | additional_evidence_required | R_with_C | 4 condition(s) [not_equals, gte, capability_unavailable, capability_available], coverage ClosedDeclared, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-83 | capability_absent_in_a | adaptation_required | additional_evidence_required | adaptation_required | R_with_C | 1 condition(s) [capability_available], coverage ClosedDeclared, adaptation keys 0, recorded failure 1, extra constraint 1, values integer |
| wide-84 | capability_absent_in_a | additional_evidence_required | adaptation_required | additional_evidence_required | R_with_C | 3 condition(s) [not_equals, absent, present], coverage ClosedDeclared, adaptation keys 2, recorded failure 0, extra constraint 1, values integer |
| wide-85 | capability_absent_in_a | insufficient_information | additional_evidence_required | insufficient_information | R_with_C | 1 condition(s) [capability_available], coverage Opaque, adaptation keys 0, recorded failure 0, extra constraint 0, values integer |
| wide-95 | capability_absent_in_a | incompatible | additional_evidence_required | incompatible | R_with_C | 4 condition(s) [equals, member_of, capability_unavailable, equals], coverage ClosedDeclared, adaptation keys 1, recorded failure 1, extra constraint 0, values text |

Against the independent oracle, C matched on 96 of 96 cases and A matched on 52.

## Not measured

- retrieval: a capsule is handed to both systems, so the discovery problem is still not exercised
- a competent baseline extended with the eight condition kinds A does not model, which is the question this run deliberately leaves open rather than answering in the direction that flatters either side
- no real consumer and no real transfer: the corpus is generated from declared distributions, not sampled from anything
- no B or C baseline using UNI, so no assurance-integration comparison exists
