# GitHub Copilot terminal guard

Approximates [docs/agents/terminal-policy.md](../docs/agents/terminal-policy.md) for
GitHub Copilot agent mode in VS Code. Copilot is a **declarative + soft** environment:
its terminal auto-approve list decides what runs *without asking you*, but it has **no
hard block** — the underlying safety is Copilot's default manual approval.

The policy **text** reaches Copilot as instruction via `.github/copilot-instructions.md`
→ `docs/agents/index.md` → `terminal-policy.md`. The settings below are the declarative
hardening.

## Activate

The entries live in [`.vscode/settings.json`](settings.json) (repo-committed, shared with
the team). If you already have a `.vscode/settings.json`, merge the
`chat.tools.terminal.autoApprove` key into it. Depending on your VS Code / Copilot
version the setting may instead be named `github.copilot.chat.tools.terminal.autoApprove`
(older builds used separate `.allowlist` / `.denylist` objects) — verify against your
version and rename if needed.

## Set the allowlist — a second copy, in VS Code format

Copilot can't read `.agent/terminal-guard/allowlist.json`, so the allowed read-only
checks are **re-expressed here** as `true` entries. Keep them in sync with the shared
allowlist:

- `"deno check": true` — auto-approve (runs without prompting).
- `"/^\\s*git\\s+(add|commit|push)\\b/": false` — never auto-approve (always ask you).

A `false` entry means **always require your approval**, not a hard block — Copilot has no
mechanism to refuse outright. The metacharacter pattern `"/[;&|`$()<>]/": false` forces a
prompt on any composed command.

## Limits

- **Soft.** The docs describe auto-approve as "a hint"; Copilot still routes many commands
  through manual approval regardless, and an org Copilot policy can override local
  settings. The real guard is **you** approving — treat this as convenience + a nudge, not
  enforcement.
- **Two copies to maintain.** This list duplicates the shared allowlist in VS Code's
  format; update both when your toolchain's checks change.
- No pre-execution engine — the two-gate test is enforced precisely only in Claude Code
  and Cursor.
