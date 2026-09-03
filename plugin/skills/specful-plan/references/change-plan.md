# Change-plan contract

Use a change plan for one coherent deliverable. Follow the repository's own template when it has one. Otherwise use the
fixed structure below; keep every section and give a brief reason when one does not apply.

Required frontmatter is `type: change-plan`, `status`, and `created`. Status is `draft`, `active`, `blocked`, or
`complete`. Optional `issue` and `relates-to` preserve external and Specful relationships. Optional `part-of` links a
child change plan back to its governing arc.

Every section holds one kind of content. A fact lives in the section typed for it and is cited by its identifier
elsewhere: a file by `path:lines`, an acceptance criterion by its `AC` number, a decision by its row.

## Fixed sections

### Outcome

Labelled one-line fields, in this order:

- **Problem:** the specific problem and who experiences it.
- **Affected user:** the user, operator, or system experiencing it.
- **Outcome:** what becomes possible or reliably different.
- **Invariant:** the observable property every acceptable solution preserves, stated without naming a mechanism.
- **Success signal:** evidence that the delivered change improved the outcome, or
  `Not measured separately: <why acceptance fully captures it>`. Never invent a metric.
- **Approach:** the chosen solution in one sentence.

### Recommendation

One paragraph on why this is the smallest coherent approach the repository supports: the existing primitives it reuses,
the machinery it avoids, and what justifies any new state or abstraction. Then:

- **Evidence:** bullets of `path:lines` with the decisive behaviour, primitive, or convention each shows; governing
  Specful artifacts and accepted ADRs first; decision-relevant tracker discussion; primary sources with versions when
  external behaviour matters.
- **Alternatives:** each rejected alternative and why it loses against the invariant, evidence, or ownership cost.

### Root cause

For asserted broken behaviour only: observed failure, causal chain, fix boundary as `path:line`, regression proof, and
remaining uncertainty. Otherwise state that no broken behaviour was asserted.

### Visuals

A before-and-after flow for an interaction change, or a component diagram for an ownership, state, or data-flow change,
when it makes a material relationship easier to verify. Otherwise state why a model does not help.

### Implementation context

- **Reading:** a table with columns `File`, `Lines`, `Why`. Every row is a file a fresh executor must read before
  starting and the primitive, contract, integration point, or test precedent it carries.
- **Patterns and primitives:** bullets of `path:lines` naming the closest precedent to follow and the primitives to
  reuse. Quote a short snippet only when prose cannot convey the shape; quote the exact content of a new file when it is
  small.
- **Integration points:** bullets of `path:line` with the current role and how the change connects.
- **Verified commands:** the repository's own commands, confirmed to exist.

### Scope

- **In scope:** the agreed outcomes.
- **Not building:** each explicit exclusion and why it is outside the invariant or belongs later.

### Implementation

Numbered tasks in dependency order. Each task is an outcome, not a file inventory, and carries exactly these fields:

```markdown
### N. <Outcome>

**Files and integration points**
- `path:line` - CREATE or UPDATE - why this location owns the change

**Implementation**
- Concrete behaviour, contract, or data flow to add or change, and the precedent to reuse by `path:lines`.
- Load-bearing boundary, failure behaviour, or gotcha. Omit anything the executor can read from the cited file.

**Tests**
- Behaviour to prove and the test surface that proves it.

**Validation**
- `<command>` - expected observable result; the command fails when this task's behaviour is absent.
```

### Acceptance

The completed behavioural contract, stated once, each criterion numbered `AC1`, `AC2`, and so on. Each is an observable
outcome or a preserved invariant, never a task.

### Validation

A table with columns `Gate`, `Command or procedure`, `Proves`, listing the repository's integrated gates in execution
order. `Proves` names acceptance criteria by number.

### Risks and decisions

A table with columns `Decision or risk`, `Recommendation`, `Evidence or mitigation`, `Consequence if different`. Minor
decisions only; an architectural fork is resolved with the user before the plan is written.

### Progress and hand-off

Current checkpoint, next action, and active blockers, then the fixed `### Amendments` and `### Final disposition`
subheadings.

## Rules

Frontmatter alone owns lifecycle status. Progress does not repeat it or accumulate routine command logs. A change plan
has no deliverable-status table because independently deliverable boundaries require an arc.

Plans are temporary and cannot amend canonical artifacts. When implementation changes current obligations, design, or a
durable decision, its tasks update the corresponding Requirement, Design, or ADR through that artifact's lifecycle.

No placeholders, generic examples, confidence scores, or coverage targets remain in a saved plan.
