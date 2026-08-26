---
type: change-plan
status: "{draft | active | complete}"
created: "{YYYY-MM-DD}"
issue: "{tracker ID, one string}"
governed-by:
  - "{PROJECT-ADR-NNNN}"
relates-to:
  - "{specful identifier or path}"
part-of: "{path to the umbrella arc plan}"
---

# {Plan title}

This plan is temporary and subordinate: where it disagrees with a committed
specification, decision, or the charter, the committed artifact wins.

All frontmatter fields are optional; a bare title is a valid plan. `status`
is one of `draft`, `active`, or `complete`. `governed-by` lists ADR
identifiers only, reusing the spec profiles' field name; plans may cite
other plans and committed specful artifacts, but MSRS, MSDD, and ADR
artifacts never cite plans. `relates-to` holds specful identifiers or
paths, uninterpreted. `issue` is one tracker-agnostic string; a
repository needing more lists them in prose. `part-of` names this change
plan's umbrella arc plan; like other path-valued fields, it goes stale
once a plan moves to `plans/archive/`, accepted for an authoring
convention.

## Problem

{What needs to change, and why it matters now.}

## Approach

<!-- Optional. Remove if the tasks below are self-explanatory. -->

{The chosen approach, and why a materially different alternative was not
taken.}

## Tasks

- {Task}

## Verification

{Commands, tests, or checks that confirm the change is complete and
correct.}

## Deferred by decision

<!-- Optional. Remove if nothing was deliberately excluded. -->

- {Scope deliberately excluded from this plan, and why.}

## Completion

Graduate durable rationale to an ADR, then move this file out of the
active set, to `plans/archive/` or by deletion, per repository policy.
