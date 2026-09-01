---
name: specful-validate
description: >-
  Use when checking whether Specful Requirements, Designs, and ADRs pass schema and cross-reference validation, or
  before committing a change to any of them.
compatibility: Requires specful 0.3.0 or later on PATH.
---

# Validating Specful artifacts

Run `specful validate $ARGUMENTS` and report the findings; validation authorises no edit. Fix artifacts only when the
user has asked for fixes, and take a fix that changes normative content back to the user for direction rather than
choosing one to make the re-run clean. See `specful validate --help` for its options.
