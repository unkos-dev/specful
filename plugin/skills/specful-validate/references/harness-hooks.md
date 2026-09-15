# Harness hooks

Use agent hooks for an optional substantive review reminder before a push. Run `specful validate` through Lefthook
pre-push and CI, following the copyable examples in
[Validation integration](https://unkos-dev.github.io/specful/reference/validation-integration/).
Do not add automatic validation to `PostToolUse` or `Stop`.

The examples below ask for `specful-review` before a shell command containing `git push` when outgoing commits change
`docs/specs`, `docs/adr`, or `.specful`. They add context to the agent; they do not run a review, enforce its verdict,
or replace the Git pre-push check. The adopting repository decides whether substantive review is required.

Merge the relevant entry into existing harness configuration, preserving unrelated hooks. These command examples use
`sh`, `git`, `grep`, and `head`; the reminder itself does not invoke the Specful binary.

## Claude Code

Add the entry to the project's `.claude/settings.json`, or to the user configuration if that scope is intended. Project
hooks require workspace trust.

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "grep -q 'git push' || exit 0; b=$(git rev-parse -q --verify '@{push}' 2>/dev/null || git rev-parse -q --verify \"$(git remote | head -n1)/HEAD\" 2>/dev/null); if [ -n \"$b\" ]; then git diff --name-only \"$b\" HEAD -- docs/specs docs/adr .specful | grep -q . || exit 0; m='Outgoing commits change Specful artifacts. Before pushing, run the specful-review skill as a change review of the commits not yet on the remote, and do not push on a NO-SHIP verdict.'; else m='The outgoing range could not be determined from the push target or the remote default branch. If these commits change Specful artifacts, run the specful-review skill as a change review before pushing, and do not push on a NO-SHIP verdict.'; fi; printf '{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"additionalContext\":\"%s\"}}' \"$m\"",
            "timeout": 10
          }
        ]
      }
    ]
  }
}
```

## Codex

Add the entry to the project's `.codex/hooks.json`, or to the user configuration if that scope is intended. The project
configuration must be trusted.

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "grep -q 'git push' || exit 0; b=$(git rev-parse -q --verify '@{push}' 2>/dev/null || git rev-parse -q --verify \"$(git remote | head -n1)/HEAD\" 2>/dev/null); if [ -n \"$b\" ]; then git diff --name-only \"$b\" HEAD -- docs/specs docs/adr .specful | grep -q . || exit 0; m='Outgoing commits change Specful artifacts. Before pushing, run the specful-review skill as a change review of the commits not yet on the remote, and do not push on a NO-SHIP verdict.'; else m='The outgoing range could not be determined from the push target or the remote default branch. If these commits change Specful artifacts, run the specful-review skill as a change review before pushing, and do not push on a NO-SHIP verdict.'; fi; printf '{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"additionalContext\":\"%s\"}}' \"$m\"",
            "timeout": 10
          }
        ]
      }
    ]
  }
}
```

## Scope and limitations

The reminder compares against the branch's configured push target, falling back to the first remote's default branch. If
neither resolves, it asks for review without claiming to know the outgoing range. An explicit push to another target can
differ from this comparison; resolve the actual review target before reviewing.

The command-text match is a convenience reminder for shell-driven pushes. Aliases and GUI pushes may not trigger it. Git
pre-push runs validation when Git performs the push; CI checks the committed corpus.
