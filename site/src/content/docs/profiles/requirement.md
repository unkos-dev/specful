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

| Field | Meaning | Read by |
|---|---|---|
| `type` | Always `REQ`. | validate (selects the schema) |
| `profile-version` | The Requirement profile version this document conforms to. | validate (schema requires `1`) |
| `id` | The allocated `PROJECT-REQ-NNNN` identifier. | validate; index; show; trace |
| `title` | Concise navigation title for the obligation. | validate (H1 parity); index; show |
| `governed-by` | ADRs whose rationale this obligation embodies. Omit when none governs it. | validate; index; show |

## Sections

`specful validate` checks structure, not judgement. It requires Statement, Rationale, and Acceptance criteria, in that
exact order and wording, and requires the Statement to carry an uppercase BCP 14 keyword. Each required heading must
appear exactly once, in order, and carry content; a missing, duplicated, out-of-order, or empty required section is a
finding. When present, More information must be non-empty. The document H1 must match the frontmatter `title`, and no
template placeholder residue may remain outside a code span. The tool cannot tell whether an acceptance criterion is
genuinely checkable, whether the Statement's condition is objectively determinable, or whether a "not applicable" is
honest: that judgement is the author's and reviewer's, and the authoring skills teach it.

A Requirement carries three canonical headings, never renamed, plus an optional fourth. A required section with nothing
to say keeps its heading and states why it does not apply.

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
- **More information** (optional): supporting context such as external references and examples, removed completely when
  it adds nothing. It never restates a `governed-by` relationship the frontmatter already carries; `specful show`
  renders that edge. An unresolved matter that affects the obligation or its acceptance boundary means the Requirement
  is not complete yet.

## One obligation per file

One obligation per file does not mean one sentence per file: complex acceptance boundaries, tables, examples, and
definitions may all be necessary. It does mean one obligation. If the Statement needs "and" to join independently
verifiable duties, split it into separate Requirements.

## Requirement versus ADR

A Requirement binds the system; an ADR records a choice. See
[Requirement versus ADR](/specful/profiles/adr/#requirement-versus-adr) on the ADR profile page for the full boundary,
including the two litmus questions that settle nearly every case.
