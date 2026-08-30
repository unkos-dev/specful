---
type: ADR
profile-version: 1
id: SPECFUL-ADR-0004
title: "Treat Open Knowledge Format as an influence"
status: accepted
recorded-on: 2026-08-30
decided-on: 2026-08-30
decision-makers:
  - "junkovich"
---

# Treat Open Knowledge Format as an influence

## Context and problem statement

Specful's repository format shares visible ancestry with the Open Knowledge Format: Markdown knowledge files with
structured frontmatter, hierarchical organisation, and progressive-disclosure indexes. That ancestry invites a question
the project must answer deliberately: does Specful conform to OKF as a specification, inheriting its conformance model,
reserved filenames, version declarations, and tolerance rules, or does it credit OKF as an influence while owning every
rule itself? OKF publishes its specification on a moving branch without releases, so any conformance claim could only
ever pin a commit.

## Decision drivers

- Documentation must describe behaviour that exists; claimed conformance without enforcement is drift.
- Every Specful rule should be justified by the charter and demonstrated workflows, not by inheritance.
- Attribution of genuine influence must remain.
- No obligation to track upstream changes should be created without value in return.

## Considered options

- Influence only: attribute OKF, own every rule
- Conform to a pinned OKF snapshot
- Leave the relationship undefined

## Decision outcome

Chosen option: **influence only: attribute OKF, own every rule**, because a conformance commitment to an unversioned
upstream would create tracking and implementation obligations with no consumer, while everything of value in the
ancestry stands on its own merits as a Specful rule.

OKF's influence remains attributed in `NOTICE.md`. The rules Specful keeps, it keeps as its own: `index.md` is reserved
for generated navigation, every specification artifact declares a non-empty `type`, author-owned root indexes are
refused rather than overwritten, and an unrecognised `type` under `docs/specs/` is a validation error. Specful adopts no
OKF conformance apparatus: no two-stage conformance result, no `okf_version` declaration, no generic-concept tolerance,
and no `log.md` update-log convention, since an update log restates history that Git already owns. Validation reports a
single Specful conformance result.

### Consequences

- Positive: documentation and validator agree; every rule has a Specful justification.
- Positive: no tracking obligation against an unversioned upstream.
- Negative: external OKF consumers cannot rely on `docs/specs/` conforming to any OKF version.
- Negative: a future shared-context concept file requires its own type decision rather than falling back on generic
  tolerance.

### Confirmation

No repository document claims OKF conformance, inheritance, or a bundle boundary; `NOTICE.md` retains the attribution;
the validator reserves only `index.md` and rejects unrecognised types; no `okf_version` or `log.md` handling exists in
code or documentation.

## Pros and cons of the options

### Influence only: attribute OKF, own every rule

- Positive: honest documentation; each rule earns its place.
- Negative: forgoes any interoperability claim toward OKF consumers.

### Conform to a pinned OKF snapshot

- Positive: a defined interoperability target for OKF-aware consumers.
- Negative: requires building a two-stage conformance model, tolerance semantics, and log validation that no Specful
  workflow needs, against an upstream with no immutable release.

### Leave the relationship undefined

- Positive: no work.
- Negative: ancestry without a stated boundary reads as implied conformance and misleads adopters and agents.

## More information

Reconsider if OKF adopts stable releases and a concrete interoperability need appears.
