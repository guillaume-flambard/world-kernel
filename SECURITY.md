# Security policy

World Kernel admits changes and records them as evidence. A flaw in that evidence is a flaw in
whatever later relies on it, so the trust boundary is the thing worth reporting.

This is an experimental research prototype. It has no release and is not published to crates.io.
Please read [the honest scope limits](README.md#scope-limits) before deciding whether a finding is a
vulnerability or a documented limit.

## What a failure actually costs

The Kernel's claim is that a second actor can accept, reject or resume an exact change from recorded
evidence without trusting the first actor. A defect in the binding between a change and the evidence
about it breaks exactly that claim, and it fails quietly: the reconstruction still runs, the receipt
still prints, and the answer is wrong.

Concretely, a real failure looks like one of these:

- A change is admitted against an assurance result that actually covered **different bytes**. The
  stored evidence names a candidate, but a different candidate was verified. Every later replay
  inherits the error and reports it as a verified fact.
- Something can mint its own authority. Producers are not trusted to mint authority, and a World
  names the assurance providers its owner trusts. If a proposal can install its own provider, the
  whole admission result is decorative.
- A rejected submission still changes state, or a replay has an effect it should not have. Replay
  must read recorded events only, with no adapter, model or external call.
- Reconstruction bootstraps its own trust, so an attacker controls the input and the check on it.

## Reporting a vulnerability

Do not open a public issue.

Use GitHub private reporting: the **Security** tab, then **Report a vulnerability**. Private
vulnerability reporting is enabled on this repository, so that link works. If it is unavailable to
you for any reason, open a minimal public issue saying only that you have a security report and how
to reach you, with no detail, and it will be moved to a private channel.

A useful report contains what an attacker can do that they should not be able to, the smallest
reproduction, the commit or version you tested, and whether the flaw needs a hostile producer, a
hostile assurance provider, a hostile adapter, or only untrusted code running in the host process.

## In scope

[docs/BOUNDARIES.md](docs/BOUNDARIES.md) records the trust model and the invariant table. The cases
that matter most:

- **Subject binding.** An assurance result must not be accepted for anything other than the exact
  candidate it covers. Bundle evidence path and digest must match, and admission must recheck the
  subject and digest rather than trusting the stored assessment.
- **Self-authorization.** Trusted assurance providers come from World bootstrap state. A manifest or
  an evaluator may never award itself a stronger trust profile than the configuration grants, and a
  refusal must stay a refusal rather than silently downgrading to a self-declared actor.
- **Evidence that lies.** A result that looks valid while its identity, scope or observed subject does
  not cover the claim: wrong revision, wrong workspace-relative path, wrong digest, truncated output,
  or a cache entry reused across a different context.
- **Atomicity.** Projection and event must share one SQLite transaction, so no failure leaves half a
  change. Restart recovery must reach the same state a clean run reaches.
- **Category collapse.** Impact must not become truth, assurance must not become authority, an
  internal commit must not become an external effect, and an unknown fact must not collapse into an
  absent one. These are documented invariants with no runtime flag, so a regression shows up as
  changed behaviour rather than an error.
- **Replay purity.** Replay must not invoke a model, an adapter or any external effect.

Reports are especially welcome with a failing case the suite does not already cover.
`tests/kernel_contract.rs`, `tests/uni_collector.rs` and `tests/m3_comparison.rs` are the
highest-value places to look.

## Out of scope

- **The host process and the configured UNI binary are trusted by design.** This is the documented
  mono-authority profile. `AcceptedAssessment` is a data type, not a signature, so untrusted code
  running inside the host can mint provider name `uni` directly. That is a stated boundary, not a bug.
- **The A-B-A filesystem race is a known, documented limit.** Without an immutable snapshot or an
  operating system lock, a candidate can change and return to its original bytes between the two
  Kernel hashes. It is recorded in [docs/BOUNDARIES.md](docs/BOUNDARIES.md). A *new* way to widen it,
  or a way around the existing hash-before-and-after check, is in scope. The existing race is not.
- Vulnerabilities in dependencies, unless this project's use of one turns a bug into a
  trust-boundary failure. Report those upstream and say so here if it affects this project.
- Anything that requires an attacker who already controls the trust configuration.
- Denial of service from an intentionally expensive input on your own machine.

## What to expect

This is a solo-maintained project, so honesty is worth more here than a service-level promise.

Expect acknowledgement when the report is read, an assessment of severity and of whether it is
really in scope, and then either a fix or a public explanation of why it is not one. Fixes land with
a regression test, and any advisory names the commit that contains it. There is no guaranteed response
time, and none is claimed. Please give a reasonable window before publishing, and say so if a
deadline matters to you.
