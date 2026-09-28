# Contributing to World Kernel

World Kernel is an experimental repository built around pre-registered questions and reproducible
measurements. A useful contribution makes the evidence clearer, repairs a demonstrated problem or
reproduces a result. It does not need to make the project larger.

## Before you start

Read these files in order:

1. [README.md](README.md) for the public overview and current status.
2. [docs/SPEC.md](docs/SPEC.md) for the normative contract and active authorization state.
3. [docs/BOUNDARIES.md](docs/BOUNDARIES.md) for ownership, trust and proof limits.
4. [docs/EXPERIMENT.md](docs/EXPERIMENT.md) for the research question and stop conditions.

The current milestone sequence is complete and no new capability is authorized. Please open an issue
before starting a feature or protocol change. A proposal should name the evidence that would justify
reopening the experiment, the smallest testable change and the result that would stop the work.

Documentation fixes, broken-link repairs, reproduction reports and small bug fixes can go directly to
a pull request when their scope is clear.

## Set up the repository

You need Git, a current stable Rust toolchain and `jq`.

```bash
git clone https://github.com/guillaume-flambard/world-kernel.git
cd world-kernel
cargo test
```

No external database or service is required for the default test suite. SQLite is built through the
bundled `rusqlite` feature. The ignored live UNI contract test is the exception and requires a local UNI
checkout and binary as documented in [docs/SPEC.md](docs/SPEC.md).

## Make a focused change

- Keep World Kernel independent from UNI, Kollio and IntentLane. Integrate through public surfaces.
- Keep domain policy in adapters or experiment fixtures.
- Preserve the separation between impact and truth, assurance and authority, and internal commit and
  external effect.
- Do not edit recorded results by hand. Rerun the example that produces them.
- Add a failing contract test before changing behavior.
- Update `docs/SPEC.md` when a durable requirement or authorization state changes.
- Update `docs/BOUNDARIES.md` when a proved guarantee or trust assumption changes.

## Verify the change

Run the complete local gate before opening a pull request:

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
cargo clippy --manifest-path experiments/consumer-transfer/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path experiments/consumer-transfer/Cargo.toml --check
cargo run --example vertical_slice
cargo run --example admission_benchmark
cargo run --example m2_continuation
cargo run --example m3_revision
cargo run --example incumbent_comparison
cargo run --example m4_branch
cargo run --example transfer_benchmark
cargo run --example consumer_check
cargo run --example wide_corpus
jq empty schemas/world-change-v0.experimental.schema.json
jq empty schemas/experience-capsule-v0.experimental.schema.json
```

Some examples rewrite their checked-in result artifacts. A clean `git status` after the gate is part of
the proof. If a result changed, explain why and include the regenerated artifact in the pull request.

## Pull request checklist

- Describe the observed problem and the evidence for it.
- Keep the change inside the repository's current scope.
- Name the commands you ran and their outcomes.
- Call out any claim that remains unverified.
- Keep generated experiment records separate from editorial changes when practical.

By contributing, you agree that your contribution is licensed under the
[Apache License 2.0](LICENSE).
