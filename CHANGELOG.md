# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/unkos-dev/specful/compare/v0.4.0...v0.4.1) - 2026-09-04

### Fixed

- *(skills)* close the planning and execution gaps the first pilot found ([#85](https://github.com/unkos-dev/specful/pull/85))

  - Add a Merge conditions section to the change-plan and arc-plan
  templates for maintainer-owned acts that must happen
    after the pull request is green and before merge.
  - Extend the specful-plan review checklist to require running every
  Verify command, checking absence-asserting checks
  against the current tree, keeping path-keyed ignore entries in step with
  file moves, and searching the whole tree for
    renamed identifiers.
  - Update specful-implement's hand-off rules so a step stays active until
  its pull request merges, reports unrun merge
    conditions, and quotes a tool's verdict before any inferred cause.

  The first adopter pilot ran a plan through the implement skill, and
  every executor stop traced to a gap in the plan
  skill's checklist, the templates, or the implement skill's hand-off
  rules.

## [0.4.0](https://github.com/unkos-dev/specful/compare/v0.3.2...v0.4.0) - 2026-09-04

### Breaking changes

- *(config)* drop the specful-version field ([#83](https://github.com/unkos-dev/specful/pull/83))

  - Remove the `specful-version` field from the config schema, its cases,
  `init`, `Config`, and every repository fixture
  - Drop the corresponding sentence from `docs/configuration.md`
  - Add an Updating section to the adoption page covering the binary, the
  skills, validation after an update, and conversion steps carried by
  release notes

  The field was written by `init` and read by no code path, so it drifted
  from the truth as soon as a migration forgot
  to set it. Existing repositories delete the `specful-version` line from
  `.specful/config.yaml`, because the schema
  rejects unknown keys.

### Other

- *(adoption)* update skills by reinstalling with --force ([#84](https://github.com/unkos-dev/specful/pull/84))

  - Replace the bare `gh skill update` instruction with the install
  command plus `--force`, which refreshes the installed skills and adds
  ones a release introduced.

  `gh skill update` without arguments walks every installed skill and
  prompts for a source repository on each skill installed by other means,
  and it does not add skills that a release introduced. The install
  command with `--force` covers both cases in one step.

- point adoption at harness hooks and record the review contract ([#80](https://github.com/unkos-dev/specful/pull/80))

  - Link the harness hooks blocks from the adoption page's skills section.
  - Add the substantive review contract ([#72](https://github.com/unkos-dev/specful/pull/72)) to the 0.3.2 changelog
  section so it matches the published release notes.

## [0.3.2](https://github.com/unkos-dev/specful/compare/v0.3.1...v0.3.2) - 2026-09-03

### Added

- *(hooks)* add harness hook adapters for validation and review ([#77](https://github.com/unkos-dev/specful/pull/77))

  - Add committed Claude Code (`.claude/settings.json`) and Codex
  (`.codex/hooks.json`) hook adapters that run `specful index --check` and
  `specful validate` after every edit and on stop, and nudge a
  `specful-review` pass before a `git push` touching `docs/specs`,
  `docs/adr`, or `.specful`.
  - Document the hooks in a new `specful-validate` skill reference
  (`harness-hooks.md`) and link it from the skill body and description.
  - Add a "Harness hooks" section to the validation integration reference
  docs, between local hooks and CI.

- *(skills)* add planning and execution workflows ([#73](https://github.com/unkos-dev/specful/pull/73))

  - add `specful-plan` and `specful-implement` as the ninth and tenth
  optional harness-side skills
  - package exact change-plan and arc-plan templates with planning-craft
  guidance
  - update the charter, package Design, canonical templates, attribution,
  and public workflow documentation

  Persistent plans need to give an agent enough exact context to execute
  the work without rediscovery or invention. The
  planning skill now produces complete work orders and separates
  unresolved choices from settled tasks. The implementation
  skill executes one step at a time, records progress in the plan, and
  stops for approval when the repository contradicts
  the plan or correctness requires a deviation. Repository instructions
  continue to own branching, publication, and
  merging.

- *(review)* add substantive review contract ([#72](https://github.com/unkos-dev/specful/pull/72))

  - Define substantive review boundaries for individual Specful artifacts,
  aggregate changes, and correction re-reviews.
  - Add minimum-context Requirement, Design, and ADR lenses plus a compact
  conversational report format.
  - Package-test the exact supporting reference set and document the
  delivered harness-side behaviour.

  The existing review skill was a single checklist. It did not distinguish
  draft from gate-grade review, tie validation evidence to the reviewed
  target, or give correction reviews a bounded contract. The revised skill
  keeps mechanical validation separate from substantive judgement while
  leaving invocation and enforcement policy with adopting maintainers.

- *(skills)* retire Claude marketplace ([#68](https://github.com/unkos-dev/specful/pull/68))

  - remove the Claude marketplace manifests and their dedicated pin gate
  - document `gh skill install` as the single installation channel
  - retain the harness-neutral manifest and Agent Skills validation

  The Claude marketplace duplicated the common skill payload without
  providing an additional capability. One installation channel now serves
  every harness supported by the GitHub CLI while keeping Specful
  responsible only for Agent Skills conformance.

### Other

- *(validation)* document adopter-owned gates ([#70](https://github.com/unkos-dev/specful/pull/70))

  - add a validation integration reference with the canonical read-only
  command sequence
  - show concise examples for an existing local hook and CI job
  - link the reference from Adoption and the documentation sidebar

  Adopters need a clear way to run Specful checks without implying that
  `specful init` installs or owns enforcement. The guidance keeps hook and
  CI policy with each adopting repository.

## [0.3.1](https://github.com/unkos-dev/specful/compare/v0.3.0...v0.3.1) - 2026-09-01

### Added

- *(init)* retire installed instruction files ([#67](https://github.com/unkos-dev/specful/pull/67))

  - Stop `specful init` from creating or modifying agent instruction
  files.
  - Remove the retired instruction templates, repository copy, and managed
  `AGENTS.md` block.
  - Update the charter and public documentation to describe the delivered
  adoption boundary.

  Installed instruction files duplicated the repository configuration,
  schemas, templates, and artifact corpus, and could become a competing
  source of project authority. Agent support now remains opt-in and
  harness-side, consistent with ADR-0005, while the convention stays
  usable through ordinary repository files.

- *(plugin)* add focused authoring and CLI skills ([#64](https://github.com/unkos-dev/specful/pull/64))

  - replace the shared authoring and adoption skills with focused
  Requirement, Design, and ADR authoring skills
  - add concise skills for validation, indexing, catalog lookup, and
  traceability without granting write authority to checks
  - align the charter and ADR with adopter-owned validation and
  substantive-review policy

  Skills should teach artifact-specific craft and make Specful operations
  available when useful without becoming canonical project policy or
  deciding how an adopting repository enforces review.

### Other

- *(readme)* surface the agent skills and refresh the status ([#61](https://github.com/unkos-dev/specful/pull/61))

  - Renames the README's `Two tiers` section to `Agent skills` and names
  the three skills (`specful-author`, `specful-review`, `specful-adopt`)
  with what each does, keeping the install commands and the
  defer-to-repository rule.
  - Drops the `Status` section: the README describes what Specful is and
  how to use it, and release maturity signalling belongs in the release
  notes.

  A reader scanning the README for agent support could miss the plugin
  entirely: the section heading did not mention skills or agents, and the
  skills were never named. The status section narrated delivery progress
  in a document that introduces the current-state writing model.

## [0.3.0](https://github.com/unkos-dev/specful/compare/v0.2.0...v0.3.0) - 2026-08-31

This release replaces the MSRS/MSDD artifact model with first-class Requirement and Design artifacts, following the
accepted ADRs SPECFUL-ADR-0002 through SPECFUL-ADR-0004. It is a direct, pre-1.0 break: there is no compatibility layer
and no automated migration. It also delivers the documentation site at <https://unkos-dev.github.io/specful/> and an
opt-in agent plugin, installed from this repository's own marketplace, carrying skills for authoring, reviewing, and
adopting the convention.

### Breaking changes

- `schemas/msrs/` and `schemas/msdd/` are removed. `schemas/requirement/v1` (`type: REQ`) and `schemas/design/v1`
  (`type: DESIGN`) replace them.
- The ADR schema's discriminator moves from `kind: adr` to `type: ADR`, in place at `profile-version: 1`. Existing ADRs
  must update their frontmatter.
- ADR section headings move to sentence case: `Context and problem statement`, `Decision drivers`, `Considered options`,
  `Decision outcome` (and, where present, `Pros and cons of the options`, `More information`).
- `.specful/config.yaml` counters change from `next-adr-sequence` / `next-msrs-sequence` /
  `next-requirement-sequence` / `next-msdd-sequence` to `next-adr-sequence` / `next-requirement-sequence` /
  `next-design-sequence`. `config-version` stays `1`.
- `specful new msrs` and `specful new msdd` are removed. Use `specful new requirement` and `specful new design`, each
  taking `--scope` as before. Requirements and Designs live under `docs/specs/<scope>/requirements/` and
  `docs/specs/<scope>/design/` respectively (previously `msrs/` and `msdd/`).
- A Requirement is one normative obligation per file (`Statement`, `Rationale`, `Acceptance criteria`,
  `More information`); the old per-module bundled requirements with source citations are gone. A Design describes one
  subject across eight canonical sections (`Purpose and boundaries`, `Structure`, `Interfaces and dependencies`,
  `Data and state`, `Runtime behaviour`, `Failure and recovery`, `Security and operations`, `More information`) and
  declares the Requirements it `satisfies`.

### Manual conversion steps for an existing repository

1. Edit `.specful/config.yaml`: replace `next-msrs-sequence` and `next-msdd-sequence` with `next-design-sequence`, and
   rename any existing Requirement counter to `next-requirement-sequence` if it is not already named that.
2. For every ADR, change `kind: adr` to `type: ADR` and update its `##` headings to sentence case.
3. For every MSRS module, split its bundled requirements into individual `type: REQ` files under a `requirements/`
   directory, one obligation per file, following `templates/requirement.md`.
4. For every MSDD module, convert it to a `type: DESIGN` file under a `design/` directory, following
   `templates/design.md`, and set `satisfies` to the Requirement identifiers it satisfies.
5. Run `specful index` to regenerate the catalog and navigation views, then `specful validate` to confirm the result.

### Added

- *(plugin)* publish the first marketplace pin ([#58](https://github.com/unkos-dev/specful/pull/58))
- *(plugin)* add the Specful agent plugin with an empty marketplace root
  ([#57](https://github.com/unkos-dev/specful/pull/57))
- deliver the documentation site ([#50](https://github.com/unkos-dev/specful/pull/50))
- [**breaking**] implement the Requirement and Design artifact model ([#49](https://github.com/unkos-dev/specful/pull/49))

### Fixed

- *(templates)* stop stamping scaffolded artifacts with an SPDX licence tag ([#56](https://github.com/unkos-dev/specful/pull/56))
- *(deps)* update rust crate jsonschema to 0.52.0 ([#47](https://github.com/unkos-dev/specful/pull/47))

### Other

- adopt the Requirement and Design artifact model ([#48](https://github.com/unkos-dev/specful/pull/48))
- use absolute masthead URLs in the README ([#41](https://github.com/unkos-dev/specful/pull/41))

## [0.2.0](https://github.com/unkos-dev/specful/compare/v0.1.2...v0.2.0) - 2026-08-28

### Other

- add repository badges ([#40](https://github.com/unkos-dev/specful/pull/40))
- apply Specful brand identity ([#38](https://github.com/unkos-dev/specful/pull/38))
- reflow the AGENTS.md block template to fill the 120-column limit ([#37](https://github.com/unkos-dev/specful/pull/37))
- [**breaking**] consolidate specful state under .specful ([#34](https://github.com/unkos-dev/specful/pull/34))
- reflow Markdown prose to fill the 120-column limit ([#35](https://github.com/unkos-dev/specful/pull/35))

## [0.1.2](https://github.com/unkos-dev/specful/compare/v0.1.1...v0.1.2) - 2026-08-27

### Fixed

- render generated views for a zero-artifact repository ([#32](https://github.com/unkos-dev/specful/pull/32))

## [0.1.1](https://github.com/unkos-dev/specful/compare/v0.1.0...v0.1.1) - 2026-08-27

### Fixed

- restrict the published crate to shipped content ([#30](https://github.com/unkos-dev/specful/pull/30))

## [0.1.0](https://github.com/unkos-dev/specful/releases/tag/v0.1.0) - 2026-08-26

### Added

- add change-plan and arc-plan templates ([#24](https://github.com/unkos-dev/specful/pull/24))
- install instruction content at adoption ([#23](https://github.com/unkos-dev/specful/pull/23))
- discover the repository root in every command ([#9](https://github.com/unkos-dev/specful/pull/9))
- answer lookup and trace queries from the catalog ([#5](https://github.com/unkos-dev/specful/pull/5))
- scaffold repositories and artifacts ([#3](https://github.com/unkos-dev/specful/pull/3))
- generate navigation indexes and catalog ([#2](https://github.com/unkos-dev/specful/pull/2))
- implement specful validate ([#1](https://github.com/unkos-dev/specful/pull/1))
- establish specful v0.1

### Fixed

- *(deps)* update rust crate jsonschema to 0.51.0 ([#29](https://github.com/unkos-dev/specful/pull/29))
- *(deps)* update rust crate jsonschema to 0.50.0 ([#14](https://github.com/unkos-dev/specful/pull/14))
- *(deps)* update rust crate saphyr-parser to 0.0.12 ([#12](https://github.com/unkos-dev/specful/pull/12))
- abort index generation on collection errors ([#8](https://github.com/unkos-dev/specful/pull/8))

### Other

- *(deps)* update github/codeql-action digest to cdf488f ([#28](https://github.com/unkos-dev/specful/pull/28))
- *(ci)* record accepted dist installer bootstrap risk ([#27](https://github.com/unkos-dev/specful/pull/27))
- replace unreleased status with install instructions ([#26](https://github.com/unkos-dev/specful/pull/26))
- publish release binaries with cargo-dist ([#25](https://github.com/unkos-dev/specful/pull/25))
- add repository guidelines ([#22](https://github.com/unkos-dev/specful/pull/22))
- *(templates)* drop frontmatter comment guidance ([#21](https://github.com/unkos-dev/specful/pull/21))
- adopt default markdown lint rules ([#20](https://github.com/unkos-dev/specful/pull/20))
- automate releases with release-plz ([#18](https://github.com/unkos-dev/specful/pull/18))
- *(deps)* update dependency jdx/mise to v2026.8.12 ([#16](https://github.com/unkos-dev/specful/pull/16))
- *(deps)* lock file maintenance ([#17](https://github.com/unkos-dev/specful/pull/17))
- *(deps)* update dependency jdx/mise to v2026.8.10 ([#11](https://github.com/unkos-dev/specful/pull/11))
- lint pull request titles and commits ([#10](https://github.com/unkos-dev/specful/pull/10))
- add contributor process and DCO check ([#4](https://github.com/unkos-dev/specful/pull/4))
- *(deps)* update rust to v1.98.0 ([#13](https://github.com/unkos-dev/specful/pull/13))
- add issue forms and pull request template ([#6](https://github.com/unkos-dev/specful/pull/6))
- adopt the justfile-driven pipeline with coverage, lint, and dependency gates
  ([#7](https://github.com/unkos-dev/specful/pull/7))
- match rustfmt import layout
