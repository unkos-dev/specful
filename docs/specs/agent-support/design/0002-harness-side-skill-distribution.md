---
type: DESIGN
profile-version: 1
id: "SPECFUL-DESIGN-0002"
title: "Harness-side skill distribution"
governed-by:
  - SPECFUL-ADR-0005
---

# Harness-side skill distribution

Specful packages harness-side skills in one repository tree and distributes them through `gh skill install`. Repository
release tags identify released skill payloads. The skills add brownfield onboarding, artifact craft, implementation
planning and execution, CLI operations, and substantive review while leaving canonical project knowledge in the adopting
repository's ordinary files.

## Purpose and boundaries

The package gives agent harnesses selected through GitHub CLI a common set of Specful workflows. The documented
user-scoped installation path writes no harness files into an adopting repository. The package owns the skill files,
their names and responsibilities, package metadata, conformance checks, and the link between repository releases and
skill releases.

The Specful convention and CLI remain the supported floor. Artifacts, configuration, schemas, templates, and generated
views in the adopting repository are authoritative. The package does not own GitHub CLI's supported-harness list,
installation scopes, or on-disk layout. It also does not install hooks, continuous-integration policy, agent
instructions, or any other repository control.

## Structure

The package lives under `plugin/`:

- `plugin/plugin.json` carries harness-neutral package metadata and no package version;
- `plugin/skills/<name>/SKILL.md` contains one Agent Skills document per workflow; `specful-review` ships concise
  Requirement, Design, ADR, change/plan, execution, and report-format references, while `specful-plan` ships change-plan
  and arc-plan templates plus a planning-craft reference;
- `tests/plugin_package.rs` checks the manifest policy, the exact skill set, directory and frontmatter naming,
  frontmatter shape, and the named, regular, non-empty review and planning references;
- the `skills-ref` preflight recipe validates every skill against the pinned Agent Skills validator.

All directory and frontmatter names use the `specful-` prefix because installers place skills from unrelated packages in
shared flat namespaces. `specful-onboard` coordinates brownfield discovery and incremental adoption. The authoring set
is `specful-requirement`, `specful-design`, and `specful-adr`. `specful-plan` plans implementation work,
`specful-implement` executes a named plan one step at a time, and `specful-review` reviews artifacts, plans and changes.
`specful-validate`, `specful-index`, `specful-show`, and `specful-trace` expose the matching CLI operations.

## Interfaces and dependencies

Each skill uses Agent Skills frontmatter for its name and trigger description, followed by Markdown instructions that a
harness loads when the skill applies. Skills that require the CLI declare a `compatibility` line requiring `specful` on
`PATH`, carrying no version number: nothing reads it, and the installer pins skills to a release tag. Authoring,
planning, implementation, and review skills also use repository artifacts and public documentation as source material.
Review can inspect ordinary files without the CLI; available read-only lookups can help retrieve corpus relationships.

Users install the package through the GitHub CLI:

```sh
gh skill install unkos-dev/specful --all --scope user
```

