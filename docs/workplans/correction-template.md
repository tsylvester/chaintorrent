* `[ ]`   `incomplete item`

* `[✅]`   `complete item`

# [Correction Plan]

## [Purpose of Correction]

## The [Correction to be Made]

## [Producers of the Item to be Corrected]

Each producer below is corrected before any node that consumes it.

- **[Producer1].** 
- **[Producer2].** 

## [Order of the Required Correction]

The specifications precede the planning documents, and the planning documents precede the workplans, because each layer states what the layer below it cites. Within the specification layer and the planning layer, a document is corrected after every document it cites. Within the workplans, the pre-WBS sections precede the nodes, and nodes are corrected in Work Breakdown Structure order, so every corrected node's producers are corrected before it. Each document and each node is corrected once, for every coupling it carries.

A node marked complete whose content changes returns to unbuilt, and its files are rebuilt from the corrected node. A node the corrected dependency map adds inside the span the workplans already author is inserted at its dependency position. A node whose content no coupling touches is unchanged.

## Document Writing Procedure

**How to work.**
- Pick up the first document that has no sub-items.
- Read the document.
- At each instance of a string or section that hosts an instance of the problem, create a new bullet under the document, and add an anchor to the start and end of the string or section that needs corrected. 
- Then give instructions for how to correct the string or section to comply with the correction to be made. 
- Increment through the document until all instances of the problem have been bulleted and have a method of correction.
- Review the document again to ensure that the bullets listed for the document are comprehensive, coherent, complete, and correct. 
- Halt and ask the user to review your work. 

## Correction Procedure

**How to work.**
- Find each node by its heading, the backticked node name at the start of a top-level bullet, in the document.
- Do not read forward. You will not benefit from swamping your context by trying to read everything at once. Read incrementally as you go. 
- Edit only the bullets named below and leave every other bullet as it is.
- Elements in a workplan node follow the status marking rules from `workplan-structure.md`.
- Where an item says "replace X with Y", X is quoted from the node exactly; find it and substitute Y word for word.
- Keep each bullet on one unbroken line.
- Put no rule names or explanations into node text. Write the text the item gives.
- Complete one item at a time. 
- In this document, mark each item completed when the relevant line in the workplan is updated. 
- Do not batch edits to any document. One edit per document per item. No lumps, blobs, or batches. 
- Move to the next item.
- Do not try to apply multiple edits from this document at once. Work one by one.  
- If you make a discovery, you must explain the discovery to the user following `discovery-halt.md`. 
- Proofs are explicitly labeled. If you haven't reached a labeled proof marker at the end of a group, you haven't completed the proofs. 
- Halt when you reach an item marked "Halt", when you complete the entire set of proofs for a group, or when you make a discovery.

## `docs/research` layer

* `[ ]`   **`[someDoc].md`.** [has some error]


## `docs/project-planning` layer

Documents are corrected in the order the planning pipeline produced them.

* `[ ]`   **`feature-spec.md`.** 

* `[ ]`   **`technical-approach.md`.**

* `[ ]`   **`risk-register.md`.** 

* `[ ]`   **`non-functional-requirements.md`.** 

* `[ ]`   **`dependency-map.md`.** 

* `[ ]`   **`product-requirements.md`.** 

* `[ ]`   **`tech-stack.md`.** 

* `[ ]`   **`system-architecture.md`.**

* `[ ]`   **`technical-requirements.md`.**

* `[ ]`   **`milestones.md`.**

* `[ ]`   **`master-plan.md`.** 

## Workplan layer

Each entry names its nodes in Work Breakdown Structure order and, per node, the elements that change. An element not named is unchanged. Within a named node, every restatement of a changed item in `objective`, `role`, `module`, `deps`, `context_slice`, and `requirements` is corrected with it.

* `[ ]`   **`[Workplan Name].md`.**
  * `[ ]`   `[Node to be fixed]`
    * `[ ]`   `[Node subitem details]`: `someItem` [isFixed].

## Completion check

Each proof is a search of the specification layer, the planning layer, and the workplans, excluding this document, or a comparison between them. The scope is complete when every proof below passes.

* `[ ]`   **[Some Checkable Proof].** [Is explained how to check it and what to look for].
* `[ ]`   **[Continue Proof Enumeration].** [Describe every means required to prove the problem is solved as an individual line]. 


