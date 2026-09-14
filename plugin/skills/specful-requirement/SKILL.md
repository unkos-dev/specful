---
name: specful-requirement
description: >-
  Use when evaluating, selecting, writing, or updating a Specful Requirement: a candidate normative obligation, a new
  obligation, or a change to what the software must do. Do not use for general product discovery.
compatibility: Requires the specful CLI on PATH.
---

# Writing a Specful Requirement

A Requirement records a current normative obligation: what the software must do now, written as though it has always
been so. It is not a Design (how the system works) and not an ADR (why a decision was made); if the change is about how
or why, load `specful-design` or `specful-adr` instead. History and transitions never appear in the prose: a transition
is a plan, and what used to be true is Git history.

## Workflow

1. Before evaluating or selecting a candidate obligation, read the full
   [Requirement profile](https://unkos-dev.github.io/specful/profiles/requirement/), including Rationale and Acceptance
   criteria. Confirm the real need and authority for the obligation and whether its acceptance criteria can observe a
   violation. Existing behaviour alone does not establish what the software must do. Apply approved local selection
   rules as supplements; surface a conflict with the profile rather than silently replacing its definitions. If the
   evidence cannot settle product intent, return that decision to the human. Zero selected Requirements is valid.
2. Start at `docs/specs/index.md` and follow the scope indexes to the module the obligation belongs to. Read the
   neighbouring Requirements and the Designs that satisfy them before writing.
3. Scaffold with `specful new requirement --title <TITLE> --scope <SCOPE>`. Never hand-allocate an identifier; the
   command owns the counter.
4. Complete the placeholders. The Statement section carries at least one uppercase BCP 14 keyword (MUST, MUST NOT,
   SHOULD, SHOULD NOT, MAY); cite governing ADRs through `governed-by`, naming only the ADR whose rationale this
   obligation embodies, never a related or organising decision such as the decision to adopt a convention, and omit the
   field when that record does not exist in the profile. Write obligations testably: a Requirement that no observable
   behaviour could violate is decoration, not specification. More information is optional and is removed completely when
   it adds nothing; it never restates a `governed-by` edge as a link, since `specful show` already renders it.
5. Run `specful index`, then `specful validate`; commit the regenerated views with the change.
6. Mechanical validation does not judge substantive quality. Use `specful-review` when the adopting repository requires
   substantive review or the user asks for it.

For field-by-field guidance, use the Requirement profile linked in step 1.
