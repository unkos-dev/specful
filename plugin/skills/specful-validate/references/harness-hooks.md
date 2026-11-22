# Harness hooks

Use agent hooks for two optional reminders: a pre-edit profile reminder when an agent edits Specful artifacts directly,
and a substantive review reminder before a push. Run `specful validate` through Lefthook pre-push and CI, following the
copyable examples in [Validation integration](https://unkos-dev.github.io/specful/reference/validation-integration/). Do
not add automatic validation to `PostToolUse` or `Stop`.

The pre-edit example matches Claude Code `Edit` and `Write` calls targeting `docs/adr/*.md` (excluding
`docs/adr/README.md`), `docs/specs/**/design/*.md` (excluding `index.md`), and `docs/specs/**/requirements/*.md`
(excluding `index.md`) when the target repository contains `.specful/config.yaml`. The push examples ask for
`specful-review` before a shell command containing `git push` when the repository contains `.specful/config.yaml` and
outgoing commits change `docs/specs`, `docs/adr`, or `.specful`. Guarding both hooks on `.specful/config.yaml` keeps
user-scoped installations quiet in non-Specful repositories. Both hooks add advisory context to the agent; they do not
block edits, run a review, enforce a verdict, or replace the Git pre-push check. The adopting repository decides whether
substantive review is required.

Merge the relevant entries into existing harness configuration, preserving unrelated hooks. These command examples use
POSIX `sh`, `git`, `grep`, and `head`; the reminders do not invoke `jq` or the Specful binary.

## Claude Code

Add the entries you want to the project's `.claude/settings.json`, or to the user configuration if that scope is
intended. Project hooks require workspace trust.

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "p=$(grep -Eo '\"file_path\"[[:space:]]*:[[:space:]]*\"[^\"]*\"' | head -n1); [ -n \"$p\" ] || exit 0; p=${p#*:}; p=${p#*'\"'}; p=${p%'\"'}; case \"$p\" in */docs/adr/README.md|docs/adr/README.md|*/index.md|index.md) exit 0 ;; *.md) ;; *) exit 0 ;; esac; d=${p%/*}; case \"$d\" in */docs/adr) r=${d%/docs/adr}; k=adr ;; docs/adr) r=.; k=adr ;; */docs/specs/design|*/docs/specs/*/design) r=${d%/docs/specs/*}; k=design ;; docs/specs/design|docs/specs/*/design) r=.; k=design ;; */docs/specs/requirements|*/docs/specs/*/requirements) r=${d%/docs/specs/*}; k=req ;; docs/specs/requirements|docs/specs/*/requirements) r=.; k=req ;; *) exit 0 ;; esac; [ -f \"$r/.specful/config.yaml\" ] || exit 0; case \"$k\" in adr) m='Editing a Specful ADR. Load the specful-adr skill and read its bundled profile (references/profiles/adr.md) or run specful profile adr so this record stays a decision record rather than a Requirement or Design.' ;; design) m='Editing a Specful Design. Load the specful-design skill and read its bundled profile (references/profiles/design.md) or run specful profile design so this document stays a current-state Design rather than a Requirement or ADR.' ;; req) m='Editing a Specful Requirement. Load the specful-requirement skill and read its bundled profile (references/profiles/requirement.md) or run specful profile requirement so this document stays a single normative obligation rather than a Design or ADR.' ;; esac; printf '{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"additionalContext\":\"%s\"}}' \"$m\"",
            "timeout": 10
          }
        ]
      },
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "grep -q 'git push' || exit 0; top=$(git rev-parse --show-toplevel 2>/dev/null) || exit 0; r=$(pwd -P); while [ ! -f \"$r/.specful/config.yaml\" ]; do [ \"$r\" = \"$top\" ] || [ \"$r\" = / ] && exit 0; r=${r%/*}; [ -n \"$r\" ] || r=/; done; b=$(git rev-parse -q --verify '@{push}' 2>/dev/null || git rev-parse -q --verify \"$(git remote | head -n1)/HEAD\" 2>/dev/null); if [ -n \"$b\" ]; then git diff --name-only \"$b\" HEAD -- \"$r/docs/specs\" \"$r/docs/adr\" \"$r/.specful\" | grep -q . || exit 0; m='Outgoing commits change Specful artifacts. Before pushing, run the specful-review skill as a change review of the commits not yet on the remote, and do not push on a NO-SHIP verdict.'; else m='The outgoing range could not be determined from the push target or the remote default branch. If these commits change Specful artifacts, run the specful-review skill as a change review before pushing, and do not push on a NO-SHIP verdict.'; fi; printf '{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"additionalContext\":\"%s\"}}' \"$m\"",
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
            "command": "grep -q 'git push' || exit 0; top=$(git rev-parse --show-toplevel 2>/dev/null) || exit 0; r=$(pwd -P); while [ ! -f \"$r/.specful/config.yaml\" ]; do [ \"$r\" = \"$top\" ] || [ \"$r\" = / ] && exit 0; r=${r%/*}; [ -n \"$r\" ] || r=/; done; b=$(git rev-parse -q --verify '@{push}' 2>/dev/null || git rev-parse -q --verify \"$(git remote | head -n1)/HEAD\" 2>/dev/null); if [ -n \"$b\" ]; then git diff --name-only \"$b\" HEAD -- \"$r/docs/specs\" \"$r/docs/adr\" \"$r/.specful\" | grep -q . || exit 0; m='Outgoing commits change Specful artifacts. Before pushing, run the specful-review skill as a change review of the commits not yet on the remote, and do not push on a NO-SHIP verdict.'; else m='The outgoing range could not be determined from the push target or the remote default branch. If these commits change Specful artifacts, run the specful-review skill as a change review before pushing, and do not push on a NO-SHIP verdict.'; fi; printf '{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"additionalContext\":\"%s\"}}' \"$m\"",
            "timeout": 10
          }
        ]
      }
    ]
  }
}
```

## Scope and limitations

A `PreToolUse` hook returning `additionalContext` lets the pending `Edit` or `Write` call proceed; the agent reads the
reminder on its next step. Treat it as an advisory reminder rather than a blocking gate. It does not cover edits
performed through other tools (such as `Bash`, scripts, or external editors).

The push reminder compares against the branch's configured push target, falling back to the first remote's default
branch. If neither resolves, it asks for review without claiming to know the outgoing range. An explicit push to another
target can differ from this comparison; resolve the actual review target before reviewing.

The command-text match is a convenience reminder for shell-driven pushes. Aliases and GUI pushes may not trigger it. Git
pre-push runs validation when Git performs the push; CI checks the committed corpus.
