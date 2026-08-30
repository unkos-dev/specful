# Repository configuration

Every Specful repository has one `.specful/config.yaml` at its root. The file is versioned canonical project state, not
an optional command preference.

```yaml
config-version: 1
project-key: EXAMPLE
specful-version: "0.1.0"
next-adr-sequence: 1
next-requirement-sequence: 1
next-design-sequence: 1
```

The project key contains 2 to 10 uppercase ASCII letters or digits and starts with a letter. It is immutable after the
first valid snapshot. Every stable identifier within the selected root uses that key.

The `specful-version` field records the Specful release that last initialized or migrated the repository, as a plain
`MAJOR.MINOR.PATCH` string. Tooling reads it to know the repository's vintage and never interprets it as policy.

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

## Instruction content

`init` also installs `docs/SPECFUL.md`, the full harness-neutral instructions for the convention, and a managed block in
the root `AGENTS.md` that points to it. Both are content, not configuration, but the same command installs them and the
same rerun rule applies.

`docs/SPECFUL.md` is written once, through an exclusive create. A pre-existing copy is left alone and reported as a
finding rather than overwritten.

`AGENTS.md` is upserted: a missing file is created containing only the block, delimited by `<!-- SPECFUL:START -->` and
`<!-- SPECFUL:END -->`. An existing file without those markers gets the block appended after a blank line. An existing
file whose markers are already well-formed, exactly one START then one END, gets only the content between them replaced.
Content outside the markers is never modified. Any other marker arrangement, a lone marker, reversed order, or
duplicates, is reported as a finding and nothing is written.

Rerunning `init` against an initialized repository fails fast with "repository is already initialized" before any other
check runs. Refreshing installed instruction content is not a rerun's job.
