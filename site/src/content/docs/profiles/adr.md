---
title: ADR
description: What an Architecture Decision Record is, its sections, and the boundary against a Requirement.
---

An Architecture Decision Record (ADR) records why a durable decision was made: the context, the options considered, and
the outcome. Once accepted, an ADR is never rewritten, only superseded by a new record, with one exception:
**More information** may be edited in place, since it is evidence depth rather than part of the decision event.

## Identity and file layout

Each ADR carries a stable identifier of the form `PROJECT-ADR-NNNN`, allocated by `specful new adr` and never edited by
hand. ADRs live in one flat directory in every adopting repository:

```text
docs/adr/<sequence>-<short-title>.md
```

## Frontmatter

| Field | Meaning | Read by |
|---|---|---|
| `type` | Always `ADR`. | validate (selects the schema) |
| `profile-version` | The ADR profile version this document conforms to. | validate (schema requires `1`) |
| `id` | The allocated `PROJECT-ADR-NNNN` identifier. | validate; index; show; trace |
| `title` | Short title naming the problem and chosen solution. | validate (H1 parity); index; show |
| `status` | One of `proposed`, `accepted`, `deprecated`, `superseded`. | validate (supersession pairing); index; show |
| `recorded-on` | The date this record was written. | validate (schema); readers |
| `decided-on` | The date the decision was made, when it differs from `recorded-on`. | validate (schema); readers |
| `decision-makers` | Who made the decision. | validate (schema); names the authority to reconsider |
| `supersedes` | Optional: the ADR identifier(s) this record replaces. | validate (reciprocity); index; show; trace |
| `superseded-by` | Optional: the ADR that replaced this record. | validate (reciprocity); index; show; trace |

Supersession is the one relationship stored in both directions: both the replaced and replacement records carry
reciprocal links, so either document remains independently navigable, and validation treats disagreement between those
links as an error.

## Sections

`specful validate` checks structure, not judgement. It requires the four level-two headings through Decision outcome, in
that exact order and wording, and requires the level-three **Consequences** heading, but only within the Decision
outcome span: a stray top-level heading of the same name elsewhere does not satisfy it. Each required heading must
appear exactly once, in order, and carry content; a missing, duplicated, out-of-order, or empty required section is a
finding. When present, **Pros and cons of the options** and **More information** must be non-empty. The document H1 must
match the frontmatter `title`, and no template placeholder residue may remain outside a code span. The tool cannot tell
whether the considered options were the materially viable ones, whether the chosen option's reasoning is honest, or
whether a stated consequence is real: that judgement is the author's and reviewer's, and the authoring skills teach it.

Through **Consequences**, every section is required. **Pros and cons of the options** and **More information** are
optional, and are removed completely when they add no useful decision evidence, unlike a Requirement or Design section,
which keeps its heading with a stated reason when not applicable: these two sections are evidence depth, not a
completeness checklist.

- **Context and problem statement**: the decision context, the problem that requires a durable choice, and the scope of
  that choice, stated without narrating implementation history. May be posed as a question.
- **Decision drivers**: the forces, constraints, and desired qualities that govern the choice.
- **Considered options**: the materially viable options, named consistently with the rest of the document.
- **Decision outcome**: "Chosen option: **{option}**, because {reason it best satisfies the decision drivers}."
  - **Consequences**: material outcomes of the choice, including adverse trade-offs, as positive and negative bullets.
- **Pros and cons of the options** (optional): each considered option's own positive, neutral, and negative points.
- **More information** (optional): supporting evidence, related decisions, or conditions that should trigger
  reconsideration. It never restates a `satisfies`, `governed-by`, `supersedes`, or `superseded-by` relationship the
  frontmatter already carries; `specful show` renders those edges.

An ADR outcome says "chosen option: X, because ...", never "the system MUST". A decision whose outcome needs to bind
mints a Requirement and links it back through `governed-by`.

## Relationship to MADR

This profile derives from the MADR 4.0.0 complete template and is tighter than MADR on every axis it touches: a
canonical MADR record does not validate against it unchanged. Section headings are sentence case and compared exactly,
so `Context and Problem Statement` fails where `Context and problem statement` passes. Decision drivers and Consequences
are required here where MADR marks them optional. MADR's Confirmation section is not part of this profile: a record that
still carries one validates, but the section is not read. `recorded-on`, with `decided-on` when the decision predates
the record, replaces MADR's single `date` field. `decision-makers` is a list rather than free text. MADR's `consulted`
and `informed` participant roles are not part of this profile; a record carrying either fails validation until the lines
are deleted. `status` drops MADR's `rejected` value. No frontmatter key outside the profile is accepted other than an
`x-` extension key. A MADR record joins the profile by being re-recorded, never edited in place; see
[adopting into an existing repository](/specful/adoption/#adopting-into-an-existing-repository).

## Requirement versus ADR

A Requirement binds the system; an ADR records a choice. A Requirement states what must be true of the system now,
observable and yes/no checkable, and is rewritten in place when it changes. An ADR records a decision event, with its
alternatives and reasoning, and is never rewritten, only superseded. The seam is `governed-by`: when a decision creates
an obligation, the ADR carries the why-this-over-that and a Requirement carries the enforceable now, linked through
`governed-by`. Neither duplicates the other: an ADR cannot bind, and a Requirement does not explain the road not taken.

With Design in the same picture, the triangle is complete: Requirement is what must hold, Design is how the system
currently works, ADR is why the durable choices were made. Most internal technology choices produce an ADR and Design
content but no Requirement at all; a Requirement exists only where something genuinely binds observable behaviour, or
where an externally imposed constraint (a mandated protocol, format, mechanism, or interoperability contract) applies.

Two litmus questions settle nearly every case:

1. Could tooling or a test ever check it against the system? Yes: Requirement. No, the value is the reasoning: ADR.
2. If it stopped being true tomorrow, would it be rewritten or superseded? Rewritten: Requirement. Superseded: ADR.

The discipline that keeps the line crisp: an ADR outcome never says "the system MUST", and whether a decision is being
followed is read from the Requirements and Designs that cite it through `governed-by`, not from the record itself. See
the [Requirement profile](/specful/profiles/requirement/) and the [Design profile](/specful/profiles/design/) for the
other two sides of the same triangle.
