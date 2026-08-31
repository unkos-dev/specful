# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
