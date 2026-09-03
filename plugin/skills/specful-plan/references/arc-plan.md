# Arc-plan contract

Use an arc plan to coordinate multiple independently deliverable changes. Follow the repository's own template when it
has one. Otherwise use the fixed structure below; keep every section and give a brief reason when one does not apply.

Required frontmatter is `type: arc-plan`, `status`, and `created`. Status is `draft`, `active`, `blocked`, or
`complete`. Optional `issue` and `relates-to` preserve external and Specful relationships.

Every section holds one kind of content. A fact lives in the section typed for it and is cited by its identifier
elsewhere: a file by `path:lines`, an acceptance criterion by its `AC` number, a decision or deliverable by its row.

## Fixed sections

### Objective

The same labelled fields as a change-plan Outcome, in this order: `Problem`, `Affected user`, `Outcome`, `Invariant`,
`Success signal`, and `Approach`. The outcome explains why it requires multiple independently deliverable changes.

### Recommendation

One paragraph on why an arc is the smallest coherent shape, which existing primitives it reuses, and what machinery it
avoids. Then `Evidence` bullets cite decisive `path:lines`, and `Alternatives` records each rejected decomposition and
why it loses against the invariant, evidence, or ownership cost.

### Binding inputs

A table with columns `Artifact`, `Lines`, `What it locks`. Governing Specful artifacts, accepted ADRs, and explicit user
decisions come before contextual evidence. A proposed or superseded ADR is context, not authority.

### Visuals

A sequence, state, data-flow, architecture, or before-and-after model when it makes dependencies, ownership, or the
integrated outcome easier to verify. Otherwise state why a model does not help.

### Deliverables

A table with columns `Deliverable`, `Outcome`, `Hard dependencies`, `Exit criteria`, `Child plan`. Each row is one
independently deliverable boundary. Create a child plan only when that boundary needs standalone coordination.

### Decision gates

A table with columns `Gate`, `Options`, `Recommendation`, `Owner`, `Consequence if different`. A change to a binding
input stops at a gate for owner approval.

### Integrated verification

A table with columns `Gate`, `Command or procedure`, `Proves`. It contains cross-deliverable proof that no child plan
establishes alone and names the acceptance criteria or invariant each gate proves.

### Risks and decisions

A table with columns `Decision or risk`, `Recommendation`, `Evidence or mitigation`, `Consequence if different`. Keep
minor coordination decisions here; resolve an architectural fork with the user before writing the arc.

### Progress and hand-off

A deliverable-status table with columns `Deliverable`, `Status`, `Child plan`, `Delivered evidence`, followed by the
current checkpoint, next action, active blockers, and fixed `### Amendments` and `### Final disposition` subheadings.

## Rules

Frontmatter owns the arc lifecycle. The progress table uses `draft`, `active`, `blocked`, and `complete` for each
deliverable; a blocked row does not by itself make the whole arc blocked. A child change plan owns its detailed
checkpoint and next action. The arc records only the child's boundary status, path, and delivered evidence.

For implementation planning, create the first executable child plan when it needs standalone coordination. Create later
children from delivered evidence rather than early assumptions. A child cannot redefine arc-level decisions.

Plans are temporary and cannot amend canonical artifacts. A binding-input change passes through Decision gates before it
is recorded under Amendments. No placeholders, generic examples, confidence scores, or coverage targets remain in a
saved plan.
