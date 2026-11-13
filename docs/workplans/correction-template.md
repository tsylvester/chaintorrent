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


