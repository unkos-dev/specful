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

Once this plan is approved and implementation is authorised, execute it as written. Stop and obtain approval before any
deviation.

## Outcome

- **Problem:** {The specific problem and who experiences it.}
- **Outcome:** {What becomes possible or reliably different.}
- **Invariant:** {The observable property every acceptable solution preserves, without naming a mechanism.}
- **Success signal:** {Evidence of improvement, or why acceptance fully captures it.}

## Not building

- {Exclusion and the reason it is out of scope.}

## Documentation

| Artifact | Disposition | Reason or task |
| --- | --- | --- |
| `{Specful identifier or path}` | {created, updated, unaffected, or none} | {Reason, or the task that changes it.} |

Unapproved documentation proposals: {Subject, artifact type and durable benefit for decision, or `None`; no committed
task until approved.}

Corpus effect: {more complete, unchanged, or less accurate, with the reason.}

## Binding inputs

| Artifact | Lines | What it locks |
| --- | --- | --- |
| `{path or identifier}` | {lines} | {Decision or constraint.} |

## Design decisions

1. **{Decision}:** {Governing artifact and operative clause, or explicit user decision, that settles this material
   choice; rationale and alternative it beats.}

## Open decisions

{Keep unresolved material choices here; do not write dependent implementation tasks until they are settled.}

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
2. {Verb-first instruction within the settled scope.}

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
