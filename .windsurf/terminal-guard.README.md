# Windsurf / Devin terminal guard

Carries [docs/agents/terminal-policy.md](../docs/agents/terminal-policy.md) into Windsurf
(now **Devin Desktop**, after Cognition's 2025 acquisition). This is an **instruction-only**
environment: no per-command hook, no sandbox knob — the policy is a rule the agent reads,
enforced by the model, not the tool.

## Activate

- **Windsurf / Devin Desktop:** [`rules/terminal-policy.md`](rules/terminal-policy.md) is an
  always-on rule (`trigger: always_on`). It loads automatically. If your version uses a
  single `.windsurfrules` file instead of the `.windsurf/rules/` directory, paste the same
  pointer there.
- **Devin (cloud agent):** rules files are not read the same way — add the policy to the
  workspace **Knowledge** / playbook so every session carries it.

## "Set the allowlist" — there is none to enforce

Nothing here hard-blocks a command, so there is no allowlist mechanism. The allowlist is
purely descriptive: what the [policy](../docs/agents/terminal-policy.md) says the agent may
run (read-only type-check and lint) and nothing else. Compliance depends on the agent
honoring the instruction.

## Limits

- **Softest tier.** No enforcement — model-dependent adherence only. Do not rely on this to
  stop a destructive command; it is a strong instruction, not a guardrail.
- For hard enforcement, use a script-hook environment (Claude Code or Cursor), which runs
  the two-gate engine and blocks before execution.
