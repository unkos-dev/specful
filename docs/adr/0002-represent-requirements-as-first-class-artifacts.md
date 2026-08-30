---
kind: adr
profile-version: 1
id: SPECFUL-ADR-0002
title: "Represent requirements as first-class artifacts"
status: proposed
recorded-on: 2026-08-30
decision-makers:
  - "junkovich"
---

# Represent requirements as first-class artifacts

## Context and Problem Statement

Specful needs a canonical unit for requirements knowledge. The requirements lineage it draws on supports several
packagings: a monolithic specification document, a per-scope module holding requirement blocks, and a requirements-only
workflow of individual requirement files with generated assembly. The requirement is the unit that designs satisfy,
readers retrieve, and reviews change, so the packaging choice decides whether that unit owns its own identity and
metadata or resolves through a container. What is the canonical unit of requirements knowledge?

## Decision Drivers

- Stable, independently retrievable identity for the unit that other artifacts reference.
- Small reviews and concurrent changes without whole-document contention.
- One metadata owner per fact; no container and child duplication.
- Generated views, not authored containers, as the assembly mechanism.
- Consistency with the decision model, where each record is one first-class file.

## Considered Options

- First-class requirement artifacts with generated assembly
- Authored per-scope specification modules holding requirement blocks
- Both forms supported per scope

## Decision Outcome

Chosen option: **first-class requirement artifacts with generated assembly**, because the requirement is the unit every
relationship, retrieval, and review targets, and a container module would add identity and metadata surface without
owning any knowledge of its own.

Each requirement is one canonical Markdown file and one artifact: a stable `PROJECT-REQ-0001`-style identifier,
frontmatter `type: REQ` with optional `governed-by` ADR references, and a complete record of statement, rationale,
acceptance criteria, and supporting information. Those four sections carry canonical headings that are never renamed; a
section with nothing to say keeps its heading and states why it does not apply. There is no standalone verification
section: acceptance criteria carry falsifiability, and where the means of checking a criterion is not obvious, the
criterion states how satisfaction is determined. Files live under the owning architectural scope, for example
`docs/specs/<scope...>/requirements/0001-short-title.md`. Generated indexes assemble requirements for navigation; no
authored container is canonical. A requirement binds the system and is rewritten in place; recording why a durable
choice was made is the role of an ADR, linked through `governed-by`.

### Consequences

- Positive: requirements are directly retrievable, individually reviewable, and referenced without resolving through a
  container.
- Positive: the artifact family is uniform, with one record per file across requirements, designs, and decisions.
- Negative: requirement-heavy scopes hold many small files.
- Negative: content shared by several requirements in a scope needs a home outside any single requirement file, which is
  decided separately.

### Confirmation

`specful new` scaffolds requirements individually with allocated identifiers; `specful validate` enforces the
four-section record with its canonical headings and no standalone verification section; generated indexes list
requirements directly; `show` and `trace` resolve requirement identifiers without an intermediate container.

## Pros and Cons of the Options

### First-class requirement artifacts with generated assembly

- Positive: identity, metadata, and prose have one owner each.
- Positive: matches the one-record-per-file decision model.
- Negative: many small files in requirement-heavy scopes.

### Authored per-scope specification modules holding requirement blocks

- Positive: one document per scope reads as a conventional specification.
- Negative: a container identity that nothing references; module and requirement metadata duplicate ownership.
- Negative: single-obligation edits contend on the whole document.

### Both forms supported per scope

- Positive: authors choose the shape per scope.
- Negative: two canonical storage models with conversion rules, mixed-form policing, and doubled tooling, for a
  container form without a demonstrated need.

## More Information

Informed by the Markdown SRS lineage and decision-record practice attributed in `NOTICE.md`. Reconsider if
requirements-only authoring proves unworkable for a real adopting repository.
