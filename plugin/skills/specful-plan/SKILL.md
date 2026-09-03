---
name: specful-plan
description: >-
  Use when a change needs a persistent implementation plan, unresolved design choices must be settled before planning,
  or an arc must coordinate multiple deliverables.
---

# Planning a change

Produce the smallest plan that a human can review and a fresh executor can implement without rediscovering material
decisions. Planning is read-only apart from the plan files the user authorises. Do not implement, create a branch,
commit, publish, or mutate a tracker. Approval of a plan grants no implementation authority.

## Resolve authority and intent

Read the applicable repository instructions and planning convention first. Treat issues, discussions, existing code, and
prior plans as evidence rather than authority. In a Specful repository, Requirements and Designs constrain the delivered
state, accepted ADRs constrain durable choices, and a plan remains temporary transition coordination.

Establish the problem, intended outcome, fixed constraints, and the observable result. Preserve authoritative and
explicitly approved decisions. Surface contrary evidence instead of silently reopening or overriding them.

## Decide whether design is settled

Read [planning craft](references/planning-craft.md) and state the invariant before judging the design. Proceed directly
when the governing artifacts and approved discussion settle the material design. Otherwise inspect the repository,
gather only evidence that can change the approach, present credible options with a recommendation, and wait for the
user's decision before writing implementation tasks.

Gather evidence the executor would otherwise rediscover: precise `path:line` references, the primitives and extension
points that already exist, the closest useful precedent, the repository's verified validation commands, and the
conventions the change must preserve. Inspect the decisive files directly.

Diagnose asserted broken behaviour before planning a fix. Use external research, a throwaway spike, or delegation only
when its result can materially change the plan. The workflow must remain executable by one capable agent.

## Select the planning shape

- Do not create a saved plan for clear, bounded work unless the user requests one.
- Use one change plan for one coherent deliverable.
- Use an arc plan when several changes are independently deliverable or have meaningful dependency or decision gates.
- For implementation planning, create the arc and its first executable child change plan when that child needs
  standalone coordination. For roadmap or decomposition work, the arc alone may be sufficient.
- Create later child plans just in time from delivered evidence. Do not expand every future child from assumptions.

Follow the repository's existing plan location, plan template, tracking, and retention policy. When none exists, propose
`plans/` and ask once whether plans should be tracked or ignored before writing. When the repository defines no filename
convention, use `YYYY-MM-DD-<descriptive-slug>.md`. A template is a file the repository designates as one; prior plans
are evidence of convention, not templates. An explicit user choice wins.

## Write the plan

For a change plan, read [the change-plan contract](references/change-plan.md). For an arc, read
[the arc-plan contract](references/arc-plan.md); also read the change-plan contract when creating its first child. Use
the repository's own plan template when it exists. Otherwise use the selected reference's canonical structure.

Keep exact paths, symbols, commands, and line references only where verified and useful. Tasks are outcome-sized and
name their integration points, behavioural evidence, material edge cases, and focused checks. Add procedural detail only
where its absence would force consequential rediscovery. State each fact once in the section that owns it. In other
sections, refer to that section when needed instead of restating the fact.

Issue and tracker inputs may supply context. Follow discussion and linked material only while they can change scope,
authority, or decisions. Never publish or update external state without separate authority.

## Validate and hand off

Before completing the plan:

- verify cited paths, claims, commands, dependencies, and artifact relationships;
- map every accepted outcome to implementation work and verification;
- confirm material decisions are approved and active blockers name an owner and exit condition;
- check that fixed sections remain present and not-applicable reasons are specific;
- confirm decisive references carry real paths and line numbers, every validation command exists and states its expected
  result, and no placeholders remain;
- confirm dependency availability from the importing package's manifest, not its lockfile, and make any dependency
  change explicit;
- confirm Implementation context holds stable cold-start context while Progress and hand-off holds changing state;
- run the repository's applicable document checks, or report that none are defined after checking its instructions and
  command surfaces.

Report the written paths, selected shape, approved decisions, unresolved gates, first executable boundary, and verified
commands. Stop there. The user separately decides whether to review or implement the plan.
