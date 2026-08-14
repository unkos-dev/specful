---
kind: adr
profile-version: 1
id: OK-ADR-0001
title: Store progress as events
status: accepted
recorded-on: 2026-08-01
decided-on: 2026-08-01
decision-makers:
  - John
---
# Store progress as events

## Context and Problem Statement

Progress must survive reconnection without a central clock.

## Decision Drivers

- Offline clients reconnect with stale state.

## Considered Options

- Store progress as replayable events.
- Store only the latest position.

## Decision Outcome

Chosen option: store progress as replayable events, because replay resolves
conflicts deterministically.

### Consequences

Replay is idempotent, so reconnection cannot duplicate progress.

### Confirmation

The replay integration tests exercise reconnection after queued edits.
