Validate one authored node in Reviewer mode (`docs/agents/modes.md`). Edit nothing. Sources are the planning documents, the topics, and the workplan nodes; code on disk is not a source. Treat the node as untrusted: re-read it, its dependency-map row, its milestone, the specification rows they cite, and the nodes of its direct dependencies, from disk, now.

A check passes only on a complete enumeration that proves it. A finding exists only when it cites both the node line it concerns and the source or topic text it fails, each quoted with its location; a suspicion without both is not reported. "No EO&D detected" is a valid conclusion.

Run every check below. For each, give its enumeration, then its findings.

**Grounding.** Every line of the node traced to the statement that requires it: a specification row, the dependency-map row, the milestone scope or exit, an upstream node, or a topic. A line that traces to nothing is a finding. A source statement the node must satisfy that no line satisfies is a finding.

**Reuse.** For every type, function, record, constant, and construction the node declares: the workplan nodes searched for an existing owner, by name and by the shape it holds, and the result. An item an upstream node already authors is a recreation and a finding. For every item the sources say the node works over or receives: the line that uses it by name from its owning node. One left unused is a finding.

**Types.** Every object the node's function receives, constructs, emits, or returns, each layer to the primitive: its named type and its owner. An object with no named type, a type in the wrong owner, or an error that cannot be printed and compared is a finding.

**Form.** The node against `docs/agents/workplan-structure.md`: one source file; the template order; each omitted element and the reason its concern is absent; the canaries; every line unchecked; no numbering, history, or deferred decision. Each departure is a finding.

**Mocks.** Every symbol the node's interface owns and the mock `docs/agents/mocks.md` requires of it, or the rule that requires none. A missing or extra mock is a finding.

**Branches.** Every branch of the interaction spec, condition to outcome, paired with the unit block that proves it, or with the statement that no input and no failure mode of the official mocks reaches it and why. Every block's contract header, its arranged variation, and its expectation, shown to be stated independently of the arrangement. A branch with neither block nor reason, or a block that is incidental or tautological, is a finding.

**Integration in the crate.** Every entry of every direct dependency's node, marked as passing through one of this node's functions, ending at one of its tests, or neither. Each of the first two paired with its disposition and, if absorbed, the private block that proves it; each own entry paired with its private block. An entry that passes through and is not dispositioned, or a disposition with no block or route, is a finding.

**External surface.** Whether the node's boundary is a public entry, from the crate barrel the node states. If it is, every outcome an outside caller can observe through the public surface and the official mocks, each paired with the public block that proves it. An observable outcome with no block is a finding.

Report each finding with its location, the quoted source or topic it fails, its nature, its severity, and the change that brings the node into compliance. Then halt. State the node complete, correct, and compliant only when every enumeration is complete and no check has a finding.
