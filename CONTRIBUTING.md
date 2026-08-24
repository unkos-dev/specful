# Contributing

Specful is early and its contracts are still settling, but issues and pull
requests are welcome.

**Security issues are reported privately, not through issues.** Use
GitHub's private vulnerability reporting on this repository.

## Developer Certificate of Origin

Contributions are accepted under the [Developer Certificate of Origin
v1.1](https://developercertificate.org/) (DCO). You keep the copyright of
your work; each contribution is licensed to the project under the license
that governs the files it touches (inbound = outbound): the Apache
License 2.0 for implementation and general repository content, and CC0
1.0 for templates and schemas. The nearest license file and any SPDX
identifier on a file determine which applies. Signing off certifies the
DCO: that you wrote the contribution, or otherwise have the right to
submit it under that license.

Every commit must carry a `Signed-off-by` trailer matching the commit's
author name and email, added with `git commit -s`:

```text
Signed-off-by: Your Name <your-email@example.com>
```

The DCO check verifies the trailer on every pull request. Because squash
merging composes the landed commit from the pull request title and body,
the durable sign-off record is each pull request's verified commits, not
the history on `main`.

## Commit messages and branches

Commit subjects and pull request titles follow Conventional Commits:

```text
<type>(<scope>): <description>
```

The accepted types are `build`, `chore`, `ci`, `docs`, `feat`, `fix`,
`perf`, `refactor`, `revert`, `style`, and `test` (for example `feat:
trace requirements to designs`). Explain the why in the body, not a
changelog of the what.

Branch names use the same type as their prefix (`feat/`, `fix/`, `docs/`,
and so on through the same list).

Squash merging makes the pull request title the commit subject on `main`,
so pull request titles follow the same format.

No releases are cut yet. When they are, versions follow Semantic
Versioning, derived from the commit types by release automation; versions
are never edited by hand, in `Cargo.toml` or anywhere else.

## Pull request process

1. Branch from `main`.
2. Write tests alongside any behaviour change. Specification artifacts
   follow the writing model in
   [docs/project-charter.md](docs/project-charter.md).
3. CI runs formatting and tests on every pull request; run the same
   checks locally before pushing:

   ```sh
   just fmt-check
   just clippy
   just test
   just doctests
   ```

   See `justfile` for the full local gate, including the lint and
   dependency checks CI also runs.
4. Open the pull request.
