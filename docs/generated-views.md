# Generated navigation views

`specful index` derives two kinds of view from the committed artifacts.
Both are disposable: they carry no canonical knowledge, and validation
fails when a committed view disagrees with the documents it derives from.
`specful index --check` reports that drift without writing.

Generation requires every artifact to load and pass schema validation. If
collection reports a defect, `specful index` reports it and leaves all
generated views unchanged. A generated view whose source artifacts have all
gone is orphaned; `specful index` deletes it and validation reports it.

## Per-scope indexes

Every architectural scope under `docs/specs/`, including the root, gets an
`index.md` listing its child scopes and its requirements and design
modules by identifier and title. An agent can navigate from
`docs/specs/index.md` to any module by reading, one link at a time.
Scope directory segments use lowercase ASCII kebab-case so their names are
safe as generated Markdown labels, headings, and link destinations.

Generated indexes open with a marker comment on their first line. An
`index.md` without the marker is author-owned: generation refuses to
overwrite it, and validation reports it, until it is removed explicitly.

## Machine catalog

`.specful/generated/catalog.json` records every artifact and its stored
relationships: identifier, kind, path, title, and, where present, status,
`supersedes`, `superseded-by`, `satisfies`, `governed-by`, and declared
requirement identifiers. Artifacts sort by identifier and object keys sort
lexically, so identical repositories produce identical bytes.

The catalog's shape is explicitly unstable. It exists for `specful` lookup
and trace queries and for agents that prefer a single file over walking
the tree; nothing outside this repository should depend on its structure
staying fixed.
