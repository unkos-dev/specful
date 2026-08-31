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
- Harness furniture must not pollute adopting repositories or couple skill versions to every adopter.
- Skills must route to canonical sources rather than becoming divergent policy copies.
- Releases need exact provenance without inventing release engineering ahead of need.
- The repository's existing tag automation matches version-shaped tag names, so a second tag series carries risk.

## Considered options

- Opt-in harness-side plugin from a self-hosted pinned marketplace
- Repository-side skill installation at adoption
- Harness-neutral documentation only, no packaged support
- Versioned plugin releases through tags

## Decision outcome

Chosen option: **opt-in harness-side plugin from a self-hosted pinned marketplace**, because it delivers workflow
support to agents while keeping adopting repositories free of harness furniture and keeping the no-plugin path as the
supported floor.

Delivery is two-tier. Tier 1 is the CLI alone: `specful init`, `docs/SPECFUL.md`, and the shipped templates are the
complete, minimal adoption path, and every capability must remain reachable this way. Tier 2 is a plugin installed once
per user into the agent harness, never written into adopting repositories. One `plugin/` directory carries both an Agent
Plugins 1.0 manifest and a Claude Code manifest over the same skill payload, and the skills are routers: each defers to
the adopting repository's own `docs/SPECFUL.md` wherever they differ and links every rule to its canonical home.

The Specful repository is its own Claude Code marketplace. The marketplace entry pins the plugin payload to a commit on
`main` by `sha` through the `git-subdir` source form; a release is a pull request advancing the pin, and the resolved
commit is the version. Neither manifest carries a `version` field and no tags are created: the pinned commit is exact
provenance, `main`'s own gates validate the payload at every candidate commit, and the existing version-shaped tag
automation stays undisturbed.

### Consequences

- Positive: adopting repositories carry no harness files, and skill fixes reach every user through one pin advance.
- Positive: the tier 1 floor keeps the convention independently usable and testable without any plugin.
- Positive: a release needs no version bookkeeping; the pin and its pull request are the complete release record.
- Negative: users must install and update the plugin per harness; nothing in an adopting repository prompts them.
- Negative: commit identifiers are opaque as version labels; readers cannot infer recency or compatibility from them.
- Negative: only Claude Code is a served harness until another harness's installation channel is exercised.

### Confirmation

`plugin/` contains both manifests without `version` fields and the marketplace entry pins a `sha` reachable from `main`,
enforced by the repository's lint gates; `specful init` output contains no harness-specific files; each skill opens by
deferring to the adopting repository's `docs/SPECFUL.md`; no plugin-related tag exists.

## Pros and cons of the options

### Opt-in harness-side plugin from a self-hosted pinned marketplace

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

Reconsider the single-harness scope when a second harness's installation channel is exercised, and the no-version rule
if real plugin release engineering ever exists. The still-deferred repository commit gate is a separate decision; plugin
hooks are adopter-facing product capability, not a gate on this repository.
