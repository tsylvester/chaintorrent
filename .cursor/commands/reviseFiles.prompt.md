In your next turn, you are going to begin the work loop against the section of the workplan the user will provide in the next message. That selection is your assignment.

**The selection is the node, and it is the whole scope.** How a delivered selection is treated — that it is the user's instruction and not ambient context, that it bounds the work, and what to do when it cannot support the instruction — is governed by `docs/agents/scope.md`, "Context the user hands you". Read it there.

In **this turn**, read `docs/agents/index.md` and all of the Process elements so you understand the basic rules of procedure for this repo.

This command overrides the "edit one file per turn" provision of `docs/agents/loop.md`. This command explicitly authorizes you to edit multiple files per turn *if and only if* you are following the *exact sequence described in this prompt*. Any deviation from this sequence obviates the "edit one file per turn" override and reinstates the sequence in `docs/agents/loop.md`. 

Specific to this command:

- The selection you will be given is a node. 
- "The user's selection includes the first line of the next node" is not a discovery, it is how selecting a section in a markdown file works. Do not report it, do not halt on it, do not reason that the user "must actually want me to read the next node instead". Focus on the first node you were explicitly handed, not the incidental first line of the next node. DO NOT read the next node "just to check". 

Before you reason about anything, read. This is a gate, not a formality:

- Read the first incomplete section of the node in full. It names the element you are evaluating.
- In `docs/agents/index.md`, find that element's row in the Implementation routing matrix, and read every Process topic and every Standards topic that row names. "Read the rules" means **all** of them — not the first file you open.
- Read the file the node references.

Then produce a **read manifest** — a short list of exactly what you read — before any analysis. Do not reason toward a solution until that manifest exists (`docs/agents/loop.md`).

You cannot begin to reason against the task you've been given until **after** you've read every relevant rule and file, and produced your read manifest to prove it. You will not reason the entire node, you will not reason every section. You will only reason for the file whose section you are actively working on. Reasoning forward of the file whose section you are working on is a failure. 

This is a canary: If you start to reason **before** you read every rule and every relevant file, the user will see your violation and be forced to restart. The only way to avoid being halted is to follow the procedure you've been given. Every user-initiated-halt here is a *failure*, and it is **YOUR** failure, for not complying with the required procedure.

Then, to complete your turn:

- Do not reorder or merge the node's steps (`docs/agents/tdd-ordering.md`). 
- You will complete each line one at a time, line by line, without batching work. 
- Obey precedence — user, then Instructions topics, then the workplan (`docs/agents/precedence.md`). Declare your mode and sign your work (`docs/agents/modes.md`, `docs/agents/traceability.md`).
- If the node element is informative but does not have an associated file, read the node, and take note of the items that are not yet satisfied. When you read forward node elements, you will satisfy these lines. Come back and mark them at the point they are satisfied. 
- If the node element has an associated file on disk, or is supposed to have an associated file on disk, compare the node element in the workplan to the state of its file on disk. 
- Walk the lines of the element in the workplan in order, read against the file the element names, and judge it against the topics that govern it (`docs/agents/linting-proof.md`, `docs/agents/modes.md`). 
- For each line in the workplan element that is not satisfied on disk, edit the file in the exact way you are instructed so that the line is satisfied, then mark the bullet in the workplan node complete. 
- Only when every line of the element is satisfied **on disk** is the element complete. 
- When every single line in the node is satisfied by its requirement on disk, mark the node satisfied. 
- ***DO NOT MOVE ON TO THE NEXT ELEMENT BEFORE THE ELEMENT IS COMPLETED ON DISK AND MARKED COMPLETED!*** 
- Select the next incomplete element in the node, and restart this command from "Before you reason about anything, read."
- When the node is satisfied, report to the user that it is completed and you are ready for the next node whose files need revised. 

Do not edit any file other than the exact file named by the section in the node you were given to satisfy, unless the user explicitly tells you otherwise. Do not output code in chat.

Follow `docs/agents/loop.md`, `docs/agents/scope.md`, and `docs/agents/precedence.md`.