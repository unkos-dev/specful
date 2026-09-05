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
supersedes:
  - "{PROJECT}-ADR-{NNNN}"
superseded-by:
  - "{PROJECT}-ADR-{NNNN}"
---

# {Short title naming the problem and chosen solution}

{INSTRUCTIONS. This file was scaffolded from the Specful ADR template; the frontmatter identifier was allocated by
`specful new adr` and is never edited by hand. Replace every braced block with real content. The sections through
Consequences are required; Pros and cons of the options and More information are optional and are removed completely
when they add no useful decision evidence. An ADR records a decision, not an obligation: the outcome says "chosen
option: X, because ...", never "the system MUST". An obligation created by this decision belongs in a Requirement that
cites this ADR through `governed-by`; once accepted, this record is never rewritten and only superseded, except More
information, which may be edited in place. `decision-makers` is required; remove `decided-on` and the supersession
fields that do not apply. The document is complete only when no braced text remains and `specful validate` passes.
Delete this block last.}

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

## Pros and cons of the options

{Optional: remove this section completely when the Decision outcome already carries the useful evidence.}

### {Option}

- Positive: {argument}
- Neutral: {argument}
- Negative: {argument}

## More information

{Optional: supporting evidence, related decisions, or conditions that should trigger reconsideration. Never restate a
`satisfies`, `governed-by`, `supersedes`, or `superseded-by` relationship the frontmatter already carries. Remove the
section when it adds nothing.}
