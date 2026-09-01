---
name: specful-new
description: >-
  Use when scaffolding a new Specful Requirement, Design, or ADR file with its next allocated identifier.
compatibility: Requires specful 0.3.0 or later on PATH.
metadata:
  argument-hint: <kind> --title <TITLE> [--scope <SCOPE>]
---

# Scaffolding a Specful artifact

The adopting repository's own `docs/SPECFUL.md` is authoritative. Where anything here differs from it, follow
`docs/SPECFUL.md` instead.

Run `specful new $ARGUMENTS` and report the output. Never hand-allocate an identifier: the command owns the counter in
`.specful/config.yaml`. See `specful new --help` for the artifact kinds and flags.
