---
name: specful-implement
description: >-
  Use when the user asks to implement, execute, resume, or continue a saved change plan or arc plan step by step.
---

# Implementing a plan

Execute the plan the user names, one step at a time. The plan is the specification. The repository's instructions own
branching, commits, pushes, and merges; this skill never merges.

A change plan is one step. Its frontmatter `status` is the step status. Checkpoint records the last completed task, its
evidence, and the branch when one exists. Next names the current task and nothing else. Blocked records a deviation and
its evidence. Amendments records approved deviations with their dates. While a step with its own change plan is active,
that plan's frontmatter is authoritative and the arc row is updated from it at the step boundary.

A change plan starts in `draft`; treat that as its pending state. For a change plan, references below to Notes mean
Blocked for a deviation and Amendments for an approved change. Its Verification table supplies the Completion
verification rows.

## Read

1. Read the plan in full, then the artifacts its Binding inputs cite at the cited lines.
2. Read the Progress log. The first `pending` or `active` step is the current step; never repeat a `complete` or
   `skipped` step.
3. For an `active` step, check its branch when the repository has one. With commits, judge the work against the step's
   Exit criteria. Without commits, treat the step as `pending`.
4. Confirm every Prerequisite with its command before the first step.

## Execute one step

1. Set the step `active` in the Progress log and record the branch when the repository has one.
2. Read the step's Context and confirm it matches the repository. A mismatch is a deviation.
3. Do the tasks in order. Every statement is exact. An open choice stated in the task with a reason is the executor's;
   record the choice made in Notes. Where the plan is silent, follow the repository's instructions and do what is
   correct.
4. Run each task's Verify command and compare the output to the expected result. A mismatch is a deviation.
5. Run the step's Verification and the plan's every-step rows. Check the Exit criteria.
6. Set the step `complete` with its evidence in Notes, then stop and report. The user starts the next step.

## Deviation

A deviation is anything the plan states that the repository contradicts, any change correctness requires beyond what the
plan states, or any verification that does not match its expected result.

1. Stop the step. Do not work around it.
2. Set the step `blocked`. Record in Notes what the plan states, what was found, and the proposed change with its
   evidence.
3. Report and wait. On approval, record the change under Amendments with the date, then continue.

## Mutate

Apply only an approved change. Record every mutation in Notes with its reason.

- Split: rename Step N to Na, add Nb, update the graph.
- Insert: add a letter suffix, never renumber, update the graph.
- Skip: mark the heading `[SKIP: reason]`, set the row `skipped`, never delete.
- Reorder: only where the graph allows, then re-check that no step reads a later step's output.
- Abandon: set frontmatter `status: complete`, record the reason in Notes, never delete.
- A change to a Binding input is not a mutation; it stops for the owner.

## Complete

When every step is `complete` or `skipped`, run the plan's Completion verification rows. When they pass, set frontmatter
`status: complete` and report. When durable rationale in the plan has no ADR, say so in the report; do not write one.
