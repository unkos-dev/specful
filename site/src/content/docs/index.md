---
title: Specful
description: Repository-native requirements, design, and decisions for people and coding agents.
---

Specful is a portable convention for keeping software requirements, design documentation, and decisions current,
connected, and usable by people and coding agents. A repository that follows the convention is readable and navigable
with ordinary files, Git, and text search; the `specful` CLI mechanises what the convention cannot deliver by hand at
acceptable cost.

This site is the documentation home for the convention: the artifact profiles, the workflow an agent or person follows
to author them, the CLI reference, and the boundary questions that come up in practice. It documents the Requirement,
Design, and ADR profiles as they exist today.

## Where to start

- **New to a repository that uses Specful?** Start with [Adoption](/specful/adoption/).
- **Authoring a Requirement, Design, or ADR?** Follow the [authoring workflow](/specful/authoring-workflow/).
- **Looking up a specific profile's rules?** See [Requirement](/specful/profiles/requirement/),
  [Design](/specful/profiles/design/), or [ADR](/specful/profiles/adr/).
- **Looking for a command's flags?** See the [CLI reference](/specful/reference/cli/).

## The information model

Specful distinguishes artifacts by responsibility:

| Artifact | Responsibility | Lifecycle |
|---|---|---|
| Requirement | States one obligation the software carries now | Rewritten or deleted in place |
| Design | Describes how one subject currently works | Rewritten or deleted in place |
| ADR | Records why a durable decision was made | Retained and superseded |
| Plan | Coordinates an active transition | Temporary; archived or deleted on completion |
| Git | Preserves what used to be true | Repository history |

A Requirement may be satisfied by multiple Designs, and one Design may satisfy multiple Requirements. Generated indexes
and a machine-readable catalog assemble the corpus for navigation; no authored container document is canonical.
