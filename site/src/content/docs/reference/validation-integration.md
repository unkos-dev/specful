---
title: Validation integration
description: Run Specful's mechanical checks from adopter-owned local and continuous-integration gates.
---

Specful provides two read-only checks for an existing repository gate:

```sh
specful index --check
specful validate
```

Run them in that order. `specful index --check` confirms that Specful can derive the navigation views from the authored
artifacts and that the committed views match, without changing any files. `specful validate` checks relationship
integrity, metadata shape, document structure, and generated-view consistency across the repository. Both commands
return a non-zero exit status when they report findings.

When an intentional artifact change makes the views stale, run `specful index` to regenerate them, inspect the result,
and commit the authored files and generated views together. A check should report drift rather than repair it.

## Local hooks

If the repository already uses a hook manager or local Git hook, its check can invoke the canonical sequence:

```sh
#!/bin/sh
set -eu

specful index --check
specful validate
```

Local hooks provide early feedback, but they run in each contributor's checkout and can be bypassed. Specful does not
install a hook or require a hook manager.

## Harness hooks

An agent harness can run the same two commands after every edit and when the agent tries to stop, and return the
findings to the agent instead of waiting for a commit. A third hook asks the agent to run the `specful-review` skill
before a `git push` that carries changes under `docs/specs`, `docs/adr`, or `.specful`. All three call the `specful`
binary already on PATH, so nothing is installed; the adopting repository owns the config.

For Claude Code the block lives in the project's `.claude/settings.json`; for Codex it lives in `.codex/hooks.json`,
differing only in the edit-tool matcher. The `specful-validate` skill ships both blocks in its
[harness hooks reference](https://github.com/unkos-dev/specful/blob/main/plugin/skills/specful-validate/references/harness-hooks.md),
and this repository commits both files as a working example.

Harness hooks are advisory feedback for the agent in the session. They do not replace a repository gate.

## Continuous integration

Add the same commands to an existing CI job after that job installs the repository's chosen Specful version. For
example, an existing GitHub Actions job can include this step:

```yaml
- name: Check Specful corpus
  run: |
    specful index --check
    specful validate
```

The adopting repository owns the workflow, the Specful version it installs, and whether a failed check blocks changes.
Running `specful init` creates none of these controls.
