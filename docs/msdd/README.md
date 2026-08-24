# MSDD profile v1

An MSDD module describes current software design for one architectural scope.
Place modules at
`docs/specs/<scope...>/msdd/NNNN-short-title.md`.
Each architectural scope directory uses lowercase ASCII kebab-case, such as
`backend` or `data-plane`. This naming rule applies only to scope segments
inside `docs/specs/`.

Frontmatter requires `type: MSDD`, `profile-version: 1`, a stable
`PROJECT-MSDD-NNNN` identifier, and a title. The filename number matches the
module sequence. Lowercase `x-` extensions are permitted.

The optional `satisfies` field is a non-empty array of unique requirement
identifiers. It stores design-owned relationships to individual MSRS
requirements. The array is semantically unordered, so an unsorted unique array
is conformant. Reverse requirement coverage is generated and is never stored
in MSRS.

An optional `governed-by` array lists the identifiers of the ADRs that govern
this module. Entries are unique ADR identifiers. The relationship is stored on
the design side and gives readers a typed path from a design to its
rationale.

Omitting `satisfies` is valid and produces no core diagnostic. A generated view
may list untraced modules as neutral coverage information.

The body has one level-one heading exactly matching the frontmatter title and
non-empty current-design content. No universal body-section taxonomy applies.
Use the views, diagrams, tables, and headings that explain the scope clearly,
in declarative present-tense prose.

Ordinary Markdown links remain optional navigation and create no typed
relationship. A governing decision belongs in `governed-by`.

Fenced code blocks (backtick or tilde) are the recognised code form; indented
code blocks are not treated as code.

Moving, renaming, or editing a module preserves its identifier. Deletion or a
requirement change updates affected typed relationships in the same logical
change. Git retains prior design states; the module itself contains no status,
migration diary, or obsolete narrative.
