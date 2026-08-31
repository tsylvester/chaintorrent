// Claude Code `PreToolUse` terminal guard.
// Self-contained: the two-gate engine is inlined (canonical spec:
// docs/agents/terminal-policy.md). Reads only the shared allowlist DATA at
// .agent/terminal-guard/allowlist.json. Setup, allowlist how-to, and limits:
// .claude/terminal-guard/README.md
//
// Portable across Deno, Bun, and Node (via `node:` builtins). Pick the runtime in
// .claude/settings.json; default is `deno run --allow-read`.

import { readFileSync } from "node:fs";
import process from "node:process";

// --- two-gate engine (canonical: docs/agents/terminal-policy.md) --------------
const METACHAR = /[;&|<>`$()\n\r]/; // composition/redirection/subshell → never eligible

function isAllowed(command: string, allow: string[]): boolean {
  const c = command.trim();
  if (!c) return false; // empty → deny (fail closed)
  if (METACHAR.test(c)) return false; // composed → deny
  return allow.some((entry) => {
    const p = entry.trim();
    return p.length > 0 && (c === p || c.startsWith(p + " "));
  });
}
// ------------------------------------------------------------------------------

function loadAllow(): string[] {
  try {
    const url = new URL(
      "../../.agent/terminal-guard/allowlist.json",
      import.meta.url,
    );
    const data = JSON.parse(readFileSync(url, "utf8"));
    return Array.isArray(data.allow) ? data.allow : [];
  } catch {
    return []; // unreadable allowlist → deny everything (fail closed)
  }
}

const REASON =
  "Terminal commands are blocked by the repo's terminal guard. Allowed: only the " +
  "read-only static-analysis commands bound in .agent/terminal-guard/allowlist.json. " +
  "Use the internal Read/Grep/Glob tools for files and the fetch tool for web " +
  "references. Tests, running the app, installs, and mutations are denied. If you truly " +
  "need a command, stop and ask the user to run it. Policy: docs/agents/terminal-policy.md";

async function main() {
  let command = "";
  try {
    process.stdin.setEncoding("utf8");
    let raw = "";
    for await (const chunk of process.stdin) raw += chunk;
    const input = JSON.parse(raw);
    command = String(input?.tool_input?.command ?? "");
  } catch {
    command = ""; // unparsable input → deny (fail closed)
  }

  if (isAllowed(command, loadAllow())) {
    process.exit(0); // permit: no opinion → defer to normal permission handling
  }

  // Deny — belt-and-suspenders. Both are deny signals; a version honoring either (or
  // both) blocks, and no combination can yield "allow" (see terminal-policy.md).
  process.stderr.write(REASON + "\n"); // exit-code-2 consumers
  process.stdout.write(
    JSON.stringify({
      hookSpecificOutput: {
        hookEventName: "PreToolUse",
        permissionDecision: "deny",
        permissionDecisionReason: REASON,
      },
    }),
  ); // permissionDecision consumers
  process.exit(2);
}

main();
