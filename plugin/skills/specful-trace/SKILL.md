---
name: specful-trace
description: >-
  Use when following requirement-to-design links for a specific Requirement identifier, checking whether a
  Requirement has a satisfying Design, or listing the artifacts that cite an ADR.
compatibility: Requires the specful CLI on PATH.
metadata:
  argument-hint: <ID>
---

# Tracing a Specful record

Run `specful trace $ARGUMENTS` and report the output. Given a Requirement or Design identifier this follows
requirement-to-design links; given an ADR identifier it lists the Requirements and Designs that embody the decision. See
`specful trace --help` for the exact identifier format.
