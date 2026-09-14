# Brownfield onboarding across five changes

A public onboarding sequence in Reverie shows why brownfield work needs investigation and correction before artifact
counts. The evidence here comes from merged diffs, commits and review exchanges. Pull request descriptions are treated
as author reports unless the change or review record corroborates them.

[#1082](https://github.com/unkos-dev/reverie/pull/1082) began by separating coherent subjects within a broad security
area, including authentication boundaries, database row-level security context and content security policy. Review
prompted repeated corrections to claims and sources. The useful unit was the behaviour that maintainers needed to
understand together, rather than the original documentation category.

[#1097](https://github.com/unkos-dev/reverie/pull/1097) recorded current conditional-request behaviour without turning
every observed response into an obligation. Weak, wildcard and list-form `If-Match` values were described as producing
the current 400 or 422 responses. The result was Design coverage without a Requirement for those responses.

[#1098](https://github.com/unkos-dev/reverie/pull/1098) retained the decision to use a synchroniser token while
recognising that older implementation narrative no longer matched current code. The author supplied implementation
evidence in [response to a review finding](https://github.com/unkos-dev/reverie/pull/1098#discussion_r3998192751), and
the reviewer [withdrew the finding](https://github.com/unkos-dev/reverie/pull/1098#discussion_r3998193906).

[#1099](https://github.com/unkos-dev/reverie/pull/1099) removed a drifting schema guide. Column facts remained in
`backend/schema.sql`, while prose needed for understanding moved into bounded Designs. Review also narrowed an
acceptance case that had claimed more than the evidence supported.

[#1101](https://github.com/unkos-dev/reverie/pull/1101) corrected references, citations and acceptance criteria. This
showed why each increment must check its claims and links against the repository state it will join.

Across the sequence, passing format, schema and link checks established that the corpus was mechanically coherent. The
corrections came from comparing claims with code, tests, source documentation and human decisions. Those checks did not
prove subject boundaries, factual accuracy or the authority for a Requirement.
