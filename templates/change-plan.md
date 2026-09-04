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

Every statement in this plan is exact. A deviation stops for approval.

## Outcome

- **Problem:** {The specific problem and who experiences it.}
- **Outcome:** {What becomes possible or reliably different.}
- **Invariant:** {The observable property every acceptable solution preserves, without naming a mechanism.}
- **Success signal:** {Evidence of improvement, or why acceptance fully captures it.}

## Not building

- {Exclusion and the reason it is out of scope.}

## Binding inputs

| Artifact | Lines | What it locks |
| --- | --- | --- |
| `{path or identifier}` | {lines} | {Decision or constraint.} |

## Design decisions

1. **{Decision}:** {Rationale, and the alternative it beats.}

## Open decisions

| Decision | Options | Recommendation | Owner | Consequence if different |
| --- | --- | --- | --- | --- |
| {Decision} | {Options} | {Recommendation} | {Owner} | {How the plan changes.} |

## Root cause

{For asserted broken behaviour: observed failure, causal chain, fix boundary as `path:line`, regression proof. Otherwise
state that no broken behaviour was asserted.}

## Reading

| File | Lines | Why |
| --- | --- | --- |
| `{path}` | {lines} | {Primitive, contract, integration point, or precedent it carries.} |

## Tasks

### 1. {Outcome}

**Files:**

- `{path}:{line}` — {CREATE or UPDATE} — {Why this location owns the change.}

**Do:**

1. {Verb-first instruction, with the precedent to mirror by `path:lines`.}
2. {Verb-first instruction. Open choice: {what is open}, because {reason}.}

**Verify:**

- `{command}` — {Expected output; fails when this task's behaviour is absent.}

## Acceptance

- **AC1:** {Observable outcome or preserved invariant.}

## Verification

| Gate | Command or procedure | Proves |
| --- | --- | --- |
| {Gate} | `{command}` | {AC numbers} |

## Merge conditions

- {Act a maintainer performs after the pull request is green and before merge, with its rollback. The executor reports
  it unrun.}

## Risks

| Risk | Mitigation | Task |
| --- | --- | --- |
| {Risk} | {Mitigation} | {Task number} |

## Progress

Checkpoint: {Last completed state.}

Next: {The next task.}

Blocked: {Deviation awaiting approval with its evidence, or `None`.}

Amendments: {Approved deviations, dated, or `None`.}
