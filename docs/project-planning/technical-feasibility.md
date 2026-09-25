<!-- Template: antithesis_feasibility_assessment.md -->
# Summary

Draft, 2026-09-23. An assessment of whether the ChainTorrent MVP, as specified in [cryptography.md](../research/cryptography.md), [MVP Scope](../research/MVP%20Scope.md), and [MVP Application Requirements](../research/MVP%20Application%20Requirements.md), and as planned in the [technical approach](technical-approach.md) and [dependency map](dependency-map.md), can be built. It draws on the [risk register](risk-register.md), the [non-functional requirements review](non-functional-requirements.md), and the [business case critique](business-case-critique.md) rather than repeating them.

**Verdict.** The MVP is technically feasible. Every cryptographic component is standard and the composition is closed at the research level with named assumptions; every engineering component is conventional Rust, Solidity, and platform work with no novel technique required; and the design's adapter discipline means the launch network and the custody implementation are choices among existing options rather than inventions. What is not established is delivery feasibility: the surface is wide, the completion boundary is strict, and no team, timeline, or budget exists to set against either. The assessment therefore separates the two. Technical feasibility is high. Delivery feasibility is undetermined until the cryptographic validation harness has been built and its throughput measured, which is why that grouping is mapped at ticket resolution together with the milestones that depend on nothing it measures.

The findings that shape the rest: the harness is the cheapest, most self-contained, and most informative early unit of work, and it is also shipped code rather than a throwaway, because its delivery verifier is the contract the registry calls. Platform breadth, not cryptography, is the largest delivery risk, because signed installation, service lifecycle, credential stores, and firewall handling on every supported OS and architecture are each a body of work with no shared shortcut. And the last third of the requirements, publishing, claims, and the transaction flow proof, depends on decisions and reviews external to engineering, so it is where the schedule will slip if those are not started in parallel with the harness.

# Constraint Checklist

## Team

**Known.** No document names a team, a headcount, or an allocation. The repository's agent process rules assume a workflow in which an agent authors and implements one file per turn under a workplan, with the user as the highest authority, which implies a small human team directing agent implementers rather than a large engineering organization.

**Required skills, by the components that need them.** Applied pairing-based cryptography in Rust, for the harness, the KEM, the envelope, and the proof, with enough depth to implement encodings and subgroup checks that match EVM precompiles exactly. Solidity with precompile-level verification, for the delivery verifier and the contract suite, plus the discipline to keep it bit-for-bit with the Rust reference. Rust systems engineering for durable jobs, IPC, storage, networking, and a BitTorrent-compatible transport. Platform packaging and service lifecycle on every supported OS, which is the skill most often underestimated. Tauri and a thin TypeScript extension. Test infrastructure for a demonstration harness that creates participants, wallets, chain state, and failures. Outside engineering: an external cryptographic reviewer, and legal counsel for the license, the eligibility restatement, and the regulatory questions.

**Assessment.** The skill set is unusual in combining applied cryptography with platform packaging, and the two rarely sit in one person. The harness grouping needs only the cryptography; the onboarding grouping needs only the packaging. That separation is a scheduling opportunity: the two can be staffed and run in parallel once the protocol core exists. Feasibility with respect to team is undetermined until the team is named, and the assessment recommends naming it against the skill list above rather than against a headcount.

## Timeline

**Known.** No document states a date, a duration, or a deadline. The workplan states an order and the dependency map states the foundation and harness tickets and their starting points.

**What can be said without dates.** The foundation and harness groupings, with the hashing, signature, and swarm milestones beside them, are the work known at ticket resolution, and the harness's throughput, tickets closed per unit of time under the repository's discipline, is the calibration the project can obtain. Everything after it is sized in sprints, epics, milestones, and objectives precisely because dates for them would be guesses. The critical path through the groupings runs harness, then registry contracts, then First Finder ingest, then package host, then credential delivery, then publisher path, transaction proof, and escrow claim; the swarm tickets and the identity epic run off the critical path. The demonstrable milestone, an ordinary `npm install` resolving against the swarm, sits after credential delivery and is the earliest point at which the project has something to show and to measure.

