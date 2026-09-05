---
type: ADR
profile-version: 1
id: "SPECFUL-ADR-0005"
title: "Deliver agent support as opt-in harness-side skills"
status: "accepted"
recorded-on: "2026-08-31"
decided-on: "2026-09-01"
decision-makers:
  - "junkovich"
---

# Deliver agent support as opt-in harness-side skills

## Context and problem statement

Specful supports multiple agent harnesses without giving any harness ownership of canonical project knowledge. A coding
agent benefits from having the relevant craft loaded at the moment it writes a Requirement, Design, or ADR. Where should
that harness support live, how should it reach users, and how should its releases be identified?

## Decision drivers

- The convention is the product: a repository must remain fully usable with no harness support installed.
- Support must reach several harnesses without forking the skill content per harness.
- Harness furniture must not pollute adopting repositories or couple skill versions to every adopter.
- Skills must not become a harness-specific source of canonical project knowledge.
- Releases need exact provenance without inventing release engineering ahead of need.
- The repository's existing tag automation matches version-shaped tag names, so a second tag series carries risk.

## Considered options

- Opt-in harness-side skills installed at immutable revisions through one channel
- Per-harness distribution channels over the same payload
- Repository-side skill installation at adoption
- Harness-neutral documentation only, no packaged support
- A dedicated plugin version and tag series

## Decision outcome

Chosen option: **opt-in harness-side skills installed at immutable revisions through one channel**, because it delivers
workflow support to agents while keeping adopting repositories free of harness furniture, keeping the no-skill path as
the supported floor, and giving every harness the same install story.

The convention and CLI remain usable without installed skills. Harness-side skills add artifact craft and structured
substantive review without becoming canonical project knowledge. They are installed once per user into the harness and
never written into adopting repositories. Adopting maintainers decide whether skill-based review is advisory or
required.

The payload is harness-neutral: each skill follows the Agent Skills convention, which harnesses read directly, so one
payload serves every harness without per-harness content or per-harness channels. Installation for every harness is the
GitHub CLI's skill installer. Installed behaviour is a function of immutable references: an install resolves a release
tag or a pinned commit, never a mutable branch.

A skill release is the repository's release tag: the payload at the tagged commit is the released payload, validated by
`main`'s own gates at every candidate commit. No manifest carries a `version` field and no plugin-specific tags are
created; the existing version-shaped tag automation stays undisturbed.

### Consequences

- Positive: adopting repositories carry no harness files, and skill fixes reach every user at the next release tag.
- Positive: the tier 1 floor keeps the convention independently usable and testable without any installed skills.
- Positive: one install and update story serves every harness; a release needs no skill-specific version bookkeeping.
- Negative: users must install and update the skills per harness; nothing in an adopting repository prompts them.
- Negative: distribution rides a channel this repository does not control; the installer changing its behaviour changes
  the install path the documentation can promise.
- Negative: capabilities a harness exposes only through its own packaging, such as hooks, are not deliverable through
  this channel and need their own harness-neutral delivery decision when they arrive.

## Pros and cons of the options

### Opt-in harness-side skills installed at immutable revisions through one channel

- Positive: one installation serves every repository the user works in; no adopter-side coupling.
- Positive: one documented command and one release event cover every harness the installer supports.
- Negative: adds a package surface and its validation gates to this repository.

### Per-harness distribution channels over the same payload

- Positive: each harness's richest native channel is available, such as a harness's own plugin marketplace.
- Negative: every additional channel is a second install path, a second update story, and a second set of release
  machinery to document and gate, bought for capabilities the payload does not yet use.

### Repository-side skill installation at adoption

- Positive: support arrives with `specful init`, with nothing else to install.
- Negative: couples skill versions to every adopting repository and pollutes adopter trees with harness furniture that
  outdates independently of the convention.

### Harness-neutral documentation only, no packaged support

- Positive: nothing to package, host, or release.
- Negative: leaves agents to unaided discovery of static documentation, with no workflow support loaded at the moment of
  authoring.

### A dedicated plugin version and tag series

- Positive: human-readable version labels and familiar release semantics.
- Negative: a second tag series in a repository whose release automation matches version-shaped tag globs risks
  misfiring that automation, and version metadata adds bookkeeping with no consumer.

## More information

Reconsider the no-version rule if skill release engineering ever exists, and the single-channel rule when a capability
arrives that the channel cannot carry: hooks are a known requirement and must be delivered harness-neutrally, so any
packaging added for them is judged against every harness, not one. Reconsider the channel itself if it stalls or
regresses; the fallback is an installer owned by this repository over the same immutable references.
