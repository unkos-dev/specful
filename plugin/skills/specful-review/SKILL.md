---
name: specful-review
description: Review a Specful Requirement, Design, ADR, or change for consequential substantive defects.
compatibility: Requires specful 0.3.0 or later on PATH.
---

# Review a Specful artifact or change

Review is read-only. Return the report in the conversation; do not edit files, post pull-request comments, submit a
formal review, or publish a report. A correction needs separate authority.

## Resolve the review boundary

Identify the requested target before reviewing it:

- An artifact review covers one named Requirement, Design, or ADR and the minimum repository evidence needed to judge
  its claims. Do not turn it into a repository audit.
- A change review resolves an immutable commit, commit range, or pull-request head and reviews the aggregate change,
  including affected Specful artifacts together. Record that identity. A moved head invalidates a gate-grade verdict.
- A draft review may cover mutable working-tree content. Identify it as draft and do not present its verdict as
  gate-grade.
- A re-review takes the previous conversational report and the new target as explicit inputs. If the prior report is
  unavailable, perform a full review and say that correction verification and prior finding IDs are unavailable.

For a gate-grade review, attribute mechanical validation only to evidence tied to the exact target. Inspect a
provider-attested status for that target SHA, a clean checkout at that SHA, an isolated materialisation, or another
demonstrably equivalent source. Implementer reports are leads, not evidence. When such evidence is unavailable, report
validation as `NOT RUN` or `UNVERIFIED`; never substitute another working tree or claim a pass.

## Select execution

When an interactive request has a real choice between independent delegation and in-session review, resolve the target
and artifact types, then ask the user to choose:

1. Delegate one independent review to the resolved reviewer model, recommended when that identity is known.
2. Complete the review in the current session.
3. Use another execution choice, such as a named model, a different topology, or additional focus.
4. Cancel without starting review work.

Name the reviewer model only when the harness exposes it. Otherwise say `default subagent model`. Do not make price or
capability claims. Do not ask again when the user or harness has already selected the topology. Respect an explicit
reviewer selection. If independent execution is unavailable, review sequentially and disclose that it was not
independent. Optional specialists are justified only by distinct artifact or evidence domains; the coordinator owns the
verdict and does not invent findings absent from the evidence gathered.

## Gather minimum evidence

Run `specful validate` first and report mechanical findings as mechanical validation, not as substantive findings.
Continue when the target remains interpretable. Stop or narrow the review when invalid identity, structure, or
relationships make substantive conclusions unreliable.

Load only the references matching the artifact types in scope:

- [Requirement lens](references/requirement-review.md)
- [Design lens](references/design-review.md)
- [ADR lens](references/adr-review.md)

For change review, also test cross-artifact consistency: contradiction, missing companion updates, authority direction,
relationship completeness, and disagreement between the aggregate change and the smallest relevant code, tests,
configuration, or public documentation. Follow a reviewed claim only into evidence that can confirm or contradict it; do
not conduct an unrelated audit.

## Decide what is reportable

Report only a consequential finding with:

- a lens ID: `R1`, `D1`, `A1`, or `X1`, incrementing within that lens;
- exact location, applicable expectation and authority, observed evidence, realistic consequence, and smallest
  defensible correction;
- severity: `blocking`, `non-blocking`, or `suggestion`;
- confidence: `high`, `medium`, or `low`.

Low-confidence concerns cannot block and become questions or evidence requests. Suggestions do not affect the verdict.
Drop stylistic preferences, duplicates, unsupported speculation, observations outside the requested boundary, and
concerns without a realistic consequence.

## Determine the verdict

- `SHIP`: applicable mechanical validation passed and no open blocking substantive finding remains at the reviewed
  boundary.
- `CONDITIONAL`: bounded correction or decision, mechanical validation failure, or missing exact-target validation
  evidence remains. `NOT RUN` and `UNVERIFIED` validation map here.
- `NO-SHIP`: a fundamental authority, artifact-boundary, contradiction, or evidence problem prevents acceptance. This
  includes validation that cannot execute on the reviewed tree or invalid structure that prevents reliable substantive
  review.

A gate-grade review cannot return `SHIP` while required mechanical validation fails. The verdict describes readiness; it
does not grant merge authority or determine the adopter's enforcement policy.

## Re-review corrections

Resolve the new immutable target. Check each previous finding ID and label it `resolved`, `open`, or `disproved` with
evidence. Inspect the aggregate correction diff for regressions and contradictions. Add a new finding only when the
correction created it or newly supplied evidence exposed it; do not relitigate unchanged material. New findings use the
next number for their lens. Do not create a finding database, content hash, or disposition ledger.

Use the [report format](references/report-format.md) when producing the response.
