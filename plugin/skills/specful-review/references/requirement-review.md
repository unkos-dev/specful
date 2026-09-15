# Requirement review lens

Apply this lens only when a Requirement is in scope. The canonical rules are the public
[Requirement profile](https://unkos-dev.github.io/specful/profiles/requirement/), the repository's
`templates/requirement.md`, and the named Requirement itself.

Inspect the coherent obligation, its linked governing ADRs where relevant, and only the evidence needed to test a
material claim. Check whether:

- the Statement is a current, normative, observable obligation with a defined scope and no unbounded qualifier;
- one document contains one coherent obligation rather than independently verifiable duties joined together;
- acceptance criteria can each fail with a yes-or-no result and cover the material boundary rather than restating the
  Statement;
- not-applicable sections state a reason when the profile permits them, and that reason fits the artifact;
- the Requirement keeps implementation design, transition history, and durable decision rationale in their proper
  artifacts;
- `governed-by` relationships are exact, each cited ADR being the one whose rationale the obligation embodies, and the
  Requirement does not duplicate that rationale;
- the obligation has a reason to exist under the profile: something genuinely binds observable behaviour, or an
  externally imposed contract applies, as the ADR profile's Requirement versus ADR section states;
- a maintainer with no prior context can detect a violation from the Statement and acceptance criteria, so a record that
  states the obvious or omits what that detection needs fails its purpose;
- the obligation does not restate one another Requirement owns and is not split across several records;
- the file sits under the scope that owns the obligation.

Report design leakage only when it constrains how the system must work without an externally imposed contract. Do not
report a stated not-applicable reason merely because it is brief, infer missing implementation detail where the
obligation does not need it, or turn a preference for prose style into a finding; a prose preference without a
purpose-fit consequence remains unreportable.
