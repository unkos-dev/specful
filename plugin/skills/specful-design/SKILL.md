---
name: specful-design
description: >-
  Use when creating, writing, or updating a Specful Design under `docs/specs/**/design/*.md`: documenting how a subject
  of the system works, or connecting a Design to the Requirements it satisfies.
compatibility: Requires the specful CLI on PATH.
---

# Writing a Specful Design

A Design describes how a subject of the system works now, written as though it has always worked this way. It is not a
Requirement (what the software must do) and not an ADR (why a decision was made); if the change is about what or why,
load `specful-requirement` or `specful-adr` instead. History and transitions never appear in the prose: a transition is
a plan, and what used to be true is Git history.

Read the bundled [Design profile](references/profiles/design.md) (or run `specful profile design`) and the
[Requirement versus ADR](references/profiles/adr.md#requirement-versus-adr) boundary before authoring or updating a
Design.

## Workflow

1. Start at `docs/specs/index.md` and follow the scope indexes to the subject's module. Read the neighbouring Designs
   and the Requirements they satisfy before writing.
2. Scaffold with `specful new design --title <TITLE> --scope <SCOPE>`. Never hand-allocate an identifier; the command
   owns the counter.
3. Complete the placeholders. Declare the Requirements the Design `satisfies` and cite governing ADRs through
   `governed-by`, naming only the ADR whose rationale this subject embodies, never a related or organising decision such
   as the decision to adopt a convention, and omit the field when that record does not exist in the profile. Describe
   the subject as it is, at the level a maintainer needs to change it safely; decision rationale belongs in an ADR, not
   here. More information is optional and is removed completely when it adds nothing; it never restates a `satisfies` or
   `governed-by` edge as a link, since `specful show` already renders those.
4. Run `specful index`, then `specful validate`; commit the regenerated views with the change.
5. Mechanical validation does not judge substantive quality. Use `specful-review` when the adopting repository requires
   substantive review or the user asks for it.

## Blurred versus clean

- **Blurred:** `## Structure` compares PostgreSQL against SQLite and explains why SQLite was rejected during migration.
  **Clean:** `## Structure` describes the current PostgreSQL storage layout in present tense and cites the governing ADR
  through `governed-by`.
- **Blurred:** `## Data and state` contains `N/A` and `## Failure and recovery` introduces new `MUST` rules with `- [ ]`
  follow-up tasks. **Clean:** Inapplicable sections state a concrete reason (`Not applicable: stateless transform`),
  normative obligations live in a Requirement, and work items live in a plan.
