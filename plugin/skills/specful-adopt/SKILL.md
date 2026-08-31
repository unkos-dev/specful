---
name: specful-adopt
description: >-
  Use when a repository has no Specful specifications yet and someone asks to adopt Specful, set up specifications, or
  start tracking Requirements, Designs, and ADRs. Guides first-time `specful init` and the first artifacts.
compatibility: Requires specful 0.3.0 or later on PATH.
---

# Adopting Specful in a repository

The adopting repository's own `docs/SPECFUL.md` is authoritative once it exists. Where anything here differs from it,
follow `docs/SPECFUL.md` instead.

## Workflow

1. Confirm the repository has no `.specful/config.yaml` yet; `specful init` fails fast on an already-initialised
   repository rather than refreshing it.
2. Choose an immutable project key: 2 to 10 uppercase ASCII letters or digits, starting with a letter. Every stable
   identifier in the repository will carry this key for as long as the repository exists, so confirm it with whoever
   owns the decision before running the command.
3. Run `specful init --project-key <KEY>`. This writes `.specful/config.yaml`, scaffolds `docs/specs/` and `docs/adr/`,
   installs `docs/SPECFUL.md`, and upserts the managed block in the root `AGENTS.md`. See `specful init --help` and the
   configuration reference at <https://unkos-dev.github.io/specful/> for exactly what it creates and the rules for
   rerunning it safely.
4. Read the freshly installed `docs/SPECFUL.md`; it is now the authoritative source for this repository's authoring
   workflow and record model.
5. Load the `specful-author` skill to write the first Requirement, Design, or ADR. A reasonable first artifact is a
   Requirement for whatever capability prompted the adoption, so the repository has at least one real record to navigate
   from.
6. Run `specful index` and `specful validate`, then commit the initialised state together with the first artifact.

## Common pitfalls

- Picking a project key that collides with another key already in use across the organisation's repositories, since it
  cannot change later without re-issuing every identifier.
- Hand-writing `docs/specs/index.md` or the catalog instead of running `specful index`; these are generated views and
  are overwritten on the next run regardless.
- Skipping `docs/SPECFUL.md` and working from memory of a different repository's conventions; the record model and
  writing rules are versioned per installation.