**Assessment.** Feasibility with respect to timeline is undetermined and cannot be determined from the documents. The recommendation is to treat the harness grouping as the calibration unit: measure its throughput, then re-map the remaining protocol core milestones to tickets and produce the first evidence-based estimate. Any timeline stated before that is not an estimate.

## Cost

**Known.** No cost model exists and no figure appears in any document. The business case names the spend categories: engineering, cryptographic review, operating subsidy for the relayer, infrastructure for the seed host and site and test chain, and legal.

**What the sources do bound.** Per-install operating cost is measurable from the first free acquisition: relayer gas per mint and per identity binding, and swarm bandwidth. Delivery-verification gas per mint and transfer is measured by the harness on both curves and is the largest single input to the relayer's cost. The grant pool that funds the relayer is unsized and grows with adoption, since every new identity costs a binding transaction and every free mint verifies a proof on chain.

**Assessment.** Engineering cost is undetermined for the same reason as timeline. Operating cost becomes measurable early, at the harness for gas and at the first dogfood run for everything else, and the assessment recommends sizing the grant pool from those measurements rather than in advance. The one cost that can be estimated now by asking is external cryptographic review, which has a market rate and a lead time and should be quoted during the harness grouping.

## Integration

**External systems the MVP must integrate with, and the feasibility of each.**

| System | Integration | Feasibility | Note |
| --- | --- | --- | --- |
| npm registry | Read-only ingest of immutable tarballs and integrity metadata; serving npm's own registry protocol locally | High | Both are documented public protocols; enterprise proxies do the same |
| npm, pnpm, yarn, bun as clients | Registry redirect through configuration | High | Standard configuration surfaces; scoping across user, project, workspace, and proxy cases is tedious rather than hard |
| Visual Studio Code extension host | Thin TypeScript shell | High | Conventional |
| Base and its pairing precompiles | Contract suite; BLS12-381 through EIP-2537 as the primary form; BN254 retained | High | Both precompile sets are live on Base; pricing is measured at the harness |
| Wallets and account abstraction | EIP-1193, WalletConnect, EIP-712, ERC-4337 paymaster on Base | High | Standards exist; the identity is a contract account under ERC-1271 with a device signer set, and the sponsorship mechanism is chosen at the relayer milestone under LC-13 |
| OS service managers, credential stores, firewall controls | Signed installation and daemon lifecycle | Medium | Conventional per platform; the breadth across platforms is the cost |
| BitTorrent clients | Compatibility transport; delegated seed host through a client's API | Medium | The compatibility adapter must carry legacy piece hashes and a client API varies by client |
| BLAKE3, Bao, AES, Ed25519, secp256k1, pairing libraries | Suite-constrained Rust wrappers | High | Mature libraries exist for all; precompile-matching encodings are the work |
| Test chain | Deployment of the verifier for the harness | High | Standard tooling |

**Assessment.** No integration requires an external party's cooperation or an undocumented interface.

## Compliance

**Known.** The [non-functional requirements review](non-functional-requirements.md) found compliance to be the least specified area. Four items sit on the release path and none has started: the ingest eligibility principle's precise restatement with its residual, the source-available license with a conformance clause, legal review of priced transferable entitlements and the paymaster, and confirmation of export constraints on a packaged cryptographic binary.

**Assessment.** None of these is a technical blocker and none is likely to be adverse for the MVP's corpus, which consists of packages their publishers distribute to the public at no charge. They are feasibility constraints in the sense that release cannot happen without them, and they have lead times that engineering does not control. The recommendation, stated in every planning document, is to start legal work in parallel with the harness.

# Findings

