<h1>
  <picture>
    <source media="(prefers-color-scheme: dark)"
            srcset="https://github.com/unkos-dev/specful/raw/main/docs/assets/brand/specful-masthead-dark.svg">
    <img src="https://github.com/unkos-dev/specful/raw/main/docs/assets/brand/specful-masthead-light.svg"
         alt="Specful" height="44">
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
[msrv]: https://github.com/unkos-dev/specful/blob/main/Cargo.toml
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

See the [project charter](docs/project-charter.md) for the product boundary and design principles. The ADR artifact
profile is documented under [`docs/adr/`](docs/adr/README.md); the Requirement and Design profiles are defined by their
schemas and templates under [`schemas/`](schemas/) and [`templates/`](templates/). Repository configuration is
documented in [`docs/configuration.md`](docs/configuration.md).

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

## Agent skills

The CLI and convention work without agent skills. `specful init` writes the repository configuration, artifact
directories, and generated navigation views; it does not create or modify agent instruction files.

Opt-in agent skills build on that floor, loaded by the harness at the moment they apply:

- `specful-requirement`, `specful-design`, and `specful-adr` each teach authoring one artifact type, from scaffolding
  with `specful new` through validation and substantive review;
- `specful-review` checks an artifact for what mechanical validation cannot: acceptance-criteria quality, artifact
  boundaries, and current-state writing;
- `specful-plan` creates a right-sized implementation plan or coordinating arc when work needs a persistent hand-off;
- `specful-validate`, `specful-index`, `specful-show`, and `specful-trace` wrap the matching CLI commands for direct
  invocation mid-session.

Install all nine skills at user scope. When run interactively, the GitHub CLI prompts for the target agent:

```sh
gh skill install unkos-dev/specful --all --scope user
```

Specful validates the package against the Agent Skills specification. The GitHub CLI owns the supported-agent list and
scope behaviour. For non-interactive installation, add `--agent` with a value from its
[`gh skill install` manual](https://cli.github.com/manual/gh_skill_install). The skills use the CLI and the adopting
repository's artifacts as ground truth. See [Adoption](https://unkos-dev.github.io/specful/adoption/) for the full
installation and update path.

## Documentation

The current artifact profiles (Requirement, Design, ADR), the authoring workflow, and the CLI reference are published at
<https://unkos-dev.github.io/specful/>. For the exact text a given release shipped with, see the corresponding
[GitHub release tag](https://github.com/unkos-dev/specful/tags).

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
