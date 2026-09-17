# Review report format

Apply the [shared standard](../SKILL.md) and return one report in the conversation.

## Outcome first

Lead with the verdict, inspected target and a short assessment explaining what holds the work, if anything. Identify the
actual head, range, file/version or local scope without implying that mutable targets cannot ship. Explain a material
scope or execution limitation where it affects the conclusion.

For several findings, use a compact summary table when it helps the reader scan ID, severity, location and finding. Mark
advisories plainly. A short report can use finding headings directly; do not repeat a one-line advisory in both a table
and a separate body. A clean review needs only its target, `SHIP` and a concise explanation.

## Findings

Assign `F1`, `F2` and subsequent IDs to new findings. Keep final IDs stable during reconciliation and a requested
re-review; retain older IDs when following an existing report. Order findings by consequence.

Include the specific location, evidence, practical consequence and correct recommendation, with applicable authority or
standards cited inline. State whether correction or a user decision is needed before proceeding. Scale the explanation
with severity:

- A critical or high finding normally needs the causal path and evidence establishing its consequence, plus the sound
  recommendation.
- A medium finding may fit in a paragraph.
- A useful low advisory can be one sentence with its location, recommendation and practical benefit.

Explain material uncertainty in prose; use neither numeric confidence nor a replacement confidence scale. An unsupported
hypothesis is not a finding. Useful best-practice or corpus advice can accompany `SHIP`.

Omit empty review-pass sections, generic praise, validation inventories, irrelevant checks not performed, acceptance
matrices, separate standards or corpus-effect fields, and compulsory proposal sections. Mention a strength when it helps
explain the assessment. Do not add machine metadata, a persistent ledger or reviewer transcripts. Expandable HTML is not
required.

## Re-review and disagreement

For a re-review, give each prior finding's `resolved`, `open` or `disproved` status with current evidence before any
newly exposed findings. Do not repeat the original explanation unless it remains necessary.

Synthesis produces one recommendation. Explain consequential unresolved disagreement through the competing evidence and
its consequence under the shared verdict rules. Do not manufacture unanimity or report rejected claims as defects. For
an incomplete review, preserve useful results and identify the consequential limitation without inventing a verdict.

## Examples

These are fictional format examples, not findings about this repository.

### Correction and advisory

> **CONDITIONAL: export plan, current draft**
>
> The approach is sound, but F1 must be resolved before implementation. F2 is advisory.
>
> **F1: High, Publication sequence**
>
> The plan replaces the live manifest before uploading the files it names. Readers can receive references to unavailable
> files during that interval, and a failed upload can leave that state indefinitely. Upload and verify the referenced
> files first, then publish the manifest through an atomic switch that preserves the previous complete version on
> failure.
>
> **F2: Low, advisory, Export Design**
>
> Document the missing retained-version policy in the Export Design so maintainers can distinguish intentional retention
> from abandoned files.

### Clean review

> **SHIP: export plan, revised draft**
>
> The solution is ready to implement. Publication preserves a complete readable version throughout replacement and
> failure, and the responsibilities are sufficiently specified. No material findings.
