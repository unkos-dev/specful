# Workflows

`ci.yml` is a thin caller. It owns the triggers (`pull_request`, `push` to
`main`, `merge_group`), top-level `permissions: contents: read`, concurrency
(cancels superseded runs on every ref except `main`), and one job per concern,
each `uses:` on a `workflow_call`-only file below. There is no path filtering
and no aggregate gate; branch protection lists the resulting per-job check
contexts directly.

## The composition pattern

A job in a called workflow reports as `<caller job> / <called job>`. The
`rust` caller job invoking a `checks` job inside `rust.yml` produces the check
run `rust / checks`. Required contexts are therefore namespaced by
construction, and branch protection lists them per job rather than as one
aggregate.

### Two invariants that will block every pull request if violated

**A caller job in `ci.yml` never gains an `if:` condition.** A skipped caller
reports a check run named after itself (`rust`), but branch protection
requires `rust / checks`. That context never reports. Filtering, where a
concern needs it, lives inside the called workflow, where a skipped job
reports success and satisfies its own context.

**No workflow carrying required contexts gets a top-level `paths:` trigger.**
A workflow that never triggers reports nothing at all, which is not the same
as reporting success. This repository runs no path filtering, so the concern
does not arise, but the rule stays load-bearing if one is ever added.

A caller job in `ci.yml` also never declares `name:`, because the context
prefix is the caller job's own id. Adding one would silently rename every
required context that caller owns into strings that never report again.

We deliberately run no `changes` detectors and no `ci-gate` backstop: with no
path filtering, per-job contexts are the entire enforcement surface.

## The justfile is canonical

Every locally runnable gate is a recipe in the repository-root `justfile`, and
a job invokes it by name from CI, so a gate changes in one place. `just
preflight` does not fully match CI: CI also runs the MSRV compile inline
(inline because it asserts the `RUSTUP_TOOLCHAIN` override took effect), a
gitleaks scan narrowed to the PR commit range (`preflight` scans full
history), coverage instrumentation via `just cov` (`preflight` runs plain
`test`), and a Snyk Code scan (needs `SNYK_TOKEN`, so it cannot run locally).
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

## Required contexts

Branch protection lists these exact `<caller> / <job>` strings:

- `rust / checks`
- `rust / tests`
- `rust / msrv`
- `lint / workflows`
- `lint / prose`
- `lint / secrets`
- `deps / cargo-deny`
- `deps / review`

`snyk` is advisory (its findings never fail the job) and `label` runs outside
the composition, so neither carries a required context.

## Permissions and secrets

A called workflow can only downgrade its caller's token, never elevate it, so
a caller job's `permissions:` must be the union of what its jobs declare. The
`snyk` caller job is the one exception to the top-level `contents: read`
default: its `code` job needs `security-events: write` to upload SARIF, so
`ci.yml` grants that at the caller job, mirroring the job-level grant already
in `snyk.yml`.

| Secret | Used by | When absent |
| --- | --- | --- |
| `CODACY_PROJECT_TOKEN` | `rust` / `tests` | The Codacy upload step is skipped with a notice; the job still passes and the `lcov.info` artifact is still uploaded. The step is also skipped on fork and bot pull requests, which carry no secrets. The upload itself is `continue-on-error`, so a Codacy outage never fails the job. |
| `SNYK_TOKEN` | `snyk` / `code` | The scan and SARIF upload steps are skipped with a notice and the job passes. The job is skipped entirely on fork pull requests. |

`GITHUB_TOKEN` is the only other credential in use: zizmor reads public
action metadata with it, the labeler writes labels with a job-scoped
`pull-requests: write`, and the SARIF upload uses `security-events: write`
scoped to the `snyk` caller job and the `code` job inside `snyk.yml`.