| Finding | Basis | Consequence |
| --- | --- | --- |
| **The cryptography is buildable from standard components.** Depth-one Boneh–Boyen KEM in Type-3 groups, ElGamal envelopes, generalized Schnorr under Fiat–Shamir, EVM pairing precompiles. Every operation is scalar multiplication, pairing, hashing, and a KDF. | Critical path; research notebook Q09 | No primitive of unknown existence or efficiency; the implementation risk is encodings, subgroup checks, and context binding, which the harness vectors are designed to catch |
| **Cost is unmeasured and the design is honest about it.** The by-formula estimates are favorable, roughly one percent sidecar overhead at 16 KiB groups depending on curve and scope and two pairings per group, but nothing is measured. | Critical path, Costs by formula; CD-07 | The harness precedes every node that encrypts; the latency budget is declared before measurement; feasibility of the interactive target is decided by measurement, not asserted |
| **The harness is shipped code.** Its delivery verifier is the contract the registry calls; its KEM, envelope, and proof files are consumed directly by credential delivery and identity. | Dependency map, Conflict Flags | The harness grouping is not throwaway; it must be written to production standard, and its ticket list is the honest size of the unit |
| **Platform breadth is the largest delivery risk.** The self-installation requirements, each proven on Windows, macOS, and Linux, plus service lifecycle, credential stores, and firewall handling. | SI-01 through SI-20; risk register R-04 | Staff the onboarding grouping separately from the cryptography |
| **The completion boundary forbids the usual shortcuts.** No mocked adapter, off-chain-only verifier, or manually prepared machine counts as completion. | Application Requirements, Completion Boundary | The demonstration harness and clean-machine test infrastructure are on the critical path, not optional tooling |
| **Custody and the escrow record's form are decided.** Custody is a local keystore under the OS credential store with version-headed blobs upgraded in place, the holder seed on root devices, and every other device admitted under its own key on the identity's contract account; the escrow record carries no maintainer commitment and no salt custodian exists. | Product requirements, Resolved Positions | The remaining open selections are the measured parameters, the license, and recovery UX |
| **Base fixes the curve's primary form, the paymaster, and the gas model.** | MVP Scope, Entitlement Ledger; product requirements | Both verifier forms are built; the harness measures both and confirms BLS12-381 against the budget |
| **Transfer is the hardest thing the MVP proves and the last thing it reaches.** The settlement boundary's failure mode is silent from the outside. | MVP Scope, What the MVP Does and Does Not Exercise | The transaction flow proof depends on the publisher path, delivery, and the relayer, and on external review before it may be priced; it is where the schedule will slip |
| **No code, no team, no timeline, no budget.** | Repository state; every planning document | Technical feasibility can be assessed; delivery feasibility cannot, until the harness grouping calibrates throughput |
| **The MVP validates the mechanism, not its load profile.** Install-once content crosses the attempt boundary rarely. | MVP Scope, What the MVP Does and Does Not Exercise | Feasibility for streaming content is not established by the MVP and is not claimed; decapsulation time per group is recorded as a baseline |

# Architecture

The architecture is assessed as stated in the [technical approach](technical-approach.md): three dependency rings with dependencies pointing inward, every external touchpoint behind an adapter with declared capabilities, composition validated at resolution and failing closed, an immutable deployment suite as the unit of cryptographic composition, and no service on the read path.

**Feasibility.** High. Each element is a known pattern. The ring structure is hexagonal architecture under another name; capability-declared adapters are ordinary interface design with a resolution-time check; the immutable suite is a versioning discipline. None requires a technique the team would have to invent. The cost of the architecture is upfront: every adapter family needs an interface, a capability declaration, and a resolution rule before its first implementation, which is why the dependency map's harness grouping begins with the tickets that own the interfaces. The benefit is that the network and custody choices are adapter choices and change nothing above them.

**One architectural property is not free.** The requirement that the Rust reference verifier and the deployed Solidity verifier agree bit for bit is a maintenance obligation for the life of the contract: every encoding, hash-to-scalar, and context-schema decision must be made once and mirrored exactly. That is feasible and it is how such systems are built, but it argues for generating the Solidity verifier's constants and test vectors from the Rust implementation rather than maintaining two hand-written copies.

# Components

Assessed per deployable application as listed in the Application Requirements. Feasibility is rated against whether a conventional implementation exists for each and what is unusual about this one.

