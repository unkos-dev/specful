# Repository configuration

Every Specful repository has one `.specful/config.yaml` at its root. The file is versioned canonical project state, not
an optional command preference.

```yaml
config-version: 1
project-key: EXAMPLE
next-adr-sequence: 1
next-requirement-sequence: 1
next-design-sequence: 1
```

The project key contains 2 to 10 uppercase ASCII letters or digits and starts with a letter. It is immutable after the
first valid snapshot. Every stable identifier within the selected root uses that key.

Each counter ranges from `1` through `10000`, is strictly greater than every corresponding sequence in the current
snapshot, and never decreases. The value `10000` is the exhausted sentinel and is not allocated.

## Loading and root selection

Configuration is restricted YAML without Markdown frontmatter or a body. Duplicate keys, explicit tags, anchors,
aliases, merge keys, complex keys, empty unquoted scalars, and invalid UTF-8 are rejected. Exact lowercase JSON
primitives and RFC 8259 numbers resolve to their JSON values; other non-empty scalars are strings.

Library operations receive the root explicitly. Command operations accept an explicit root or search upward from the
working directory and select the nearest ancestor containing `.specful/config.yaml`. Traversal remains within that root.

Canonical paths are relative UTF-8 paths using `/`. Absolute paths, drive prefixes, backslashes, empty segments, and `.`
or `..` segments are invalid.

## Snapshot and transition rules

A snapshot is invalid when configuration is missing or malformed, its version is unsupported, project keys are mixed, or
a counter lags an allocated identifier.

A transition is invalid when the project key changes, a counter decreases, an identifier is reused, or the after
snapshot is not independently valid. Reading, validation, and indexing do not rewrite configuration.

## Initialisation

`init` writes the configuration, artifact directories, and empty generated navigation views. It does not create or
inspect agent instruction files. Rerunning it against an initialised repository fails fast with "repository is already
initialized".
