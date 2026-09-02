---
type: DESIGN
profile-version: 1
id: "SPECFUL-DESIGN-0002"
title: "Harness-side skill distribution"
governed-by:
  - SPECFUL-ADR-0005
---

# Harness-side skill distribution

Specful packages eight harness-side skills in one repository tree and distributes them through `gh skill install`.
Repository release tags identify released skill payloads. The skills add artifact craft, CLI operations, and substantive
review while leaving canonical project knowledge in the adopting repository's ordinary files.

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
- `plugin/skills/<name>/SKILL.md` contains one Agent Skills document per workflow; `specful-review` also ships concise
  Requirement, Design, ADR, and report-format references under its `references/` directory;
- `tests/plugin_package.rs` checks the manifest policy, the exact skill set, directory and frontmatter naming,
  frontmatter shape, and the named, regular, non-empty review references;
- the `skills-ref` preflight recipe validates every skill against the pinned Agent Skills validator.

All eight directory and frontmatter names use the `specful-` prefix because installers place skills from unrelated
packages in shared flat namespaces. The authoring set is `specful-requirement`, `specful-design`, and `specful-adr`.
`specful-review` provides substantive artifact review. `specful-validate`, `specful-index`, `specful-show`, and
`specful-trace` expose the matching CLI operations.

## Interfaces and dependencies

Each skill uses Agent Skills frontmatter for its name, trigger description, and compatibility declaration, followed by
Markdown instructions that a harness loads when the skill applies. The skills require `specful` 0.3.0 or later on
`PATH`. Each skill invokes the CLI where its workflow calls for it. Authoring and review skills also use repository
artifacts and public documentation as source material.

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
Authoring skills scaffold through `specful new`, guide completion of one artifact type, and finish with indexing and
mechanical validation. They point to substantive review when the adopting repository requires it or the user asks for
it. Operation skills run the matching CLI command or commands and report the result. `specful-review` resolves an
artifact, draft, immutable change, or bounded re-review target; validates first; loads only the references for artifact
types in scope; and reports evidence-backed substantive findings with `SHIP`, `CONDITIONAL`, or `NO-SHIP`. Mechanical
validation remains distinct from substantive judgement. The skill returns its compact report in the conversation only;
it does not edit a repository or publish a pull-request comment or formal review. An interactive harness may offer
independent or in-session execution, while the adopting maintainer decides whether review is advisory or blocking.
Non-interactive invocation policy remains outside this package.

## Failure and recovery

Package fixtures and `skills-ref` reject malformed metadata, unexpected skill names, and a manifest that carries an
independent version. Installation failures and unsupported target selections are reported by GitHub CLI, which owns
their recovery behaviour. A skill invocation reports a missing or incompatible `specful` binary through the harness or
command failure rather than installing a binary or changing repository state.

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

[SPECFUL-ADR-0005](../../../adr/0005-deliver-agent-support-as-an-opt-in-harness-side-plugin.md) records the distribution
decision and its alternatives. The [project charter](../../../project-charter.md) defines the portability and
source-of-truth boundaries. The public [Adoption](https://unkos-dev.github.io/specful/adoption/) page carries current
installation instructions.
