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
