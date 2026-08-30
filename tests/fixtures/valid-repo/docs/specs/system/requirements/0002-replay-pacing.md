---
type: REQ
profile-version: 1
id: OK-REQ-0002
title: Replay pacing
---
# Replay pacing

## Statement

The client SHOULD pace replay to avoid saturating the backend.

## Rationale

Bulk replay after long offline periods can starve interactive requests.

## Acceptance criteria

- Replay throughput backs off when the backend reports load pressure.

## More information

No further context beyond the rationale above.
