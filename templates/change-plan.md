---
type: change-plan
status: draft
created: "{YYYY-MM-DD}"
issue: "{tracker ID or URL}"
relates-to:
  - "{Specful identifier or path}"
part-of: "{path to the governing arc plan}"
---

# {Plan title}

This plan is temporary and cannot amend canonical artifacts by itself. It may describe an outcome that differs from the
current specifications; the implementing change updates the corresponding Requirement, Design, or ADR through its normal
lifecycle. An unresolved conflict with a governing decision stops execution; a planned divergence from current state
does not.

`type`, `status`, and `created` are required. `status` is `draft`, `active`, `blocked`, or `complete`. `issue`,
`relates-to`, and `part-of` are optional and are removed when they do not apply. `relates-to` holds Specful identifiers
or paths without interpreting them. `part-of` links a child change plan back to its governing arc plan. Keep every
section below; when one does not apply, state that briefly and give the reason.

## Outcome

{The problem, intended result, why it matters, and what the change includes and excludes.}

## Authority and evidence

{Governing artifacts, approved decisions, applicable standards, and the relevant current behaviour, integration points,
and constraints.}

## Design

{The settled approach, material alternatives, and any owner-approved choices. Include a sequence, state, data-flow,
architecture, or before-and-after model when it makes a material relationship easier to assess.}

## Acceptance criteria

- {Observable completed behaviour and the proof that establishes it.}

## Context capsule

- **Read first:** {Files and artifacts a fresh executor needs.}
- **Binding invariants:** {Rules this change cannot violate.}
- **Ownership:** {Branch, worktree, session, or external-system ownership where relevant.}
- **Commands:** {Verified repository-native implementation and validation commands.}

## Implementation

1. **{Outcome-sized task}**
   - {Affected files or integration points.}
   - {Behavioural test or evidence seam and material edge cases.}
   - {Focused verification.}

## Risks and contingencies

- {Credible failure mode, compatibility concern, rollback need, or reason none applies.}

## Progress and hand-off

Current checkpoint: Not started.

Next action: {The first concrete action.}

Active blockers: None.

### Amendments

None.

### Final disposition

Pending.
