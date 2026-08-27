---
kind: adr
profile-version: 1
id: SPECFUL-ADR-0001
title: "Consolidate specful state under .specful"
status: accepted
recorded-on: 2026-08-27
decision-makers:
  - "junkovich"
---

# Consolidate specful state under .specful

## Context and Problem Statement

Specful adds files to a host repository it does not own, so adopters reasonably want a minimal root footprint.
Configuration sat at `.specful.yaml` while generated state sat under `.specful/generated/`, giving specful two root
entries where one would suffice.

## Decision Drivers

- Minimise the number of root-level entries specful adds to an adopting repository.
- Keep the split between canonical and generated content legible without adding a second root entry to express it.

## Considered Options

- Keep `.specful.yaml` at the root alongside the existing `.specful/generated/` directory.
- Move canonical configuration into `.specful/config.yaml`, so the whole footprint sits inside one directory.

## Decision Outcome

Chosen option: **move canonical configuration into `.specful/config.yaml`**, because it consolidates specful's footprint
into the single `.specful/` directory. Canonical configuration lives at `.specful/config.yaml`; generated, disposable
views live under `.specful/generated/`. The `.specful/` directory is the root-discovery sentinel. The split between
canonical and generated content is expressed inside the directory, following the precedent of committed configuration
inside dot-directories such as `.changeset/config.json` and `.cargo/config.toml`.

### Consequences

- Positive: a single hidden directory is specful's whole root footprint, besides `docs/` content and the `AGENTS.md`
  block.
- Negative: dot-directories pattern-match to tool caches, so an adopter may gitignore `.specful/` wholesale, which would
  exclude the canonical configuration and its identifier counters. The failure only surfaces on a fresh clone. Recovery
  is bounded: counters can be reconstructed by scanning existing artifacts for the highest allocated identifier, with
  the narrow caveat that identifiers of deleted artifacts could be reused. This is accepted without tooling guards and
  recorded here instead.

### Confirmation

Root discovery, `init`, and identifier allocation all resolve `.specful/config.yaml`, and `specful validate` passes
against a repository initialised under this layout.
