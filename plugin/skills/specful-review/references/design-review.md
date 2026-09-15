# Design review lens

Apply this lens only when a Design is in scope. The canonical rules are the public
[Design profile](https://unkos-dev.github.io/specful/profiles/design/), the repository's `templates/design.md`, and the
named Design itself.

Inspect the coherent subject and follow only material claims into the smallest relevant implementation, tests,
configuration, interfaces, or public documentation. Check whether:

- its material structure, interfaces, state, runtime behaviour, failure handling, and operational or security boundary
  are accurate enough for the subject, with a reason where a canonical section does not apply;
- `satisfies` and `governed-by` relationships are exact and the document does not duplicate durable decision rationale;
- a material claim agrees with the claim-directed evidence inspected, and a universal claim such as "every handler" or
  "no other resource" rests on an enumeration rather than an impression;
- the Design has a reason to exist under the profile: one coherent subject that is independently understood and
  independently maintained, per the profile's one subject, one document rule, and it describes present behaviour;
- a maintainer with no prior context can change the subject safely from the record, so a Design that states the obvious
  or omits what that change needs fails its purpose;
- the subject is not one another Design already owns, and is not split across Designs that are not independently
  understood and maintained;
- the prose describes current state and carries no transition, migration, or former state;
- the file sits under the scope that owns the subject.

Do not require every canonical section to have equal depth, report a reasoned not-applicable section without evidence
that the reason is false, demand an unrelated code audit, or treat migration history in an external source as a defect
in current-state Design prose. A prose preference without a purpose-fit consequence remains unreportable.
