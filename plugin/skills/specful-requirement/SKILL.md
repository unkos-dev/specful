---
name: specful-requirement
description: >-
  Use when writing or updating a Specful Requirement: a new normative obligation, or a change to what the software
  must do.
compatibility: Requires specful 0.3.0 or later on PATH.
---

# Writing a Specful Requirement

A Requirement records a current normative obligation: what the software must do now, written as though it has always
been so. It is not a Design (how the system works) and not an ADR (why a decision was made); if the change is about how
or why, load `specful-design` or `specful-adr` instead. History and transitions never appear in the prose: a transition
is a plan, and what used to be true is Git history.

## Workflow

1. Start at `docs/specs/index.md` and follow the scope indexes to the module the obligation belongs to. Read the
   neighbouring Requirements and the Designs that satisfy them before writing.
2. Scaffold with `specful new requirement --title <TITLE>`. Never hand-allocate an identifier; the command owns the
   counter.
3. Complete the placeholders. The Statement section carries at least one uppercase BCP 14 keyword (MUST, MUST NOT,
   SHOULD, SHOULD NOT, MAY); cite governing ADRs through `governed-by`, naming only the ADR whose rationale this
   obligation embodies, never a related or organising decision such as the decision to adopt a convention, and omit the
   field when that record does not exist in the profile. Write obligations testably: a Requirement that no observable
   behaviour could violate is decoration, not specification. More information is optional and is removed completely when
   it adds nothing; it never restates a `governed-by` edge as a link, since `specful trace` already renders it.
4. Run `specful index`, then `specful validate`; commit the regenerated views with the change.
5. Mechanical validation does not judge substantive quality. Use `specful-review` when the adopting repository requires
   substantive review or the user asks for it.

For the full Requirement profile and field-by-field guidance, see <https://unkos-dev.github.io/specful/>.
