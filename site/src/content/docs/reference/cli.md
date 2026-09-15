---
title: CLI reference
description: Every specful subcommand, its flags, and what it does.
---

The `specful` CLI sets up a repository, allocates artifact identifiers, generates navigation, checks document structure
and relationships, and retrieves recorded metadata. It is a single static binary with no runtime dependencies.

Choose the command for the job:

| Command | Use it to | Writes files? |
| --- | --- | --- |
| `init` | Set up the convention in a repository | Yes |
| `new` | Allocate an ID and create a draft artifact | Yes |
| `index` | Refresh navigation after artifact changes | Yes |
| `index --check` | Detect stale navigation without regenerating it | No |
| `validate` | Check corpus structure, identities and relationships | No |
| `show` | Find an artifact's path and recorded metadata by ID | No |
| `trace` | Follow recorded relationships between artifacts | No |

For pre-push and CI examples, see [Validation integration](/specful/reference/validation-integration/).

`validate` and `index` accept an optional positional root; `new`, `show` and `trace` use `--root <ROOT>`. When omitted,
these commands search from the current directory upward for the nearest `.specful/config.yaml`. `init` also accepts an
optional positional root, but defaults to the current directory without searching ancestors.

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
modify agent instruction files, install skills or hooks, or infer requirements and designs from existing code.

## `specful new`

Create an artifact from its scaffold with the next allocated identifier.

```sh
specful new <adr|requirement|design> --title <TITLE> [--scope <SCOPE>] [--root <ROOT>]
```

| Flag | Meaning |
|---|---|
| `<KIND>` | Positional. One of `adr`, `requirement`, `design`. |
| `--title <TITLE>` | Required. Artifact title; also derives the filename slug. |
| `--scope <SCOPE>` | Required for a requirement or design, for example `backend/sync`. Not for an ADR. |
| `--root <ROOT>` | Repository root; defaults to the nearest ancestor containing `.specful/config.yaml`. |

Prints the created file's path, then a reminder to complete the remaining placeholders and run `specful index`.
Allocation advances the corresponding counter in `.specful/config.yaml`. The result is a draft: its placeholders must be
replaced with project content before it can pass validation. Creating an artifact does not regenerate the views.

## `specful validate`

Check the structure, identifiers, references and generated navigation of the Specful documents. This is useful when an
edit or merge leaves a Design pointing to a deleted Requirement, two documents with the same ID, or a catalog that no
longer reflects the source documents. A hook or CI job makes those failures visible before others rely on the corpus.

```sh
specful validate [--json] [ROOT]
```

| Flag | Meaning |
|---|---|
| `--json` | Emit findings as JSON. The shape is explicitly unstable. |
| `ROOT` | Repository root; defaults to the nearest ancestor containing `.specful/config.yaml`. |

### What it checks

| Check | Example failure it catches |
| --- | --- |
| Configuration and identity | Duplicate IDs, wrong project key, allocation counters behind existing IDs |
| Artifact layout and metadata | Wrong directory or filename, unsupported profile, malformed frontmatter |
| Document structure | Missing or empty required sections, title mismatch, template placeholders |
| Requirement statement form | A Statement section without an uppercase BCP 14 keyword such as `MUST` |
| Recorded relationships | A `satisfies` target that is not an existing Requirement, or a missing `governed-by` ADR |
| ADR supersession | Missing reciprocal links, inconsistent superseded status, self-links or cycles |
| Generated navigation | Missing, stale or orphaned indexes and catalog entries |

The checks use the configuration, Markdown artifacts under `docs/specs/` and directly under `docs/adr/`, and generated
views in the selected root. Scope indexes are checked as generated views; `docs/adr/README.md` is excluded from artifact
validation. Unreadable content and symlinks in the checked trees produce findings.

This is a filesystem check of the current corpus. It includes unstaged and untracked documents in those locations and
does not select files from Git's index, a commit range or `.gitignore`. It neither edits files nor executes project
code, tests or an agent review.

### What a pass does not establish

A passing result means the checks listed above found no defects. It does not establish:

- **Substantive quality:** that a Requirement is necessary, testable or justified, or that a Design accurately describes
  the system. A non-empty Rationale section can still contain poor reasoning.
