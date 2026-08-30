---
kind: adr
profile-version: 1
id: SPECFUL-ADR-0003
title: "Represent design as subject-oriented Design artifacts"
status: proposed
recorded-on: 2026-08-30
decision-makers:
  - "junkovich"
---

# Represent design as subject-oriented Design artifacts

## Context and Problem Statement

Specful needs a canonical unit for current design knowledge. The design-description lineage it draws on models
stakeholder concerns, viewpoints, and views, and supports packagings from an assembled description document to
independently stored per-viewpoint views. Design knowledge is interwoven: a subject's structure, data, and runtime
behaviour explain one another, and its readers include developers, maintainers, adopters, and self-hosters asking the
same question of the same document. What is the canonical unit of design knowledge, and how much of the viewpoint
framework does Specful adopt?

## Decision Drivers

- A reader asking "how does this subject work?" should be answered by one document.
- Designs serve developers, maintainers, adopters, and self-hosters alike.
- Completeness pressure must come from authoring guidance, not schema ceremony.
- Related structure, data, and runtime behaviour explain each other and lose value when separated.
- Stable, searchable document structure across the corpus.

## Considered Options

- Subject-oriented Design artifacts with canonical sections
- One artifact per viewpoint-scoped design view
- Subject-oriented Design artifacts with flexible structure

## Decision Outcome

Chosen option: **subject-oriented Design artifacts with canonical sections**, because the natural unit of design
knowledge is the subject a reader needs to understand as one thing, and viewpoint-scoped fragmentation would optimise
classification at the cost of ordinary retrieval.

A Design is one canonical Markdown file with a stable `PROJECT-DESIGN-0001`-style identifier and frontmatter
`type: DESIGN`, living at `docs/specs/<scope...>/design/0001-short-title.md`. It explains how one coherent subject
currently works, in declarative present-tense prose combining diagrams, tables, models, and examples as needed. The
template carries a canonical section set as a completeness baseline; headings are never renamed, an inapplicable section
states why, and subject-specific sections extend the set. Splitting follows cohesion: only when the resulting subjects
are independently understandable and maintained. Specful adopts no formal viewpoint or view concepts in its artifact
model, schema, identifiers, relationships, or queries; the lineage's concern catalogue survives as authoring prompts.
The artifact owns `satisfies` (the described design contributes to satisfying the referenced requirements) and
`governed-by` (ADRs whose durable rationale the design embodies).

### Consequences

- Positive: one document answers a subject question end to end for every audience.
- Positive: canonical sections keep the corpus searchable and give reviews a completeness footing.
- Negative: subject boundaries are judgement calls; the cohesion rule guides but cannot mechanise them.
- Negative: a large subject produces a long document before the cohesion rule justifies a split.

### Confirmation

`specful new design` scaffolds the canonical section set with an allocated identifier; `specful validate` enforces the
Design profile; `trace` follows design-owned `satisfies` links from requirement identifiers; generated indexes list
Designs directly; Design metadata, schemas, identifiers, relationships, and queries contain no viewpoint or view
concepts.

## Pros and Cons of the Options

### Subject-oriented Design artifacts with canonical sections

- Positive: retrieval matches how the design is actually asked about and changed.
- Positive: the lineage's completeness concerns survive as authoring prompts without determining file boundaries.
- Negative: a large subject produces a long document.

### One artifact per viewpoint-scoped design view

- Positive: strong correspondence with architecture-description standards; per-view identity and relationships.
- Negative: fragments one subject across files, repeats context, and makes common questions multi-file reconstructions.
- Negative: imports a viewpoint vocabulary without a demonstrated need.

### Subject-oriented Design artifacts with flexible structure

- Positive: rich completeness prompts still exert authoring pressure while each subject chooses its own headings.
- Positive: structure adapts precisely to the subject with no not-applicable ceremony.
- Negative: cross-corpus searching and parsing cannot rely on stable heading vocabulary.
- Negative: an omitted concern leaves no visible trace, and interpretation drifts from one document to the next.

## More Information

Informed by the Markdown SDD lineage attributed in `NOTICE.md` and by architecture-description standards terminology.
Reconsider if subject-scale documents prove unmanageable in a real adopting repository.
