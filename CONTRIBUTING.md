# Contributing

Specful is early and its contracts are still settling, but issues and pull
requests are welcome.

## Developer Certificate of Origin

Every commit must carry a `Signed-off-by` trailer with your real name and
email, added with `git commit -s`. The sign-off certifies the Developer
Certificate of Origin, version 1.1 (https://developercertificate.org/):

```
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
CC0 1.0 for templates, schemas, and examples. The nearest license file and
any SPDX identifier on a file determine which applies.

## Process

- Branch from `main`; every change lands through a pull request and a
  squash merge, so the pull request title becomes the commit subject.
- Commit subjects and pull request titles follow Conventional Commits:
  `<type>(<scope>): <description>` with one of `build`, `chore`, `ci`,
  `docs`, `feat`, `fix`, `perf`, `refactor`, `revert`, `style`, or `test`
  (for example `feat: trace requirements to designs`). A check enforces
  this on every pull request title. Explain the why in the body, not a
  changelog of the what.
- Released versions follow Semantic Versioning, derived from the commit
  types by release automation. Versions are never edited by hand, in
  `Cargo.toml` or anywhere else.
- Documentation follows the writing model in
  [docs/project-charter.md](docs/project-charter.md): current state only,
  no history, no em dashes.

## Local checks

Continuous integration runs formatting, tests, and the DCO check on every
pull request. Before pushing:

```
cargo fmt
cargo clippy --all-targets
cargo test
```

Tests accompany behaviour changes in the same pull request.
