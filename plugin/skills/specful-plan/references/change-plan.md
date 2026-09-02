# Change-plan contract

Use a change plan for one coherent deliverable. Follow the repository's own template when it has one. Otherwise use the
fixed structure below; keep every section and give a brief reason when one does not apply.

Required frontmatter is `type: change-plan`, `status`, and `created`. Status is `draft`, `active`, `blocked`, or
`complete`. Optional `issue` and `relates-to` preserve external and Specful relationships. Optional `part-of` links a
child change plan back to its governing arc.

The fixed sections are:

1. `Outcome`: the problem, intended result, why it matters, and included and excluded work.
2. `Authority and evidence`: governing artifacts, approved decisions, applicable standards, relevant current behaviour,
   integration points, and constraints.
3. `Design`: the settled approach and material alternatives, with a model when one improves assessment.
4. `Acceptance criteria`: observable completed behaviour and the proof for each criterion.
5. `Context capsule`: the minimum stable files, invariants, ownership, and verified commands a fresh executor needs.
6. `Implementation`: outcome-sized tasks with integration points, evidence seams, edge cases, and focused checks.
7. `Risks and contingencies`: credible failure, compatibility, and recovery concerns.
8. `Progress and hand-off`: current checkpoint, next action, active blockers, then the fixed `### Amendments` and
   `### Final disposition` subheadings.

Frontmatter alone owns lifecycle status. Progress does not repeat it or accumulate routine command logs. A change plan
has no deliverable-status table because independently deliverable boundaries require an arc.

Plans are temporary and cannot amend canonical artifacts. When implementation changes current obligations, design, or a
durable decision, its tasks update the corresponding Requirement, Design, or ADR through that artifact's lifecycle.
