# Harness hooks

An agent harness can run `specful index --check` and `specful validate` automatically and feed findings back to the
agent, and can ask for a review pass before a push. The hooks below call the `specful` binary already on PATH; nothing
is installed and the adopting repository owns the config.

## What the hooks do

- After every edit or write, run the two checks and return findings to the agent.
- When the agent tries to stop, run the same checks and continue the turn when they fail, at most once per turn.
- Before a `git push` that carries changes under `docs/specs`, `docs/adr`, or `.specful`, tell the agent to run the
  `specful-review` skill first.

The push hook is advisory. The other two report findings. None of them blocks a commit or a push.

## Claude Code

The block goes in the project's `.claude/settings.json`, which is committed, or in `~/.claude/settings.json` for one
user across every repository. Project hooks run only after the workspace trust prompt is accepted.

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "grep -q 'git push' || exit 0; git diff --name-only \"$(git rev-parse -q --verify '@{push}' 2>/dev/null || echo origin/main)\" HEAD -- docs/specs docs/adr .specful 2>/dev/null | grep -q . || exit 0; printf '%s' '{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"additionalContext\":\"Outgoing commits change Specful artifacts. Before pushing, run the specful-review skill as a change review of the commits not yet on the remote, and do not push on a NO-SHIP verdict.\"}}'",
            "timeout": 10
          }
        ]
      }
    ],
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "specful index --check >&2 && specful validate >&2 || exit 2",
            "timeout": 10
          }
        ]
      }
    ],
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "grep -q '\"stop_hook_active\": *true' && exit 0; specful index --check >&2 && specful validate >&2 || exit 2",
            "timeout": 10
          }
        ]
      }
    ]
  }
}
```

## Codex

The file is `.codex/hooks.json` in the project or `~/.codex/hooks.json` for the user. Project hooks load only when the
project `.codex` layer is trusted. The only difference from the Claude Code block is the edit matcher.

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "grep -q 'git push' || exit 0; git diff --name-only \"$(git rev-parse -q --verify '@{push}' 2>/dev/null || echo origin/main)\" HEAD -- docs/specs docs/adr .specful 2>/dev/null | grep -q . || exit 0; printf '%s' '{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"additionalContext\":\"Outgoing commits change Specful artifacts. Before pushing, run the specful-review skill as a change review of the commits not yet on the remote, and do not push on a NO-SHIP verdict.\"}}'",
            "timeout": 10
          }
        ]
      }
    ],
    "PostToolUse": [
      {
        "matcher": "apply_patch|Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "specful index --check >&2 && specful validate >&2 || exit 2",
            "timeout": 10
          }
        ]
      }
    ],
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "grep -q '\"stop_hook_active\": *true' && exit 0; specful index --check >&2 && specful validate >&2 || exit 2",
            "timeout": 10
          }
        ]
      }
    ]
  }
}
```

## Adjusting

- Delete any hook entry not wanted.
- The push hook falls back to `origin/main` when the branch has no upstream.
- The commands need only `sh`, `git`, `grep`, and `specful`.
- The hooks run whichever `specful` is first on PATH, so keep that install at the version the repository targets.
- Exit status 2 is the harness convention that returns stderr to the agent.
