---
name: specful-adr
description: >-
  Use when recording, correcting, or superseding a Specful ADR (architecture decision record), including moving
  misplaced content into a Design or Requirement.
compatibility: Requires the specful CLI on PATH.
---

# Recording a Specful ADR

An ADR records why a durable decision was made: the forces, the credible alternatives, and the consequences. It is not a
Requirement (what the software must do) and not a Design (how the system works); if the change is about what or how,
load `specful-requirement` or `specful-design` instead. Keep it at decision level: no task sequences, file inventories,
or progress notes.

## Workflow

1. Read `docs/adr/` and its index for the numbering, statuses, and the decisions already on record; a new ADR that
   contradicts an accepted one supersedes it rather than silently disagreeing.
2. For a new decision, scaffold with `specful new adr --title <TITLE>`. Never hand-allocate an identifier; the command
   owns the counter. A decision recorded before the profile existed enters it the same way: scaffold a new record, carry
   the original decision date in `decided-on` and the re-recording date in `recorded-on`. Under More information,
   identify the archived original by title and link to its archive path outside `docs/adr/` and `docs/specs/`. An
   optional pre-adoption path is historical context, not the provenance link. Preserve the archived original unchanged
   at the linked path; it is never edited into the profile's shape. The maintainer owns retirement under the existing
   convention; record retirement separately if that convention would otherwise modify or remove the archived original.
3. For a new record, complete the placeholders. State the decision in present tense; represent the credible alternatives
   honestly rather than as strawmen for a preferred answer; record negative consequences with the same care as positive
   ones. An ADR with no credible alternative or no downside is advertising, not a decision record.
4. Lifecycle belongs to the maintainer: a new record starts proposed, and only the maintainer accepts, deprecates, or
   supersedes. A change to the architectural choice, its scope, or its operative conditions requires supersession.
5. Correct an existing record in place when the decision stays the same, including accepted, deprecated, and superseded
   records. Any section and factual metadata may be corrected, including rationale, alternatives, consequences, dates,
   and participant names. Preserve the identifier, lifecycle state, and supersession relationships; do not reset dates
   to the editing date or present later knowledge as the original rationale. Move misplaced implementation details to a
   Design or obligations to a Requirement using its authoring skill, retaining the choice, rationale, credible
   alternatives, and material consequences in the ADR. Destination artifacts cite it through `governed-by` where its
   rationale governs them. An authorised correction needs no renewed acceptance, special exception, or mandatory
   amendment log. The maintainer resolves uncertainty about whether the decision changes; accepted status, the section
   edited, and diff size alone do not justify supersession. Git preserves earlier versions.
6. Run `specful index`, then `specful validate`; commit the regenerated views with the change. Use `specful-review` when
   the adopting repository requires substantive review or the user asks for it.

For the full ADR profile and field-by-field guidance, see <https://unkos-dev.github.io/specful/>.
