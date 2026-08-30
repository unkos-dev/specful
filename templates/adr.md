---
type: ADR
profile-version: 1
id: "{PROJECT}-ADR-{NNNN}"
title: "{Short title naming the problem and chosen solution}"
status: "{proposed | accepted | deprecated | superseded}"
recorded-on: "{YYYY-MM-DD}"
decided-on: "{YYYY-MM-DD}"
decision-makers:
  - "{person or collective role}"
consulted:
  - "{person or collective role}"
informed:
  - "{person or collective role}"
supersedes:
  - "{PROJECT}-ADR-{NNNN}"
superseded-by:
  - "{PROJECT}-ADR-{NNNN}"
---

<!-- SPDX-License-Identifier: CC0-1.0 -->

# {Short title naming the problem and chosen solution}

{INSTRUCTIONS. This file was scaffolded from the Specful ADR template; the frontmatter identifier was allocated by
`specful new adr` and is never edited by hand. Replace every braced block with real content. The sections through
Confirmation are required; Pros and cons of the options and More information are optional and are removed completely
when they add no useful decision evidence. An ADR records a decision, not an obligation: the outcome says "chosen
option: X, because ...", never "the system MUST". An obligation created by this decision belongs in a Requirement that
cites this ADR through `governed-by`; once accepted, this record is never rewritten, only superseded. Remove the
frontmatter roles and supersession fields that do not apply. The document is complete only when no braced text remains
and `specful validate` passes. Delete this block last.}

## Context and problem statement

{The decision context, the problem that requires a durable choice, and the scope of that choice. State the problem
without narrating implementation history; you may pose it as a question.}

## Decision drivers

{The forces, constraints, and desired qualities that govern the choice.}

- {Decision driver}

## Considered options

{The materially viable options, using the same names used below.}

- {Option}

## Decision outcome

Chosen option: **{option}**, because {reason it best satisfies the decision drivers}.

### Consequences

{Material outcomes of the choice, including adverse trade-offs.}

- Positive: {benefit or improvement}
- Negative: {cost, limitation, or risk}

### Confirmation

{Observable evidence that the decision is implemented and continues to be followed, such as a test, review, rule, or
operational check. Confirmation shows the decision is being honoured; whether an obligation is satisfied is a
Requirement's acceptance criteria, not this section.}

## Pros and cons of the options

{Optional: remove this section completely when the Decision outcome already carries the useful evidence.}

### {Option}

- Positive: {argument}
- Neutral: {argument}
- Negative: {argument}

## More information

{Optional: supporting evidence, related decisions, or conditions that should trigger reconsideration. Remove the section
when it adds nothing.}
