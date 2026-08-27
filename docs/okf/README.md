# Open Knowledge Format bundle

Specful uses `docs/specs/` as an Open Knowledge Format v0.2 bundle. Every Markdown file below that directory is
classified by basename before any Specful profile is selected.

## Native OKF boundary

Exact lowercase `index.md` and `log.md` are reserved at every level. Every other Markdown file is a concept and requires
YAML frontmatter containing a non-empty `type`.

Indexes are optional sectioned Markdown link lists. They have no frontmatter, except that the bundle-root index may
declare only `okf_version: "0.2"`. Specful accepts that declaration but neither emits nor requires it. Such an index is
author-owned and must be removed explicitly before Specful-generated root navigation can take its place.

Logs are optional and have no frontmatter. A log has one level-one title and a flat newest-first sequence of level-two
`YYYY-MM-DD` date groups containing prose list entries.

Native OKF permits missing indexes, unknown concept types, unknown concept fields, and broken Markdown links. These
conditions do not become native OKF errors merely because Specful applies a stricter profile later.

Malformed or missing concept frontmatter, an empty `type`, frontmatter on a nested index or log, an unsupported root
version declaration, and malformed index or log structure are native OKF failures.

## Specful specialization

Specful v0.1 recognizes `type: MSRS` and `type: MSDD`. These concepts must also satisfy their profile schema, Markdown,
path, and repository contracts. A native OKF concept with another non-empty type remains a generic concept and is not a
Specful MSRS or MSDD artifact.

Ordinary Markdown links retain navigation semantics only. Typed Specful relationships are stored in profile metadata and
use stable Specful identifiers. Moving a concept therefore changes its path-based OKF identifier without changing its
Specful identity.

Public validation has one Specful conformance result. A conformant result implies that the inherited OKF checks passed
before the stricter profile and repository checks. A non-conformant result does not classify the same input under OKF
alone.
