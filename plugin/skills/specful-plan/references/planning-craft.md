# Planning craft

The highest-leverage planning decision is usually what not to build. A request arrives bundled with a proposed
mechanism; separate the outcome from the mechanism before designing.

## Find the invariant

State the requirement as an observable property that stays true across every acceptable implementation. It names what a
user or adjacent system must observe and what must remain unchanged. It does not name storage, services, abstractions,
or lifecycle.

- Proposed mechanism: "add a colour-token allowlist and a lint rule for utility classes."
- Invariant: "no stock palette utility compiles; every brand token utility still does."

Keep three things distinct: the invariant, which every solution must keep true; acceptance, which proves the agreed
implementation is complete; and the success signal, which shows the delivered change improved the outcome. Never invent
a persona, business case, or metric to make a plan look complete.

## Search from cheapest to most structural

Before proposing new machinery, check each of these in order and stop at the first that satisfies the invariant cleanly
and can be validated authoritatively:

1. Existing configuration or supported behaviour.
2. Composition of existing commands, APIs, or components.
3. A small extension to a primitive the repository already owns.
4. A new abstraction with one clear owner.
5. A new subsystem with its own lifecycle.

Choose the first shape that fits, not the first familiar pattern. Do not preserve a known poor local convention merely
because it exists.

## Signs of a missing primitive

- The same policy or representation must be maintained at several entry points.
- State or a decision has no obvious owner.
- A new signal must be threaded through types, schemas, or layers that do not own it.
- Compatibility, cleanup, recovery, or synchronisation logic dominates the outcome.

When a missing primitive is load-bearing, recommend building it first and say what it unlocks.

## Apply the laziness test

For every proposed abstraction, state store, background process, configuration surface, or defensive path, ask which
invariant requires it, what evidence rules out the smaller option, what new failure modes it creates, and what
disappears if it is removed. Prefer deletion, direct control flow, shallow call paths, and one resolved decision over
pass-through helpers or policy repeated across layers. Simplicity is fewer states, concepts, and ownership boundaries,
not fewer lines. If state is shared, ask what happens when another actor changes it concurrently; if the answer is not
"nothing", isolate ownership rather than add synchronisation.

## Make verification prove behaviour

A check that can pass while the behaviour is absent proves nothing. For every guard or test, name what would make it
pass vacuously and add the control that rules that out. Task-level validation fails when that task's behaviour is
missing; syntax-only checks do not count when behaviour changed.

## Surface decisions with meaning

Every unresolved item states the decision, the recommendation, the evidence, what changes if the user chooses
differently, and the safe default if deferral is harmless. A decision that changes product behaviour, architecture, or a
foundational data shape is put to the user before the plan is written. The plan is not a hiding place for decisions the
user needs to make.

## Consider delivery only where it applies

When existing users, behaviour, or stored data can be affected, decide the applicable concerns among discoverability,
compatibility, rollout, migration, observability, reversibility, and documentation. Put the resulting work into tasks;
do not add headings for concerns that do not apply.
