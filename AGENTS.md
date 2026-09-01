# Repository Guidelines

## Project structure

Specful is a Rust 2024 edition CLI and library. The command entry point is `src/main.rs`; `src/lib.rs` exposes reusable
modules. Integration tests live in `tests/`, with repository-shaped fixtures in `tests/fixtures/`. Keep JSON Schemas and
their language-neutral cases together under `schemas/<profile>/`. Public artifact guidance is in `docs/`, and reusable
source files are in `templates/`. Schema behaviour changes require corresponding conformance cases. Generated indexes
and catalogs are disposable views, not hand-edited source.

## Build, test, and development commands

Install pinned tools with `mise install`. The `justfile` is the canonical command surface:

- `just fmt` formats Rust code.
- `just check` runs formatting checks, Clippy, rustdoc link checks, and tests.
- `just test` runs the nextest suite; pass nextest arguments after the recipe when narrowing a run.
- `just doctests` runs documentation tests, which nextest does not cover.
- `just preflight` runs the full local gate, including dependency, prose, workflow, and secret checks. Run it before
  pushing.
- `cargo run -- <command>` exercises the CLI locally, for example `cargo run -- validate`.

## Code and comments

Use rustfmt defaults and four-space indentation. Clippy warnings are errors. Follow Rust conventions: `snake_case` for
modules, functions, and tests; `UpperCamelCase` for types; and `SCREAMING_SNAKE_CASE` for constants.

Do not add comments by default. Judge the code as a cold reader would: if its purpose or behaviour is unclear, first fix
the names, types, structure, or control flow. If the missing context belongs to a module, workflow, or public contract,
put it in the nearest relevant README or document. Add a comment only as a last resort when a non-obvious constraint,
invariant, or reason still cannot be inferred from the code or documented in a better place. Keep it brief and do not
narrate the next line, the change history, or why the implementation is correct. When in doubt, omit the comment.

## Testing

Add tests with every behaviour change. Prefer integration tests named after the observable capability, such as
`cli_root_discovery.rs`, and cover the happy path, invalid input, and non-obvious edge cases. Add fixture repositories
only when filesystem layout is part of the behaviour. Use `just cov` when coverage evidence helps.

## Contribution policy

[`CONTRIBUTING.md`](CONTRIBUTING.md) is the authority for commit messages, branch names, DCO sign-off, pull requests,
licensing, releases, and security reporting. Follow it rather than duplicating those rules here.
