# Contributing

Specful is early and its contracts are still settling, but issues and pull
requests are welcome.

Report suspected security vulnerabilities privately through GitHub's
private vulnerability reporting on this repository, never in a public
issue.

## Developer Certificate of Origin

Every commit must carry a `Signed-off-by` trailer matching the commit's
author name and email, added with `git commit -s`; the DCO check verifies
this on every pull request. Because squash merging composes the landed
commit from the pull request title and body, the durable sign-off record
is each pull request's verified commits, not the history on `main`. The
sign-off certifies the Developer Certificate of Origin, version 1.1
(<https://developercertificate.org/>):

```text
Developer Certificate of Origin
Version 1.1

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license (unless I am
    permitted to submit under a different license), as indicated
    in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified
    it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
```

Contributions land under the license that governs the touched files: the
Apache License 2.0 for implementation and general repository content, and
CC0 1.0 for templates and schemas. The nearest license file and any SPDX
identifier on a file determine which applies.

## Process

- Branch from `main`; every change lands through a pull request and a
  squash merge, so the pull request title becomes the commit subject.
- Commit subjects and pull request titles follow Conventional Commits:
  `<type>(<scope>): <description>` with one of `build`, `chore`, `ci`,
  `docs`, `feat`, `fix`, `perf`, `refactor`, `revert`, `style`, or `test`
  (for example `feat: trace requirements to designs`). Explain the why in
  the body, not a changelog of the what.
- No releases are cut yet. When they are, versions follow Semantic
  Versioning, derived from the commit types by release automation;
  versions are never edited by hand, in `Cargo.toml` or anywhere else.
- Specification artifacts follow the writing model in
  [docs/project-charter.md](docs/project-charter.md).

## Local checks

Every pull request runs formatting and tests in continuous integration,
and the DCO app verifies the sign-off on every commit. Run the same
recipes CI runs before pushing:

```sh
just fmt-check
just clippy
just test
just doctests
```

See `justfile` for the full local gate, including the lint and
dependency checks CI also runs.

Tests accompany behaviour changes in the same pull request.
