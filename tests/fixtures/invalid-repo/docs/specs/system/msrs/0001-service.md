---
type: MSRS
profile-version: 1
id: BAD-MSRS-0002
title: Service requirements
governed-by:
  - BAD-ADR-0009
requirements:
  BAD-REQ-0001:
    sources:
      - type: artifact
        artifact-id: BAD-ADR-0404
      - type: artifact
        artifact-id: BAD-MSRS-0002
      - type: path
        path: docs/specs/system/msrs/0404-missing.md
      - type: path
        path: docs/specs/system/msrs/0001-service.md
---
# Service requirements

## Requirements

### BAD-REQ-0001: Availability
The service MUST stay available.
