---
name: specful-adr
description: >-
  Use when recording a durable decision as a Specful ADR (architecture decision record), or superseding an accepted
  one.
compatibility: Requires specful 0.3.0 or later on PATH.
---

# Recording a Specful ADR

An ADR records why a durable decision was made: the forces, the credible alternatives, and the consequences. It is not a
Requirement (what the software must do) and not a Design (how the system works); if the change is about what or how,
load `specful-requirement` or `specful-design` instead. Keep it at decision level: no task sequences, file inventories,
or progress notes.

## Workflow

1. Read `docs/adr/` and its index for the numbering, statuses, and the decisions already on record; a new ADR that
   contradicts an accepted one supersedes it rather than silently disagreeing.
2. Scaffold with `specful new adr --title <TITLE>`. Never hand-allocate an identifier; the command owns the counter.
3. Complete the placeholders. State the decision in present tense; represent the credible alternatives honestly rather
   than as strawmen for a preferred answer; record negative consequences with the same care as positive ones. An ADR
   with no credible alternative or no downside is advertising, not a decision record.
4. Lifecycle belongs to the maintainer: a new record starts proposed, and only the maintainer accepts, rejects, or
   supersedes.
5. Run `specful index`, then `specful validate`; commit the regenerated views with the change. Load `specful-review`
   before treating the change as done.

For the full ADR profile and field-by-field guidance, see <https://unkos-dev.github.io/specful/>.
