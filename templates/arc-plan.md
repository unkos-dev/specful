---
type: arc-plan
status: draft
created: "{YYYY-MM-DD}"
issue: "{tracker ID or URL}"
relates-to:
  - "{Specful identifier or path}"
---

# {Arc plan title}

This plan is temporary and cannot amend canonical artifacts by itself. It may describe outcomes that differ from the
current specifications; each implementing change updates the corresponding Requirement, Design, or ADR through its
normal lifecycle. An unresolved conflict with a governing decision stops execution; a planned divergence from current
state does not.

`type`, `status`, and `created` are required. `status` is `draft`, `active`, `blocked`, or `complete`. `issue` and
`relates-to` are optional and are removed when they do not apply. `relates-to` holds Specful identifiers or paths
without interpreting them. Keep every section below; when one does not apply, state that briefly and give the reason.

## Objective

{The integrated outcome and why it requires more than one independently deliverable change.}

## Scope

{What the arc includes and excludes.}

## Binding inputs and decisions

{Each governing artifact or approved decision and what it locks. A proposed or superseded ADR is context, not
authority.}

## Models and flows

{A sequence, state, data-flow, architecture, or before-and-after model when it makes a material relationship easier to
assess. Otherwise state why a model does not help.}

## Dependency model

| Deliverable | Outcome | Hard dependencies | Exit criteria |
|---|---|---|---|
| {A} | {Coherent outcome} | {None or deliverable IDs} | {Observable completion condition} |

## Decision gates

{Unresolved choices that stop a deliverable and the owner who decides them. A change to a binding input stops for the
arc owner rather than becoming a routine amendment.}

## Integrated verification

{End-to-end, compatibility, or cross-deliverable proof that no child plan establishes alone.}

## Context capsule

- **Read first:** {Files and artifacts a fresh coordinator needs.}
- **Binding invariants:** {Rules every deliverable must preserve.}
- **Ownership:** {Branch, worktree, session, or external-system ownership where relevant.}
- **Commands:** {Verified coordination and validation commands.}

## Risks and contingencies

- {Credible cross-deliverable failure mode, compatibility concern, rollback need, or reason none applies.}

## Progress and hand-off

The status column uses the plan lifecycle values `draft`, `active`, `blocked`, and `complete`, applied to each
deliverable rather than to the whole arc.

| Deliverable | Status | Child plan | Delivered evidence |
|---|---|---|---|
| {A} | draft | {Path or not created} | {Commit, pull request, artifact, or verification result} |

Current checkpoint: Not started.

Next action: {The first concrete action.}

Active blockers: None.

### Amendments

None. A reality-driven split, insertion, skip, reorder, or abandoned deliverable is recorded here after its decision
gate is cleared.

### Final disposition

Pending.
