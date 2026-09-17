# Change and plan review lens

Apply the [shared review standard](../SKILL.md) to the agreed change or plan. Use the applicable corpus and the
[authoring workflow](https://unkos-dev.github.io/specful/authoring-workflow/#2-decide-what-the-change-affects) to
understand the affected subjects. Its documentation obligations do not override the review's advisory treatment of
missing coverage.

## Changes

Understand the intended outcome and inspect the aggregate change with the surrounding source needed to establish its
behaviour. Trace affected interfaces, callers, state and failure paths across files. Judge correctness, security and
technical fitness, including whether the project already provides a suitable solution. Inspect existing tests where they
bear on a concrete concern; a passing check proves only what it exercises.

Follow relevant scope indexes to the Designs describing affected subjects, their Requirements and governing ADRs. Read
related records and inbound references where they can expose a consequential contradiction. Inspect source relationships
directly or use available read-only lookups; a particular CLI command is not a prerequisite. Do not audit unrelated
subjects or demand a disposition table for every file.

## Plans

Judge whether the proposed solution is technically appropriate, coherent, sufficiently specified and executable. Trace
its intended behaviour through the relevant existing implementation and contracts. Check the ordering of dependent work,
whether each dependency can actually be supplied, and whether the proposed boundaries, failure handling and relevant
standards fit the problem.

Report missing decisions that would make an implementer build the wrong solution or become stuck. Distinguish these from
details an implementer can reasonably decide. Do not demand every implementation detail, an acceptance-evidence matrix,
or completed tests and artifacts for work that has not begun. A sound plan can receive `SHIP` before implementation.

Challenge unnecessary machinery even when the plan faithfully follows a proposed approach. Consider existing project
solutions and standard alternatives, preserving required semantics and explaining trade-offs. Present the sound
recommendation rather than silently replacing a user decision.

## Corpus impact and authority

Assess whether the change warrants a Requirement, Design update or durable decision record even when no Specful files
changed. Most internal changes create no new observable obligation. Recommend an artifact only when it has a reason to
exist, and explain its subject, type and durable benefit.

Missing or stale coverage stays advisory whether it predates the change or the change creates or widens it. An authoring
rule requiring an update does not make that omission blocking. Do not require a separate corpus-effect label or proposal
inventory; include useful advice in the report.

A substantive conflict with an existing applicable obligation is different: identify what the project requires or
forbids, the observed or proposed behaviour, and its consequence. It may require correction or a user decision about
changing the obligation. Calling it a missing Requirement update must not conceal the violation.

When a Design and implementation disagree, investigate whether the behaviour violates an applicable constraint or the
Design needs updating. Disagreement alone does not identify which side is wrong. Give the evidence and recommendation;
the user makes the final decision. Technically unsound content in a reviewed plan or artifact is assessed on its own
consequence and is not excused as missing corpus coverage.
