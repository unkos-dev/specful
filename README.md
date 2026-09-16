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
[![OpenSSF Scorecard][sv]][scorecard]

[crates]: https://crates.io/crates/specful
[cv]: https://img.shields.io/crates/v/specful?style=flat-square&label=crates.io&labelColor=5F646A&color=C54A3B
[docs]: https://docs.rs/specful/latest/specful/
[dv]: https://img.shields.io/docsrs/specful?style=flat-square&label=docs.rs&labelColor=5F646A&color=C54A3B
[build]: https://github.com/unkos-dev/specful/actions/workflows/ci.yml
[bv]: https://img.shields.io/github/actions/workflow/status/unkos-dev/specful/ci.yml?branch=main&style=flat-square&label=build&labelColor=5F646A&color=C54A3B
[msrv]: https://github.com/unkos-dev/specful/blob/main/Cargo.toml
[mv]: https://img.shields.io/crates/msrv/specful?style=flat-square&label=MSRV&labelColor=5F646A&color=C54A3B
[scorecard]: https://scorecard.dev/viewer/?uri=github.com/unkos-dev/specful
[sv]: https://api.scorecard.dev/projects/github.com/unkos-dev/specful/badge

Changing code is harder when requirements are implicit and decision rationale is scattered across chats and issues.
Specful is a portable convention for keeping software requirements, design documentation and decisions in your
repository. People and coding agents read the same Markdown files.

