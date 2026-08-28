<h1>
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/assets/brand/specful-masthead-dark.svg">
    <img src="docs/assets/brand/specful-masthead-light.svg" alt="Specful" height="44">
  </picture>
</h1>

> Re-spec your repository.

[![crates.io][cv]][crates] [![docs.rs][dv]][docs] [![build][bv]][build] [![MSRV][mv]][msrv]

[crates]: https://crates.io/crates/specful
[cv]: https://img.shields.io/crates/v/specful?style=flat-square&label=crates.io&labelColor=5F646A&color=C54A3B
[docs]: https://docs.rs/specful/latest/specful/
[dv]: https://img.shields.io/docsrs/specful?style=flat-square&label=docs.rs&labelColor=5F646A&color=C54A3B
[build]: https://github.com/unkos-dev/specful/actions/workflows/ci.yml
[bv]: https://img.shields.io/github/actions/workflow/status/unkos-dev/specful/ci.yml?branch=main&style=flat-square&label=build&labelColor=5F646A&color=C54A3B
[msrv]: https://github.com/unkos-dev/specful/blob/main/rust-toolchain.toml
[mv]: https://img.shields.io/crates/msrv/specful?style=flat-square&label=MSRV&labelColor=5F646A&color=C54A3B

Specful is a portable convention for keeping software requirements, design documentation, and decisions current,
connected, and usable by people and coding agents, served by a single-binary CLI.

The convention is the product. A repository that follows the layout, templates, and writing model is a Specful
repository, readable and navigable with ordinary files, Git, and text search. The `specful` CLI mechanises what the
convention cannot deliver by hand at acceptable cost: allocating stable identifiers, regenerating navigation views, and
validating the repository.

## Direction

Specful is designed around a few commitments:

- Requirements define what the software must do now.
- Design descriptions explain how the software works now.
- Architecture Decision Records preserve the rationale for durable decisions.
- Private plans coordinate transitions without becoming permanent public documentation.
- Markdown and structured metadata in the adopting repository are canonical.
- Indexes and catalogs are generated, disposable views.
- The core information model is independent of any agent harness.

See the [project charter](docs/project-charter.md) for the product boundary and design principles. The artifact profiles
are documented under [`docs/adr/`](docs/adr/README.md), [`docs/msrs/`](docs/msrs/README.md),
[`docs/msdd/`](docs/msdd/README.md), and [`docs/okf/`](docs/okf/README.md), with repository configuration in
[`docs/configuration.md`](docs/configuration.md).

## Status

The v0.1 information model is defined: artifact profiles, JSON Schemas with language-neutral conformance cases, and
templates. The CLI (`init`, `new`, `validate`, `index`, `show`, `trace`) is implemented.

## Install

Install from crates.io with `cargo install --locked specful` (requires Rust 1.97.1 or newer), or download a prebuilt
binary archive for Linux (static musl), macOS, or Windows from the
[GitHub releases](https://github.com/unkos-dev/specful/releases); each archive ships with a SHA-256 checksum.

## Quick start

Adopt the convention in a repository, then validate it:

```sh
specful init --project-key MYAPP
specful validate
```

The project key (2 to 10 uppercase letters or digits, starting with a letter) prefixes every allocated artifact
identifier.

## Influences

Specful builds on established ideas from requirements engineering, architecture documentation, knowledge interchange,
and agent-assisted development. [NOTICE.md](NOTICE.md) records the principal influences and their licenses.

## License

Specful's implementation and general repository content are licensed under the [Apache License 2.0](LICENSE).

Reusable templates, schemas, and examples are dedicated to the public domain under
[CC0 1.0 Universal](templates/LICENSE). The nearest license file and any SPDX identifier on an individual file determine
which terms apply.

The Specful marks under [`docs/assets/brand/`](docs/assets/brand/) are reserved brand assets governed by their
[own licence](docs/assets/brand/LICENSE).

Specful claims no rights over material supplied by an adopting project. To the extent Specful holds rights in the
Specful-provided portions of generated artifacts, those portions are available under CC0-1.0.
