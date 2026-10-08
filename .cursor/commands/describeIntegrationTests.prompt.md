# Task: Write the integration test descriptions in each node

Read `docs/agents/index.md` and every topic it points to before doing anything else. Follow the work loop (`docs/agents/loop.md`) and the precedence order (`docs/agents/precedence.md`) every turn. 

Your job is to *author the integration elements of each node's interaction spec, and the integration test elements that embody them*. It is NOT to write tests, read the implementation, compute values, or redesign anything. Write every integration test description to comply exactly with the rules, following `docs/agents/workplan-structure.md` and the test rules (`docs/agents/tests.md`, `integrationTest.md`, `mocks.md`).

The workplan is the author's complete instruction to the implementer. For each test it states:
- the contract;
- the arrangement;
- the single call;
- the assertion.

The implementer then builds tests instead of inventing them. Everything you need is in the node text and the rules: each node's `deps`, `context_slice`, `interaction.spec`, mock, and test elements, and the integration entries recorded in the nodes of its direct dependencies.

Work on the provided workplan only. You will read the first indicated node, read the rules, audit the node content against the rules, explain to the every test that is required by the node and the rules, write those test descriptions to the node, review your work against the node and the rules, correct it to fix any deviations, and halt. 

## The procedure

Integration elements. Integration obligations travel from producer to consumer through the interaction spec (`docs/agents/workplan-structure.md`, "The interaction spec carries the integration elements"). This node answers what its direct dependencies recorded and records what its own boundary adds:
* Read the integration entries of every direct dependency, from the dependency nodes named in this node's `deps`. Those nodes are the only other nodes you read.
* Disposition every one of those entries in this node's spec: absorbed, citing the private integration test element of this node that proves it; or carried, citing the entry by name, restating it only where this node transforms it, and stating its route and the outer-edge collaborators on that route.
* Record this node's own entries. Each states, in this node's own terms, the condition and outcome, the input variation that separates a pass from a fail, and the failure or edge that must survive. Every entry is written from this node and what is beneath it.
* Declare the private surface: the chain of real functions, the outer-edge collaborators that are mocked, and the observable result. Declare the public surface only when this node's boundary is a public entry (`docs/agents/boundaries.md`).
* Describe the private integration test element, `integration_test.rs` in the module directory, when this node completes an in-crate chain: one block per own entry and per absorbed entry. It is never a unit test file, and no integration block goes in the unit test element. 
* Describe the public integration test element, `[module]_integration_test.rs` under the crate's `tests/`, only when this node's boundary is a public entry: one block per entry on the public surface. It is outside the crate so that it can only test the public API. Do NOT whine about this or try to work around it. Use it. 
* The entire point of the public API approach is to force the author to set up scenarios that match how a public user will reach the behavior. Every entry on the public surface gets a scenario that triggers its route through the public API. 
* Do not complain that you need private interface members or values for a public test - you do not. The problem you are having is that you're fighting how the integration test works instead of accepting it. A public entry that cannot be observed through the public API is a discovery (`docs/agents/discovery-halt.md`). 

## The surface of the node you update

This task edits exactly these parts of the node, so that the carry-forward works, and no other part (`docs/agents/workplan-structure.md`, "What the integration flow edits in a node"):

- the interaction spec's own entries, callee dispositions, routes, and private surface;
- the interaction spec's public surface, when the node's boundary is a public entry;
- the private integration test element, when the node completes an in-crate chain;
- the public integration test element, when the node's boundary is a public entry.

Every other element of the node is outside this task.

## The method, per node, in order

- **Read the node.** Read its `deps`, `context_slice`, `interaction.spec`, everything *before* the unit test portion, and the integration entries in the nodes of its direct dependencies.
- **Read the mock, test, and integration test rules.**
- **Disposition and record:** disposition every direct dependency entry, record this node's own entries, and declare the private surface and, for a public entry, the public surface.
- **Describe the integration test elements:** the private integration test element and, for a public entry, the public integration test element. Each block states the contract (transcribing an entry), the arrangement naming the entry's variation, the single call, the assertion, `Boundary`, and `Mocked`. Report the enumeration: every entry and the disposition or block that answers it.
- **Move to the next node.**

## Do not

- Justify to yourself that you already read the rules once, so you don't need to read them again.
- Read the entire workplan at once. 
- Narrow your scope or change your objectives from the scope and objective of writing the integration elements for the node you're working on.
- Read source files, run code, or compute vectors or values. If an expectation needs a value, name its source. Never derive it.
- Reference state, later nodes, later workplans, or "where a chain closes later". The implementer of this node doesn't care what happens later. No no-op lines or justification for why something "is not". You are not litigating, you are not giving history lessons. You are stating requirements for the implementer to obey exactly. 
- Change a design to make a test easier. Consumers are written against the family's traits and never know which concrete serves them. Never make a consumer, or a consumer's mock, aware of a concrete. A factory's own contract is admission and construction, proven against the official consumer mock.
- Ask for direction on anything the rules already settle. 
- A producer written in deps order is unused until its consumer node follows, so unused-item warnings against work that isn't performed yet is not a conflict.
- Treat a question or comment from the user as an instruction to edit. Answer it and halt.
- Number anything, add history, or narrate changes. The workplan states what is.
- Read other tests or workplans for "examples". The nodes named in `deps` are read for their integration entries and nothing else.
- Defer a disposition, restate a dependency's entry instead of citing it, or write an entry about a consumer or a later node (`docs/agents/workplan-structure.md`). 
- Pretend that you're unable to follow the rules for reasons you make up to justify not following the rules. 

Do not do anything other than this work, exactly as described.
