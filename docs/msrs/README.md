# MSRS profile v1

An MSRS module defines current software obligations for one architectural
scope. A module need not describe the whole project. Place modules at
`docs/specs/<scope...>/msrs/NNNN-short-title.md`.

Frontmatter requires `type: MSRS`, `profile-version: 1`, a stable
`PROJECT-MSRS-NNNN` identifier, a title, and a non-empty `requirements`
mapping. The filename number matches the module sequence. Lowercase `x-`
extension fields are permitted; other unprefixed fields are not.

An optional `governed-by` array lists the identifiers of the ADRs that govern
this module. Entries are unique ADR identifiers, and the relationship is
stored on the specification side.

Each requirement uses a repository-wide `PROJECT-REQ-NNNN` identifier. Its
frontmatter entry is an object that may be empty, may carry lowercase `x-`
extension fields, and may record optional `sources` listing where the
obligation comes from. Exact source objects are unique and array order has no
meaning. Source objects are closed variants:

- `artifact` requires `artifact-id`;
- `path` requires a canonical repository-relative `path` and may add an exact
  RFC 3986 fragment without `#`;
- `uri` requires an absolute non-`file` URI without authority userinfo and may
  add a human-readable `locator`;
- `citation` requires a description.

Artifact and path targets resolve within the selected repository. A requirement
cannot cite its containing module or file as its own source.

## Markdown body

The body has one level-one heading exactly matching the frontmatter title and
one level-two `Requirements` section. Every mapping key has exactly one
level-three `### ID: title` block, and every such block has one mapping entry.

Each block contains one normative paragraph with at least one uppercase BCP 14
term. `#### Rationale` is required for `SHOULD` and `SHOULD NOT`.
`#### Verification` is optional guidance.

Atomicity, verifiability, appropriate rationale, architectural placement, and
accidental history are review concerns. They do not add schema fields or
change deterministic profile conformance.

Moving a module or requirement preserves its stable identifiers. Deleting an
obligation also updates every inbound typed relationship in the same logical
change. Git retains prior states; current modules contain no status or obsolete
history.
