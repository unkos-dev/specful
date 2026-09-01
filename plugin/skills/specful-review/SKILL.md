---
name: specful-review
description: >-
  Use when reviewing a Specful Requirement, Design, or ADR, or a pull request that changes one, for substance rather
  than mechanics: acceptance criteria quality, the not-applicable discipline, and the Requirement-versus-ADR boundary.
compatibility: Requires specful 0.3.0 or later on PATH.
---

# Reviewing a Specful artifact

The adopting repository's own `docs/SPECFUL.md` is authoritative. Where anything here differs from it, follow
`docs/SPECFUL.md` instead.

## Before a substantive review

Run `specful validate` first. It is necessary and catches schema and cross-reference defects, but it says nothing about
whether the content is any good. This skill covers the judgement calls validation cannot make.

## Checklist

- **Acceptance criteria quality.** A Requirement's acceptance criteria should be specific and checkable, not a
  restatement of the Statement in weaker words. Reject a criterion that cannot fail.
- **The reasoned not-applicable discipline.** A section marked not applicable needs a stated reason, not a blank or a
  placeholder left over from the scaffold. See the artifact profile on the documentation site for which sections permit
  this and what a sufficient reason looks like.
- **The Requirement-versus-ADR boundary.** A Requirement states a normative obligation; an ADR records why a durable
  decision was made and what alternatives were rejected. A change that only explains reasoning belongs in an ADR, not
  folded into a Requirement's Statement. When a boundary call is genuinely unclear, route it to the artifact profile and
  record model on the documentation site rather than guessing.
- **Current-state writing.** Requirements and Designs describe the system as it stands, not its history or an in-flight
  transition. Language like "now supports" or "was changed to" belongs in git history or a plan, not in the artifact.
- **Links.** A Design's `satisfies` list and any `governed-by` citations should point at the Requirements and ADRs the
  change actually depends on, no more and no fewer.

Route every rule above to its canonical home rather than re-deriving it: the repository's `docs/SPECFUL.md`, the scope
indexes under `docs/specs/`, `specful --help`, and the full record model at
<https://unkos-dev.github.io/specful/>.
