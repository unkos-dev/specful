---
name: specful-index
description: >-
  Use when the navigation indexes or catalog under `docs/specs/` are missing or out of date after adding or editing a
  Requirement, Design, or ADR, or when checking whether they are.
compatibility: Requires the specful CLI on PATH.
---

# Regenerating Specful's navigation views

When asked whether the views are current, run `specful index --check` and report the drift; checking authorises no
write. Run `specful index` to regenerate only when the user asked for an update or approves one, then confirm with
`--check`. These views are generated and disposable: never hand-edit them. See `specful index --help` for its options.
