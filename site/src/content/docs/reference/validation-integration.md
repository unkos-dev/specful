---
title: Validation integration
description: Add Specful validation to Lefthook pre-push and CI, with optional agent review reminders.
---

Add `specful validate` to pre-push and CI to catch broken references, duplicate identifiers, incomplete documents and
stale navigation before they reach other contributors. The examples below use Lefthook for the local check and GitHub
Actions for CI. Repositories using other tools can run the same command.

## What validation protects

`specful validate` checks the repository's Specful documents and configuration:

| Check | Why it matters |
| --- | --- |
| IDs are unique and match the project key and filenames | Unambiguous document identity |
| Configuration is valid; counters exceed existing IDs | Avoid allocating an ID already in use |
| Directories, filenames and frontmatter follow the profiles | Artifacts can be recognised and read |
| Titles agree; required sections have content; placeholders are gone | Catch incomplete or malformed documents |
| Requirement Statements contain an uppercase keyword such as `MUST` | Explicit obligation wording |
| `satisfies` and `governed-by` targets exist with the correct type | No dangling Requirement or ADR references |
| ADR replacement links are reciprocal, match status and have no cycles | A consistent chain of replacement decisions |
| Generated indexes and catalog match authored documents | Current navigation and ID lookups |

For example, deleting a Requirement while leaving a Design's `satisfies` reference fails validation. So does merging two
documents with the same ID, or changing a document title without regenerating its index entry. These are reasons to run
the check automatically: an otherwise reasonable edit can leave another part of the corpus inconsistent.

A pass does **not** prove that an obligation is worthwhile, that the Design is accurate, or that the implementation
meets either document. The validator checks that a Rationale section has content, not whether its reasoning is sound. It
does not require full documentation coverage, run implementation tests, or check ordinary Markdown links and images.
Those need substantive review or other tools. The [CLI reference](/specful/reference/cli/#specful-validate) covers input
scope, diagnostics, historical limits and recovery in more detail.

## Which command belongs in a gate?

Use `specful validate` for pre-push and CI. It includes the generated-view check as well as the other corpus checks.

| Command | What it checks | Writes files? |
| --- | --- | --- |
| `specful index --check` | Generated indexes and catalog match authored documents | No |
| `specful validate` | View freshness, configuration, artifact structure, IDs and relationships | No |
| `specful index` | Regenerates navigation; does not establish corpus validity | Yes |

Running `specful index --check && specful validate` repeats the freshness check when the first command passes. Use
`index --check` on its own when diagnosing navigation drift; a full validation gate only needs `validate`.

Both check commands are read-only and return a non-zero exit status on failure. Use `index` to regenerate navigation
after an intentional artifact change, then commit the generated views with the source documents.

## Lefthook pre-push

Prerequisites: a repository [initialised with Specful](/specful/adoption/#initialise-a-repository), the Specful CLI, and
[Lefthook](https://lefthook.dev/installation/). Use your repository's tool manager to provide both executables on Git's
`PATH`, and use the same Specful release locally and in CI. Specful does not install Git hooks automatically.

Add this job to the repository's `lefthook.yml`:

```yaml
pre-push:
  jobs:
    - name: specful
      run: specful validate
```

If `pre-push` already exists, add the job to that section rather than replacing it or creating a second YAML key. For an
existing `commands` configuration, add a `specful` command with the same `run` value. If a repository check command
already runs validation, use that command as `run` and reuse it in CI.

The job checks the whole corpus from the repository root before each push. A failure blocks the push and prints the
findings. This includes references affected by deleted or moved documents.

After adding the configuration, install the Git hook in the checkout:

```sh
lefthook install pre-push
```

If the repository already installs hooks as part of setup, add pre-push to that process. See Lefthook's
[installation reference](https://lefthook.dev/usage/commands/install/) for integration with an existing hook setup.

To run just this job manually:

```sh
lefthook run pre-push --job specful
```

The local check reads the working tree, including uncommitted documents and edits. CI checks the committed result and
provides enforcement when local hooks are bypassed.

## When validation fails

Read the reported file path and diagnostic. Repair malformed documents or broken references at their source. If the
finding says a generated view is stale, regenerate and check it:

```sh
specful index
specful validate
```

Review and commit the changes before pushing again. CI should report stale views rather than regenerate them: otherwise
it would check a repaired copy instead of the files in the commit. See
[Reading and resolving findings](/specful/reference/cli/#reading-and-resolving-findings) for more detail.

## Continuous integration

Add the same check to an existing CI job after checking out the revision under review and installing the chosen Specful
release. For GitHub Actions, the step is:

```yaml
- name: Check Specful corpus
  run: specful validate
```

Make the CI job a required status check in your branch protection or ruleset if invalid documents should block merging.

## Harness hooks

Agent hooks can request substantive review before a push. Automatic validation runs through Git pre-push and CI, so it
does not interrupt every edit or agent turn.

An optional pre-push reminder can ask the agent to run `specful-review` when outgoing commits change Specful artifacts.
The skill's
[harness hooks reference](https://github.com/unkos-dev/specful/blob/main/plugin/skills/specful-validate/references/harness-hooks.md)
contains copyable Claude Code and Codex examples. The reminder requests a substantive review; it neither runs the CLI
checks nor enforces a review verdict. Lefthook performs mechanical validation when Git runs the push.
