# Workflows

Each workflow here is standalone: it triggers on `pull_request`, `push` to
`main`, and `merge_group`, grants `contents: read` at the top level, and
cancels superseded runs on every ref except `main`. There is no caller
workflow, no path filtering, and no aggregate gate; branch protection lists
the job names directly.

## The justfile is canonical

Every check CI runs is a recipe in the repository-root `justfile`, and a job
invokes it as `just <recipe>`. Workflows never restate a command inline, so a
gate changes in one place and `just preflight` locally runs what CI runs.
Non-cargo tools are pinned in `.mise.toml` and installed through the
`.github/actions/setup` composite action; the Rust toolchain comes from
`rust-toolchain.toml` through `.github/actions/rust-toolchain`. A tool appears
in both a recipe and a job, or in neither.

## Layout

| Workflow | Jobs | Purpose |
| --- | --- | --- |
| `rust.yml` | `checks`, `tests`, `msrv` | Formatting, clippy, doc links, unused dependencies, doctests; the test suite under coverage; a compile on the declared `rust-version`. |
| `lint.yml` | `workflows`, `prose`, `secrets` | actionlint and zizmor over this directory; typos and rumdl over the tree; gitleaks over the PR commit range or the working tree. |
| `deps.yml` | `cargo-deny`, `review` | Advisories, licenses, bans, and sources from `deny.toml`; GitHub dependency review on pull requests. |
| `snyk.yml` | `code` | Snyk Code SAST, advisory only: findings upload to code scanning and never fail the job. |
| `label.yml` | `label` | Applies `area/*` and `dependencies` labels from `.github/labeler.yml`. Runs on `pull_request_target` so the config on `main` is authoritative. |

Every third-party action is pinned to a full commit SHA with a trailing
version comment that Renovate keeps current. `.github/zizmor.yml` records the
one audit rule that is disabled and why.

## Secrets

| Secret | Used by | When absent |
| --- | --- | --- |
| `CODACY_PROJECT_TOKEN` | `rust.yml` / `tests` | The Codacy upload step is skipped with a notice; the job still passes and the `lcov.info` artifact is still uploaded. The step is also skipped on fork and bot pull requests, which carry no secrets. The upload itself is `continue-on-error`, so a Codacy outage never fails the job. |
| `SNYK_TOKEN` | `snyk.yml` / `code` | The scan and SARIF upload steps are skipped with a notice and the job passes. The job is skipped entirely on fork pull requests. |

`GITHUB_TOKEN` is the only other credential in use: zizmor reads public
action metadata with it, the labeler writes labels with a job-scoped
`pull-requests: write`, and the SARIF upload uses `security-events: write`
scoped to the Snyk job.
