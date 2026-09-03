---
title: Authoring workflow
description: The end-to-end path an agent or person follows to change a Specful repository's knowledge.
---

This is the complete path from "something changed" to "the corpus reflects it, and validation proves that." Follow it in
order; each step depends on the one before it.

The type-specific authoring skills, `specful-plan`, `specful-implement`, and `specful-review`, part of the opt-in
[agent skills](/specful/adoption/#install-agent-skills-optional), load this workflow in supported harnesses; this page
stays the canonical copy either way.

## 1. Retrieve what already exists

Start at `docs/specs/index.md` and follow the child indexes down to the module you need. Frontmatter carries each
artifact's identity and its relationships to others, so a module is navigable on its own once you reach it. Use
`specful show <ID>` to print the catalog record for an identifier, `specful trace <ID>` to follow requirement-to-design
links, or plain text search across `docs/specs/` and `docs/adr/`, which always works, with or without the CLI installed.
This step avoids duplicating an obligation, subject, or decision that is already recorded.

## 2. Decide which artifact changes

| Kind of change | Artifact | How it changes |
|---|---|---|
| Normative obligation | Requirement | Rewrite in place |
| How the system works | Design | Rewrite in place |
| Durable decision rationale | ADR | New record, old one superseded |
| Active transition | Plan | Archived or deleted once the transition lands |
| What used to be true | Git history | Never restated in current-state docs |

See [Requirement versus ADR](/specful/profiles/adr/#requirement-versus-adr) if the line between an obligation and a
decision is not obvious for this change.

## 3. Allocate an identifier

Create every new Requirement, Design, or ADR with `specful new`; never hand-allocate an identifier.

```sh
specful new requirement --title "Short navigation title" --scope backend/sync
specful new design --title "Short navigation title" --scope backend/sync
specful new adr --title "Short title naming the problem and chosen solution"
```

`specful new` scaffolds the artifact from its canonical template with the next allocated identifier for its kind, under
the owning architectural scope for a requirement or design.

## 4. Write the obligation, subject, or decision

Requirements and Designs describe current state only, written as though the system has always worked this way. Migration
history and rejected alternatives do not belong in either; durable rationale for a governing decision belongs in the ADR
it cites.

- A Requirement's Statement section uses at least one uppercase BCP 14 keyword (MUST, MUST NOT, SHOULD, SHOULD NOT,
  MAY), names the acting system or component and the triggering condition, and states an observable, checkable
  behaviour.
- A Design explains how one coherent subject currently works, in declarative present-tense prose, covering the canonical
  section set as a completeness baseline.
- An ADR records a decision event with its alternatives and reasoning; its outcome says "chosen option: X, because ...",
  never "the system MUST".

Every section heading in the scaffold stays, exactly as written; where a section does not apply, keep the heading and
state why.

## 5. Link relationships

A Design declares the Requirements it `satisfies`. A Requirement or Design cites its governing ADRs through
`governed-by`. ADR supersession is reciprocal: both the replaced and replacement records store the link, so either
document remains independently navigable. Remove a relationship field entirely when it is empty; do not leave a
placeholder.

## 6. Coordinate a multi-step transition, if this change is one

A settled change that fits one sitting may need no saved plan. Use `templates/change-plan.md` for one coherent work
order that needs a persistent hand-off. Use `templates/arc-plan.md` when several independently deliverable changes need
coordination. An arc contains the ordered steps, and each step is one pull request with its own context, tasks,
rollback, verification, and exit criteria. Write a separate change plan only when a step needs a standalone execution
packet.

`specful-plan` selects and fills the same templates. Every statement in the resulting plan is exact unless its task
states an open choice and the reason. `specful-implement` executes one step, stops when the repository contradicts the
plan or correctness requires a deviation, records the evidence, and waits for approval. It never merges. The adopting
repository owns the plan location, branch and publication rules, and retention policy; `plans/` is the default when it
has no established planning convention.

A plan is temporary. It cannot become the canonical home for durable rationale, which graduates to an ADR before the
plan is archived or deleted. Every fixed section remains present; a section that does not apply gives the reason.

## 7. Regenerate the navigation views

```sh
specful index
```

This rebuilds the per-scope `index.md` files and the machine-readable catalog under `.specful/generated/` from the
current source documents. Both views are disposable and carry no canonical knowledge; never hand-edit them.

## 8. Validate

```sh
specful validate
```

Validation must pass before the change is complete. A finding is fixed in the documents themselves, not managed in
configuration: there is no diagnostic rule registry, severity policy, or waiver system.

## 9. Commit the source and the regenerated views together

Commit the authored documents and the output of `specful index` in the same change. A committed view that disagrees with
the documents it was generated from is itself a validation failure, so the two can never be split across separate
commits without breaking the repository for whoever reviews the first one. If this change carries durable rationale that
has not yet graduated to an ADR, write that ADR now, before moving a completed plan out of the active set.
