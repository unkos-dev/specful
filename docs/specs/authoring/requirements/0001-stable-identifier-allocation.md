---
type: REQ
profile-version: 1
id: SPECFUL-REQ-0001
title: "Stable identifier allocation"
---

# Stable identifier allocation

## Statement

WHEN `specful new` creates an artifact, the CLI MUST allocate its identifier from the persistent per-kind counter in
`.specful/config.yaml`, and an identifier once allocated MUST NOT be issued again, even after the artifact that consumed
it is deleted.

## Rationale

Typed relationships (`satisfies`, `governed-by`), generated views, and Git history all refer to artifacts by identifier.
A reissued identifier silently rebinds those references to a different document, corrupting traceability without any
observable error, and an identifier derived by scanning the tree collapses back to reuse as soon as artifacts are
deleted. Counter-based allocation is what lets an identifier outlive its file.

## Acceptance criteria

- Two consecutive `specful new` invocations for the same kind allocate consecutive sequence numbers, and the
  corresponding `next-*-sequence` counter in `.specful/config.yaml` advances by one per allocation.
- Deleting an allocated artifact and creating a new one of the same kind does not reissue the deleted identifier.
- Removing every artifact of a kind does not reset numbering: the next allocation continues from the persisted counter,
  not from the tree's contents.
- An interrupted creation may leave an unused sequence number; a skipped number is valid, and the counter never lags an
  identifier that appears in an artifact (checked by inspecting `.specful/config.yaml` against the highest allocated
  identifier of each kind).
- Two concurrent `specful new` invocations never allocate the same identifier: both may succeed with distinct
  identifiers, and an invocation that encounters the allocation lock while it is held reports the collision rather than
  proceeding.

## More information

The identifier grammar and the counter ownership rule are defined in the [project charter](../../../project-charter.md).
[SPECFUL-ADR-0002](../../../adr/0002-represent-requirements-as-first-class-artifacts.md) records why identity is
allocated rather than derived from file placement.
