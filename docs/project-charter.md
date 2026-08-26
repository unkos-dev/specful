# Project charter

## Purpose

Specful provides a portable, repository-native system for expressing and
retrieving the knowledge needed to build and maintain software reliably.

Its primary outcome is better human and agent understanding of a codebase:
requirements are explicit, current design is discoverable, durable decisions
remain explainable, and relationships can be validated mechanically.

## Product outcomes

Specful is intended to:

- make current software behaviour and constraints explicit;
- make current architecture and component design easy to retrieve;
- preserve durable decision rationale without adding history to
  current-state documents;
- connect requirements, designs, decisions, verification, and code
  ownership;
- give coding agents a deterministic navigation and retrieval workflow;
- detect missing, broken, duplicated, and orphaned relationships;
- support gradual adoption in existing repositories;
- install into multiple agent harnesses without giving any harness
  ownership of canonical project knowledge.

## Information model

Specful distinguishes artifacts by responsibility:

| Artifact | Responsibility | Lifecycle |
|---|---|---|
| MSRS | Defines what the software must do now | Rewritten or deleted in place |
| MSDD | Describes how the software works now | Rewritten or deleted in place |
| ADR | Records why a durable decision was made | Retained and superseded |
| Plan | Coordinates an active transition | Temporary; archived or deleted on completion |
| Git | Preserves what used to be true | Repository history |

Requirements and design descriptions are logical collections that may be
split into multiple Markdown modules. A requirement may be satisfied by
multiple design modules, and one design module may satisfy multiple
requirements. A change plan delivers one coherent change; an arc plan
coordinates a sequence or graph of change-sized deliverables.

## Architectural organization

Specification paths follow architectural scope rather than a fixed
capability taxonomy:

```text
docs/specs/<architectural-scope...>/<artifact-kind>/<subject...>.md
```

Architectural and subject paths may be nested as needed. File placement
aids navigation, while stable identifiers and explicit metadata define
artifact identity and relationships.

Cross-component requirements belong to the narrowest scope that completely
owns the obligation. System-wide requirements remain at system scope rather
than being duplicated across components.

`docs/specs/` is the native Open Knowledge Format v0.2 bundle boundary.
Specful MSRS and MSDD profiles specialize that inherited contract without
replacing it. The OKF guide defines the pinned source, permissive native
classification, and stricter Specful specialization.

Architecture Decision Records live in `docs/adr/`, one flat directory in
every adopting repository.

## Writing model

MSRS and MSDD content describes current state only. Current-state documents
are written as though the system has always existed in its present form.

Migration history, rejected alternatives, and decision chronology do not
belong in requirements or design descriptions. Durable rationale belongs in
an ADR. Obsolete behaviour is available through Git history.

MSRS requirements use the normative vocabulary defined by BCP 14. Uppercase
MUST, MUST NOT, SHOULD, SHOULD NOT, and MAY carry normative meaning. MSDD
prose is declarative and present tense so it does not become a competing
source of requirements.

## Source-of-truth boundary

Markdown documents and structured metadata committed to the adopting
repository are canonical.

Relationships are stored in one authoritative direction with the artifact
that owns them. Design modules own the `satisfies` links to the
requirements they implement, and specification modules own citations of the
ADRs that govern them. ADR supersession is the narrow exception: both the
replaced and replacement records store reciprocal links so either document
remains independently navigable. Validation treats disagreement between
those links as an error.

Reverse relationships, indexes, traceability matrices, catalogs, search
indexes, and databases are derived and disposable.

The system must remain usable with ordinary files, Git, and text search.
Generated views improve retrieval but cannot own project knowledge.

## Convention and tool

The convention is the product. A repository that follows the layout,
templates, and writing model is a Specful repository, readable and
navigable with no tooling installed.

The `specful` CLI serves the convention. It mechanizes the three tasks the
convention cannot deliver by hand at acceptable cost: allocating stable
identifiers, regenerating navigation views, and validating the repository.
It also scaffolds adoption and answers retrieval queries. The CLI is a
single static binary with no runtime dependencies.

## Identifiers

Artifacts and requirements carry allocated sequential identifiers of the
form `<PROJECT>-<KIND>-<NNNN>`. Allocation state lives in `.specful.yaml`
as monotonic per-kind counters, so identifiers are never reused and never
depend on scanning the tree. Filenames and paths aid navigation; the
identifier in metadata is the durable identity.

## Generated navigation

Two derived views are generated, committed, and drift-checked:

- a per-scope `index.md` inside each `docs/specs/` scope, listing that
  scope's children with identifiers, titles, and one-line summaries, so an
  agent can navigate from the root by reading;
- a machine-readable catalog under `.specful/generated/` recording every
  artifact and relationship edge, powering lookup and trace queries.

Validation fails when a committed view disagrees with the documents it is
derived from. Both views are disposable and carry no canonical knowledge.

## Validation

Validation is mechanical and covers three layers:

- relationship integrity: identifiers resolve, `satisfies` targets exist,
  supersession links agree, citations point at real ADRs, generated views
  match their sources;
- metadata shape: frontmatter conforms to the artifact's published JSON
  Schema profile;
- document structure: required headings present, requirement blocks
  well-formed, title and heading agree, template placeholder text absent,
  and every requirement block uses at least one uppercase BCP 14 keyword.

Diagnostics are human-readable text with a meaningful exit status. A
`--json` flag emits a plain machine-readable listing that is explicitly
unstable. There is no diagnostic rule registry, no severity policy, and no
waiver system; a finding is fixed in the documents, not managed in
configuration.

Adopting repositories run validation in their local gate and continuous
integration. Specful does not require commit hooks.

## Portability boundary

The core schema, templates, lifecycle, and workflow are independent of
agent harnesses.

Adoption installs harness-neutral instructions in `docs/SPECFUL.md` and a
small managed pointer block in the root `AGENTS.md`, teaching the
convention: the retrieval recipe and the authoring workflow, including
which artifact changes for which kind of work. Harness-specific adapters
may generate native skills, commands, or context files, but those files are
generated integration surfaces. They cannot become divergent copies of
project policy or canonical knowledge.

## Initial scope

The initial product scope includes:

- artifact schemas and metadata conventions for MSRS, MSDD, ADR, and
  repository configuration;
- templates for requirements, design descriptions, ADRs, and plans;
- stable identifiers and validated relationships;
- generated per-scope indexes and a machine-readable catalog;
- a CLI providing `init`, `new`, `validate`, `index`, `show`, and
  `trace`;
- harness-neutral instruction content installed at adoption.

Planned after the initial release: traceability views, harness-specific
adapters and skills, managed instruction-file updates, verification and
code-ownership relationship types, and brownfield adoption guidance for
documenting existing repositories incrementally.

## Non-goals

Specful does not provide:

- a hosted requirements-management service;
- a canonical relational or graph database;
- a replacement for Git history, issue tracking, or source code;
- automatic generation of a complete specification from an undocumented
  codebase;
- a harness-specific source of truth;
- management, validation, or indexing of implementation plans; plan
  retention is the adopter's choice;
- historical narrative inside current-state requirements or design
  documents;
- a diagnostic governance framework: no rule registries, severity
  policies, waivers, or stable machine-readable result contracts.
