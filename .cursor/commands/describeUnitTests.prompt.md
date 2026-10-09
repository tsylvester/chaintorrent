# Task: Write the unit test descriptions in each node

Read `docs/agents/index.md` and every topic it points to before doing anything else. Follow the work loop (`docs/agents/loop.md`) and the precedence order (`docs/agents/precedence.md`) every turn. 

Your job is to *author unit test elements that embody the interaction spec and the unit test rules*. It is NOT to write tests, read the implementation, compute values, or redesign anything. Write every unit test description to comply exactly with the rules, following `docs/agents/workplan-structure.md` and the test rules (`docs/agents/tests.md`, `unitTest.md`, `mocks.md`).

The workplan is the author's complete instruction to the implementer. For each test it states:
- the contract;
- the arrangement;
- the single call;
- the assertion.

The implementer then builds tests instead of inventing them. Everything you need is in the node text and the rules: each node's `deps`, `context_slice`, `interaction.spec`, mock, and test elements.

Work on the provided workplan only. You will read the first indicated node, read the rules, audit the node content against the rules, explain to the every test that is required by the node and the rules, write those test descriptions to the node, review your work against the node and the rules, correct it to fix any deviations, and halt. 

## The procedure

**Unit test.** It proves only the function its node writes:
- Every collaborator is replaced by that family's official mock: the family's mock concrete, reached through its factory by configuration and never named. A test needing another outcome reaches it by input, or by a failure mode the selecting configuration names, as `mocks.md` states.
- It never drives an outside function through a test-local consumer or stub; a test's consumer exists only to receive a family's types and make the single call to the subject (`mocks.md`).
- Act is the single call to the function under test, and no other function of the module runs in the block. Where the subject receives its types from a family's factory, the single call to the subject is made inside the test's consumer, which the factory call runs with configuration selecting the mock concrete (`unitTest.md`, `mocks.md`); the subject is the only function of its module that runs.
- One behavior per test.
- It never proves an integration entry of the spec. A block that runs more than the subject is not a unit test; integration entries belong to the integration test elements (`docs/agents/integrationTest.md`).
- Fixtures come only from builders and invalidators.
- The test file holds only imports, contract headers, test blocks, and assertions; a test's consumer is part of its block. Constants, fixtures, and helpers belong in the mock file.
- An error is asserted whole.
- The expectation is stated independently of the arrangement. A value a mock or builder supplied that the subject only relays is never asserted as if it proved the subject.

## The method, per node, in order

- **Read the node.** Read its `deps`, `context_slice`, `interaction.spec`, everything *before* the unit test portion.
- **Read the mock, test, and unit test rules.**
- **Describe a test for every branch and provable behavior:**
   - For every unit-test block: the one function under test, the single act, each collaborator and the official mock that replaces it, and where the expectation comes from. 
- **Update the mock section if required:** Unit tests rely on mocks. If a mock is missing, update the mock section to describe it. 
- **Write the unit test description to match the function description and requirements from the rules.**
- **Move to the next node.**

## Do not

- Justify to yourself that you already read the rules once, so you don't need to read them again.
- Read the entire workplan at once. 
- Narrow your scope or change your objectives from the scope and objective of writing a unit test for the node you're working on.
- Read source files, run code, or compute vectors or values. If an expectation needs a value, name its source. Never derive it.
- Reference state, later nodes, later workplans, or "where a chain closes later". The implementer of this node doesn't care what happens later. No no-op lines or justification for why something "is not". You are not litigating, you are not giving history lessons. You are stating requirements for the implementer to obey exactly. 
- Change a design to make a test easier. Consumers are written against the family's traits and never know which concrete serves them. Never make a consumer, or a consumer's mock, aware of a concrete. A factory's own contract is admission and construction, proven against the official consumer mock.
- Ask for direction on anything the rules already settle. 
- A producer written in deps order is unused until its consumer node follows, so unused-item warnings against work that isn't performed yet is not a conflict.
- Treat a question or comment from the user as an instruction to edit. Answer it and halt.
- Number anything, add history, or narrate changes. The workplan states what is.
- Read other tests or workplans for "examples". 
- Pretend that you're unable to follow the rules for reasons you make up to justify not following the rules. 
- Trust anything the node says about unit tests, anywhere. The rules tell you over and over, prior reasoning is never trusted, and the rules supercede the workplan. Do not defer to the workplan. Follow the rules exactly as written. Do not use "but the workplan says" as an excuse not to do what the rules require you to do. 
Do not do anything other than this work, exactly as described.
