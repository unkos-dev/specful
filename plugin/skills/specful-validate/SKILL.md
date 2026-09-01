---
name: specful-validate
description: >-
  Use when checking whether Specful Requirements, Designs, and ADRs pass schema and cross-reference validation, or
  before committing a change to any of them.
compatibility: Requires specful 0.3.0 or later on PATH.
---

# Validating Specful artifacts

The adopting repository's own `docs/SPECFUL.md` is authoritative. Where anything here differs from it, follow
`docs/SPECFUL.md` instead.

Run `specful validate $ARGUMENTS` and report the findings. If it reports defects, fix the offending artifacts and re-run
until the command reports none. See `specful validate --help` for its options.
