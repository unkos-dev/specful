---
# Frontmatter uses the JSON-compatible YAML 1.2 subset.
kind: adr
profile-version: 1
id: "{PROJECT}-ADR-{NNNN}"
title: "{Short title naming the problem and chosen solution}"
status: "{proposed | accepted | deprecated | superseded}"
recorded-on: "{YYYY-MM-DD}"
# Remove decided-on when the ADR is proposed or the historical date is unknown.
decided-on: "{YYYY-MM-DD}"
decision-makers:
  - "{person or collective role}"
# Remove optional participant fields that do not apply.
consulted:
  - "{person or collective role}"
informed:
  - "{person or collective role}"
# Remove relationship fields that do not apply.
supersedes:
  - "{PROJECT}-ADR-{NNNN}"
superseded-by:
  - "{PROJECT}-ADR-{NNNN}"
# Add project-specific metadata only with lowercase x- extension keys.
---

<!-- SPDX-License-Identifier: CC0-1.0 -->

# {Short title naming the problem and chosen solution}

<!--
Replace every placeholder and remove all instructional comments before the ADR
is complete.
-->

## Context and Problem Statement

<!--
Describe the decision context, the problem that requires a durable choice, and
the scope of that choice. State the problem without narrating implementation
history.
-->

{Decision context and problem statement}

## Decision Drivers

<!-- List the forces, constraints, and desired qualities that govern the choice. -->

- {Decision driver}

## Considered Options

<!-- List the materially viable options using the same names used below. -->

- {Option}

## Decision Outcome

Chosen option: **{option}**, because {reason it best satisfies the decision
drivers}.

### Consequences

<!-- Record material outcomes of the choice, including adverse trade-offs. -->

- Positive: {benefit or improvement}
- Negative: {cost, limitation, or risk}

### Confirmation

<!--
Describe observable evidence that can confirm the decision is implemented and
continues to be followed, such as a test, review, rule, or operational check.
-->

{Confirmation method}

<!--
The remaining sections are conditional. Remove a section completely when it
does not add useful decision evidence.
-->

## Pros and Cons of the Options

### {Option}

- Positive: {argument}
- Neutral: {argument}
- Negative: {argument}

## More Information

{Supporting evidence, related decisions, or conditions that should trigger
reconsideration}
