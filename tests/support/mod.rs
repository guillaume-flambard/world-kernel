//! The shared test-only modules, compiled by three targets.

//!
//! `corpus_contract.rs` includes `admission_case.rs` directly, `comparative_admission.rs`
//! includes this directory as `support`, and `examples/admission_benchmark.rs` includes it
//! by path. No single one of those targets uses every item, so dead-code analysis is
//! disabled here rather than in any individual module.
#![allow(dead_code)]

#[path = "admission_case.rs"]
pub mod admission_case;
#[path = "application_incremental.rs"]
pub mod application_incremental;
#[path = "assurance.rs"]
pub mod assurance;
#[path = "baseline_a.rs"]
pub mod baseline_a;
#[path = "baseline_b.rs"]
pub mod baseline_b;
#[path = "branch_fixture.rs"]
pub mod branch_fixture;
/// The check-1 comparison and the control that proves it can fail.
#[path = "consumer_check.rs"]
pub mod consumer_check;
#[path = "harness.rs"]
pub mod harness;
#[path = "impact_fixture.rs"]
pub mod impact_fixture;
#[path = "incumbent_comparison.rs"]
pub mod incumbent_comparison;
#[path = "m3_stories.rs"]
pub mod m3_stories;
#[path = "second_domain.rs"]
pub mod second_domain;
#[path = "system_c.rs"]
pub mod system_c;
// The transfer planner. It was `src/transfer.rs` until docs/ADR-005-reduce-to-the-representation.md moved
// it here: the measured result said a competent application gate reaches the same decisions in fewer
// lines, so the procedure is not shipped. It is kept rather than deleted because the reversal case is a
// measurement. The rules are written down for a consumer in docs/TRANSFER-CONTRACT.md.
#[path = "transfer_core.rs"]
pub mod transfer_core;
#[path = "transfer_fixture.rs"]
pub mod transfer_fixture;
/// The wide corpus: the bounded seeded generator and the independent oracle. The reversal condition
/// that uses it is pre-registered in `docs/WIDE-CORPUS-PROTOCOL.md`.
#[path = "wide_corpus.rs"]
pub mod wide_corpus;
