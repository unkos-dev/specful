# Review report format

Use this compact format in the conversation only.

```markdown
## Review: <scope>

Target: <draft description or immutable identity>
Artifacts: <count and types>
Execution: <in-session, independent, or sequential topology>
Mechanical validation: <passed, failed, NOT RUN, or UNVERIFIED, with exact-target evidence>
Verdict: <SHIP | CONDITIONAL | NO-SHIP>

<one short readiness summary>

### Findings

- <ID> (<severity>, <confidence>): `<location>`
  - Expectation: <authority and applicable expectation>
  - Evidence: <observed evidence>
  - Impact: <realistic consequence>
  - Required outcome: <smallest defensible correction or evidence request>

### Re-review status

- <previous ID>: <resolved | open | disproved>, <evidence>
```

Omit empty sections. When there are no findings, say so directly. Include limitations only when evidence was unavailable
or the user narrowed scope. A re-review shows previous finding status before correction-caused findings. Do not add
generic praise, duplicated summaries, machine metadata, model identity, or unused sections.
