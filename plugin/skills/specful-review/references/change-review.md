# Change review lens

Apply this lens only when a change is in scope. The canonical rules are step 2 of the public
[authoring workflow](https://unkos-dev.github.io/specful/authoring-workflow/#2-decide-what-the-change-affects), which
states the corpus questions once, and the affected artifacts themselves.

Traverse from each touched module through its scope index to the Designs that describe it and the Requirements those
Designs satisfy. Check whether:

- every affected artifact has a disposition: updated, created, or unaffected with a reason, where unaffected is a
  complete answer when the touched subject's Design remains accurate;
- a gap the change creates or widens is corrected within the change; it is a finding under this lens;
- the change adds a durable decision that an ADR should record, or an obligation that needs a Requirement, given that
  most internal changes produce no new Requirement;
- artifacts in the aggregate change agree with each other and with the smallest relevant code, tests, configuration, or
  public documentation: no contradiction, missing companion update, authority in the wrong direction, or incomplete
  relationship.

A gap that already existed before the change is a proposal, not a finding: name its subject, artifact type, and lasting
benefit under Proposals, and never let it affect the verdict. Close with the change's corpus effect and its reason: more
complete, unchanged, or less accurate. Do not demand an artifact for every change or audit subjects the change does not
touch.
