# Arc-plan contract

Use an arc plan to coordinate multiple independently deliverable changes. Follow the repository's own template when it
has one. Otherwise use the fixed structure below; keep every section and give a brief reason when one does not apply.

Required frontmatter is `type: arc-plan`, `status`, and `created`. Status is `draft`, `active`, `blocked`, or
`complete`. Optional `issue` and `relates-to` preserve external and Specful relationships.

The fixed sections are:

1. `Objective`: the integrated outcome and why it needs an arc.
2. `Scope`: included and excluded work.
3. `Binding inputs and decisions`: governing artifacts, approved decisions, and what each locks.
4. `Models and flows`: a useful sequence, state, data-flow, architecture, or before-and-after model, or why none helps.
5. `Dependency model`: each deliverable's outcome, hard dependencies, and exit criteria.
6. `Decision gates`: unresolved choices and their owner. A binding-input change stops here for owner approval.
7. `Integrated verification`: cross-deliverable proof that no child establishes alone.
8. `Context capsule`: stable files, invariants, ownership, and verified commands for a fresh coordinator.
9. `Risks and contingencies`: credible cross-deliverable failure, compatibility, and recovery concerns.
10. `Progress and hand-off`: deliverable status and child path, delivered evidence, current checkpoint, next action, and
    active blockers, then the fixed `### Amendments` and `### Final disposition` subheadings.

Frontmatter owns the arc lifecycle. The progress table uses the same `draft`, `active`, `blocked`, and `complete` values
for each deliverable; a blocked row does not by itself make the whole arc blocked. A child change plan owns its detailed
checkpoint and next action. The arc records only the child's boundary status, path, and delivered evidence.

Create child plans only when a deliverable needs standalone coordination. For an implementation-planning request, that
normally means the first executable child. Create later children from delivered evidence rather than early assumptions.

Plans are temporary and cannot amend canonical artifacts. A proposed or superseded ADR is context, not authority. A
change to a binding input is approved through Decision gates before it is recorded as an amendment.
