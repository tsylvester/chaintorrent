# Claude Code terminal guard

Enforces [docs/agents/terminal-policy.md](../../docs/agents/terminal-policy.md) in Claude
Code. Claude Code is a **script-hook** environment: a `PreToolUse` hook runs a script that
hard-**denies** a command before it executes. The hook fires *before* the permission-mode
check, so it blocks even under `--dangerously-skip-permissions` / bypass mode.

## What's here

- `../settings.json` — wires the `PreToolUse` hook (fixed path Claude Code requires).
- `guard.ts` — self-contained: inlined two-gate engine + Claude's deny envelope.
- reads `../../.agent/terminal-guard/allowlist.json` — the shared allowlist (see below).

## Activate

`settings.json` is already in place. If you already had `.claude/settings.json`, merge the
`hooks.PreToolUse` entry into your existing file rather than overwriting (keep your
`permissions` and other keys). The `matcher` is `Bash|PowerShell`; add any other
shell-executing tool names your setup exposes.

## Pick your runtime

Set the command in `../settings.json` to match your toolchain (copy/paste one):

```jsonc
// Deno (default) — --allow-read lets it read the allowlist file
{ "command": "deno run --allow-read \"$CLAUDE_PROJECT_DIR/.claude/terminal-guard/guard.ts\"" }

// Bun
{ "command": "bun run \"$CLAUDE_PROJECT_DIR/.claude/terminal-guard/guard.ts\"" }

// Node 23.6+ (native TS) — or `npx tsx \"$CLAUDE_PROJECT_DIR/.claude/terminal-guard/guard.ts\"`
{ "command": "node \"$CLAUDE_PROJECT_DIR/.claude/terminal-guard/guard.ts\"" }
```

## Set the allowlist

The allowlist lives **once**, shared with every script-hook environment, in
[`.agent/terminal-guard/allowlist.json`](../../.agent/terminal-guard/allowlist.json). It
ships **deny-all** (empty `allow`). Permit your toolchain's read-only checks by copying
entries from `examples` into `allow`:

```json
{ "allow": ["deno check", "deno lint"] }
```

Entry rules (full spec in the policy doc): each entry is a **literal command prefix**; any
shell metacharacter (`; & | > < ` `` ` `` ` $ ( )`) forces a deny even under an allowed
prefix (`deno check && rm -rf x` → denied); add only commands that pass **both gates** —
read-only *and* no internal-tool equivalent.

## Limits

- **Hard enforcement, full policy** — Claude Code runs the engine, so this blocks exactly
  per the two-gate test, before the tool runs, even in bypass mode. No approximation.
- Allowlisted commands `exit 0` (no opinion) and fall through to normal permission
  handling — the hook guarantees denied commands are blocked, not that allowed ones skip
  your prompts. Add a matching `permissions` allow-rule if you want them prompt-free.
