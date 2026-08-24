---
type: MSRS
profile-version: 1
id: OK-MSRS-0001
title: Progress sync requirements
governed-by:
  - OK-ADR-0001
requirements:
  OK-REQ-0001:
    sources:
      - type: artifact
        artifact-id: OK-ADR-0001
      - type: path
        path: docs/adr/0001-store-progress-events.md
  OK-REQ-0002:
    sources:
      - type: citation
        description: Support policy, edition 2
---
# Progress sync requirements

## Requirements

### OK-REQ-0001: Offline replay

The client MUST replay queued progress events after reconnection.

### OK-REQ-0002: Replay pacing

The client SHOULD pace replay to avoid saturating the backend.

#### Rationale

Bulk replay after long offline periods can starve interactive requests.
