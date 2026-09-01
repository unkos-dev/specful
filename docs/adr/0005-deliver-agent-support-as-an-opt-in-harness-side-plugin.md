---
type: ADR
profile-version: 1
id: "SPECFUL-ADR-0005"
title: "Deliver agent support as an opt-in harness-side plugin"
status: "accepted"
recorded-on: "2026-08-31"
decided-on: "2026-09-01"
decision-makers:
  - "junkovich"
---

# Deliver agent support as an opt-in harness-side plugin

## Context and problem statement

Specful supports multiple agent harnesses without giving any harness ownership of canonical project knowledge. A coding
agent benefits from having the authoring workflow loaded at the moment it writes a Requirement, Design, or ADR, rather
than relying on unaided discovery of `AGENTS.md` and `docs/SPECFUL.md`. Where should that harness support live, how
should it reach users, and how should its releases be identified?

## Decision drivers

- The convention is the product: a repository must remain fully usable with no harness support installed.
- Support must reach several harnesses without forking the skill content per harness.
- Harness furniture must not pollute adopting repositories or couple skill versions to every adopter.
- Skills must route to canonical sources rather than becoming divergent policy copies.
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
workflow support to agents while keeping adopting repositories free of harness furniture, keeping the no-plugin path as
the supported floor, and giving every harness the same install story.

Delivery is two-tier. Tier 1 is the CLI alone: `specful init`, `docs/SPECFUL.md`, and the shipped templates are the
complete, minimal adoption path, and every capability must remain reachable this way. Tier 2 is a set of skills
installed once per user into the agent harness, never written into adopting repositories. The package is one `plugin/`
directory in the Agent Plugins 1.0 shape, the vendor-neutral standard for packaging agent skills, and the skills are
routers: each defers to the adopting repository's own `docs/SPECFUL.md` wherever they differ and links every rule to its
canonical home.

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

### Confirmation

`plugin/` contains one Agent Plugins 1.0 manifest without a `version` field; every skill passes the pinned Agent Skills
validator; `specful init` output contains no harness-specific files; each skill opens by deferring to the adopting
repository's `docs/SPECFUL.md`; no plugin-specific tag exists.

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
