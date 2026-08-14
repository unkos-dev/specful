---
type: MSDD
profile-version: 1
id: OK-MSDD-0001
title: Progress pipeline
satisfies:
  - OK-REQ-0001
  - OK-REQ-0002
governed-by:
  - OK-ADR-0001
---
# Progress pipeline

The pipeline ingests progress events into an ordered queue and replays them
on reconnection in arrival order.
