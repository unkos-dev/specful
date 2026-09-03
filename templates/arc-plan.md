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

- **Problem:** {The specific problem and who experiences it.}
- **Affected user:** {The user, operator, or system experiencing it.}
- **Outcome:** {What becomes possible or reliably different.}
- **Invariant:** {The observable property every acceptable solution preserves, without naming a mechanism.}
- **Success signal:** {Evidence of improvement, or why acceptance fully captures it.}
- **Approach:** {The chosen coordinating solution in one sentence.}

## Recommendation

{Why an arc is the smallest coherent shape, which primitives it reuses, and what machinery it avoids.}

### Evidence

- `{path}:{lines}`: {Decisive behaviour, primitive, authority, or convention.}

### Alternatives

- {Rejected decomposition and why it loses against the invariant, evidence, or ownership cost.}

## Binding inputs

| Artifact | Lines | What it locks |
|---|---|---|
| `{path or identifier}` | {lines} | {Decision or constraint.} |

## Visuals

{A useful dependency, sequence, state, ownership, architecture, or before-and-after model. Otherwise state why a model
does not help.}

## Deliverables

| Deliverable | Outcome | Hard dependencies | Exit criteria | Child plan |
|---|---|---|---|---|
| {A} | {Coherent outcome} | {None or deliverable IDs} | {Observable completion condition} | {Path or not created} |

## Decision gates

| Gate | Options | Recommendation | Owner | Consequence if different |
|---|---|---|---|---|
| {Gate} | {Options} | {Recommendation} | {Owner} | {How the arc changes.} |

## Integrated verification

| Gate | Command or procedure | Proves |
|---|---|---|
| {Gate} | `{command or procedure}` | {Acceptance criteria or invariant.} |

## Risks and decisions

| Decision or risk | Recommendation | Evidence or mitigation | Consequence if different |
|---|---|---|---|
| {Decision or risk} | {Recommendation} | {Evidence or mitigation} | {Consequence} |

## Progress and hand-off

The status column uses `draft`, `active`, `blocked`, or `complete` for each deliverable, not for the whole arc.

| Deliverable | Status | Child plan | Delivered evidence |
|---|---|---|---|
| {A} | draft | {Path or not created} | {Commit, pull request, artifact, or verification result} |

Current checkpoint: Not started.

Next action: {The first concrete action.}

Active blockers: None.

### Amendments

None. Record a reality-driven split, insertion, skip, reorder, or abandoned deliverable after its decision gate clears.

### Final disposition

Pending.
