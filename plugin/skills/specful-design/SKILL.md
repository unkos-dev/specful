---
name: specful-design
description: >-
  Use when writing or updating a Specful Design: documenting how a subject of the system works, or connecting a
  Design to the Requirements it satisfies.
compatibility: Requires specful 0.3.0 or later on PATH.
---

# Writing a Specful Design

A Design describes how a subject of the system works now, written as though it has always worked this way. It is not a
Requirement (what the software must do) and not an ADR (why a decision was made); if the change is about what or why,
load `specful-requirement` or `specful-adr` instead. History and transitions never appear in the prose: a transition is
a plan, and what used to be true is Git history.

## Workflow

1. Start at `docs/specs/index.md` and follow the scope indexes to the subject's module. Read the neighbouring Designs
   and the Requirements they satisfy before writing.
2. Scaffold with `specful new design --title <TITLE>`. Never hand-allocate an identifier; the command owns the counter.
3. Complete the placeholders. Declare the Requirements the Design `satisfies` and cite governing ADRs through
   `governed-by`. Describe the subject as it is, at the level a maintainer needs to change it safely; decision rationale
   belongs in an ADR, not here.
4. Run `specful index`, then `specful validate`; commit the regenerated views with the change.
5. Mechanical validation is necessary but not sufficient: load `specful-review` before treating the change as done.

For the full Design profile and field-by-field guidance, see <https://unkos-dev.github.io/specful/>.
