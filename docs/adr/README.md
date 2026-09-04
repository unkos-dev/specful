# Architecture Decision Records

Specful Architecture Decision Records use a project profile derived from
[MADR 4.0.0](https://github.com/adr/madr/tree/4.0.0/template).

An ADR explains why a durable architectural choice was made. It does not define current requirements, duplicate the
current design description, or serve as an implementation diary.

## Relationship to MADR

The profile is derived from the MADR 4.0.0 complete template and is tighter than MADR on every axis it touches, so a
canonical MADR record does not validate unchanged. Section headings are sentence case and compared exactly, so
`Context and Problem Statement` fails where `Context and problem statement` passes. Decision drivers, Consequences, and
Confirmation are required where MADR marks them optional. `recorded-on` and `decided-on` replace MADR's single `date`
field. `decision-makers`, `consulted`, and `informed` are lists rather than free text. `status` has no `rejected` value.
No frontmatter key outside the profile is accepted. A MADR record enters the profile by being re-recorded rather than
converted in place; see
[adopting into an existing repository](https://unkos-dev.github.io/specful/adoption/#adopting-into-an-existing-repository).

## Authoring template

New ADRs start from [`templates/adr.md`](../../templates/adr.md). The guided template is based on MADR's complete
explanatory template rather than a bare or minimal variant. This retains the semantic guidance needed by authors and
coding agents without requiring that guidance to remain in completed records.

Every completed ADR contains:

- an identifier, title, status, recorded date, and decision-maker metadata;
- Context and problem statement;
- Decision drivers;
- Considered options;
- Decision outcome;
- Consequences;
- Confirmation.

Pros and cons of the options and More information are conditional sections. They are included only when they add
material decision evidence.

## Identifiers

ADR identifiers use the form `<PROJECT>-ADR-NNNN`, such as `SPF-ADR-0001`.

- `PROJECT` is the adopting repository's immutable project key.
- `ADR` identifies the artifact type.
- `NNNN` is a repository-wide sequence from `0001` through `9999`, allocated by Specful.

The project key contains 2 to 10 uppercase ASCII letters or digits, starts with a letter, and matches
`^[A-Z][A-Z0-9]{1,9}$`. It is selected when Specful is initialized and remains immutable.

The identifier defines document identity and does not encode architectural scope, subject, status, or file location.
Moving, renaming, or superseding an ADR does not change its identifier.

`0000` is invalid. Identifiers are never reused, and sequence gaps are valid. Validation rejects duplicates. When
concurrent branches allocate the same unpublished identifier, one record is reallocated before integration.

Allocation fails with a clear exhaustion error when no value remains. Specful does not silently widen the sequence or
change the identifier grammar. Supporting a wider sequence requires a later profile version.

The adopting repository stores `next-adr-sequence` in `.specful/config.yaml` as the next value available for allocation.
It starts at `1`, advances atomically with each durable allocation, and never decreases when an ADR is deleted. The
value must be greater than every ADR sequence present in the current snapshot. Transition validation rejects a decrease
and requires each newly allocated sequence to consume the prior next value. Allocating several ADRs in one transition
consumes consecutive values and advances the high-water mark by the same count.

`next-adr-sequence` may range from `1` through `10000`; `10000` is an exhausted sentinel and is never allocated to an
ADR. Brownfield initialization sets the value to one greater than the highest imported sequence, or to `1` when no ADR
exists. A completely discarded uncommitted experiment is not a durable allocation.

## Titles

Every ADR stores its title in the required frontmatter `title` field and repeats it as the document's single level-one
Markdown heading. The two representations form one logical value. The frontmatter string must exactly match the heading
text after the `#` marker and the space following it are removed.

A mismatch is invalid, and neither representation takes precedence. This parity preserves deterministic metadata access
while keeping each ADR useful as a standalone Markdown document.

## Filenames

ADR records are stored directly under `docs/adr/`. Architectural scope and status do not create subdirectories. Metadata
and generated indexes provide filtered views without making classification part of the canonical path.

ADR filenames use lowercase `NNNN-short-title.md`, such as `0001-use-postgresql.md`.

The four-digit filename number must match the sequence in the ADR identifier. The remaining filename is an
author-selected ASCII kebab-case navigation slug. The project key and `ADR` artifact prefix are not repeated because the
repository and `docs/adr/` path already provide that context.

The slug contains 1 to 64 characters and matches `^[a-z0-9]+(?:-[a-z0-9]+)*$`. It may begin with a letter or digit.
Leading, trailing, and repeated hyphens are invalid. The length limit applies to the slug alone, excluding the sequence,
separator, and `.md` suffix.

The identifier remains authoritative. A filename helps navigation and search but does not define document identity.
Validation checks the slug's syntax, not whether it summarizes or can be derived from the title.

Proposed ADRs may be renamed freely. Accepted, deprecated, or superseded ADRs may be renamed only to correct or
materially clarify the navigation slug. The title and slug are independent; changing one does not require changing the
other. The ADR identifier and four-digit filename number remain unchanged.

A post-acceptance slug rename must update the filename, every repository-internal reference, and each generated index in
the same logical operation. Validation rejects broken references and number-to-identifier mismatches. Renaming is
navigation maintenance and must not alter the substantive decision. External links may still break and must be
considered before a rename.

## Status

An ADR has exactly one of four statuses:

- `proposed`: under consideration and not authoritative;
- `accepted`: approved and currently authoritative;
- `deprecated`: no longer applicable and has no direct replacement;
- `superseded`: replaced by another ADR.

Only an accepted ADR may be treated as architectural authority. Specful does not define `draft` or `rejected` statuses.
Incomplete or abandoned proposals may be deleted. When rejecting an option is itself a durable architectural choice,
that choice is recorded as the outcome of an accepted ADR.

Status transitions move forward:

```text
proposed -> accepted -> deprecated
                     -> superseded
```

An ADR cannot transition directly from `proposed` to a terminal status. `deprecated` and `superseded` are terminal. If a
terminal decision becomes appropriate again, a new ADR records the new context and decision rather than reactivating the
old record.

## Dates

`recorded-on` is required and records the date the ADR entered the repository. It is immutable.

`decided-on` records the date the decision became authoritative. It is omitted while an ADR is `proposed` and when the
acceptance date of a brownfield decision is unknown. Accepted and terminal ADRs include it when that date is known. Once
present, it is immutable.

Both fields use the RFC 3339 full-date form `YYYY-MM-DD`. Deprecation, supersession, corrections, and renames do not
alter either date. Git history records those later events.

The metadata schema applies both an exact lexical pattern and the JSON Schema `date` format. Specful validation
independently verifies that each value is a real RFC 3339 calendar date and does not rely on an external validator to
assert `format`.

The fields have no relative ordering constraint. A repository-native proposal may be recorded before it is decided,
while a historical decision may be recorded later. Equal dates are also valid.

## Participants

`decision-makers` is required and identifies the people or roles with authority over the decision.

`consulted` and `informed` provide the optional MADR/RACI participant roles. `consulted` identifies those whose opinions
were sought through two-way communication. `informed` identifies those kept up to date through one-way communication. An
adopting project may omit either field when that governance record adds no value.

Every participant entry is a human-readable string naming a person or collective role. Participant objects and
repository-wide participant identifiers are not part of the ADR profile.

Entries must be unique within each participant field using exact string equality. The same participant may appear in
more than one field when the participant serves multiple roles. Specful does not case-fold or apply fuzzy identity
matching.

Optional participant fields are omitted rather than stored as empty arrays.

## Classification

ADR metadata does not include architectural scopes or free-form tags. Scope is derived from explicit relationships with
requirements and design artifacts. Generated indexes may use those relationships to provide scope-oriented views without
maintaining a separate classification taxonomy.

## Text encoding

ADR files use UTF-8. Core human-readable metadata, comprising `title` and all participant values, must use Unicode
Normalization Form C. The level-one heading inherits the title requirement through exact parity.

Validation rejects non-NFC core values rather than rewriting them. Markdown body content is not globally normalized, and
extension values are preserved without normalization. Identifiers, schema keys, and filenames retain their separate
ASCII constraints.

Titles and participant values are single-line strings with no leading or trailing Unicode whitespace and no C0 or C1
control characters. Internal spacing and punctuation are preserved exactly. Validation does not trim, fold, or otherwise
rewrite these values.

## Frontmatter format

Frontmatter uses the JSON-compatible subset of YAML 1.2. Its top-level value is a mapping with string keys, and every
loaded value must belong to the JSON data model: null, boolean, number, string, array, or object.

Custom tags, anchors, aliases, merge keys, and complex mapping keys are invalid. Dates are strings governed by their
field schema rather than YAML-specific timestamp values. These restrictions apply to core and extension fields.

Required core fields are present and meaningful. Optional core fields are omitted when unavailable or inapplicable. A
present core field cannot be null, an empty or whitespace-only string, or an empty array. Extension fields may use any
permitted JSON-compatible value, including null.

Mapping keys must be unique. Specful rejects duplicates during YAML loading, before schema validation, and never applies
a first-value-wins or last-value-wins rule.

### Canonical field order

Mapping order has no semantic effect and validation is order-independent. Templates and formatting use this canonical
order:

1. `type`;
2. `profile-version`;
3. `id`;
4. `title`;
5. `status`;
6. `recorded-on`;
7. `decided-on`, when present;
8. `decision-makers`, `consulted`, and `informed`, when present;
9. `supersedes` and `superseded-by`, when present;
10. extension fields, sorted by key.

Canonical ordering keeps documents and diffs predictable without rejecting a semantically valid hand-edited mapping.

### Array order

Array position carries no seniority, priority, dependency, or other semantic meaning. Participant arrays preserve author
order as presentation. Relationship arrays use ascending identifier order in canonical formatting. Validation treats
reordered arrays as semantically equivalent.

## Schema and validation

The canonical ADR metadata schema uses JSON Schema Draft 2020-12 and declares
`https://json-schema.org/draft/2020-12/schema` as its meta-schema.

The repository copy is stored at `schemas/adr/v1.schema.json` and has the canonical identifier
`https://unkos-dev.github.io/specful/schemas/adr/v1.schema.json`. Local validation does not require network access. Once
published, that identifier must never be reassigned to a different validation contract.

Language-neutral positive and negative cases are stored in `schemas/adr/v1.cases.json`. Each case contains a loaded
JSON-compatible frontmatter instance and its expected schema validity. These cases cover only the JSON Schema boundary.
YAML loading, Unicode normalization, Markdown, filenames, repository relationships, configuration, and transitions
remain separate validator responsibilities.

Every ADR contains the required discriminator `type: ADR` and integer `profile-version: 1`. These fields identify the
artifact and its complete Specful conformance profile without relying on its repository path. The path, filename, and
identifier remain consistency checks.

Each profile version identifies one exact conformance contract. Before Specful reaches 1.0, an accepted decision may
still change what conforms to the current profile version in place, without a version bump; the release notes for the
change carry the manual repository edits it requires. From 1.0, a change that alters whether a snapshot or transition is
valid creates the next integer version instead, editorial corrections and equivalent implementation refactors do not,
and existing documents retain their declared profile version until explicitly migrated. Extension usage does not change
the profile version.

Reading, indexing, and validation never migrate documents. Supported older profile versions are validated against their
declared schemas and semantic rules, and an unknown newer profile version is an error rather than a fallback target.
Only an explicit `specful migrate` operation may transform documents to another profile version. Migration updates
affected documents, references, and generated artifacts as one logical operation for review in Git.

JSON Schema validates the loaded frontmatter value. Specful document validation enforces invariants that cross into
Markdown content, filenames, other artifacts, or repository state, including title parity and reciprocal supersession.

Specful separates current-state validation from transition validation. Current-state validation evaluates one repository
snapshot and does not inspect Git history. It verifies that each document and the complete document set are internally
valid.

Transition validation compares a valid before snapshot with a valid after snapshot. It enforces lifecycle rules that
cannot be inferred from either snapshot alone, including forward-only status changes, immutable fields, identifier
non-reuse, and the establishment or preservation of supersession relationships. Specful authoring commands apply
transition validation to their proposed changes. Continuous integration may apply the same validation to a base and
proposed repository snapshot, including changes made without Specful authoring commands.

After an ADR first reaches `accepted`, transition validation emits a non-blocking review warning when its Markdown body
or non-lifecycle metadata changes. Expected lifecycle metadata changes comprise forward status changes, supersession
relationships governed by the status matrix, and the one-time addition of a previously unknown `decided-on` date. The
warning does not make the transition invalid. An adopting project may promote it into blocking local policy.

Transition validation does not reconstruct or require complete Git history. Brownfield adoption establishes the first
validated snapshot as its baseline. Historical auditing, if introduced, remains a separate operation and does not change
current-state validity.

## Extensions

The Specful metadata vocabulary is closed. Unknown unprefixed fields are invalid, which prevents misspelled core fields
from being silently ignored.

Adopting projects may add extension fields whose names begin with `x-` and use lowercase ASCII kebab-case, such as
`x-acme-approval-board`. Specful preserves extension values but assigns them no core semantics.

## Supersession

Supersession is stored in both directions:

- the replacement ADR stores a `supersedes` relationship to the old ADR;
- the old ADR stores a `superseded-by` relationship to the replacement ADR.

The two fields represent one bidirectional relationship rather than independent claims. Every relationship must have an
exact reciprocal entry. A mismatch is invalid, and neither side takes precedence.

A supersession relationship is established only while the replacement has status `accepted`. The replaced ADR changes to
`superseded`, and both records change in the same logical operation. Proposed replacements do not supersede accepted
decisions. A replacement may itself become `superseded` later without invalidating an existing relationship.

Reciprocal storage is a deliberate exception to Specful's general rule that one artifact owns each relationship. It
keeps both ADRs independently navigable without requiring a generated index or running tool.

A replacement ADR may supersede one or more old ADRs. Each superseded ADR has exactly one replacement. When an old
decision is decomposed, one coordinating replacement ADR explains the decomposition and links to any narrower decisions.
This preserves a deterministic successor for every superseded record.

Each relationship identifies the direct replacement and is never rewired to a later descendant. Tools resolve the
current endpoint by following the `superseded-by` chain until it reaches an ADR that is not superseded.

The supersession graph must be acyclic. Direct self-references and multi-record cycles are invalid. Repository
validation reports the complete cycle path because JSON Schema cannot enforce graph-wide invariants.

Established outgoing `supersedes` relationships remain when an ADR later becomes deprecated or superseded. Relationship
fields follow this status matrix:

| Status | `supersedes` | `superseded-by` |
|---|---|---|
| `proposed` | forbidden | forbidden |
| `accepted` | optional | forbidden |
| `deprecated` | optional if previously established | forbidden |
| `superseded` | optional if previously established | required |

A chain may therefore end at a deprecated ADR. That valid state means no authoritative successor currently exists.

Both fields are non-empty arrays of unique ADR identifiers:

```yaml
supersedes:
  - SPF-ADR-0004
  - SPF-ADR-0007

superseded-by:
  - SPF-ADR-0012
```

`supersedes` contains one or more entries. `superseded-by` contains exactly one. A field is omitted when it does not
apply; empty arrays are invalid. Both fields may appear when an ADR replaces earlier decisions and is later superseded
itself.

## Completion and review

### Deterministic conformance

A structurally complete ADR:

- contains every required section in the defined order;
- contains non-whitespace content in every required section;
- contains no unresolved Specful template placeholder;
- contains none of the instructional comments supplied by the template;
- omits an optional section rather than retaining it empty.

Specful validation enforces only these objective conditions. It recognizes its own template residue explicitly and does
not attempt to classify arbitrary prose as filler or judge its engineering quality.

### Advisory review

Before an ADR is accepted, its human or agent reviewer considers whether:

- option names refer consistently to the same choices throughout;
- consequences identify material adverse trade-offs as well as benefits;
- confirmation describes observable evidence;
- each retained optional section adds useful decision evidence.

These checks are shared authoring guidance, not conformance conditions. An adopting project may promote any of them into
local review policy without changing the Specful profile.

## Lifecycle

The substantive content of an accepted ADR is expected to remain stable. Whether a content edit is substantive requires
engineering judgement, so this expectation is a review rule rather than a conformance condition. A later decision that
replaces an accepted ADR is recorded in a new ADR, and the relationship between the records is expressed through their
metadata.

Git records editorial history. Requirements and design descriptions continue to describe current state and do not absorb
ADR chronology.

The ADR v1 metadata profile is fully specified above and ready to encode in its canonical schema.
