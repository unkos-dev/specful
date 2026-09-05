---
type: DESIGN
profile-version: 1
id: OK-DESIGN-0001
title: Progress pipeline
satisfies:
  - OK-REQ-0001
  - OK-REQ-0002
governed-by:
  - OK-ADR-0001
---
# Progress pipeline

## Purpose and boundaries

The progress pipeline ingests progress events into an ordered queue and replays them on reconnection in arrival order.

## Structure

A single ingestion queue accepts progress events; a replay worker drains the queue on reconnection.

## Interfaces and dependencies

Depends on the client's connectivity events to trigger replay. Exposes no public API beyond the sync client.

## Data and state

The queue persists events until they are replayed, keyed by arrival sequence.

## Runtime behaviour

On reconnection, the replay worker drains the queue oldest first, pacing entries when the backend reports load
pressure.

## Failure and recovery

A replay failure retries the same event once, then continues in arrival order.

## Security and operations

No secrets are handled; observability comes from the replay worker's own event log.
