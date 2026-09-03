---
name: specful-plan
description: >-
  Use when a change needs a persistent implementation plan, unresolved design choices must be settled before planning,
  or an arc must coordinate multiple deliverables.
---

# Planning a change

The plan's reader is an agent with no prior context. Planning is read-only apart from the plan files the user
authorises. Do not implement, create a branch, commit, publish, or mutate a tracker. Approval of a plan grants no
implementation authority.

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
user's decision before writing tasks.

Gather evidence the executor would otherwise rediscover: precise `path:line` references, the primitives and extension
points that already exist, the closest useful precedent, the repository's verified validation commands, and the
conventions the change must preserve. Inspect the decisive files directly.

Diagnose asserted broken behaviour before planning a fix. Use external research, a throwaway spike, or delegation only
when its result can materially change the plan. The workflow must remain executable by one capable agent.

## Select the planning shape

- Do not create a saved plan for clear, bounded work unless the user requests one.
- Use one change plan for one coherent deliverable.
- Use an arc plan when several changes are independently deliverable or have meaningful dependency or decision gates.
  One step is one pull request; a step's own change plan exists only when the step needs a standalone execution packet,
  and it cites the arc's step and task numbers rather than restating them.
- For implementation planning, write the arc and the first executable step's change plan when that step needs one. Write
  later steps' change plans from delivered evidence, never from assumptions.

Follow the repository's existing plan location, plan template, tracking, and retention policy. When none exists, propose
`plans/` and ask once whether plans should be tracked or ignored before writing. When the repository defines no filename
convention, use `YYYY-MM-DD-<descriptive-slug>.md`. A template is a file the repository designates as one; prior plans
are evidence of convention, not templates. An explicit user choice wins.

## Write the plan

For a change plan, read [the change-plan template](references/change-plan.md). For an arc, read
[the arc-plan template](references/arc-plan.md). Use the repository's own plan template when it exists. Keep every
section; a section that does not apply says so with the reason. Remove an optional frontmatter field that does not
apply.

Every statement in a plan is exact; the executor treats deviation as a stop. State an open choice in its task with the
reason. Write tasks verb-first. Every task verifies with a command and its expected output, and the command fails when
that task's behaviour is absent. When a task creates a short file whose content is the specification, quote the exact
content in the task. A step's Context describes the repository state when the step starts, not the history that produced
it.

Issue and tracker inputs may supply context. Follow discussion and linked material only while they can change scope,
authority, or decisions. Never publish or update external state without separate authority.

## Review and hand off

Check the plan against each item, then fix what fails:

- every cited path, line range, command, and dependency exists; a dependency is declared in the importing package's
  manifest, not its lockfile;
- every step or task can be executed from its own section plus Binding inputs, Design decisions, and Verification;
- no two steps share a verification command unless they change the same behaviour;
- every exit criterion and expected output is checkable without judgement;
- every step with an irreversible effect has a rollback;
- every declared dependency edge matches an artifact one step produces and another consumes;
- every open choice names what is open and why;
- every deliverable in the objective maps to a task and a verification row;
- every count and quantity stated in prose matches its source;
- no placeholder, narrative paragraph, or generic example remains.

Run the repository's applicable document checks, or report that none are defined after checking its instructions and
command surfaces.

Report the written paths, selected shape, approved decisions, open decisions, first executable step, and verified
commands. Stop there. The user separately decides whether to review or implement the plan.
