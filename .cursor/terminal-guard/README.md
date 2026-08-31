# Cursor terminal guard

Enforces [docs/agents/terminal-policy.md](../../docs/agents/terminal-policy.md) in Cursor.
Cursor is a **script-hook** environment: `beforeShellExecution` runs a script that can
hard-**deny** a command before it executes. This is the robust path — Cursor's built-in
allow/deny lists have documented bypass gaps, so the hook is what actually enforces.

## What's here

- `../hooks.json` — wires `beforeShellExecution` to the guard (fixed path Cursor requires).
- `guard.ts` — self-contained: inlined two-gate engine + Cursor's deny envelope.
- reads `../../.agent/terminal-guard/allowlist.json` — the shared allowlist (see below).

## Activate

`hooks.json` is already in place. If you already had a `.cursor/hooks.json`, merge the
`beforeShellExecution` entry into your existing `hooks` object rather than overwriting.
Requires Cursor 1.7+ (agent hooks). Reload the window after adding hooks.

## Pick your runtime

The guard runs under Deno, Bun, or Node — set the command in `../hooks.json` to match
your toolchain (copy/paste one):

```jsonc
// Deno (default) — --allow-read lets it read the allowlist file
{ "command": "deno run --allow-read .cursor/terminal-guard/guard.ts" }

// Bun
{ "command": "bun run .cursor/terminal-guard/guard.ts" }

// Node 23.6+ (native TS) — or use `npx tsx .cursor/terminal-guard/guard.ts`
{ "command": "node .cursor/terminal-guard/guard.ts" }
```

## Set the allowlist

The allowlist lives **once**, shared by every script-hook environment, in
[`.agent/terminal-guard/allowlist.json`](../../.agent/terminal-guard/allowlist.json). It
ships **deny-all** (empty `allow`). To permit your toolchain's read-only checks, copy
entries from `examples` into `allow`:

```json
{
  "allow": ["deno check", "deno lint"],
  "examples": ["deno check", "deno lint", "tsc --noEmit", "eslint", "biome check"]
}
```

Rules for entries (full spec in the policy doc):

- Each entry is a **literal command prefix** — `deno check` also permits `deno check src/`.
- A command with any shell metacharacter (`; & | > < ` `` ` `` ` $ ( )`) is **always
  denied**, even if it starts with an allowed prefix (`deno check && rm -rf x` → denied).
- Only add commands that pass **both gates**: read-only *and* with no internal-tool
  equivalent. Type-check and lint qualify; file reading, web fetch, tests, and anything
  that mutates or installs do not.

## Limits

- **Hard enforcement, full policy** — Cursor can run the engine, so this adapter blocks
  exactly per the two-gate test. No approximation.
- The guard only governs the **agent's shell tool**. It does not (and should not) touch
  the integrated terminal you type into yourself.
- Cursor also exposes Settings → Agents → Approvals & Execution (run modes + allow/deny
  lists). You can leave those at defaults; the hook enforces regardless. If you set a
  Cursor *denylist* too, it takes priority and can only tighten, never loosen, this guard.