| Component | Feasibility | What is unusual |
| --- | --- | --- |
| Cryptographic validation harness | High | Nothing; it is a benchmark driver over the KEM, envelope, proof, and a deployed verifier |
| Contract suite | High | Delivery verification through pairing precompiles in two forms; the sidecar-coverage rule on deployment registration; per-entitlement interval state with envelope digests. All expressible in Solidity; verification gas is the only unknown |
| Local daemon and package host | High | Single-instance machine service with authenticated IPC, durable jobs, and an npm-protocol endpoint; enterprise proxies and pnpm's store are prior art for the serving and caching halves |
| First Finder bootstrap | High | Foreground serve with asynchronous durable bootstrap; state-locked registration race with loser destruction. Conventional job engineering with a chain interaction |
| Swarm transport and seed host | Medium | The owned transport must be built; BitTorrent compatibility means carrying legacy piece hashes and driving external clients through varying APIs; Bao custody challenges against a delegated store are new but simple |
| Credential delivery and per-attempt authorization | High | The attempt rule is a state read plus local pairing work; delivery is the harness's prover and the contract's verifier; interval-end destruction is zeroization on a settlement event |
| Identity, wallet, and custody | Medium | The custody adapter's capability list, envelope keys in pairing groups, multi-device by device admission on a contract account without moving the seed, in-place upgrade verified against the chain, and recovery from chain history; conventional under the local keystore, with Matrix cross-signing, Keybase sibling keys, and Chromium's versioned credential encryption as prior art |
| Installation coordinator and shells | Medium | Signed installation, service lifecycle, reversible package-manager configuration, repair, update, and uninstall on every supported platform; each is conventional, the matrix is the cost |
| Relayer or paymaster | High | ERC-4337 where supported; rate limits and cost reporting are ordinary service work |
| Claim verifier | High | An attestor signing vouchers under a registered key; the OAuth and provenance verifications are documented flows |
| Demonstration harness | Medium | Controlled participants, wallets, chain state, and induced failures across process, network, disk, and chain; the completion boundary makes this load-bearing and it is substantial infrastructure |
| Project seed host and site | High | A persistent daemon instance and a static site |

**Assessment.** No component is infeasible. The Medium ratings, transport, custody, installation, and the demonstration harness, are for breadth rather than technical difficulty, and they are also the components with the least prior art inside the project's own research, which concentrated on the cryptography.

# Data

Assessed against the technical approach's Data section: on-chain records, swarm objects, local stores, memory-only material, and the secret inventory.

**On-chain state is small and conventional.** Constant storage per entitlement per interval, with envelopes and proofs in calldata and events. The parameter-set registry and the sidecar-coverage rule add modest state per asset. Feasible on any EVM chain; the cost is gas per settlement, which the harness measures.

**Swarm objects are large and their integrity structure is standard.** One ciphertext and one sidecar per live parameter set per deployment, committed by BLAKE3/Bao roots; the compatibility adapter additionally carries SHA-1 piece hashes. Feasible; Bao verified streaming is an existing construction.

**Local stores are two content-addressed stores with different owners and lifecycles**, a custody store, a job store, and a configuration registry. The separation of plaintext CAS from ciphertext store is a policy discipline, not a technical difficulty. The custody store's requirement never to persist the decrypted credential or piece-group keys is the one place where an ordinary implementation habit, caching the decrypted value, would violate the specification; the requirement is feasible and must be tested by inspecting storage after decryption and after a transfer out, as IW-04 specifies.

**Data availability of envelopes rests on chain history.** A holder who loses its envelope recovers it from any honest full-history node. Feasible, with the dependency stated: a client needs access to at least one archive node, which is a configuration and operating matter.

**The secret inventory is complete.** The research requirement to identify who creates every secret at every lifecycle event is met, and the inventory is short: master scalar, credential exponent and offsets, envelope coins, envelope secrets, capsule randomness, publisher seed. Nothing is held by a party the design does not name.

# Deployment

**Target platforms.** Windows, macOS, and Linux, with signed native artifacts, a daemon under the platform service manager or an equivalent persistent user service, and no developer toolchain on user machines. Feasible; the matrix is the cost, and each entry is a body of work.

**Chain deployment.** A factory-injected contract suite on Base, with the verifier deployed to Base Sepolia by the harness. Feasible; both precompile sets are live and the harness measures their pricing.

**Services.** Relayer, claim verifier, seed host and site, as Rust services. Conventional operations; none is on a client's critical path.

**Updates and uninstall.** Signed, atomic, schema-aware, reversible updates and an uninstall that restores package-manager configuration. Conventional but must be built and proven per platform, with failure injection at every checkpoint.

