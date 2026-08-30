---
title: Requirement
description: What a Requirement is, its sections, and how it differs from an ADR.
---

A Requirement is one file carrying one coherent normative obligation with its complete record. It states what the system
must do now, observable and yes/no checkable, and is rewritten in place when the obligation changes.

## Identity and file layout

Each Requirement carries a stable identifier of the form `PROJECT-REQ-NNNN`, allocated by `specful new requirement` and
never edited by hand. Files live under the owning architectural scope:

```text
docs/specs/<architectural-scope...>/requirements/<sequence>-<subject...>.md
```

Cross-component requirements belong to the narrowest scope that completely owns the obligation; system-wide requirements
stay at system scope rather than being duplicated across components. Scope is a placement and navigation hierarchy, not
an authored container artifact: moving a requirement between scopes preserves its identifier.

## Frontmatter

| Field | Meaning |
|---|---|
| `type` | Always `REQ`. |
| `profile-version` | The Requirement profile version this document conforms to. |
| `id` | The allocated `PROJECT-REQ-NNNN` identifier. |
| `title` | Concise navigation title for the obligation. |
| `governed-by` | ADR identifiers whose durable rationale this obligation embodies. Omit when no ADR governs it. |

## Sections

A Requirement carries four canonical headings, never renamed. A section with nothing to say keeps its heading and states
why it does not apply.

- **Statement**: one normative paragraph, using at least one uppercase BCP 14 keyword (MUST, MUST NOT, SHOULD, SHOULD
  NOT, MAY). Name the acting system or component where one exists, the triggering condition, and the observable
  behaviour; put the condition before the keyword. How the system meets the obligation belongs in the Design that
  satisfies it, not in the Statement, though an externally imposed constraint (a mandated protocol, format, mechanism,
  or interoperability contract) is itself a legitimate obligation. Avoid vague qualifiers ("reasonable", "appropriate",
  "quickly") and escape clauses ("where possible", "as appropriate"); every term the statement relies on must be defined
  or linked, and every bound must be objectively determinable.
- **Rationale**: why the obligation exists, the need it serves, who or what depends on it, and the consequence of not
  meeting it. Concise; it does not restate the Statement.
- **Acceptance criteria**: observable conditions that distinguish satisfaction from non-satisfaction, each independently
  checkable with a yes or no answer. Use precise units, tolerances, and thresholds for quantitative bounds, and cover
  the negative and edge cases that define the obligation's boundary, not only the happy path. There is no standalone
  verification section: acceptance criteria carry falsifiability directly, and where the means of checking a criterion
  is not obvious, the criterion states how satisfaction is determined.
- **More information**: supporting context, related Requirements and Designs, external references, and examples. An
  unresolved matter that affects the obligation or its acceptance boundary means the Requirement is not complete yet.

## One obligation per file

One obligation per file does not mean one sentence per file: complex acceptance boundaries, tables, examples, and
definitions may all be necessary. It does mean one obligation. If the Statement needs "and" to join independently
verifiable duties, split it into separate Requirements.

## Requirement versus ADR

A Requirement binds the system; an ADR records a choice. See
[Requirement versus ADR](/specful/profiles/adr/#requirement-versus-adr) on the ADR profile page for the full boundary,
including the two litmus questions that settle nearly every case.
