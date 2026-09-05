---
title: Design
description: What a Design is, its sections, and how one subject stays one document.
---

A Design is one canonical file explaining how one coherent subject currently works: a system, component, surface,
mechanism, or collaboration a reader normally needs to understand as one thing. It serves developers, maintainers,
adopters, and self-hosters through one complete explanation, in declarative present-tense prose, as though the system
has always worked this way.

## Identity and file layout

Each Design carries a stable identifier of the form `PROJECT-DESIGN-NNNN`, allocated by `specful new design` and never
edited by hand. Files live under the owning architectural scope:

```text
docs/specs/<architectural-scope...>/design/<sequence>-<subject...>.md
```

## Frontmatter

| Field | Meaning | Read by |
|---|---|---|
| `type` | Always `DESIGN`. | validate (selects the schema) |
| `profile-version` | The Design profile version this document conforms to. | validate (schema requires `1`) |
| `id` | The allocated `PROJECT-DESIGN-NNNN` identifier. | validate; index; show; trace |
| `title` | Concise name of the design subject. | validate (H1 parity); index; show |
| `satisfies` | Requirements this design contributes to satisfying. Omit when empty. | validate; index; show; trace |
| `governed-by` | ADRs whose rationale this design embodies. Omit when empty. | validate; index; show; trace |

## Sections

`specful validate` checks structure, not judgement. It requires the seven canonical headings, from Purpose and
boundaries through Security and operations, in that exact order and wording; subject-specific sections added between
them do not break the check. Each required heading must appear exactly once, in order, and carry content; a missing,
duplicated, out-of-order, or empty required section is a finding. When present, More information must be non-empty. The
document H1 must match the frontmatter `title`, and no template placeholder residue may remain outside a code span. The
tool cannot tell whether a "not applicable" reason is honest, whether the structure described matches the real system,
or whether a failure mode was left out: that judgement is the author's and reviewer's, and the authoring skills teach
it.

A Design carries a canonical section set as a completeness baseline, not a ceiling: subject-specific sections may be
added freely. Headings are never renamed; where a required section does not apply, keep the heading and state why, since
a bare not-applicable with no reason is a review flag, not an answer. More information is the exception: it is optional
and is removed completely when it adds nothing.

- **Purpose and boundaries**: what this subject is for, what it owns, where it begins and ends, and what it deliberately
  leaves to neighbouring subjects. Names the systems, components, or actors it depends on and the ones that depend on
  it.
- **Structure**: the current parts and how they fit together: components, modules, layers, or services and their
  relationships. A diagram often carries this better than prose.
- **Interfaces and dependencies**: the contracts this subject offers and consumes: APIs, commands, file formats,
  protocols, events, or library boundaries. Links canonical interface definitions rather than duplicating them.
- **Data and state**: what the subject stores or owns, its shape and lifetime, where it lives, and which operations
  mutate it. Includes configuration that changes runtime behaviour and its consequences.
- **Runtime behaviour**: how the subject behaves when exercised, the main flows from trigger to outcome, with ordering,
  timing, and concurrency where they matter.
- **Failure and recovery**: what goes wrong and what the subject does about it: error propagation, partial-failure
  handling, rollback, retries, and the user-visible outcomes of each failure class.
- **Security and operations**: trust boundaries, credentials and secrets handling, and the operational surface:
  observability, troubleshooting entry points, and what a self-hoster must understand to run this subject safely.
- **More information** (optional): links to canonical user documentation, runbooks, external references, and other
  material a reader may need next. It never restates a `satisfies` or `governed-by` relationship the frontmatter already
  carries; `specful show` renders those edges.

## Cohesion, not length

Material belongs in one Design when a reader normally needs it together to understand or change that subject. Split into
separate Designs only when the resulting subjects are independently understandable and independently maintained, never
merely because the document grows long.

## Design versus ADR

Most internal technology choices produce Design content (how it currently works) and an ADR (why it was chosen), with no
Requirement at all: a Requirement exists only where something genuinely binds observable behaviour. A Design's
`governed-by` field cites the ADRs whose rationale it embodies; it never restates that rationale, and the ADR never
describes current structure. See [Requirement versus ADR](/specful/profiles/adr/#requirement-versus-adr) for the litmus
questions that generalise to this boundary too.