**Assessment.** Deployment is feasible and it is the second-largest body of work after the cryptography, with the least prior work in the repository. It should be planned as its own workstream with its own platform matrix decision.

# Sequencing

The [dependency map](dependency-map.md) holds the sequence at decaying resolution. The assessment of it:

- **The order is dependency-correct.** Every edge in the map runs from producer to consumer, and the harness's starting points are genuinely independent.
- **The harness early is the right call.** It resolves the most parameters, it is shipped code, and it is the throughput calibration the project lacks.
- **The hashing, signature, and swarm tickets run beside the harness.** They have no dependency on the contract sprints and no KEM; they are the earliest thing after the harness the project can prove end to end.
- **The demonstrable milestone is named and measured.** An ordinary `npm install` resolving against the swarm, after credential delivery, is the earliest point the north star and the latency guardrail can be observed. It is not a release, and the completion boundary is unchanged.
- **External work starts at the harness, not after it.** Cryptographic review can begin on the composition claims while the harness is being built and finish on its output; legal drafting has no engineering dependency at all.
- **The escrow claim is correctly last** by dependency alone, since the escrow record carries no maintainer commitment and no policy gate remains.
- **Resolution decay is the right response to uncertainty.** Detail authored now for distant groupings would be rewritten; re-mapping one grouping outward after each closes keeps the plan honest.

# Risk Mitigation

Feasibility-specific mitigations, with the risk register identifier where one exists.

| Feasibility risk | Mitigation |
| --- | --- |
| Throughput unknown (R-04) | Treat the harness grouping as calibration; re-map the remaining protocol core milestones to tickets from measured throughput; state no timeline before that |
| Platform breadth (R-04) | Windows, macOS, and Linux, each proven on clean machines; staff the onboarding grouping separately from the cryptography |
| Rust and Solidity verifiers drift (R-03) | Generate the Solidity constants and vectors from the Rust reference; cross-verify on every vector in CI |
| Custody (R-09) | The local keystore under the OS credential store is the first `IKeyCustodyAdapter`; a later implementation is a second adapter |
| Chain properties (R-12) | Both verifier forms built and measured; BLS12-381 confirmed against the budget |
| Cryptographic review on the critical path (R-03) | Quote and start review during the harness grouping; gate only the priced deployment on it |
| Legal work on the release path (R-10) | Start in parallel with the harness; none of it has an engineering dependency |
| Demonstration harness underestimated | Treat it as a component with its own tickets, not as test tooling; it is load-bearing under the completion boundary |
| The transaction flow proof reached last and slipping | Keep its dependencies, publisher path, delivery, relayer, off the critical path where the map allows, and start review early so pricing is not blocked at the end |

# Decisions

**Supported platform matrix.** Windows, macOS, and Linux.

**Team composition.** A small team directing agent implementers, with applied cryptography and platform packaging present or contracted.

**Throughput calibration.** The harness grouping's measured throughput is the basis for the first timeline estimate, and no timeline is stated before it.

**External cryptographic review timing.** Starts during the harness on the composition claims, concludes on harness output, gates only the priced deployment.

**Solidity verifier.** Generated from the Rust reference.

**Demonstration harness.** Planned and ticketed as a component.

# Additional Content

**What this assessment does not do.** It does not estimate dates, headcount, or cost, because no source supports an estimate and an unsupported one would be mistaken for a plan. It does not re-derive the risk register, the non-functional review, or the critique; it cites them. It does not assess the feasibility of deferred V2 items, which the MVP does not encounter.

**Relationship to the other planning documents.** The business case states the case for building; the critique tests it; the technical approach states how; the dependency map states in what order; the risk register states what could go wrong; the non-functional review states what must hold; the success metrics state how success is measured. This assessment answers one question the others leave open: can it be built. The answer is that the technical part can, and that whether it will be delivered is a question the harness grouping is designed to start answering.

**A note on what the research already settled.** The single largest feasibility question the project ever faced, whether distinct native decryption material per ownership interval could exist without a provisioning network, was open on 2026-09-15 and closed at the research level on 2026-09-22 with a construction, proof sketches, and an independent re-derivation. Everything this assessment rates as Medium is ordinary engineering breadth. That is the correct shape for a project at this stage: the hard problem is behind it, and the wide problem is ahead.
