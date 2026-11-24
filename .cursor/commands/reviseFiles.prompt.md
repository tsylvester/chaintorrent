# Revise files against a node

You are handed a workplan node with this command. The node is already implemented and has been revised. Your job is to bring its files on disk into line with the node as it is written now. The node is the whole scope; `docs/agents/scope.md`, "Context the user hands you", governs how a handed selection is treated. If the selection includes the first line of the next node, that is how selecting a markdown section works. Ignore that line, and do not read the next node.

## First

1. Read `docs/agents/index.md` and every Process topic it lists. Acknowledge them in one line.
2. State your scope, as the canary: the node's path, and the file of the first element you will work. A response whose first substantive output is not this statement is discarded.

## The rule

The node's line is the whole instruction. Make the edit the line states, in the file the element names, and nothing else. Content in the file that the node does not mention stays exactly as it is. The only removal you may make is one a line names: this item, from this file. In this command, this rule replaces the infill and completeness directives of `scope.md` and the Standards topics. You do not add what a standard requires and the line does not state, and you do not remove what the line does not name.

## The loop

Work the node's elements in order, one at a time:

- Read the file the element names. If it does not exist and a line requires it, create it.
- Walk every line of the element, marked or not. The mark is a claim; the file is the fact.
   - A line the file does not meet: make the edit, then mark the line `[✅]`.
   - A line the file meets: mark it `[✅]` if it is not already.
   - A line marked `[✅]` that the file does not meet: make the edit. It stays `[✅]`.
- Lint the file. Errors that a later element of this node resolves are expected; leave them. Report any other error, and continue.
- When every line of the element is `[✅]`, mark the element `[✅]` and move to the next element.

Planning sections (`objective`, `role`, `module`, `deps`, `context_slice`, `construction`, `directionality`, `requirements`) have no file. Read them as context for the file elements. Mark their lines `[✅]` when every file element of the node is `[✅]`.

When every line of the node is `[✅]`, mark the node's top-level line `[✅]`. Then report that the node is complete, declare your mode, and sign.

## What you may edit and read

- You may edit the file the current element names, and the checkboxes of this node in the workplan. Nothing else in either. This overrides the one-file-per-turn rule in `docs/agents/loop.md` for this command.
- You may read the file the current element names. You may read another file only when a line names it and its edit cannot be written without that file's exact text. Then read only the named symbol.

## Halt only on these

- A line's target text or symbol does not exist in the named file, and the line does not say to create it.
- Two lines of the node contradict each other.
- A line cannot be met without editing a file it does not name.
- An edit result shows content destroyed. Follow `docs/agents/output.md`.

On a halt, report the line, the file, and what blocks it, following `docs/agents/discovery-halt.md`. Everything not listed here is not a halt: continue.

## If you notice yourself thinking any of these, stop and make the current line's edit

- "Let me check the consumers / the blast radius / the rest of the crate first." Not your concern.
- "This file has content the node doesn't mention; I'll remove it." It stays.
- "A later node will write this back, so I can remove it now." You are reasoning about other nodes. Stop.
- "The standard says this element also needs X." Only the line decides.
- "loop.md says halt after one file." This command overrides that.
- "Let me read ahead to the next element." Finish this one on disk first.

Do not output code in chat. Obey precedence (`docs/agents/precedence.md`): the user, then the Instructions topics, then the workplan.