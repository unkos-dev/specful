---
title: ADR
description: What an Architecture Decision Record is, its sections, and the boundary against a Requirement.
---

An Architecture Decision Record (ADR) records why a durable decision was made: the context, the options considered, and
the outcome. Once accepted, an ADR is never rewritten, only superseded by a new record.

## Identity and file layout

Each ADR carries a stable identifier of the form `PROJECT-ADR-NNNN`, allocated by `specful new adr` and never edited by
hand. ADRs live in one flat directory in every adopting repository:

```text
docs/adr/<sequence>-<short-title>.md
```

## Frontmatter

| Field | Meaning |
|---|---|
| `type` | Always `ADR`. |
| `profile-version` | The ADR profile version this document conforms to. |
| `id` | The allocated `PROJECT-ADR-NNNN` identifier. |
| `title` | Short title naming the problem and chosen solution. |
| `status` | One of `proposed`, `accepted`, `deprecated`, `superseded`. |
| `recorded-on` | The date this record was written. |
| `decided-on` | The date the decision was made, when it differs from `recorded-on`. |
| `decision-makers` | Who made the decision. |
| `consulted` | Optional: roles consulted before the decision. |
| `informed` | Optional: roles informed of the decision after the fact. |
| `supersedes` | Optional: the ADR identifier(s) this record replaces. |
| `superseded-by` | Optional: the ADR identifier that replaced this record. |

Supersession is the one relationship stored in both directions: both the replaced and replacement records carry
reciprocal links, so either document remains independently navigable, and validation treats disagreement between those
links as an error.

## Sections

Through **Confirmation**, every section is required. **Pros and cons of the options** and **More information** are
optional, and are removed completely when they add no useful decision evidence, unlike a Requirement or Design section,
which keeps its heading with a stated reason when not applicable: these two sections are evidence depth, not a
completeness checklist.

- **Context and problem statement**: the decision context, the problem that requires a durable choice, and the scope of
  that choice, stated without narrating implementation history. May be posed as a question.
- **Decision drivers**: the forces, constraints, and desired qualities that govern the choice.
- **Considered options**: the materially viable options, named consistently with the rest of the document.
- **Decision outcome**: "Chosen option: **{option}**, because {reason it best satisfies the decision drivers}."
  - **Consequences**: material outcomes of the choice, including adverse trade-offs, as positive and negative bullets.
  - **Confirmation**: observable evidence that the decision is implemented and continues to be followed, such as a test,
    review, rule, or operational check. Confirmation shows the decision is being honoured; whether an obligation is
    satisfied is a Requirement's acceptance criteria, not this section.
- **Pros and cons of the options** (optional): each considered option's own positive, neutral, and negative points.
- **More information** (optional): supporting evidence, related decisions, or conditions that should trigger
  reconsideration.

An ADR outcome says "chosen option: X, because ...", never "the system MUST". A decision whose outcome needs to bind
mints a Requirement and links it back through `governed-by`.

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

The discipline that keeps the line crisp: an ADR outcome never says "the system MUST", and its Confirmation section
confirms the decision is being followed, not that an obligation is satisfied. See the
[Requirement profile](/specful/profiles/requirement/) and the [Design profile](/specful/profiles/design/) for the other
two sides of the same triangle.
