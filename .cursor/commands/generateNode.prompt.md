Follow `docs/agents/loop.md`, `docs/agents/scope.md`, and `docs/agents/precedence.md`. 

You will rely on `docs/agents/workplan-structure.md` for this task. It explains the structure of a node and cites each rule that each element of the node structure relies on. Read the entire document, then read all of the rules that are cited by each element of the node. Only once you have read all the preparatory rules, continue to the next step. 

The planning documents and the process topics are already read; read now, from the workplans by ticket name, the nodes of the node's direct dependencies and the nodes that author each type or construction its dependency-map row names. Code on disk is not a source. After having read all of the relevant rules on workplan nodes and their elements, you will write one node into its placeholder, or after the last node if no placeholder is named. 

Before writing, establish from those sources: the statements the node must satisfy, with their locations; every type, function, record, and construction an upstream node already authors that the node needs, with its owning node; and every item the node must declare because no upstream node provides it. If a requirement the node must meet is absent from every source, or two sources require incompatible things of this node, write nothing and give the discovery report (`docs/agents/discovery-halt.md`).

Do not read any node in any workplan other than the direct dependencies. Do not read other nodes "for comparison", "for standards", "for patterns", "for examples" or any other justification you may try to reason yourself into. The rules and resources you are handed and pointed to are the total surface you require to write the node. No excuse is valid. No reasoning otherwise is valid. Reading any node other than the direct dependencies of the node you are writing is an immediate failure. 

You have now reached explicit instruction to edit the workplan to add the node you have generated. 

Write every element in the template order of `docs/agents/workplan-structure.md` and the element order of `docs/agents/tdd-ordering.md`. Every element states what its sources state. Use every upstream item by name from its owning node; never restate, rebuild, rename, or re-declare it. Declare only what no upstream node provides. Disposition exactly the dependency entries that pass through one of the node's functions or end at one of its tests. Resolve every path, symbol, and import from the owning node. Do not emit the node in chat.

Review the `docs/agents/workplan-structure.md` requirements again, and every rule that relates to a workplan element. Check the node you just wrote against the planning documents, the workplan tickets, the nodes of the direct dependencies, and all of the rules. Revise the node so that it complies with all of the rules for workplans and their elements, and reflects all of the planning document requirements, the workplan tickets, and the direct dependencies. 

Once you are certain the node is compliant with all its source material, report the upstream items the node uses, with their owning nodes, and the items it declares, with the source that requires each.