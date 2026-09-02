# Design review lens

Apply this lens only when a Design is in scope. The canonical rules are the public
[Design profile](https://unkos-dev.github.io/specful/profiles/design/), the repository's `templates/design.md`, and the
named Design itself.

Inspect the coherent subject and follow only material claims into the smallest relevant implementation, tests,
configuration, interfaces, or public documentation. Check whether:

- the Design describes one understandable, independently maintained subject and its present behaviour;
- its material structure, interfaces, state, runtime behaviour, failure handling, and operational or security boundary
  are accurate enough for the subject, with a reason where a canonical section does not apply;
- `satisfies` and `governed-by` relationships are exact and the document does not duplicate durable decision rationale;
- a material claim agrees with the claim-directed evidence inspected.

Do not require every canonical section to have equal depth, report a reasoned not-applicable section without evidence
that the reason is false, demand an unrelated code audit, or treat migration history in an external source as a defect
in current-state Design prose.
