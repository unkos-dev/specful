# This repository uses Specful

Specifications and decisions are canonical Markdown with structured metadata, kept in `docs/specs/` and `docs/adr/`.
Generated indexes and the machine-readable catalog are derived from that source and disposable: rebuild them, never
hand-edit them.

## Retrieval recipe

Start at `docs/specs/index.md` and follow the child indexes down to the module you need. Frontmatter carries each
artifact's identity and its relationships to others, so a module is navigable on its own once you reach it.
`specful show <ID>` prints the catalog record for an identifier; `specful trace <ID>` follows requirement-to-design
links. Plain text search across `docs/specs/` and `docs/adr/` always works, with or without the CLI installed.

## Authoring workflow

Create every new specification, design description, or decision record with `specful new`; never hand-allocate an
identifier. Requirements and design descriptions describe current state only, written as though the system has always
worked this way. A requirement block uses at least one uppercase BCP 14 keyword (MUST, MUST NOT, SHOULD, SHOULD NOT,
MAY). A design module declares the requirements it `satisfies`; a specification cites its governing ADRs through
`governed-by`. After editing, run `specful index` to regenerate the navigation views, then `specful validate` to check
the result, and commit the regenerated views with your change. Coordinate a transition with a plan in `plans/`, copied
by hand from `templates/change-plan.md` or `templates/arc-plan.md` at the versioned repository link below; graduate any
durable rationale to an ADR, then move the plan out of the active set once the transition lands.

## Which artifact changes

| Kind of change | Artifact | How it changes |
|---|---|---|
| Behaviour or constraint | MSRS | Rewrite in place |
| How the system works | MSDD | Rewrite in place |
| Durable decision rationale | ADR | New record, old one superseded |
| Active transition | Plan | Archived or deleted once the transition lands |
| What used to be true | Git history | Never restated in current-state docs |

## About Specful

For the full convention, its artifact profiles, and the CLI reference matching this installation, see
<https://github.com/unkos-dev/specful/tree/v0.2.0>.
