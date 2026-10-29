---
type: arc-plan
status: draft
created: "{YYYY-MM-DD}"
issue: "{tracker ID or URL}"
relates-to:
  - "{Specful identifier or path}"
---

# {Arc plan title}

Once this plan is approved and implementation is authorised, execute it as written. Stop and obtain approval before any
deviation.

## Objective

{What is delivered and why.}

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

## Prerequisites

- [ ] `{command}` — {Expected result before Step 01.}

## Dependency graph

```mermaid
graph LR
    s01[Step 01] --> s02[Step 02] --> s04[Step 04]
    s01 --> s03[Step 03] --> s04
```

Parallel: {Steps that share no files and no outputs, or `None`.}

## Steps

One step is one pull request.

### Step 01: {Title}

**Depends on:** {Step numbers or `None`}

**Context:** {The state of the repository when this step starts, as an executor with no history needs it.}

**Tasks:**

1. {Verb-first instruction.}
2. {Verb-first instruction within the settled scope.}

**Rollback:** `{command}` — {What it restores.}

**Verification:**

- `{command}` — {Expected output.}

**Exit criteria:** {The resulting state, checkable without judgement.}

## Verification

| When | Command or procedure | Proves |
| --- | --- | --- |
| Every step | `{command}` | {Property that must hold throughout.} |
| Completion | `{command}` | {Objective it proves.} |

## Merge conditions

- {Act a maintainer performs after the pull request is green and before merge, with its rollback. The executor reports
  it unrun.}

## Risks

| Risk | Mitigation | Step |
| --- | --- | --- |
| {Risk} | {Mitigation} | {Step number} |

## Progress log

This table is the only record of execution state.

| Step | Status | Branch | PR | Notes |
| --- | --- | --- | --- | --- |
| 01 | pending | — | — | — |

Status is `pending`, `active`, `blocked`, `complete`, or `skipped`. A deviation awaiting approval is recorded in Notes
with the step `blocked`.
