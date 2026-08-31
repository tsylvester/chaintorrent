# Codex terminal guard

Approximates [docs/agents/terminal-policy.md](../docs/agents/terminal-policy.md) in
OpenAI Codex. Codex is a **declarative** environment: it has no per-command allowlist
script, so it **cannot run the two-gate engine**. It enforces a *posture* through
`sandbox_mode` + `approval_policy` instead.

The policy **text** still reaches Codex as instruction: Codex reads `AGENTS.md`, which
points at `docs/agents/index.md` → `terminal-policy.md`. That is the soft floor. The
config below is the declarative hardening on top of it.

## Activate

Codex config is **user-global**, not repo-committed. Copy [`config.sample.toml`](config.sample.toml)
into `~/.codex/config.toml`, or save it as a named profile (`~/.codex/strict.config.toml`)
and run `codex -p strict`. Each teammate installs it themselves — the repo can only ship
the sample.

## "Set the allowlist" — you can't, precisely

Codex has no command allowlist to bind. The two knobs are:

- `sandbox_mode = "read-only"` — hard-blocks writes, network, and destructive ops.
- `approval_policy = "untrusted"` — auto-runs only what *Codex* judges known-safe;
  everything else (including your `deno check` / `deno lint`) prompts **you** for
  approval. Approve the read-only checks when asked.

So the effective allowlist is Codex's own safe-read classifier plus your approvals — not
the exact two-gate list. That precision is only available in the script-hook environments
(Claude Code, Cursor).

## Limits

- **Coarser than the policy.** No per-command allowlist; `read-only` blocks mutation and
  network hard, but the fine line between allowed static analysis and denied execution is
  drawn by Codex's classifier + your approval, not by the two-gate engine.
- **Not repo-enforced.** Because config lives in `~/.codex/`, a teammate who skips the
  copy step runs unguarded — the `AGENTS.md` instruction floor is their only backstop.
- Do **not** relax `sandbox_mode` to `workspace-write` or `danger-full-access` to make
  commands run — that defeats the policy. If a command is genuinely needed, ask the user.
