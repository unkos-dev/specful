---
type: REQ
profile-version: 1
id: OK-REQ-0001
title: Offline replay
governed-by:
  - OK-ADR-0001
---
# Offline replay

## Statement

The client MUST replay queued progress events after reconnection.

## Rationale

Clients that reconnect after an offline period need reliable delivery of the progress recorded while disconnected.

## Acceptance criteria

- Reconnection triggers replay of every progress event queued while offline.
- Replay preserves arrival order.
