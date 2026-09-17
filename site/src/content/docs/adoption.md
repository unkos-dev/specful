---
title: Adoption
description: Bring the Specful convention into a repository, greenfield or existing.
---

Adopting Specful means installing its convention (directory layout, templates, and writing model) into a repository,
then keeping requirements, designs, and decisions current as the repository changes.

Pin the binary and skills to the same release. Follow [Updating](#updating) for conversion and rollback procedures.

## Install

Choose a version from the [GitHub releases](https://github.com/unkos-dev/specful/releases) and read its release notes.
Replace `<VERSION>` with that release's version number, without the tag's `v` prefix, to install from crates.io:

```sh
cargo install --locked --version <VERSION> specful
```

Prebuilt archives for Linux (static musl), macOS, and Windows are published with each release; see the
[GitHub releases](https://github.com/unkos-dev/specful/releases) page. Each archive ships with a SHA-256 checksum.

## Initialise a repository

First inspect `docs/adr/` and `docs/specs/`: these are reserved corpus locations, not directories where arbitrary legacy
documents can remain unconverted. Empty directories are fine, but existing documents and hand-written indexes need an
adopter-approved relocation or conversion before initialisation. `init` does not convert them. Follow
[Adopting into an existing repository](#adopting-into-an-existing-repository) when either location is already in use.

Run `specful init` once, from the repository root, with a project key:

```sh
specful init --project-key MYAPP
```

The project key is 2 to 10 uppercase letters or digits, starting with a letter, and prefixes every identifier `specful`
allocates for this repository (for example `MYAPP-REQ-0001`). The key is immutable once chosen: it is recorded in
`.specful/config.yaml` alongside the monotonic per-kind identifier counters, so identifiers are never reused and never
depend on scanning the tree.

`init` creates:

- `.specful/config.yaml`: canonical configuration, the root-discovery sentinel for every other command.
- `docs/specs/`: the root of the requirements and design corpus.
- `docs/adr/`: the flat directory for Architecture Decision Records.
- Empty generated navigation views for the new artifact corpus.

`init` does not create or modify agent instruction files.

## Validate

Run `specful validate` when a corpus change is ready to check:

```sh
specful validate
```

Validation is mechanical and covers three layers: relationship integrity (identifiers resolve, `satisfies` targets
exist, supersession links agree, generated views match their sources), metadata shape (frontmatter conforms to the
artifact's JSON Schema profile), and document structure (canonical headings present, requirement records well-formed,
template placeholder text absent, and every requirement statement uses at least one uppercase BCP 14 keyword).
Diagnostics are human-readable text with a meaningful exit status; a `--json` flag emits a plain machine-readable
listing that is explicitly unstable.

This validates the current snapshot, not Git history. It cannot prove that the project key never changed, that counters
never decreased or that a deleted identifier was never reused. Review those historical guarantees when integrating
branches; a passing snapshot alone does not establish a valid transition.

The [validation integration reference](/specful/reference/validation-integration/) explains the read-only checks and
provides copyable Lefthook pre-push and continuous-integration examples. Specful does not install or require either
control.

## Install agent skills (optional)

The convention and CLI work without agent skills. The optional skills add workflows for authoring, planning,
step-by-step implementation, reviewing, indexing, validation, and retrieval. They are installed once per user into the
selected agent harness and never written into an adopting repository.

Install the skills at user scope. When run interactively, the GitHub CLI prompts for the target agent:

```sh
gh skill install unkos-dev/specful --all --scope user --pin <TAG>
```

Specful validates the package against the Agent Skills specification. The GitHub CLI owns the supported-agent list and
scope behaviour. For non-interactive installation, add `--agent` with a value from the current
[`gh skill install` manual](https://cli.github.com/manual/gh_skill_install). Replace `<TAG>` with the exact tag of the
release chosen for the binary, including its `v` prefix. Without `--pin`, installation selects the latest release, or
the default branch when no release exists, so it does not reproduce an earlier installation.

The skills use the CLI, configuration, schemas, templates, and repository artifacts as ground truth. The
`specful-validate` skill also carries the [harness hooks](/specful/reference/validation-integration/#harness-hooks)
examples that ask for substantive review before a push from Claude Code or Codex. Automatic mechanical checks belong in
Git pre-push and CI, rather than after each edit or agent turn.

## Updating

Before updating, preserve a clean pre-conversion Git revision, the previous binary and its skill release pin. Read the
conversion notes for every intervening release. For example,
[v0.3.0](https://github.com/unkos-dev/specful/releases/tag/v0.3.0) replaced the artifact model and configuration
counters, while [v0.5.0](https://github.com/unkos-dev/specful/releases/tag/v0.5.0) removed ADR fields without changing
`profile-version: 1`. Specful provides no automated migration.

Install the chosen version with an explicit pin, or download its prebuilt archive. Replace `<VERSION>` and `<TAG>` in
these commands with the version number and matching release tag as described under Install:

```sh
cargo install --locked --version <VERSION> specful
```

Update the skills with the matching tag and `--force`, which overwrites the installed copies and adds any skill the
release introduced:

```sh
gh skill install unkos-dev/specful --all --scope user --pin <TAG> --force
```

Apply the release's manual conversion steps on a branch, then run `specful index` and `specful validate` before merging
the converted corpus. To roll back, preserve any later work, restore the pre-conversion Git state including its
configuration, artifacts and generated views, and use the previous binary and matching skills. Downgrading the binary
alone does not undo a format conversion. Harness hook blocks are copied by hand, so copy them again only when the
reference changes.

## Adopting into an existing repository

Specful does not generate a specification from an undocumented codebase. Bring an existing repository under the
convention incrementally:

1. Inspect the reserved `docs/adr/` and `docs/specs/` locations and agree where legacy material will live. Move
   originals to an archive outside both roots without changing their bytes, preserving their provenance in Git. Keep
   them there during any deliberate conversion; only documents conforming to the current profiles belong inside the
   corpus. If `.specful/config.yaml` already exists, follow Updating instead of rerunning `init`.
2. Run `specful init` after clearing legacy material and hand-written indexes from the reserved locations.
3. Start with the requirements and designs that matter most for the next piece of work, using `specful new requirement`
   and `specful new design`, rather than attempting to document everything at once.
4. Keep a decision record that predates the profile outside the reserved corpus, in its own shape: an accepted record is
   never edited into the profile. Bring one into the profile by re-recording it, either on demand when a Requirement or
   Design needs to cite it through `governed-by`, or through a deliberate review that retires the records kept only by
   ceremony and re-records the survivors. Either way, scaffold the new record with `specful new adr`, carry the original
   decision date in `decided-on` and the re-recording date in `recorded-on`. Under More information, identify the
   archived original by title and link to its archive path. An optional pre-adoption path is historical context, not the
   provenance link. Retire the older record under its own convention while retaining the archived original unchanged at
   the linked path; record retirement separately if that convention would otherwise modify or remove it.
5. Run `specful index` to regenerate the navigation views, then `specful validate`, and commit both the source documents
   and the regenerated views together. A repository formatter that rewrites JSON or Markdown needs ignore entries for
   `.specful/generated/` and for every `index.md` that `specful index` writes under `docs/specs/` in the same commit,
   because a reformatted view no longer matches its sources and fails validation as stale.

A partially documented repository is a valid, ongoing state: validation checks the documents that exist, and does not
require full coverage of the codebase.

### Agent-led brownfield onboarding

Ask `specful-onboard` to investigate one coherent subject, or a deliberately broad baseline, before drafting artifacts.
The agent inspects repository authority, existing documentation, current behaviour, tests and Specful state, then
proposes a bounded increment. It separates verified behaviour, false draft claims, stale documentation, implementation
defects and unresolved product decisions. It also assesses candidate obligations against the Requirement profile;
observed behaviour alone does not make a Requirement, so a useful increment may contain none.

The maintainer directs content and scope and supplies intent that the repository cannot establish. Once the proposed
increment is authorised, the agent uses the type-specific skills for artifact craft and reports any later evidence that
changes the approved meaning. Existing approval remains valid unless the scope or intent changes. A partially covered
repository remains a valid outcome.

When starting adoption, the agent first applies the legacy-material and ADR provenance rules above. When resuming, it
uses the adopter's existing record and distinguishes human rulings from agent proposals, superseded decisions, delivered
artifacts and evidence revisions. The primary method lives in the `specful-onboard` skill rather than a second public
procedure.

A compatible Specful CLI is required before CLI-backed authoring. Installed skills, harness hooks and repository gates
are separate opt-in integrations. Inspect and preserve existing configuration before adding any of them. Configuration
does not prove that feedback was demonstrated in the target harness, and mechanical checks do not prove substantive
accuracy.

## Use specifications during development

The [authoring workflow](/specful/authoring-workflow/) covers ordinary code changes and human onboarding as well as
artifact writing. It does not require a saved plan or a new artifact for every task.

An adopter can add a reference to their existing contributor or harness instructions after reviewing it against local
policy. For example:

```markdown
For code changes, follow the [Specful development workflow](https://unkos-dev.github.io/specful/authoring-workflow/).
Start at docs/specs/index.md, inspect the relevant artifacts and implementation, and propose worthwhile missing
documentation for a decision. Record implementation verification with the change. For substantive review, reuse that
evidence and investigate concrete concerns; missing or stale corpus coverage is advisory, while substantive obligation
violations can hold the work.
```

The adopter owns whether and where this guidance belongs; `specful init` does not install it or change instruction
files. Contributors can use the same workflow directly without an agent harness.

### Review changes, plans and artifacts

`specful-review` assesses correctness, technical fitness and relevant standards. It reports substantive defects and
useful advice, including warranted corpus updates. Review runs in the current session by default. Select another mode in
ordinary language:

- "Use specful-review on this plan."
- "Use specful-review on PR #123 with an independent reviewer."
- "Use specful-review on this branch with multi-agent review."

Independent mode uses one isolated reviewer. Multi-agent mode starts with two independent reviewers and reconciles only
when needed, with a fixed maximum of three rounds including the first. Review is read-only; delegated reviewers do not
run development checks.

## Reconciling branch allocations

The allocation lock protects one checkout; independent branches can allocate the same identifier. Preserve identifiers
already established on the target branch and re-scaffold an unpublished conflicting artifact:

1. Move the unpublished artifact outside `docs/adr/` and `docs/specs/`, preserving its content and identifying its
   incoming references.
2. Update the working branch from the target branch. Keep the project key and resolve each counter to at least its value
   on either branch and above every retained identifier; do not reset counters to close gaps.
3. Use `specful new` to allocate a fresh identifier. Copy the saved content into that scaffold while retaining its new
   identifier, then repair incoming relationships and links to its old path. A duplicate identifier can refer to two
   different subjects, so inspect each reference rather than globally replacing it.
4. Run `specful index` and `specful validate` on the combined snapshot, then review the identities and relationships in
   the diff before merging.

If both conflicting identities are already established, stop for an explicit reconciliation decision. Snapshot
validation cannot decide which subject owns an identity or prove historical non-reuse.

## What Specful does not do

Specful is not a hosted requirements-management service, a relational or graph database, or a replacement for Git
history, issue tracking, or source code. It does not generate a complete specification from an undocumented codebase,
and it does not become a harness-specific source of truth: harness adapters may generate native skills, commands, or
context files, but those are generated integration surfaces, never divergent copies of project policy.
