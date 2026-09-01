---
title: Adoption
description: Bring the Specful convention into a repository, greenfield or existing.
---

Adopting Specful means installing its convention (directory layout, templates, and writing model) into a repository,
then keeping requirements, designs, and decisions current as the repository changes.

## Install

Install the `specful` binary from crates.io:

```sh
cargo install --locked specful
```

Prebuilt archives for Linux (static musl), macOS, and Windows are published with each release; see the
[GitHub releases](https://github.com/unkos-dev/specful/releases) page. Each archive ships with a SHA-256 checksum.

## Initialise a repository

Run `specful init` once, from the repository root, with a project key:

```sh
specful init --project-key MYAPP
```

The project key is 2 to 10 uppercase letters or digits, starting with a letter, and prefixes every identifier `specful`
allocates for this repository (for example `MYAPP-REQ-0001`). The key is immutable once chosen: it is recorded in
`.specful/config.yaml` alongside the monotonic per-kind identifier counters, so identifiers are never reused and never
depend on scanning the tree.

`init` creates:

- `.specful/config.yaml`: canonical configuration, the root-discovery sentinel for every other command.
- `docs/specs/`: the root of the requirements and design corpus.
- `docs/adr/`: the flat directory for Architecture Decision Records.
- Empty generated navigation views for the new artifact corpus.

`init` does not create or modify agent instruction files.

## Validate

Run `specful validate` after any change to the corpus:

```sh
specful validate
```

Validation is mechanical and covers three layers: relationship integrity (identifiers resolve, `satisfies` targets
exist, supersession links agree, generated views match their sources), metadata shape (frontmatter conforms to the
artifact's JSON Schema profile), and document structure (canonical headings present, requirement records well-formed,
template placeholder text absent, and every requirement statement uses at least one uppercase BCP 14 keyword).
Diagnostics are human-readable text with a meaningful exit status; a `--json` flag emits a plain machine-readable
listing that is explicitly unstable.

Run validation in your local gate and in continuous integration. Specful does not require commit hooks.

## Install the agent plugin (optional)

The convention and CLI work without an agent plugin. An opt-in plugin adds skills for authoring, reviewing, and adopting
the convention. It is installed once per user into the agent harness and never written into an adopting repository.

In Claude Code:

```sh
claude plugin marketplace add unkos-dev/specful
claude plugin install specful@specful
```

The skills use the CLI, configuration, schemas, templates, and repository artifacts as ground truth.

## Adopting into an existing repository

Specful does not generate a specification from an undocumented codebase. Bring an existing repository under the
convention incrementally:

1. Run `specful init` to install the layout.
2. Start with the requirements and designs that matter most for the next piece of work, using `specful new requirement`
   and `specful new design`, rather than attempting to document everything at once.
3. Record any durable decision that already governs the repository as an ADR with `specful new adr`, even when its
   original context predates Specful.
4. Run `specful index` to regenerate the navigation views, then `specful validate`, and commit both the source documents
   and the regenerated views together.

A partially documented repository is a valid, ongoing state: validation checks the documents that exist, and does not
require full coverage of the codebase.

## What Specful does not do

Specful is not a hosted requirements-management service, a relational or graph database, or a replacement for Git
history, issue tracking, or source code. It does not generate a complete specification from an undocumented codebase,
and it does not become a harness-specific source of truth: harness adapters may generate native skills, commands, or
context files, but those are generated integration surfaces, never divergent copies of project policy.
