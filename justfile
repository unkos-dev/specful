# Canonical definition of every check for Specful. CI jobs call these recipes
# by name and never restate the command inline, so a gate changes in exactly
# one place. Tool versions come from .mise.toml; the Rust toolchain from
# rust-toolchain.toml.

set shell := ["bash", "-ueo", "pipefail", "-c"]

# List recipes (default target).
_default:
    @just --list

# Format the crate in place.
[group('rust')]
fmt:
    cargo fmt --all

# Verify formatting without writing (the CI form).
[group('rust')]
fmt-check:
    cargo fmt --all -- --check

# Lint every target; warnings are errors.
[group('rust')]
clippy:
    cargo clippy --all-targets --locked -- -D warnings

# Reject broken intra-doc links in the public docstrings.
[group('rust')]
doc-lint:
    RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links" cargo doc --locked --no-deps

# Unused dependencies.
[group('rust')]
machete:
    cargo machete

# Unit and integration tests. nextest runs each test in its own process, so
# a hang or abort names the test instead of taking the harness with it.
[group('rust')]
test *args:
    cargo nextest run --locked {{ args }}

# nextest does not run doctests, so they are a separate recipe and CI step.
[group('rust')]
doctests:
    cargo test --locked --doc

# Test suite under coverage instrumentation; writes lcov.info at the root.
[group('rust')]
cov:
    cargo llvm-cov nextest --locked --lcov --output-path lcov.info

# Advisories, licenses, bans, sources (config: deny.toml). Reads the RustSec
# database over the network, so it stays out of `check`.
[group('rust')]
deny:
    cargo deny check

# Spell check the tree (honours .gitignore and _typos.toml).
[group('lint')]
typos:
    typos

# Markdown lint (config: .rumdl.toml).
[group('lint')]
markdownlint:
    rumdl check .

# Workflow syntax and expression lint.
[group('lint')]
actionlint:
    actionlint -color

# --config is explicit: from a linked worktree zizmor does not discover
# .github/zizmor.yml, and every suppression in it would silently stop applying.
[group('lint')]
zizmor:
    zizmor --config .github/zizmor.yml .

# Secret scan of the full commit history. `gitleaks dir` would also read
# untracked build output, which .gitignore hides from git but not from it.
# CI narrows this to the PR's commit range on pull requests.
[group('lint')]
gitleaks:
    gitleaks git . --no-banner --redact

# Offline Rust gate: what to run before pushing.
[group('aggregate')]
check: fmt-check clippy doc-lint test

# Everything CI runs that can run locally (Snyk needs an account token).
[group('aggregate')]
preflight: check doctests machete deny typos markdownlint actionlint zizmor gitleaks
