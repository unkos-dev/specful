# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/unkos-dev/specful/compare/v0.2.0...v0.3.0) - 2026-08-31

### Added

- *(plugin)* publish the first marketplace pin ([#58](https://github.com/unkos-dev/specful/pull/58))
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
