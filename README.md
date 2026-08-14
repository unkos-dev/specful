# Specful

> Re-spec your repository.

Specful is a portable convention for keeping software requirements, design
documentation, and decisions current, connected, and usable by people and
coding agents, served by a single-binary CLI.

The convention is the product. A repository that follows the layout,
templates, and writing model is a Specful repository, readable and navigable
with ordinary files, Git, and text search. The `specful` CLI mechanizes what
the convention cannot deliver by hand at acceptable cost: allocating stable
identifiers, regenerating navigation views, and validating the repository.

## Direction

Specful is designed around a few commitments:

- Requirements define what the software must do now.
- Design descriptions explain how the software works now.
- Architecture Decision Records preserve the rationale for durable decisions.
- Private plans coordinate transitions without becoming permanent public
  documentation.
- Markdown and structured metadata in the adopting repository are canonical.
- Indexes and catalogs are generated, disposable views.
- The core information model is independent of any agent harness.

See the [project charter](docs/project-charter.md) for the product boundary
and design principles. The artifact profiles are documented under
[`docs/adr/`](docs/adr/README.md), [`docs/msrs/`](docs/msrs/README.md),
[`docs/msdd/`](docs/msdd/README.md), and [`docs/okf/`](docs/okf/README.md),
with repository configuration in
[`docs/configuration.md`](docs/configuration.md).

## Status

The v0.1 information model is defined: artifact profiles, JSON Schemas with
language-neutral conformance cases, and templates. The CLI (`init`, `new`,
`validate`, `index`, `show`, `trace`) is in progress; no release or
installation contract exists yet.

## Influences

Specful builds on established ideas from requirements engineering,
architecture documentation, knowledge interchange, and agent-assisted
development. [NOTICE.md](NOTICE.md) records the principal influences and
their licenses.

## License

Specful's implementation and general repository content are licensed under
the [Apache License 2.0](LICENSE).

Reusable templates, schemas, and examples are dedicated to the public domain
under [CC0 1.0 Universal](templates/LICENSE). The nearest license file and
any SPDX identifier on an individual file determine which terms apply.

Specful claims no rights over material supplied by an adopting project. To
the extent Specful holds rights in the Specful-provided portions of generated
artifacts, those portions are available under CC0-1.0.
