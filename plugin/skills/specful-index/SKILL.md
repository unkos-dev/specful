---
name: specful-index
description: >-
  Use when the navigation indexes or catalog under `docs/specs/` are missing or out of date after adding or editing a
  Requirement, Design, or ADR, or when checking whether they are.
compatibility: Requires specful 0.3.0 or later on PATH.
---

# Regenerating Specful's navigation views

The adopting repository's own `docs/SPECFUL.md` is authoritative. Where anything here differs from it, follow
`docs/SPECFUL.md` instead.

Run `specful index $ARGUMENTS` and report the output. These views are generated and disposable: never hand-edit them.
Use `--check` to report drift without writing, then re-run without it to fix. See `specful index --help` for its
options.
