# ADR review lens

Apply this lens only when an ADR is in scope. The canonical rules are the public
[ADR profile](https://unkos-dev.github.io/specful/profiles/adr/), the repository's `templates/adr.md`, and the named ADR
itself.

Inspect the durable decision record and the smallest linked artifacts needed to judge it. Check whether:

- the context and decision drivers make the durable choice and its forces clear;
- considered options are materially viable, distinct, and evaluated against the drivers;
- the outcome names a choice and credible reason, while consequences include material trade-offs;
- More information, where present, does not restate a `satisfies`, `governed-by`, `supersedes`, or `superseded-by`
relationship the frontmatter already carries;
- lifecycle state, supersession, and relationships are appropriate, and the ADR does not bind the system or become a
mutable description of how it currently works;
- the decision has a reason to exist under the profile: its considered options were materially viable and its
  consequences are material, per the profile's Considered options and Consequences sections;
- a maintainer with no prior context can understand from the record why the losing options lost, so an ADR that states
  the obvious or omits that reasoning fails its purpose;
- the decision is not one another ADR already records, and one decision is not split across records.

Do not require a fixed number of options, reject an option solely because it lost, demand a Requirement for an internal
choice that creates no observable obligation, or report prose preferences without a consequence; a prose preference
without a purpose-fit consequence remains unreportable.
