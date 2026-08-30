---
type: DESIGN
profile-version: 1
id: SPECFUL-DESIGN-0001
title: "Identifier allocation and scaffolding"
satisfies:
  - SPECFUL-REQ-0001
governed-by:
  - SPECFUL-ADR-0002
---

<!-- SPDX-License-Identifier: CC0-1.0 -->

# Identifier allocation and scaffolding

`specful new` turns an artifact kind, a title, and (for requirements and designs) an architectural scope into a
scaffolded Markdown file carrying a freshly allocated stable identifier. The subject covers the allocation counters, the
locking that keeps allocation exclusive, and the template instantiation that produces the scaffold; it serves anyone
changing the `new` command, the configuration counters, or the shipped templates.

## Purpose and boundaries

The subsystem owns the only write path for identifiers: every `PROJECT-KIND-0001`-style identity enters the repository
through it. It begins at the parsed `specful new` invocation and ends with a scaffold file on disk and the advanced
counter committed to `.specful/config.yaml`. Validating the resulting document belongs to the validation subsystem;
regenerating navigation belongs to the index subsystem; both consume what this subsystem creates. `specful init`, which
writes the initial configuration and directory skeleton, shares the module and the atomic-write helpers but allocates
nothing.

## Structure

Three parts collaborate inside `src/authoring.rs`:

- the configuration counters (`next-adr-sequence`, `next-requirement-sequence`, `next-design-sequence`) persisted in
  `.specful/config.yaml` and loaded through the configuration module;
- a `ConfigLock` guarding the configuration file's read-modify-rename cycle;
- the scaffold sources: `templates/adr.md`, `templates/requirement.md`, and `templates/design.md`, embedded into the
  binary at compile time as the only copies of artifact shape, so the CLI and the public templates cannot drift.

## Interfaces and dependencies

Inbound: `specful new adr | requirement | design --title <title> [--scope <path>]`. ADRs take no scope (the ADR
directory is flat); requirements and designs require one, validated as lowercase kebab-case path segments. Outbound: the
created file at `docs/adr/0001-slug.md` or `docs/specs/<scope>/requirements|design/0001-slug.md`, and the rewritten
`.specful/config.yaml`. The slug derives from the title; a title yielding an empty slug is refused. The module depends
only on the standard library, the configuration module, and the repository path constants.

## Data and state

The counters are the subsystem's whole persistent state: monotonic per-kind integers that only ever advance. Identifiers
are formatted from the project key, the kind segment, and the zero-padded sequence at allocation time and are never
parsed back out of the tree, so deleting artifacts cannot influence future allocation. The lock file
`.specful/config.yaml.lock` exists only for the duration of an allocation.

## Runtime behaviour

An invocation validates the scope and slug, acquires the lock, loads the configuration, allocates the next sequence for
the kind, and renders the identifier. The template for the kind is instantiated by line-prefix substitution: exactly the
identifier, the title, and (for ADRs) the recorded-on date are filled; every other placeholder, including optional
relationship fields, is left for the author, and validation reports them until the document is completed. The advanced
counters are committed by atomic rename before the artifact file is created, and the file is created with exclusive
semantics so an existing path is refused rather than overwritten.

## Failure and recovery

The ordering of the two writes is the recovery model: a run interrupted between the counter commit and the file creation
leaves an allocation gap, which is permitted, never a counter that lags an allocated identifier, which would be invalid.
A failed write of the scaffold removes the partial file rather than stranding an invalid artifact under an
already-advanced counter. A second allocation racing the first observes the lock file's exclusive creation fail and
reports the collision instead of corrupting the counters; the lock is released on every exit path once acquired. A hard
kill mid-write can still leave a partial file, accepted as the same class as a skipped identifier and reported by
validation.

## Security and operations

Not applicable beyond ordinary repository hygiene: the subsystem reads and writes only files inside the repository root,
holds no credentials, and touches no network. The lock file is the one operational artefact worth knowing: a crash can
strand `.specful/config.yaml.lock`, and the collision finding names it so an operator can remove a stale lock after
confirming no allocation is running.

## More information

The [project charter](../../../project-charter.md) defines the identifier grammar and the source-of-truth boundary this
subsystem serves. The obligation it satisfies is
[SPECFUL-REQ-0001](../requirements/0001-stable-identifier-allocation.md).
