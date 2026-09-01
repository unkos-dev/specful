---
type: ADR
profile-version: 1
id: "SPECFUL-ADR-0005"
title: "Deliver agent support as an opt-in harness-side plugin"
status: "proposed"
recorded-on: "2026-08-31"
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

- Opt-in harness-side plugin released by pinned commit
- Repository-side skill installation at adoption
- Harness-neutral documentation only, no packaged support
- Versioned plugin releases through tags

## Decision outcome

Chosen option: **opt-in harness-side plugin released by pinned commit**, because it delivers workflow support to agents
while keeping adopting repositories free of harness furniture and keeping the no-plugin path as the supported floor.

Delivery is two-tier. Tier 1 is the CLI alone: `specful init`, `docs/SPECFUL.md`, and the shipped templates are the
complete, minimal adoption path, and every capability must remain reachable this way. Tier 2 is a plugin installed once
per user into the agent harness, never written into adopting repositories. The package is one `plugin/` directory in the
Agent Plugins 1.0 shape, the vendor-neutral standard for packaging agent skills, and the skills are routers: each defers
to the adopting repository's own `docs/SPECFUL.md` wherever they differ and links every rule to its canonical home.
Harnesses that consume the standard install the package as it stands.

A release is a pull request pinning the plugin payload to a commit on `main` by `sha`; the resolved commit is the
version. No manifest carries a `version` field and no plugin-specific tags are created: the pinned commit is exact
provenance, `main`'s own gates validate the payload at every candidate commit, and the existing version-shaped tag
automation stays undisturbed. The repository's release tags identify the same validated payload for channels that
resolve tags.

The payload itself is harness-neutral: each skill follows the Agent Skills convention, which harnesses read directly, so
one payload serves every harness without per-harness content. What varies per harness is the distribution channel over
that unmodified payload. Claude Code installs through this repository's own marketplace, so the package carries a Claude
Code manifest and the marketplace holds one entry pinning the payload through the `git-subdir` source form. Other
harnesses install through the GitHub CLI's skill installer, which resolves the payload at a release tag by default and
accepts an exact commit pin. Whatever the channel, installed behaviour is a function of immutable references: an
installer resolves a tag or a pinned commit, never a mutable branch.

### Consequences

- Positive: adopting repositories carry no harness files, and skill fixes reach every user through one pin advance.
- Positive: the tier 1 floor keeps the convention independently usable and testable without any plugin.
- Positive: a release needs no version bookkeeping; the pin and its pull request are the complete release record.
- Negative: users must install and update the plugin per harness; nothing in an adopting repository prompts them.
- Negative: commit identifiers are opaque as version labels; readers cannot infer recency or compatibility from them.
- Negative: distribution rides channels this repository does not control; a harness or installer changing its behaviour
  changes the install path the documentation can promise.

### Confirmation

`plugin/` contains both manifests without `version` fields and the marketplace entry pins a `sha` reachable from `main`,
enforced by the repository's lint gates; `specful init` output contains no harness-specific files; each skill opens by
deferring to the adopting repository's `docs/SPECFUL.md`; no plugin-related tag exists.

## Pros and cons of the options

### Opt-in harness-side plugin released by pinned commit

- Positive: one installation serves every repository the user works in; no adopter-side coupling.
- Negative: adds a package surface and its validation gates to this repository.

### Repository-side skill installation at adoption

- Positive: support arrives with `specful init`, with nothing else to install.
- Negative: couples skill versions to every adopting repository and pollutes adopter trees with harness furniture that
  outdates independently of the convention.

### Harness-neutral documentation only, no packaged support

- Positive: nothing to package, host, or release.
- Negative: leaves agents to unaided discovery of static documentation, with no workflow support loaded at the moment of
  authoring.

### Versioned plugin releases through tags

- Positive: human-readable version labels and familiar release semantics.
- Negative: a second tag series in a repository whose release automation matches version-shaped tag globs risks
  misfiring that automation, and version metadata adds bookkeeping with no consumer.

## More information

Reconsider the no-version rule if plugin release engineering ever exists, and the reliance on external distribution
channels if a channel the documentation depends on stalls or regresses; the recorded fallback is an installer owned by
the `specful` binary resolving the same pinned payload.
