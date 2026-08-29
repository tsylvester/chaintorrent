---
trigger: always_on
---

# Terminal policy (pointer)

Follow [docs/agents/terminal-policy.md](../../docs/agents/terminal-policy.md) and the
index at [docs/agents/index.md](../../docs/agents/index.md).

Terminal command execution is **denied by default**. Only the read-only static-analysis
commands listed in the policy's allowlist may run — type-check and lint. File reading and
search go through the internal tools (never `cat`/`grep`/`find`); web references go through
the fetch tool (never `curl`/`wget`); tests, running the app, installs, and any mutation
are denied.

This environment enforces by **instruction only** — there is no hard pre-execution block.
If a denied command is genuinely needed, stop and ask the user to run it.