The convention is the product. Its layout, templates and writing model work with ordinary files, Git and text search.
The single-binary CLI allocates stable identifiers, generates navigation and validates document structure and
relationships. To explore the convention before installing the CLI, start with the [templates](templates/) and
[convention guidance](#the-convention).

**Start with one subject that matters to your next change.** A partially documented repository is a valid, ongoing
state; you do not need to document the whole codebase to adopt Specful.

[Document model](#one-subject-connected-documents) · [Create your first artifact](#create-your-first-artifact) ·
[Adoption guide](https://unkos-dev.github.io/specful/adoption/)

## One subject, connected documents

| Document | What it records |
| --- | --- |
| Requirement | What the software must do now |
| Design | How the software works now and which Requirements it satisfies |
| Architecture Decision Record (ADR) | The rationale for a durable choice |

![An API scope: two Designs each satisfy two Requirements, and both are governed by a shared API conventions ADR. A generated index lists the scope's Requirements and Designs.](docs/assets/api-scope.svg)

[View full-size diagram](docs/assets/api-scope.svg)

In this illustrative API scope, the API error-handling Design connects a consistent error format with safe handling of
internal failures. A second Design explains how version checks protect concurrent edits. Their shared API conventions
ADR lives in `docs/adr/`. The Designs record these relationships in their frontmatter.

## The convention

Requirements and Designs describe current state only. Durable rationale belongs in an ADR; Git preserves what used to be
true. Plans coordinate changes without becoming part of the permanent specification.

- Markdown and structured metadata in the adopting repository are canonical.
- Indexes and catalogs are generated, disposable views.
- The core information model is independent of any agent harness.

Specful does not replace issue tracking or source code, or automatically generate a complete specification from an
undocumented codebase.

The profile sources are available in the [ADR reference](docs/adr/README.md), [schemas](schemas/) and
[templates](templates/). The [configuration reference](docs/configuration.md) covers repository settings.

## Create your first artifact

Choose one subject you need to understand. This walkthrough uses identifier allocation and scaffolding, the subject of
an existing Specful Design. For your repository, substitute a subject from its implementation.

### Install

Choose a version from the [GitHub releases](https://github.com/unkos-dev/specful/releases) and read its release notes.
With Rust 1.97.1 or newer installed, run:

```sh
cargo install --locked --version <VERSION> specful
```

Replace `<VERSION>` with the release number without its `v` prefix. Optional agent skills use the matching `<TAG>`,
including that prefix: version `X.Y.Z` pairs with tag `vX.Y.Z`.

Alternatively, download the release's prebuilt binary archive for Linux (static musl), macOS or Windows. Each archive
ships with a SHA-256 checksum.

<details>
<summary>Verify release binaries</summary>

From v0.5.2, release binaries embed their Rust dependency inventory. After installing
[`cargo-audit`](https://github.com/rustsec/rustsec/tree/main/cargo-audit#installation), scan an extracted binary with
`cargo audit bin <path-to-specful>`. Verify a downloaded archive's build provenance with
`gh attestation verify <archive> --repo unkos-dev/specful` using the [GitHub CLI](https://cli.github.com/).

</details>

<details>
<summary>Version pinning and upgrades</summary>

Pin the binary and skills to the same release. Before upgrading, preserve the repository's pre-conversion Git state and
previous binary. Specful does not migrate artifacts automatically. Follow the
[update instructions](https://unkos-dev.github.io/specful/adoption/#updating) for conversion and rollback procedures.

</details>

### Initialise your repository

Run these commands from your repository root. If `docs/specs/` or `docs/adr/` already contains documents or hand-written
indexes, follow
[existing-repository adoption](https://unkos-dev.github.io/specful/adoption/#adopting-into-an-existing-repository)
first. If `.specful/config.yaml` already exists, follow the
[update instructions](https://unkos-dev.github.io/specful/adoption/#updating) instead of initialising again.

Choose a project key of 2 to 10 uppercase letters or digits, starting with a letter. The key is immutable and prefixes
every allocated artifact identifier, such as `MYAPP-REQ-0001`. Replace `MYAPP` below with your chosen key.

```sh
specful init --project-key MYAPP
specful validate
```

`init` creates `.specful/config.yaml`, the artifact directories and empty generated navigation views. It does not create
or modify agent instruction files. `validate` checks this initial structure; your first document supplies the content.

### Describe one subject

Scaffold a Design to describe how identifier allocation and scaffolding work:

Replace the title and scope with your chosen subject and its area of the system.

```sh
specful new design \
  --title "Identifier allocation and scaffolding" \
  --scope authoring
```

Open the Markdown file named in the output. For a completed example of the same subject, read Specful's
[identifier allocation Design](docs/specs/authoring/design/0001-identifier-allocation-and-scaffolding.md).

Replace the placeholders with an accurate description of that subject in your repository, checking it against the
implementation and tests. Add relationships to existing Requirements or ADRs where they apply, and remove unused
optional fields. The [authoring workflow](https://unkos-dev.github.io/specful/authoring-workflow/) guides you through
completing the document.

### Validate and start reading

```sh
specful index
specful validate
```

**Start reading at `docs/specs/index.md`** and follow your chosen scope (`authoring` in this example) to your new
Design. Commit the source document with the regenerated views.

Validation checks structure and recorded relationships. Review the content against the code and tests to establish its
accuracy. Add `specful validate` to pre-push or CI to check future changes; see
[validation integration](https://unkos-dev.github.io/specful/reference/validation-integration/) for examples.

## Agent skills (optional)

The CLI and convention work without agent skills. Opt-in skills teach authoring and development workflows, loaded by the
harness when they apply.

### Install the skills

Install at user scope, using the exact tag of the release chosen for the binary, including its `v` prefix. When run
interactively, the GitHub CLI prompts for the target agent:

```sh
gh skill install unkos-dev/specful \
  --all --scope user --pin <TAG>
```

Specful validates the package against the Agent Skills specification. The GitHub CLI owns the supported-agent list and
scope behaviour. For non-interactive installation, add `--agent` with a value from its
[`gh skill install` manual](https://cli.github.com/manual/gh_skill_install). The skills use the CLI and the adopting
repository's artifacts as ground truth. See the [adoption guide](https://unkos-dev.github.io/specful/adoption/) for
installation and updates.

### Adopt, document and review

- `specful-onboard` coordinates evidence-led adoption in an existing repository, from selecting a coherent subject to
  handing approved artifacts to the type-specific skills;
- `specful-requirement`, `specful-design`, and `specful-adr` each teach authoring one artifact type, from scaffolding
  with `specful new` through validation and substantive review;
- `specful-review` checks artifacts and changes for substantive defects, including acceptance-criteria quality, factual
  accuracy, and missing or stale documentation affected by a change.

### Plan and implement

- `specful-plan` creates a right-sized implementation plan or coordinating arc when work needs a persistent hand-off;
- `specful-implement` executes a named plan one step at a time, stopping when the repository contradicts it.

### Validate and look up records

- `specful-validate`, `specful-index`, `specful-show`, and `specful-trace` wrap the matching CLI commands for direct
  invocation mid-session.

## Documentation

- [Adoption](https://unkos-dev.github.io/specful/adoption/): introduce Specful to a repository or update an
  installation.
- [Development workflow](https://unkos-dev.github.io/specful/authoring-workflow/): work in a repository that uses
  Specful.
- [CLI reference](https://unkos-dev.github.io/specful/reference/cli/): commands, arguments and effects.
- [Rust API reference](https://docs.rs/specful/latest/specful/): use Specful as a library.

The [user documentation](https://unkos-dev.github.io/specful/) publishes the current Requirement, Design and ADR
profiles. For the exact text shipped with a release, see its [GitHub tag](https://github.com/unkos-dev/specful/tags).

## Contributing

Issues and pull requests are welcome. See [Contributing](CONTRIBUTING.md) for the process and private security
reporting.

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
