---
name: specful-onboard
description: >-
  Use when introducing Specful into an existing repository, selecting coherent Design subjects for a brownfield
  baseline, or resuming an adopter's existing Specful onboarding record. Do not use for generic documentation,
  ordinary artifact authoring, implementation, or installation-only requests.
compatibility: Requires the specful CLI on PATH before CLI-backed authoring.
---

# Onboard an existing repository

Lead the investigation and drafting while the human directs content and scope and supplies intent that the repository
cannot establish. Work in bounded increments: one coherent subject by default, or a deliberately broad baseline when the
user asks for one. A partially documented repository and an increment with no Requirements are both valid outcomes.

This skill coordinates discovery, evidence and authority. It does not replace `specful-design`, `specful-requirement`,
`specful-adr`, `specful-index`, `specful-validate`, or `specful-review`. Load those skills when their owned work
applies. A saved plan and `specful-implement` are optional and only apply when the work independently needs them.

## Start or resume

Infer the available context before asking questions. Read repository instructions and governing project material, then
inspect `.specful`, `docs/specs/`, `docs/adr/`, generated indexes, existing artifacts, relevant documentation, code,
tests, gates, and any adopter-owned planning or adoption record.

If `.specful/config.yaml` exists, resume the existing adoption. Reconcile the record with the repository before relying
on it. Distinguish human rulings from agent proposals, delivered artifacts, and decisions later superseded. Never turn
silence, a draft, a count, or an agent proposal into approval. Record new rulings and evidence revisions in the
adopter's existing record rather than creating a second progress format.

If Specful is absent, inventory the reserved `docs/specs/` and `docs/adr/` roots before proposing `specful init`. Follow
the [Adoption guidance](https://unkos-dev.github.io/specful/adoption/#adopting-into-an-existing-repository) for
relocating legacy material: preserve original bytes and Git provenance, and do not rewrite an accepted legacy decision
into a Specful ADR. Confirm a compatible CLI before any CLI-backed authoring. Do not install a binary or initialise the
repository unless the user has authorised it.

Ask only for information the repository cannot supply: the desired outcome or boundary, missing content or scope
direction, the authority behind a proposed obligation, or the existing record location when persistence is needed and no
convention can be found. Preserve approval already given. Return to the human only when the evidence exposes a real
scope change or unresolved product intent.

## Discover the next increment

Map candidate Design subjects from components that change together, share an interface or state boundary, or must be
understood together to change safely. Give each candidate a clear purpose and boundary. Prefer the smallest subject that
remains coherent; group subjects into a baseline only when that breadth is deliberate.

For each candidate, inspect the closest authoritative sources and classify what they show:

- verified current behaviour, including the revision and sources that support it;
- a false or unsupported claim in a draft;
- documentation that has become stale or duplicates a stronger source;
- an implementation defect against an approved obligation;
- an unresolved product decision that evidence cannot settle;
- durable rationale that may warrant an ADR.

Documentation may be retained, corrected, replaced, moved, or deleted. State the proposed disposition and the source
that will remain authoritative. Do not turn every existing document into a Specful artifact.

Before selecting any candidate Requirement, load `specful-requirement` in full and apply the Requirement profile's tests
for rationale and observable acceptance criteria. Existing behaviour is evidence of what the system does; it is not by
itself authority for what the system must do. Apply adopter-approved local selection rules as supplements. If a local
ruling conflicts with the profile, surface the conflict instead of silently changing the normative definition.

## Brief, author and verify

Before scaffolding, give the human a compact increment brief covering:

- the subject and its boundary;
- proposed Design, Requirement and ADR artifacts, including when a kind is absent;
- the need and authority for each proposed obligation;
- any governing decision embodied by the subject;
- documentation dispositions;
- evidence revision and sources;
- verification planned for each current-state claim;
- product intent or scope that remains unresolved.

For an authorised increment, proceed without seeking approval again. Use the type-specific skills to scaffold and draft
the selected artifacts, then use `specful-index` and `specful-validate`. Use `specful-review` when the adopter's policy
requires it or the user asks for it. Mechanical checks establish only their stated schema, reference and package
properties; they do not prove that the prose is accurate or that an obligation is justified.

Adjudicate review findings against the cited evidence and authority. Correct false claims, stale references and other
supported defects. Return unresolved intent to the human. After a meaning-changing correction, verify the corrected
meaning and its dependent claims together rather than checking only the edited sentence. When verification exposes an
exploitable gap, keep it out of the public artifact: report it to the adopter's tracker and let the fix land before the
Design records the corrected behaviour.

When code, tests, interfaces or governing artifacts change during onboarding, identify which recorded claims depend on
the changed source. Reverify those claims and their dependants at the new revision while retaining evidence that remains
valid for unaffected subjects. Update the adopter's existing record with the new revision and disposition.

## Optional integrations

Treat the CLI prerequisite, installed skills, harness hooks and repository gates as distinct layers. Inspect and
preserve existing configuration before offering any opt-in integration work as a separate scope. The adopter owns
whether hooks or gates are advisory or blocking.

When integration is authorised, distinguish configured from demonstrated. Exercise feedback in the actual harness with
disposable material and without a remote push. A displayed review reminder proves only that the reminder appeared; it
does not invoke or guarantee a substantive review. Repository gates likewise prove only the checks they run.

For a short public example of a brownfield sequence and its correction cycles, see
[Brownfield onboarding across five changes](references/reverie-onboarding-example.md).
