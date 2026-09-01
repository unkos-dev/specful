---
name: specful-author
description: >-
  Use when writing or updating a Specful Requirement, Design, or ADR: a new specification, a change to normative
  obligations or how the system works, or a durable decision record.
compatibility: Requires specful 0.3.0 or later on PATH.
---

# Authoring a Specful artifact

The adopting repository's own `docs/SPECFUL.md` is authoritative. Where anything here differs from it, follow
`docs/SPECFUL.md` instead.

## Workflow

1. Read `docs/SPECFUL.md` in the repository root for the current record model, the writing rules (current-state prose,
   the BCP 14 Statement keyword requirement, `satisfies` and `governed-by` links), and the retrieval recipe.
2. Start at `docs/specs/index.md` and follow the child scope indexes down to the module the change belongs to. Inspect
   related Requirements, Designs, and ADRs already there before writing anything, so the new or changed artifact is
   consistent with its neighbours.
3. Scaffold the artifact with `specful new`. Never hand-allocate an identifier; the command owns the counter in
   `.specful/config.yaml`. Run `specful new --help` for the exact flags for each artifact kind.
4. Complete the scaffold's placeholders. Requirements and Designs describe current state only, written as though the
   system has always worked this way; a transition belongs in a plan under `plans/`, not in the artifact prose. A Design
   declares the Requirements it `satisfies`; a Requirement or Design cites its governing ADRs through `governed-by`.
5. Run `specful index` to regenerate the navigation views and catalog, then `specful validate` to check the result.
   Commit the regenerated views with the change.
6. Review the artifact for substantive completeness. Mechanical validation is necessary but not sufficient: load the
   `specful-review` skill for the review checklist before treating the change as done.

For the full artifact profiles, field-by-field guidance, and CLI reference, see
<https://unkos-dev.github.io/specful/>.
