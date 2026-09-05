---
type: REQ
profile-version: 1
id: "{PROJECT}-REQ-{NNNN}"
title: "{Concise navigation title for the obligation}"
governed-by:
  - "{PROJECT}-ADR-{NNNN}"
---

# {Concise navigation title for the obligation}

{INSTRUCTIONS. This file was scaffolded from the Specful Requirement template; the frontmatter identifier was allocated
by `specful new requirement` and is never edited by hand. Replace every braced block with real content. Every section
heading stays, exactly as written, so Requirements stay searchable and parseable across the repository; where a section
has nothing to say, keep the heading and state "Not applicable: {reason}". More information is the exception: it is
optional and is removed completely when it adds nothing. The document is complete only when no braced text remains,
`specful validate` passes, and every remaining section carries real content or a reasoned not-applicable. Delete this
block last.}

{A Requirement is one coherent normative obligation plus the information needed to understand and evaluate it. One
obligation per file does not mean one sentence per file: complex acceptance boundaries, tables, examples, and
definitions may all be necessary. It does mean one obligation: if the Statement needs "and" to join independently
verifiable duties, split them into separate Requirements. Requirements describe current obligations only, written as
though the system has always carried them; delivery status, migration history, and rejected alternatives do not belong
here, and durable rationale for a governing decision belongs in the ADR that `governed-by` cites (remove the field when
no ADR governs this obligation). The boundary: a Requirement binds the system and is rewritten in place; an ADR records
a choice and is only ever superseded. When a decision's outcome needs to bind, it is stated here, not in the ADR. Delete
this paragraph.}

## Statement

{One normative paragraph stating what the system does, using at least one uppercase BCP 14 keyword (MUST, MUST NOT,
SHOULD, SHOULD NOT, MAY). Name the acting system or component where one exists, the trigger or condition where one
applies, and the observable behaviour; put the condition before the keyword, for example "WHEN the catalog is stale, the
CLI MUST ...". Do not leak your own design choices into the obligation: how the system meets it belongs in the Design
that satisfies this Requirement, though an externally imposed constraint (a mandated protocol, format, mechanism, or
interoperability contract) is itself a legitimate obligation. Avoid vague qualifiers ("reasonable", "appropriate",
"quickly") and escape clauses ("where possible", "as appropriate"); every term the statement relies on must be defined
here or linked, and every bound must be objectively determinable, with a number wherever the obligation is
quantitative.}

## Rationale

{Why the obligation exists: the need it serves, who or what depends on it, and the consequence or risk of not meeting
it. Keep it concise and do not restate the Statement. Cite sources through ordinary Markdown links where a standard,
issue, decision, or user need motivates the obligation.}

## Acceptance criteria

{Observable conditions that distinguish satisfaction from non-satisfaction. Each criterion is independently checkable
with a yes/no answer; use precise units, tolerances, and thresholds for quantitative bounds, and cover the negative and
edge cases that define the obligation's boundary, not only the happy path. A criterion may arrive by reference to an
external standard's own conformance requirements. Where the means of checking a criterion is not obvious, say how
satisfaction is determined in the same line. Bullets are the default form; switch to a table when several criteria share
the same shape. If the criteria keep splitting into unrelated groups, consider whether the Statement carries more than
one obligation.}

- {Observable, yes/no-checkable criterion}
- {Observable, yes/no-checkable criterion}

## More information

{Optional: supporting context such as external references, examples, and runbooks that help implementers and verifiers.
Never restate a `satisfies` or `governed-by` relationship the frontmatter already carries. Peripheral open questions may
live here; an unresolved matter that affects the obligation or its acceptance boundary means the Requirement is not
complete yet. Remove the section when it adds nothing.}
