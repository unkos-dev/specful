# Security

Specful is a documentation convention and a command-line tool that reads and writes files in an adopting repository.
This file covers how to report a vulnerability in Specful itself, not in a repository that uses it.

## Supported versions

Security fixes target the latest release. If you use an older version, confirm that the issue still affects the latest
release before reporting it. The [adoption guide](https://unkos-dev.github.io/specful/adoption/) explains how to update.

## Reporting a vulnerability

Use GitHub's private advisory path:
<https://github.com/unkos-dev/specful/security/advisories/new>

Do not open a public issue or disclose elsewhere until a fix is released.

Please include:

- What the issue is
- Steps to reproduce it against a local build, including a minimal repository or document where relevant
- Affected version or commit SHA
- Your assessment of impact

You will receive an acknowledgement within 72 hours and an initial assessment within seven days. Most fixes ship within
90 days, and simpler ones much sooner. Where a vulnerability warrants a CVE, GitHub Security Advisories can issue one at
publication.

## What is in scope

- The `specful` binary and library crate, including how they handle untrusted Markdown, YAML frontmatter and
  configuration in a repository they are run against.
- The published release artifacts, their checksums and the workflows that build them.
- The agent skills shipped in this repository, where their instructions could lead a harness to act outside the adopting
  repository or against its owner's intent.
- Templates, schemas and generated output, where a Specful-supplied default is less safe than it should be.

## What is not in scope

- The content of an adopting repository's own Requirements, Designs and decision records.
- The agent harness, the GitHub CLI or other third-party tools that install or run Specful, unless Specful's own code or
  skills are what enable the problem.

## Safe harbour

Good-faith research within the scope above is welcome. Test against repositories and machines you control and report
through the process described here, and no legal action will follow.

## Credit

Unless you would rather stay anonymous, your name or handle appears in the release notes and the associated advisory.
Say at report time whether you want it there.
