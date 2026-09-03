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

- **Problem:** {The specific problem and who experiences it.}
- **Affected user:** {The user, operator, or system experiencing it.}
- **Outcome:** {What becomes possible or reliably different.}
- **Invariant:** {The observable property every acceptable solution preserves, without naming a mechanism.}
- **Success signal:** {Evidence of improvement, or why acceptance fully captures it.}
- **Approach:** {The chosen solution in one sentence.}

## Recommendation

{Why this is the smallest coherent approach the repository supports, which primitives it reuses, and what machinery it
avoids.}

### Evidence

- `{path}:{lines}`: {Decisive behaviour, primitive, authority, or convention.}

### Alternatives

- {Rejected alternative and why it loses against the invariant, evidence, or ownership cost.}

## Root cause

{For asserted broken behaviour: observed failure, causal chain, fix boundary, regression proof, and uncertainty.
Otherwise state why this section does not apply.}

## Visuals

{A useful before-and-after flow or ownership, state, component, or data-flow diagram. Otherwise state why a model does
not help.}

## Implementation context

### Reading

| File | Lines | Why |
|---|---|---|
| `{path}` | {lines} | {Primitive, contract, integration point, or test precedent.} |

### Patterns and primitives

- `{path}:{lines}`: {Precedent or primitive to reuse.}

### Integration points

- `{path}:{line}`: {Current role and how the change connects.}

### Verified commands

- `{command}`

## Scope

### In scope

- {Agreed outcome.}

### Not building

- {Explicit exclusion and why it falls outside the invariant or belongs later.}

## Implementation

### 1. {Outcome}

#### Files and integration points

- `{path}:{line}` - {CREATE or UPDATE} - {Why this location owns the change.}

#### Implementation

- {Concrete behaviour, contract, or data flow, with a precedent cited by path and lines.}
- {Load-bearing boundary, failure behaviour, or gotcha.}

#### Tests

- {Behaviour to prove and the test surface that proves it.}

#### Validation

- `{command}` - {Expected observable result; the command fails when the behaviour is absent.}

## Acceptance

- **AC1:** {Observable outcome or preserved invariant.}

## Validation

| Gate | Command or procedure | Proves |
|---|---|---|
| {Gate} | `{command}` | {AC numbers} |

## Risks and decisions

| Decision or risk | Recommendation | Evidence or mitigation | Consequence if different |
|---|---|---|---|
| {Decision or risk} | {Recommendation} | {Evidence or mitigation} | {Consequence} |

## Progress and hand-off

Current checkpoint: Not started.

Next action: {The first concrete action.}

Active blockers: None.

### Amendments

None.

### Final disposition

Pending.
