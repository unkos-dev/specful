# Review execution

All modes use the [shared review standard](../SKILL.md). Execution changes who reviews the target, not the criteria,
finding threshold or verdict. Honour the user's selected mode and reviewer choice; otherwise use `In-session`.

## Modes and isolation

In-session review is performed by the current assistant. Independent review uses one separate reviewer. The coordinator
may clarify unsupported claims and organise its report, but does not automatically conduct another full review.

In either delegated mode, the initial reviewer receives no coordinator assessment, suspected-defect list or preferred
outcome. Preserve concerns the user explicitly supplied. Isolation includes inherited conversation history, not only the
prompt text. Use the harness's isolated-context mechanism; if it cannot provide this, report that independent execution
is unavailable.

Multi-agent review uses two separate reviewers, both covering the whole agreed scope. Give them identical target,
authority, shared-standard and user-focus inputs. Neither receives the other's conclusions during the initial review.

Do not replace a requested mode silently, claim sequential work was independent, or add further reviewer fan-out. Use
the same reviewers and retain their context for reconciliation. Later rounds share evidence and are collaborative, not
independent assessments.

## Delegation brief

Supply concrete target and source locations when filling this brief. Point to the shared standard rather than
paraphrasing it into another rubric. Repeat the operational restrictions below in every delegated round.

```text
Review target: <resolved head, range, document/version or local scope, with its source location>.
User scope and focus: <the user's actual instructions, including any explicit concerns>.
Applicable authority: <repository instructions and relevant artifact locations>.
Standard: read <path to specful-review/SKILL.md> and its relevant references; apply that shared standard.
Round: <initial review, or focused reconciliation with the shared unresolved questions and evidence>.

Operate read-only. Inspect source, documents and existing results. Do not edit, fix, commit, publish or change external
state. Do not launch development gates, test suites, builds or mechanical validation. Do not delegate further.
If additional execution appears necessary, report the concrete question and why existing evidence cannot resolve it
to the coordinator. Do not execute it yourself.

Return supported findings and your verdict under the shared standard, or explain consequential incompleteness.
There is no finding quota or preferred verdict. Explain any consequential uncertainty or disagreement with evidence.
During reconciliation, address only the shared unresolved matters. Revise or withdraw claims when evidence warrants it.
```

If execution is genuinely needed, the coordinator applies the shared standard's focused-investigation rule once and
shares the evidence. Reviewer requests do not authorise independent check runs or a routine validation phase.

## Multi-agent reconciliation

Only the first round is routine. Finish whenever evidence supports a coherent outcome. An additional round must be
necessary to achieve that outcome; do not use one for reassurance, more findings or apparent unanimity. If another round
cannot resolve a matter because it needs a user decision or unavailable evidence, report that matter and stop.

The maximum is three rounds total, including the first. It is fixed, not an invocation parameter or an allowance to
spend. A round is a reviewer assessment against the initial brief or a shared follow-up. Evidence retrieval within an
assessment does not create a round; additional reviewer assessments cannot evade the cap by being called clarifications.
The ceiling applies to the review as a whole, not separately to each finding.

1. Collect both initial independent assessments. Check support, reconcile duplicates and identify any precise
   disagreement. If their evidence already supports the findings and verdict, deliver the report. Different wording or
   one reviewer identifying a supported defect does not itself require another round.
2. Only if needed, send both reviewers the same unresolved claims, supporting and contrary evidence, and questions whose
   answers would settle validity, consequence, recommendation or verdict. Ask them to reconsider those matters, without
   repeating the whole review. Retain their existing context and repeat the read-only and no-check restrictions.
3. Only if still needed, resolve consequential matters that remain using further evidence where it can help. After this
   assessment, deliver the outcome within the cap, including any consequential disagreement that survives it.

The coordinator actively drives convergence through evidence and reconsideration. Agreement is not proof, a lone
objection is not automatically true, and votes do not determine validity. Do not force reviewers to agree or conceal
supported contrary evidence to produce one verdict.

Remove settled matters from active discussion, retaining valid findings for the final report. Rejected claims are not
defects. Reopen a settled claim only when new evidence materially changes it. Consider a newly exposed material defect
within scope without restarting the review or expanding into unrelated subjects.

## Synthesis and recovery

Deliver one report using the [report guidance](report-format.md), not concatenated reviewer reports. Deduplicate claims
and assign stable final finding IDs. Explain consequential unresolved disagreement, the competing evidence and its
consequence for the recommendation. Disagreement alone does not lower the verdict; apply the shared standard's rules for
supported defects and credible unresolved concerns. Leave decisions about changing project obligations to the user.

If a reviewer becomes unavailable or loses context, preserve its useful findings and evidence. A replacement is an
exception, does not reset the round cap, and is not an original independent opinion if it receives prior conclusions.
Disclose when the requested mode could not be completed instead of presenting a different mode as equivalent.

On interruption, return useful partial results and explain consequential incompleteness. Do not discard the evidence or
automatically restart because the user challenges the duration. Reconciliation never includes fixes. A later change to
the target can be re-reviewed when requested; reaching the cap does not authorise a new review or another round.
