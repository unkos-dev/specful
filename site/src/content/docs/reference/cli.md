---
title: CLI reference
description: Every specful subcommand, its flags, and what it does.
---

The `specful` CLI is a single static binary with no runtime dependencies. It mechanises three tasks the convention
cannot deliver by hand at acceptable cost: allocating stable identifiers, regenerating navigation views, and validating
the repository. It also scaffolds adoption and answers retrieval queries.

Every command except `init` accepts an optional repository root argument; when omitted, it resolves to the nearest
ancestor directory containing `.specful/config.yaml`.

## `specful init`

Initialise a Specful repository: configuration and directories.

```sh
specful init --project-key <KEY> [ROOT]
```

| Flag | Meaning |
|---|---|
| `--project-key <KEY>` | Required. 2 to 10 uppercase letters or digits, starting with a letter. Immutable once set. |
| `ROOT` | Repository root; defaults to the current directory. Never resolved by upward search. |

Prints each path it creates and exits successfully, or prints findings and exits with failure. It does not create or
modify agent instruction files.

## `specful new`

Create an artifact from its scaffold with the next allocated identifier.

```sh
specful new <adr|requirement|design> --title <TITLE> [--scope <SCOPE>] [ROOT]
```

| Flag | Meaning |
|---|---|
| `<KIND>` | Positional. One of `adr`, `requirement`, `design`. |
| `--title <TITLE>` | Required. Artifact title; also derives the filename slug. |
| `--scope <SCOPE>` | Architectural scope for a requirement or design, for example `backend/sync`. Not for an ADR. |
| `ROOT` | Repository root; defaults to the nearest ancestor containing `.specful/config.yaml`. |

Prints the created file's path, then a reminder to complete the remaining placeholders and run `specful index`.

## `specful validate`

Validate the repository against the Specful profiles.

```sh
specful validate [--json] [ROOT]
```

| Flag | Meaning |
|---|---|
| `--json` | Emit findings as JSON. The shape is explicitly unstable. |
| `ROOT` | Repository root; defaults to the nearest ancestor containing `.specful/config.yaml`. |

Prints one line per finding in human-readable text, followed by a summary line, and exits with failure if any finding
was reported. With `--json`, prints a single JSON object carrying the findings array and a count.

## `specful index`

Regenerate the committed navigation views: the per-scope indexes and the machine-readable catalog.

```sh
specful index [--check] [ROOT]
```

| Flag | Meaning |
|---|---|
| `--check` | Report drift without writing anything. Fails if the committed views disagree with their sources. |
| `ROOT` | Repository root; defaults to the nearest ancestor containing `.specful/config.yaml`. |

## `specful show`

Show the catalog record for an identifier.

```sh
specful show <ID> [ROOT]
```

| Argument | Meaning |
|---|---|
| `<ID>` | Identifier to look up, for example `PROJECT-DESIGN-0001`. |
| `ROOT` | Repository root; defaults to the nearest ancestor containing `.specful/config.yaml`. |

## `specful trace`

Trace requirement-to-design links for an identifier.

```sh
specful trace <ID> [ROOT]
```

| Argument | Meaning |
|---|---|
| `<ID>` | Identifier to trace, for example `PROJECT-REQ-0001`. |
| `ROOT` | Repository root; defaults to the nearest ancestor containing `.specful/config.yaml`. |
