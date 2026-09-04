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

The [validation integration reference](/specful/reference/validation-integration/) shows the canonical read-only command
sequence and examples for adopter-owned local hooks and continuous integration. Specful does not install or require
either control.

## Install agent skills (optional)

The convention and CLI work without agent skills. The optional skills add workflows for authoring, planning,
step-by-step implementation, reviewing, indexing, validation, and retrieval. They are installed once per user into the
selected agent harness and never written into an adopting repository.

Install all ten skills at user scope. When run interactively, the GitHub CLI prompts for the target agent:

```sh
gh skill install unkos-dev/specful --all --scope user
```

Specful validates the package against the Agent Skills specification. The GitHub CLI owns the supported-agent list and
scope behaviour. For non-interactive installation, add `--agent` with a value from the current
[`gh skill install` manual](https://cli.github.com/manual/gh_skill_install). Without an explicit pin, the installer uses
the latest repository release, or the default branch when no release exists. Add `--pin` with a tag or commit when an
exact revision is required.

The skills use the CLI, configuration, schemas, templates, and repository artifacts as ground truth. The
`specful-validate` skill also carries the [harness hooks](/specful/reference/validation-integration/#harness-hooks)
blocks that run the checks automatically from Claude Code or Codex and ask for a review before a push.

## Updating

Update the binary with the same command that installed it, or with the next prebuilt archive:

```sh
cargo install --locked specful
```

Update the skills by running the install command again with `--force`, which overwrites the installed copies from the
latest release and adds any skill the release introduced:

```sh
gh skill install unkos-dev/specful --all --scope user --force
```

After either update, run `specful validate` in the repository. A compatible release reports no findings. When a release
changes the profiles, its notes on the [releases](https://github.com/unkos-dev/specful/releases) page carry the
conversion steps: apply them, then run `specful index` and `specful validate`. Harness hook blocks are copied by hand,
so copy them again only when the reference changes.

## Adopting into an existing repository

Specful does not generate a specification from an undocumented codebase. Bring an existing repository under the
convention incrementally:

1. Run `specful init` to install the layout.
2. Start with the requirements and designs that matter most for the next piece of work, using `specful new requirement`
   and `specful new design`, rather than attempting to document everything at once.
3. Leave a decision record that predates the profile where it is, in its own shape: an accepted record is never edited
   into the profile. Bring one into the profile by re-recording it, either on demand when a Requirement or Design needs
   to cite it through `governed-by`, or through a deliberate review that retires the records kept only by ceremony and
   re-records the survivors. Either way, scaffold the new record with `specful new adr`, carry the original decision
   date in `decided-on` and the re-recording date in `recorded-on`, name the older record by its title and original path
   under More information, and then retire that older record under its own convention; Git holds its history at that
   path afterwards.
4. Run `specful index` to regenerate the navigation views, then `specful validate`, and commit both the source documents
   and the regenerated views together.

A partially documented repository is a valid, ongoing state: validation checks the documents that exist, and does not
require full coverage of the codebase.

## What Specful does not do

Specful is not a hosted requirements-management service, a relational or graph database, or a replacement for Git
history, issue tracking, or source code. It does not generate a complete specification from an undocumented codebase,
and it does not become a harness-specific source of truth: harness adapters may generate native skills, commands, or
context files, but those are generated integration surfaces, never divergent copies of project policy.