- **Implementation correctness:** that code meets a Requirement. A valid `satisfies` link records a claim; it does not
  verify that claim against code or tests.
- **Complete coverage:** that every feature is documented or every Requirement has a satisfying Design. Incremental
  adoption and Requirements without Design links can pass.
- **Historical integrity:** that the project key never changed, counters never decreased, or a deleted ID was never
  reused. Those need review of history.
- **General documentation health:** that arbitrary Markdown links, external URLs and images work, or that prose is
  factually correct. This is not a general link checker or prose linter.

Use substantive review and implementation tests for those questions. In particular, a valid corpus is not evidence that
brownfield onboarding is complete.

### Reading and resolving findings

With no findings, the command prints `valid: no findings` and exits with status 0. Otherwise it prints each finding's
path, an available line number and diagnostic, followed by a count, and exits with status 1. For example,
`satisfies target BAD-REQ-0404 does not exist` identifies a broken recorded relationship.

Repair the source of the finding: correct a mistaken target, restore an accidentally deleted document, or deliberately
revise the relationship. Do not create an obligation or remove a relationship merely to make the check pass. After an
intentional artifact change, regenerate stale views with `specful index`, inspect the diff, and validate again. A
malformed artifact can cause additional relationship and view findings, so address its parsing or metadata error first.

With `--json`, validation results are a JSON object containing `findings` and `count`. The format is unstable; scripts
should use the exit status for pass/fail. Root-discovery errors can still be plain text even with `--json`.

## `specful index`

Regenerate the navigation files kept in version control: the per-scope indexes and the machine-readable catalog.

```sh
specful index [--check] [ROOT]
```

| Flag | Meaning |
|---|---|
| `--check` | Report drift without writing. Fails if the views on disk disagree with their sources. |
| `ROOT` | Repository root; defaults to the nearest ancestor containing `.specful/config.yaml`. |

`index` writes the scope indexes under `docs/specs/` and `.specful/generated/catalog.json`, and removes obsolete
generated views. Review and commit these changes with the authored documents. It does not write the authored
Requirements, Designs or ADRs, and it refuses to overwrite an author-owned `index.md`.

`index --check` compares the views without writing. It is a focused navigation check, not a substitute for corpus
validation. **`validate` already includes generated-view checks.** Use `validate` alone for a full validation gate;
prefixing it with `index --check` repeats the freshness check when the first command passes. Successful regeneration
alone does not prove that the corpus is valid.

## `specful show`

Show the catalog record for an identifier: its path, title, kind and recorded relationships. Use this to locate a
document without guessing its scope or filename. The command reads `.specful/generated/catalog.json`; it does not read
the document body or check that the catalog is current. Validate or regenerate stale views before relying on a lookup.

```sh
specful show <ID> [--root <ROOT>]
```

| Argument | Meaning |
|---|---|
| `<ID>` | Identifier to look up, for example `PROJECT-DESIGN-0001`. |
| `--root <ROOT>` | Repository root; defaults to the nearest ancestor containing `.specful/config.yaml`. |

## `specful trace`

Trace requirement-to-design links, or the artifacts that cite an ADR. A Requirement query lists the Designs whose
`satisfies` field names it; a Design query lists its recorded Requirement targets. An ADR query lists its citing
Requirements and Designs, plus supersession links.

Like `show`, this reads the catalog without checking freshness. The output is recorded traceability, not proof of
implementation or coverage. An existing Requirement with no recorded Design link prints `(untraced)` and exits
successfully; an unknown identifier fails.

```sh
specful trace <ID> [--root <ROOT>]
```

| Argument | Meaning |
|---|---|
| `<ID>` | Identifier to trace, for example `PROJECT-REQ-0001`, `PROJECT-DESIGN-0001`, or `PROJECT-ADR-0001`. |
| `--root <ROOT>` | Repository root; defaults to the nearest ancestor containing `.specful/config.yaml`. |

Tracing an ADR lists every Requirement and Design whose `governed-by` names it, then its supersession links:

```text
$ specful trace PROJECT-ADR-0001
cited-by: PROJECT-REQ-0001 (docs/specs/system/requirements/0001-offline-replay.md)

$ specful trace PROJECT-ADR-0002
(uncited)
```
