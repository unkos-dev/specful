---
title: Authoring workflow
description: Use specifications to understand a codebase, change it, and review the result.
---

Use this workflow for ordinary code changes as well as artifact authoring. Start by finding the relevant knowledge and
deciding what the change affects. A code change may need no artifact edit or saved plan; apply the authoring steps only
when a document needs to change. Coordinate a multi-step transition before implementation, as described in step 6.

The opt-in [agent skills](/specful/adoption/#install-agent-skills-optional) apply this workflow during planning,
implementation and review; the type-specific skills cover artifact craft. This page is the canonical procedure for
people and agents, including contributors who use neither skills nor saved plans.

## 1. Retrieve what already exists

Read the repository's instructions, then start at `docs/specs/index.md` and follow the scope indexes to the subject you
need. Read its relevant Requirements for obligations and acceptance criteria, Designs for structure and behaviour, and
governing ADRs for the alternatives and reasons behind decisions. Use `specful show <ID>` to print the catalog record
for an identifier, `specful trace <ID>` to follow requirement-to-design links or list the artifacts that cite an ADR.
The Markdown files and their links remain readable without the CLI.

Follow the relevant Design into the implementation and existing tests. Check its claims against those sources before
choosing an approach: the documents guide inspection but do not prove what the code does. If an index is missing, or
coverage or directions are inaccurate, use targeted text search, broadening only as needed. Report the gap rather than
treating missing documentation as permission to invent a contract.

## 2. Decide what the change affects

Identify the obligations and acceptance criteria the requested change could affect, including behaviour it must
preserve. If code and documentation disagree, surface the evidence and obtain a decision before changing an approved
obligation or durable choice. A refactor that preserves the documented behaviour may need only code changes and
verification; skip steps 3-5 when no artifact change is warranted.

| Kind of change | Artifact | How it changes |
|---|---|---|
| Normative obligation | Requirement | Rewrite in place |
| How the system works | Design | Rewrite in place |
| Durable decision rationale | ADR | New record, old one superseded |
| Active transition | Plan | Archived or deleted once the transition lands |
| What used to be true | Git history | Never restated in current-state docs |

See [Requirement versus ADR](/specful/profiles/adr/#requirement-versus-adr) if the line between an obligation and a
decision is not obvious for this change.

Missing coverage prompts a decision, not automatic authoring. Propose a Requirement when an unrecorded durable
obligation needs to be explicit, a Design when understanding a coherent subject or its interacting components would
otherwise be repeatedly lost, or an ADR when a consequential choice needs its alternatives and rationale retained. Name
the proposed subject, artifact type and lasting benefit, then ask for a decision and any missing input before writing
it. Small implementation details and already-explained subjects do not need another document. Gradual, partial coverage
is valid; do not require an artifact for every task.

Implement the authorised code change under the repository's contribution rules, keeping affected artifacts accurate. Use
step 6 first if the work needs a saved plan; otherwise proceed to the relevant implementation and corpus checks without
creating one.

## 3. Allocate an identifier

After approval to add an artifact, create it with `specful new`; never hand-allocate an identifier. Existing artifacts
keep their identifiers when edited.

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

Every required section heading in the scaffold stays, exactly as written; where a section does not apply, keep the
heading and state why. More information is optional on every artifact kind, as is an ADR's Pros and cons of the options,
and either is removed completely when it adds nothing.

## 5. Link relationships

A Design declares the Requirements it `satisfies`. A Requirement or Design cites its governing ADRs through
`governed-by`. ADR supersession is reciprocal: both the replaced and replacement records store the link, so either
document remains independently navigable. Remove a relationship field entirely when it is empty; do not leave a
placeholder. Relationships live in frontmatter only: `specful show` renders them, and More information never restates a
`satisfies`, `governed-by`, or supersession edge as a prose link.

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

A plan is temporary. If it contains durable rationale worth retaining, propose an ADR for a decision before archiving or
deleting the plan; do not create it without approval. Every fixed plan section remains present; a section that does not
apply gives the reason.

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

Run the repository's relevant implementation checks as well. Reuse existing tests and inspection evidence before
proposing additional tests. For each consequential affected acceptance criterion, record the test or inspection that
supports it, the observed result and the reviewed revision in the ordinary change record or review. State any unresolved
gap explicitly. A passing `specful validate` establishes corpus conformance, and a `satisfies` edge declares a
relationship; neither proves that the implementation meets the Requirement. Keep verification results with the change,
not as permanent pass badges in the Requirement.

## 9. Commit the source and the regenerated views together

When authorised to commit, keep authored documents and the output of `specful index` in the same change. A committed
view that disagrees with the documents it was generated from is itself a validation failure, so the two can never be
split across separate commits without breaking the repository for whoever reviews the first one.

## Worked example: understand identifier allocation

A contributor changing `specful new` can follow this path in the Specful source checkout:

1. Open [docs/specs/index.md][spec-index], then its [authoring scope][authoring-index]. The scope lists
   [SPECFUL-REQ-0001][allocation-req], Stable identifier allocation, and [SPECFUL-DESIGN-0001][allocation-design],
   Identifier allocation and scaffolding.
2. Read the Requirement to learn what must hold: identifiers come from persistent per-kind counters and are not reused
   after deletion. Its acceptance criteria also cover consecutive allocations, skipped numbers and lock collisions.
3. Read the Design to find the mechanism in [src/authoring.rs][authoring-source]: `new_artifact` acquires `ConfigLock`,
   allocates from configuration and commits the advanced counter before creating the artifact. Inspect those operations
   and [tests/authoring.rs][authoring-tests] to check the description, especially failure paths where a number is
   consumed without a completed file.
4. Follow the Design's `governed-by` relationship to [SPECFUL-ADR-0002][requirement-adr]. It explains why a Requirement
   owns its identity and metadata instead of belonging to an authored container, including the trade-off of more files.
   Its More information section names a reason to reconsider: requirements-only authoring proving unworkable in a real
   adopting repository. Contrary evidence warrants a lifecycle decision, not an unannounced rewrite of the accepted ADR.

From that checkout, use the source-built CLI to check the records and their relationships:

```sh
cargo run -- show SPECFUL-REQ-0001
cargo run -- trace SPECFUL-REQ-0001
cargo run -- show SPECFUL-DESIGN-0001
cargo run -- trace SPECFUL-DESIGN-0001
cargo run -- show SPECFUL-ADR-0002
cargo run -- trace SPECFUL-ADR-0002
```

The Requirement traces to the Design, the Design traces back to the Requirement, and the ADR trace lists that Design as
a citing artifact. These results establish navigation, not correctness. For a lock-handling change, inspect and run the
existing authoring tests and relate their results to the affected criteria; a stale-lock test alone does not prove all
concurrent interleavings. Before editing, explain the obligation, the code that implements it and the reason for the
identity model. Record remaining verification gaps with the change.

[spec-index]: https://github.com/unkos-dev/specful/blob/main/docs/specs/index.md
[authoring-index]: https://github.com/unkos-dev/specful/blob/main/docs/specs/authoring/index.md
[allocation-req]: https://github.com/unkos-dev/specful/blob/main/docs/specs/authoring/requirements/0001-stable-identifier-allocation.md
[allocation-design]: https://github.com/unkos-dev/specful/blob/main/docs/specs/authoring/design/0001-identifier-allocation-and-scaffolding.md
[authoring-source]: https://github.com/unkos-dev/specful/blob/main/src/authoring.rs
[authoring-tests]: https://github.com/unkos-dev/specful/blob/main/tests/authoring.rs
[requirement-adr]: https://github.com/unkos-dev/specful/blob/main/docs/adr/0002-represent-requirements-as-first-class-artifacts.md
