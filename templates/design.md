---
type: DESIGN
profile-version: 1
id: "{PROJECT}-DESIGN-{NNNN}"
title: "{Concise name of the design subject}"
satisfies:
  - "{PROJECT}-REQ-{NNNN}"
governed-by:
  - "{PROJECT}-ADR-{NNNN}"
---

<!-- SPDX-License-Identifier: CC0-1.0 -->

# {Concise name of the design subject}

{INSTRUCTIONS. This file was scaffolded from the Specful Design template; the frontmatter identifier was allocated by
`specful new design` and is never edited by hand. Replace every braced block with real content. Every section heading
below stays, exactly as written, so Designs stay searchable and parseable across the repository; where a section does
not apply to this subject, keep the heading and state "Not applicable: {reason}", since a bare not-applicable with no
reason is a review flag, not an answer. Add subject-specific sections freely; the set below is the completeness
baseline, not a ceiling. The document is complete only when no braced text remains, `specful validate` passes, and every
section carries real content or a reasoned not-applicable. Delete this block last.}

{A Design explains how one coherent subject currently works: a system, component, surface, mechanism, or collaboration a
reader normally needs to understand as one thing. It serves developers, maintainers, adopters, and self-hosters through
one complete explanation. Write declarative present-tense prose, as though the system has always worked this way, and
combine prose, diagrams, tables, models, and examples as the subject requires. Cohesion rule: material belongs here when
a reader normally needs it together to understand or change this subject; split into separate Designs only when the
resulting subjects are independently understandable and independently maintained, never merely because the document
grows long. `satisfies` lists the requirements the described design contributes to satisfying; `governed-by` lists the
ADRs whose durable rationale it embodies; remove either field when empty. Replace this paragraph with a short summary of
the subject.}

## Purpose and boundaries

{What this subject is for, what it owns, where it begins and ends, and what it deliberately leaves to neighbouring
subjects. Name the systems, components, or actors it depends on and the ones that depend on it.}

## Structure

{The current parts and how they fit together: components, modules, layers, or services and their relationships. A
diagram often carries this better than prose.}

## Interfaces and dependencies

{The contracts this subject offers and consumes: APIs, commands, file formats, protocols, events, or library boundaries.
Link canonical interface definitions rather than duplicating them.}

## Data and state

{What the subject stores or owns, its shape and lifetime, where it lives, and which operations mutate it. Include
configuration that changes runtime behaviour and its consequences.}

## Runtime behaviour

{How the subject behaves when exercised: the main flows from trigger to outcome, with ordering, timing, and concurrency
where they matter. Sequence diagrams and worked examples earn their place here.}

## Failure and recovery

{What goes wrong and what the subject does about it: error propagation, partial-failure handling, rollback, retries, and
the user-visible outcomes of each failure class.}

## Security and operations

{Trust boundaries, credentials and secrets handling, and the operational surface: observability, troubleshooting entry
points, and what a self-hoster must understand to run this subject safely.}

## More information

{Links to canonical user documentation, runbooks, related Designs and Requirements, and other material a reader may need
next. A Design explains how the system works; step-by-step usage and procedures belong in the linked canonical
documents.}