GitHub CLI owns target-agent selection, scope handling, and installation behaviour. The public
[`gh skill install` manual](https://cli.github.com/manual/gh_skill_install) is the compatibility reference rather than a
copied list in this repository.

## Data and state

The committed package tree is the complete skill payload. It contains no harness-specific manifest and no independent
version field. The repository's release tag identifies the released payload at that commit; an exact commit pin can
identify a payload between releases.

When installed with the documented `--scope user` flag, copies live in user-scoped locations managed by the selected
harness and GitHub CLI. They contain no repository-specific knowledge or configuration. `specful init` neither reads nor
writes that installed state, and an adopting repository records no skill version.

## Runtime behaviour

An unpinned install resolves the latest repository release, with the default branch as GitHub CLI's fallback when no
release exists. A user can select an exact release tag or commit with `--pin`. The installer discovers the skill
directories, installs the selected skills for the chosen harness, and records any installer-owned provenance outside the
adopting repository.

At invocation time, the harness matches a request against each skill's description and loads the corresponding body.
`specful-onboard` inspects repository authority and existing Specful state before selecting a bounded increment. It
identifies coherent Design subjects, verifies relevant behaviour, distinguishes documentation drift from implementation
defects and unresolved product decisions, and assesses candidate obligations against the Requirement profile before
drafting. The user directs content and scope and supplies intent that repository evidence cannot establish. Existing
behaviour alone does not make an obligation, and an onboarding increment can contain no Requirements.

The coordinator resumes from an adopter's existing record by separating human rulings from agent proposals, superseded
decisions, delivered artifacts and evidence revisions. It records documentation dispositions and reverifies claims
affected by later source changes. It delegates artifact craft to the authoring skills and uses indexing and validation
for their stated mechanical checks. Substantive review follows adopter policy or an explicit user request.

Authoring skills scaffold through `specful new`, guide completion of one artifact type, and finish with indexing and
mechanical validation. They point to substantive review when the adopting repository requires it or the user asks for
it. Operation skills run the matching CLI command or commands and report the result.

`specful-review` resolves a PR, plan, branch, commit, local change or named artifact and records the inspected scope. It
assesses correctness, technical fitness, relevant standards, simplification and applicable Specful authority, loading
the relevant artifact or change/plan lenses. Plans are judged for coherence and executability without requiring future
implementation artifacts or results. Draft and mutable targets can receive `SHIP`. Review reuses existing evidence;
routine development checks, validation and CI inspection are outside its workflow.

Proposed checks, author assertions, observed results and reviewer inferences remain distinct. Prior findings guide
inspection but need support against the current target. Reviewers assess contrary evidence, existing safeguards and
recovery paths before retaining a finding; severity follows the supported impact, reach and recoverability.

The shared review standard treats warranted missing or stale corpus coverage as advisory, including newly introduced
gaps and updates required by authoring rules. A substantive conflict with an existing obligation is assessed on its
consequence. When implementation and a Design disagree, the reviewer investigates which side needs correction and
recommends an outcome for the user to decide.

Execution defaults to in-session review. Ordinary invocation language selects independent review by one isolated
reviewer or multi-agent review by two. The execution reference defines initial isolation from coordinator conclusions
and other reviewers, including persistent memories and cached reviews, retained context during reconciliation and early
stopping. Multi-agent review has a fixed maximum of three rounds including the first; later rounds occur only when
needed to reach an outcome. Settled findings leave active discussion but remain in the final report, and consequential
unresolved disagreement is explained.

Review is read-only and returns one report in the conversation. Delegated reviewers receive explicit prohibitions on
mutations and development checks in every round. They route concrete evidence requests to the coordinator, which may
undertake a necessary, focused, non-mutating investigation once under existing user authority. The report leads with
`SHIP`, `CONDITIONAL` or `NO-SHIP` when there is enough evidence for a verdict and uses a compact summary table for
multiple findings. Detail scales with severity; minor advisories remain in the table without repeated explanations.
Material uncertainty is explained in prose. Scratch artifact generation, tool downloads and other mutations need
separate authority. The report has no confidence scores or routine validation inventory. Review does not fix or publish;
the adopting maintainer owns whether its recommendation is advisory or blocking. Non-interactive invocation policy
remains outside this package.

The compatible CLI is a prerequisite for CLI-backed authoring. Installed skills, harness hooks and repository gates are
separate, opt-in integration layers. The coordinator inspects existing configuration before offering them as separate
scope. The package neither installs those controls nor decides whether they are advisory or blocking; the adopter owns
that policy. A configured hook or gate is distinct from observed feedback in the target harness, and neither proves
substantive accuracy.

`specful-plan` first decides whether the work needs a saved plan. Before judging the design, it loads planning craft
that separates the invariant from a proposed mechanism and searches for the smallest supported solution. It then loads
the change-plan or arc-plan template selected for the work. A change plan is one coherent work order. An arc divides
several independently deliverable changes into ordered steps, where each step is one pull request and carries its own
context, tasks, rollback, verification, and exit criteria. Every plan carries a Documentation section recording a
disposition for each affected artifact, where unaffected with its reason is a valid entry.

The planning skill follows the adopting repository's plan location and retention policy, and proposes `docs/plans/` only
when no location is established. It asks whether plans are tracked or ignored before writing when tracking policy is
absent. By default, plans are excluded from Git and documentation publishing inputs before writing; placement under
`docs/` does not imply publication intent. The skill seeks approval for any needed ignore or publishing configuration
changes rather than applying them automatically. Plans stay outside the permanent corpus and are not inputs to Specful
indexing or validation.

The planning skill stops after the authorised plan files are written. `specful-implement` executes the named plan one
step at a time under its exact-by-default contract. It stops on a contradiction or required deviation, records the
evidence in the plan, and waits for approval. The implementation skill follows repository-owned branch, commit, and
publication rules and never merges. Neither skill changes an external tracker.

## Failure and recovery

Package fixtures and `skills-ref` reject malformed metadata, unexpected skill names, missing reference files, and a
manifest that carries an independent version. Installation failures and unsupported target selections are reported by
GitHub CLI, which owns their recovery behaviour. A skill invocation reports a missing or incompatible `specful` binary
through the harness or command failure rather than installing a binary or changing repository state.

Skill updates become available with repository releases. To return to a known payload, a user installs an exact tag or
commit. The convention remains usable without any installed skill, so an unavailable installer or harness does not block
direct use of the repository artifacts and CLI.

## Security and operations

Skills execute with the permissions of the agent harness. They carry no credentials, manage no secrets, and add no
repository-side execution hook. The installer and harness form external trust boundaries; exact pins provide payload
identity when a maintainer needs it, while repository tags provide the normal release identity.

Validation has two distinct layers. Repository tests and `skills-ref` establish package shape and Agent Skills
conformance. Clean installation and invocation in real harnesses establish that the distribution path works. Neither
layer proves the quality of a substantive review, which remains a judgement made through `specful-review` under the
adopting maintainer's policy.

## More information

The [project charter](../../../project-charter.md) defines the portability and source-of-truth boundaries. The public
[Adoption](https://unkos-dev.github.io/specful/adoption/) page carries current installation instructions.
