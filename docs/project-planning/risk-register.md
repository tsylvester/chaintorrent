<!-- Template: antithesis_risk_register.md -->
# Risk Register

## Overview

Draft, 2026-09-23. Specific identified risks to the ChainTorrent MVP, consolidated from the risks named in [cryptography.md](../research/cryptography.md), [MVP Scope](../research/MVP%20Scope.md), [MVP Application Requirements](../research/MVP%20Application%20Requirements.md), the [workplan To-Do list](../workplans/current/ChainTorrent%20MVP.md), and the planning documents written against them: the [business case](business-case.md), its [critique](business-case-critique.md), the [technical approach](technical-approach.md), and the [success metrics](success-metrics.md). Each risk is one block below with the template's seven fields; the Notes field of each carries affected components, dependencies, sequencing, guardrails, signals, and open questions, which the document descriptor asks for and the template does not name separately.

Ratings are qualitative. **Impact** is High when the risk would block release, break an invariant, or lose the adopting population; Medium when it would degrade a release-defining metric or require rework across more than one component; Low when it is contained. **Likelihood** is High when the sources already record the condition as present or unmeasured; Medium when it depends on a decision or measurement not yet made; Low when the design already forecloses it and only implementation error remains. The sources supply no probabilities and none are invented. Risks are ordered approximately by the product of the two ratings, highest first, with accepted residuals last because they are not open. R-01 keeps its position after being re-rated from High to Medium on both axes, because it is the risk that most needs a decision before the First Finder node is authored.

A summary table is under Additional Content.

---

## Risk
**R-01. Unauthorized public redistribution and irrevocability of ingested packages.** The specification confines ingest adapters to archives whose content is already free to use, and the MVP implements that as public npm availability. The two are close: npm serves every public package to anyone at no charge regardless of its license, so availability tests that the publisher chose free public distribution, and a user obtains nothing from the swarm they could not already obtain from npm. What the swarm changes is who distributes and whether it can be withdrawn. First Finders and seeders redistribute to the public without the grant the publisher gave npm, and some restrictive licenses forbid redistribution even of freely obtainable bytes; and the swarm is irrevocable by design, so a publisher loses the ability to unpublish. The escrow contract also mints a transferable entitlement the publisher never authorized, which at $0.00 confers nothing beyond what npm gave but is a new artifact carrying their content's identity.

## Impact
Medium. No revenue is denied, since npm charged nobody for the download, so the post-claim pricing paradox does not arise for this corpus. The exposure is an objection from a rights holder of a freely distributed package to unauthorized redistribution or to the loss of withdrawal, landing on the ingest path and on seeders. Seeders hold ciphertext they cannot read, which places them closer to a cache than to a public copy.

## Likelihood
Medium. Public npm mirrors and enterprise proxies redistribute the same packages today and are largely uncontested, which is evidence that objections are uncommon; a swarm serving strangers and refusing withdrawal is a different posture from a proxy serving one organization's licensees, which is why the likelihood is not Low.

## Mitigation
State the principle precisely in MVP Scope and the business case: free public distribution by the rights holder's own choice, with public npm availability as the implementing test, and the residual named as redistribution and irrevocability rather than denied revenue. Whether to additionally refuse First Finder ingest for packages whose declared license forbids redistribution is a policy choice the workplan already holds behind a deliberate line; it narrows an already small exposure and should be decided on that footing, not as a repair. Advisory backlinks and the local trust set are the specification's mechanism for a publisher's later objection to reach clients without registry mutation.

## Seed Examples
- A vendor publishes a proprietary SDK to npm at no charge under a license that forbids redistribution. A First Finder ingests it and the swarm serves it permanently. Every user could already download it from npm; the vendor's objection is to the redistribution and to being unable to withdraw it, not to lost sales.
- A publisher unpublishes a package from npm for a reason of their own; the swarm continues to serve it and the publisher's only recourse is an advisory.

## Mitigation Plan
1. Restate the eligibility principle and its residual in MVP Scope and the business case before the First Finder ingest node is authored.
2. Decide, as a recorded policy choice, whether the npm ingest adapter also refuses licenses that forbid redistribution; if so, implement it in the adapter's eligibility contract so every First Finder applies it identically, with the initiating install still served from upstream bytes.
3. Ensure the advisory mechanism, when built, gives a publisher a visible channel for objection that conforming clients can act on locally.

## Notes
- **Affected components:** `IIngestSourceAdapter` npm implementation, First Finder bootstrap workflow, seed hosts, MVP Scope's ingest section, business case; advisory backlinks in V2.
- **Dependencies:** none technical; a policy decision.
- **Sequencing:** before workplan step five.
- **Guardrails:** no First Finder ingest from a source the public cannot retrieve at no charge.
- **Signals:** a rights holder's objection to a specific escrowed asset; any escrow record whose upstream manifest declares a license forbidding redistribution, if that becomes a tracked field.
- **Open questions:** whether to refuse redistribution-forbidding licenses; whether npm license metadata is reliable enough to gate on; whether a missing license field is refused or accepted.

---

## Risk
**R-02. Measured cryptographic cost exceeds the interactive latency budget.** No byte, latency, proof, or gas figure has been measured. If decapsulation per piece group multiplied by groups per package, plus a state read per attempt, adds a noticeable fraction to a developer's install, the north-star benefit is not felt and the adopting population notices.

## Impact
High. Interactive install wall-clock is the binding constraint; the success metrics make the latency budget the only threshold that can fail a run and the first stop criterion.

## Likelihood
Medium. The research's by-formula estimates put sidecar overhead at roughly one percent of payload at 16 KiB groups, depending on curve and scope, and decapsulation at two pairings per group, which is cheap, but nothing is measured, the piece-group size is unchosen, and state-read latency depends on the launch network.

## Mitigation
Build the validation harness first and choose the piece-group size and curve from its measurements. Declare the latency budget before the acceptance run so instrumentation produces a failure rather than a number. Instrument state-read volume and latency per install to choose `τ_soft`. Install-once content crosses the attempt boundary rarely, which bounds the exposure for the MVP's content class.

## Seed Examples
- On the chosen L2, a state view at the declared tier takes long enough that a thousand-package closure, even paginated, adds seconds a developer notices.
- The piece-group size chosen to bound key leakage is small enough that a large package needs hundreds of decapsulations.

## Mitigation Plan
1. Harness on both curves, recording sizes, decapsulation time, proof times, and gas (CD-07, AS-21).
2. Declare the budget, including the acceptable authorization fraction, before measurement.
3. Choose piece-group size against the budget; record the choice in release evidence.
4. Measure state-read latency during the dogfood run; choose `τ_soft` and `τ_wallet`.
5. If no parameter within the specification's bounds meets the budget, report it as a finding against the design rather than relaxing the budget.

## Notes
- **Affected components:** validation harness, encrypted consumption, settlement and entitlement-state adapters, deployment hash-card parameters.
- **Dependencies:** launch network for the curve; harness on BN254 meanwhile.
- **Sequencing:** workplan step two, before any node that encrypts.
- **Guardrails:** latency budget (RO-06).
- **Signals:** authorization fraction of install wall-clock rising toward the budget; `PENDING_SETTLEMENT` dwell time rising.
- **Open questions:** the budget's value; whether batch reads at the declared tier are available on the chosen network.

---

## Risk
**R-03. Delivery is not proof-secure, or the on-chain verifier diverges from the specification.** The composition has proof sketches with loss terms and one independent re-derivation, not a paper's proofs or an external audit. The delivery proof is verified in two forms depending on the curve, and the Rust verifier must match the deployed contract bit for bit.

## Impact
High. Delivery soundness is what prevents a seller being paid without delivering and an envelope for one entitlement being recorded against another; a verifier flaw silently breaks the settlement boundary the MVP exists to prove.

## Likelihood
Low for the mathematics, which uses standard components and named assumptions; Medium for implementation, where encoding, subgroup checks, hash-to-scalar, and context schema are exactly the places such systems fail.

## Mitigation
External cryptographic review of the composition and the contract verifier before any priced deployment. Cross-verification of the Rust prover against the deployed contract on each curve form, with every statement field and response mutated and replay across settlements attempted (CR-09, CD-03, AS-13). Contract-side subgroup validation where the precompile does not perform it (E47).

## Seed Examples
- The BN254 batched pairing-product check is implemented unweighted, which the research notes is not equivalent to three separate equalities; a crafted proof passes.
- The challenge hash omits one context field, and a proof transplants to a second settlement.
- A registered `G2` key on BLS12-381 is accepted without an MSM or pairing check and is off the prime-order subgroup.

## Mitigation Plan
1. Implement the verifier in Rust and Solidity from the same statement of the relations; treat the Rust verifier as the reference and the contract as its port.
2. Mutation and replay test suite over every statement field, both curve forms.
3. Commission external review in parallel with the harness, scoped to the composition claims and the verifier.
4. Gate the priced dogfood deployment on the review's result.

## Notes
- **Affected components:** `SchnorrFsDeliveryProofAdapter`, `IPairingAdapter` implementations, contract suite, validation harness.
- **Dependencies:** curve choice for the verifier form.
- **Sequencing:** harness at step two; contract at step four; review before the transaction flow proof at step eight.
- **Guardrails:** zero accepted invalid deliveries.
- **Signals:** any divergence between Rust and on-chain verification on the harness vector set.
- **Open questions:** who performs the review and when the result is needed.

---

## Risk
**R-04. Execution breadth and the strict completion boundary.** Every operational requirement carrying a boundary-crossing proof, every acceptance scenario packaged on clean machines, four control surfaces, multi-platform signed installation and service lifecycle, a contract suite, three services, and a demonstration harness. No code exists and no team is named.

## Impact
High. Delivery risk, not design risk: the last third of the work, publishing, claims, and the transaction proof, is where schedules slip, and the completion boundary forbids mocked adapters, off-chain-only verifiers, and manually prepared machines as completion evidence.

## Likelihood
High. The breadth is a fact of the requirements and no schedule or team exists to set against it.

## Mitigation
The workplan's dependency-ordered sequence, test-first and bottom-up, with one file per turn. An internal demonstrable milestone at workplan step six, where an ordinary `npm install` is served at the registry's speed, registers and prefetches, and resolves against the swarm with the registry down, without reducing the release boundary. Deferred items held deferred.

## Seed Examples
- Platform-specific service lifecycle on one supported OS consumes the effort planned for credential delivery.
- The license text waits on legal drafting while everything else is done, and release waits on it.

## Mitigation Plan
1. Ratify the build sequence and author the first node.
2. Define the step-six milestone with its own north-star and latency measurement.
3. Author nodes only from settled decisions; hold the escrow-claim node until R-13 resolves.
4. Size the team and a timeline once the harness is done and the first nodes have measured throughput.

## Notes
- **Affected components:** all.
- **Dependencies:** blocking selections (R-13).
- **Sequencing:** throughout.
- **Guardrails:** completion boundary as written; no test-only bypass.
- **Signals:** requirement-to-proof reconciliation lagging node completion.
- **Open questions:** team, timeline, and funding, none of which any document states.

---

## Risk
**R-05. Beachhead value proposition is parity with existing tools.** Cross-project reuse, the north-star benefit, is already available from pnpm's content-addressable store; outage insulation is available to enterprises from private proxies. The protocol's distinguishing property, transferable irrevocable entitlements, is invisible at $0.00 on permissively licensed content.

## Impact
High. If the individual developer feels no benefit beyond what they have, the adopting population does not adopt and nothing downstream follows.

## Likelihood
Medium. The long-tail outage-survival benefit and network accumulation are real and no substitute offers them, but they depend on adoption already having happened, and the business case does not lead with them.

## Mitigation
Lead the developer value proposition with what substitutes cannot do: the swarm holds what anyone fetched, not what one machine fetched. Use the workplan's build-platform superseeder mechanism as the adoption engine. Present supply-chain benefits, on-chain provenance and advisories that follow content, as first-class. Measure the north star on the dogfood population before release so the claim is evidenced.

## Seed Examples
- A developer already on pnpm installs ChainTorrent, sees identical reuse, notices a daemon and a seeding archive, and removes it.
- A registry operator ships a native peer cache and captures the outage benefit.

## Mitigation Plan
1. Revise the business case's beachhead argument per the critique.
2. Describe the cold-start dynamics: seed host for the core closure, build platforms as superseeders, long tail by accumulation.
3. Instrument and report north-star and remote-encrypted-hit rate from the first dogfood run.

## Notes
- **Affected components:** business case, onboarding messaging, seed host, discovery.
- **Dependencies:** none technical.
- **Sequencing:** before release; messaging before the demonstrable milestone.
- **Guardrails:** none; a market risk.
- **Signals:** north star flat against baseline; uninstall rate.
- **Open questions:** whether build platforms will in fact superseed without being asked.

---

## Risk
**R-06. Consumption privacy for the adopting population.** Every entitlement is a public on-chain record binding an identity to an asset, a first run in the default mode publishes the identity's whole dependency closure as grant requests at once rather than as it is used, and per-attempt state reads form a traffic stream to whichever node serves them. The specification's argument that public consumption is a benefit concerns organizations concealing inherited risk; it says a solo participant's install history is a behavioral profile and that pseudonymity only partially helps.

## Impact
High for adoption; Medium for the protocol. Individual developers are the MVP's adopting population and the population the argument excludes. Disclosure of their install history at onboarding may be a reason not to adopt.

## Likelihood
High. The condition is inherent in the design for public content and the MVP ships no privacy construction.

## Mitigation
State the exposure at onboarding, where the request and prefetch behaviors are consent items and a cache-only mode with no chain identity and no request exists for a user who wants only the local cache and the registry. Read state from the client's own node where it can, which discloses nothing through the read path. Record the private-content and individual cases as open in the specification, with candidate constructions, and gate any private-registry adapter on a consumption-privacy answer.

## Seed Examples
- A developer's identity is linked to a real name by one transaction; their entire install history, with timing, is retroactively public.
- A configured RPC provider logs a client's state-read stream and reconstructs its dependency set in real time.

## Mitigation Plan
1. Add the exposure to the business case's Risks and Differentiation, and to onboarding consent text.
2. Prefer own-node state reads in the default configuration where a light client is available.
3. Keep the candidate constructions, ephemeral wallets, blinded reads, aggregate settlement, zero-knowledge entitlement proofs, on the workplan as a V2 design question.

## Notes
- **Affected components:** entitlement contract, entitlement-state adapter, onboarding, documentation.
- **Dependencies:** the auditability-versus-privacy tension, unresolved.
- **Sequencing:** consent text before release; constructions post-MVP.
- **Guardrails:** telemetry records no identifying data beyond what the ledger publishes.
- **Signals:** adoption objections citing privacy.
- **Open questions:** which side of the tension a resolution sacrifices.

---

## Risk
**R-07. The daemon and registry redirect are a supply-chain attack surface.** A local registry endpoint on every developer machine, redirecting the package manager, is precisely the component an attacker would want. The protocol's own distribution channel, signed artifacts and updates, is a second surface.

## Impact
High. A compromised daemon can serve arbitrary tarballs to every project on the machine; a compromised update channel can do so to every machine.

## Likelihood
Medium. The Application Requirements address it: authenticated artifacts and updates, least-privilege authenticated local API, untrusted-until-validated inputs, verified content addressing before anything is served. Residual risk is implementation error and key management for signing.

## Mitigation
Artifact and update authentication before execution (SI-04, IC-02, SI-17); plaintext served only after verification against the authenticated root or the upstream attestation (PR-04, EC-07, FF-01); authenticated least-privilege IPC (XA-02); fuzzing at every boundary (XA-01); explicit, visible, reversible redirect (SI-07).

## Seed Examples
- A malicious local process calls the daemon's IPC and obtains a decrypted credential or administrative authority.
- A tampered update artifact with valid-looking metadata replaces the working daemon.

## Mitigation Plan
1. Signing-key custody and rotation for artifacts as part of the release process.
2. IPC authentication and capability separation tested from untrusted local users and project scripts.
3. Fuzz every adapter contract and IPC request.
4. Security review of the daemon alongside the cryptographic review.

## Notes
- **Affected components:** installer, daemon, IPC, update mechanism, package host.
- **Dependencies:** platform credential stores for signing verification.
- **Sequencing:** with steps three through six.
- **Guardrails:** no protected side effects from untrusted input; no unauthenticated artifact executed.
- **Signals:** any IPC call from an unprivileged principal succeeding beyond package requests.
- **Open questions:** where the artifact signing key lives and who holds it.

---

## Risk
**R-08. Free-path abuse and subsidy exhaustion.** Every free mint verifies a proof on chain at relayer expense, every new identity costs a binding transaction, and payment locks can be opened and abandoned. The subsidy is unfunded, unsized, and scales with adoption.

## Impact
Medium. Exhaustion stops free onboarding, which is the MVP's only onboarding; griefing ties up buyers' funds briefly.

## Likelihood
Medium for cost growth with success, reduced from High by the Base launch decision, under which L2 execution is cheap and the L1 data-posting fee is the dominant and measurable component; Medium for deliberate abuse.

## Mitigation
Per-identity rate limits under relayer policy; short published lock expiries with refund; explicit failure states that never produce false authorization; relayer gas recorded per install with L2 execution and L1 data fee reported separately (LC-10, RO-01, RO-03, AS-27). Bundlers and paymasters on Base make sponsored transactions available but do not remove the project's gas bill. The subsidy is named as scaffolding to be replaced.

## Seed Examples
- Many fresh identities flood free mints for one package; each mint costs a precompile verification.
- A buyer opens locks against many listings and lets them lapse.

## Mitigation Plan
1. Size the grant pool from measured gas per mint and per binding after the harness and dogfood run.
2. Set rate-limit policy as a relayer parameter, not a contract constant.
3. Define what replaces the subsidy and when free onboarding ends, in the business case.

## Notes
- **Affected components:** relayer, entitlement contract, lock logic.
- **Dependencies:** launch network for gas; ERC-4337 support.
- **Sequencing:** relayer with step seven.
- **Guardrails:** grant pool not drained by a flood.
- **Signals:** subsidy consumption rising faster than identity creation.
- **Open questions:** pool funding source and size; replacement path.

---

## Risk
**R-09. Blocking selections delay node authoring.** Attempt-rule parameters and piece-group size, license text, custody recovery UX, and the multi-device sync mechanism remain; default key custody, launch network and curve, and the escrow-salt question are resolved and recorded in the product requirements.

## Impact
Medium. Each delays the nodes that depend on it; the parameters gate every node that encrypts and are resolved by the harness; the license gates release, not nodes.

## Likelihood
High. The condition is present; the technical approach records default assumptions but they are assumptions.

## Mitigation
Resolve the parameters from harness measurement before any node that encrypts; treat the license as a release prerequisite; confirm the multi-device sync recommendation in the product requirements.

## Seed Examples
- Custody is deferred pending a browser password-manager evaluation, and the identity node cannot be authored.
- The piece-group size is assumed before the harness reports, and every node that encrypts is authored twice.

## Mitigation Plan
1. Confirm or overrule each assumption in the technical approach's Feedback blocks.
2. Author the custody adapter as the first `IKeyCustodyAdapter` with declared capabilities, so a later implementation is a second adapter rather than a rewrite.
3. Hold every node that encrypts until the harness records the piece-group size.

## Notes
- **Affected components:** custody, identity, escrow claim, deployment parameters, licensing.
- **Dependencies:** harness output for parameters and curve.
- **Sequencing:** before the nodes that depend on each.
- **Guardrails:** no node authored against an unrecorded decision.
- **Signals:** a node's `deps` element naming an unresolved adapter.
- **Open questions:** the six selections themselves.

---

## Risk
**R-10. Legal framings overreach and regulatory questions are unexamined.** The planning documents invoke digital first sale and the fair-use line for machine-learning corpora; neither transfers to verbatim redistribution. Tradeable priced entitlements, even in the dogfood proof, and a paymaster subsidizing transactions may attract securities, consumer-protection, or money-transmission scrutiny that no document considers.

## Impact
Medium for the MVP, whose corpus was distributed to the public at no charge by its publishers and whose priced proof is nominal; High for the path to priced content.

## Likelihood
Medium. The MVP's exposure is bounded as R-01 describes; the regulatory questions are unexamined rather than known to be adverse.

## Mitigation
Restate the legal framings as analogies and name licensing as the actual protection. Commission legal review scoped to the license text, the eligibility rule, and the regulatory treatment of priced entitlements and the paymaster, in parallel with the harness.

## Seed Examples
- A reader of the business case takes "digital first sale" as a legal basis and relies on it.
- The paymaster is characterized in some jurisdiction as a regulated payment service.

## Mitigation Plan
1. Correct the business case per the critique.
2. Legal review in parallel with the harness.
3. Record the outcome in MVP Scope's licensing section and in the workplan.

## Notes
- **Affected components:** business case, licensing, relayer, priced dogfood path.
- **Dependencies:** R-01.
- **Sequencing:** before the transaction flow proof.
- **Guardrails:** no priced deployment before legal review.
- **Signals:** none technical.
- **Open questions:** which jurisdictions; whether the reference client and protocol libraries carry different licenses.

---

## Risk
**R-11. First Finder bootstrap fails, duplicates, or races incorrectly.** Two finders ingest the same version and produce different ciphertexts; a loser fails to destroy its master scalar; a bootstrap job completes twice or never after the initiating process exits.

## Impact
Medium. Duplicate deployments or parameter sets fragment the swarm; a retained loser scalar is plaintext-equivalent capability outside custody rules; a never-completing job means the asset never reaches the swarm.

## Likelihood
Low. State-locked registration, loser destruction, and durable idempotent jobs are specified with explicit proofs (FF-03, FF-05, FF-07, AS-10, AS-12, AS-18).

## Mitigation
The first state-locked registration wins; the loser destroys ciphertext, sidecar, master scalar, cipher state, and keystream and resolves the winner; jobs are checkpointed and idempotent; the plaintext root proves the loser's discarded object was the same content.

## Seed Examples
- Two CI runners in one organization install the same new version within the same block.
- The daemon is killed between encryption and registration and restarts with a second deployment identifier.

## Mitigation Plan
1. Race the demonstration harness's independent finders and inspect chain state, stores, and secret lifecycle.
2. Terminate at every checkpoint and prove single completion.

## Notes
- **Affected components:** First Finder workflow, registry contract, custody, seed host.
- **Dependencies:** registry contract's state lock.
- **Sequencing:** step five.
- **Guardrails:** one deployment and one parameter set per asset from bootstrap.
- **Signals:** any asset with two escrow deployments.
- **Open questions:** none.

---

## Risk
**R-12. Chain properties change under the deployment.** Base is the launch network. Its centralized sequencer, L1 data-fee pricing, precompile pricing, settlement latency at each tier, and RPC availability are outside the project's control. A sequencer outage stalls mints, grants, and transfers but not reads, which come from any configured node; a fee or precompile repricing changes delivery cost; a censoring RPC denies reads. Entitlements and contract state do not move to another chain adapter automatically, and no migration mechanism is specified.

## Impact
Medium. Reads are protected by multi-node views and continue through a sequencer outage; settlement stalls delay installs that need acquisition but not those served from local plaintext; cost changes affect the subsidy. Migration is a later-phase obligation, held in the workplan To-Do.

## Likelihood
Medium. OP Stack upgrades are routine. EIP-2537 is confirmed live on Base since Isthmus activated on 9 May 2025, so the precompile gate is closed; Arbitrum's remains unconfirmed and it is excluded as a fallback until confirmed.

## Mitigation
Chain behind an adapter with settlement expressed as tiers rather than confirmation counts; state views from a quorum of configured nodes at a common reference with divergence failing closed (XA-06); precompile availability confirmed per launch network before selection; gas instrumented.

## Seed Examples
- The chosen L2 has not deployed the BLS12-381 precompiles and the verifier form must be the batched pairing product on BN254.
- A hard fork changes pairing precompile gas and the free path's cost per mint doubles.

## Mitigation Plan
1. Confirm precompile availability and pricing as part of network selection.
2. Keep both verifier forms implemented and tested.
3. Monitor gas per mint and per transfer against the dogfood baseline.

## Notes
- **Affected components:** chain adapter, settlement adapter, pairing adapter, relayer.
- **Dependencies:** network selection.
- **Sequencing:** step one and ongoing.
- **Guardrails:** attempts proceed with one node failed; halt on disagreement.
- **Signals:** rising `PENDING_SETTLEMENT` dwell; gas deviation from baseline.
- **Open questions:** the network.

---

## Risk
**R-13. Escrow-salt custodian choice leaves a record unclaimable or reintroduces a dependency. Resolved: no commitment is stored and no custodian exists.** Siting a salt with the First Finder would make a claim depend on a party designed to be unnecessary; siting it with the claim verifier would make a lost salt a permanently unclaimable record; and the commitment confers nothing the attestor verifier can use. The verifier establishes the claim set from upstream metadata at verification time; a commitment is reopened only with a ZK-Email verifier that computes its own at claim without a custodian.

## Impact
Medium. Affects the escrow-claim surface only, but a wrong choice is a contract-level change after deployments exist.

## Likelihood
Closed. The decision is made and recorded in MVP Scope, PC-02, PC-04, and the product requirements; the residual is the absence of any maintainer binding at ingest, which the per-version claim design did not use.

## Mitigation
Decide before the escrow-claim node. The technical approach's default is to drop the on-chain commitment and have the verifier establish the claim set from upstream metadata at verification time.

## Seed Examples
- The verifier holds the salt, loses it in a key rotation, and ninety versions of a package become unclaimable forever.

## Mitigation Plan
1. Confirm or overrule the default in the technical approach's Feedback block.
2. Author the claim-set state layout accordingly; keep the claim-set mechanism, which is unchanged by the choice.

## Notes
- **Affected components:** escrow contract, claim verifier, PC-02, PC-04.
- **Dependencies:** none.
- **Sequencing:** before step eight's escrow claim.
- **Guardrails:** claim completes with the First Finder and any portal-like component absent.
- **Signals:** none until the surface exists.
- **Open questions:** the decision.

---

## Risk
**R-14. Governance of the adapter registry and centralization scaffolding.** Adapters are resolved from a governance-controlled on-chain registry, immutable once bound, and no document says who governs it. The relayer, project seed host, and claim verifier are centralized conveniences the project operates.

## Impact
Medium. For a decentralization thesis, an unstated governor of adapter resolution is a material omission; captured scaffolding could become authoritative in practice if clients default to it.

## Likelihood
Medium. The scaffolding is bounded by design, none is on a client's critical path, and the verifier key is revocable on chain; the registry governance is simply unstated.

## Mitigation
State the registry's governance in the specification. Keep every service behind an interface with a documented replacement path; the seed host is a convenience no client depends on, the verifier is one implementation with a revocable key, the relayer covers the free path only.

## Seed Examples
- The adapter registry's governing key is a single project-held key and a compromise rebinds every new escrow contract's adapters.
- Clients ship with the project's discovery endpoint as the only configured source and the swarm degrades when it is down.

## Mitigation Plan
1. Record the registry governance model as a workplan item.
2. Ship default configuration with more than one discovery source and more than one state-read node.
3. Publish the replacement path for each service in the business case.

## Notes
- **Affected components:** adapter registry contract, factory, relayer, seed host, claim verifier, default configuration.
- **Dependencies:** none.
- **Sequencing:** registry governance before step four.
- **Guardrails:** protocol resolves with every project-run service unreachable.
- **Signals:** any acceptance scenario that passes only with a project service reachable.
- **Open questions:** who governs the registry and how it changes.

---

## Risk
**R-15. Settlement reversal.** A settlement included below `SETTLED` is reverted after a buyer read its envelope, leaving the buyer one entitlement's credential without a settled transfer, or after a seller's transfer, which under the attempt rule leaves the seller still able to read.

## Impact
Low. The exposure is one entitlement's credential held by a non-holder, the same bounded quantity a seller could produce by leaking its own; forward-only, non-compounding, self-healing, and the seller may re-post.

## Likelihood
Low for public free packages reading at `INCLUDED`, which accept a higher frequency of an already-accepted failure; lower for priced deployments declaring a deeper tier.

## Mitigation
Reads at `SOFT`, destruction at `HARD`, tier declared per deployment and immutable; scarcity is ordered by the ledger, never by a credential (LC-04, LC-05, AS-19).

## Seed Examples
- A reorganization on the L2 reverts a transfer after the buyer decrypted; the buyer holds a working credential and no entitlement; the seller re-posts and the sale completes.

## Mitigation Plan
1. Exercise included, pending, reorganized, stale, and insufficient-tier references through the attempt rule (LC-04).
2. Declare and justify a deeper tier for the priced dogfood deployment.

## Notes
- **Affected components:** settlement adapter, attempt rule, destruction logic.
- **Dependencies:** chain tier mapping.
- **Sequencing:** step seven.
- **Guardrails:** destruction only at `HARD`.
- **Signals:** reorg depth on the chosen network exceeding the tier mapping's assumptions.
- **Open questions:** none; a stated property.

---

## Risk
**R-16. Claim verifier attestation key compromise.** The MVP claim verifier is an attestor signing vouchers under an on-chain key; its compromise would hand publisher authority over escrowed assets to an attacker.

## Impact
High if realized; the attacker gains issuance and pricing authority over every escrowed asset it claims.

## Likelihood
Low. The key is registered on chain with rotation and revocation, vouchers are bound to claimant, claim set, contract, chain, nonce, and expiry, and revoked-key vouchers are rejected (PC-03, PC-07, AS-27).

## Mitigation
Key rotation and revocation; voucher binding; front-running protection by binding proofs to the claimant's chain identity.

## Seed Examples
- A leaked verifier key signs a voucher claiming a popular package's escrow records to an attacker address before revocation lands.

## Mitigation Plan
1. Key custody for the verifier under the same custody discipline as issuance material.
2. Revocation drill in the acceptance run.
3. Consider a ZK-Email or DNSSEC verifier as a peer implementation post-MVP, which removes the attestor key.

## Notes
- **Affected components:** claim verifier, escrow contract, verifier key registry.
- **Dependencies:** none.
- **Sequencing:** step eight.
- **Guardrails:** revoked key's vouchers rejected.
- **Signals:** any voucher signed outside the verifier's audited path.
- **Open questions:** none for the MVP.

---

## Risk
**R-17. Secret leakage through storage, logs, diagnostics, or crash artifacts.** Decrypted credentials, piece-group keys, cipher state, keystream, master scalars, or envelope secrets written where they should not be.

## Impact
High if realized; a persisted decrypted credential defeats interval-end destruction and a logged master scalar is plaintext-equivalent capability for the asset.

## Likelihood
Low by design, since the specification mandates memory-only material and zeroization at every transition; Medium in practice, since this is a common implementation failure.

## Mitigation
Secret lifecycle services that minimize copies, exclude secrets from logs, and zeroize at success, interval end, cancellation, loss, and error (CR-07); custody surfaces inspected before and after decryption and after a transfer out (IW-04); telemetry scanned for sensitive fields (RO-03, AS-20).

## Seed Examples
- A crash dump after a failed decryption contains expanded AES key schedules.
- A debug log line prints an envelope's decrypted tuple.

## Mitigation Plan
1. Instrument lifecycle boundaries and crash artifacts in the acceptance run.
2. Treat any sensitive field in a telemetry scan as a release-blocking security defect.

## Notes
- **Affected components:** all components handling decrypt-capable material; logging; diagnostics.
- **Dependencies:** none.
- **Sequencing:** with step seven and throughout.
- **Guardrails:** no protected material in any telemetry, log, storage, or crash artifact.
- **Signals:** any telemetry scan hit.
- **Open questions:** none.

---

## Risk
**R-18. Installation leaves a machine misconfigured or reroutes the toolchain without visible consent.** Partial installation after a failure, a redirect the user did not knowingly accept, or an uninstall that leaves npm pointed at a dead endpoint.

## Impact
High for trust; a toolchain silently rerouted is indistinguishable from a compromised one, and the protocol cannot ask for trust while behaving like the thing it warns about.

## Likelihood
Low. The requirements mandate explicit reversible consent, backup of every change, rollback on later failure, idempotent reinstall, repair, and exact restoration on uninstall (SI-07, SI-08, SI-14 through SI-18).

## Mitigation
As specified; proven by injecting failure after every mutating stage and comparing to the pre-install snapshot.

## Seed Examples
- Installation fails after writing the npm registry configuration and before starting the daemon; every install on the machine fails until the user finds the change.

## Mitigation Plan
1. Failure injection at every checkpoint in the acceptance run (AS-04, AS-05).
2. Consent trace review for any unexplained interaction (SI-19).

## Notes
- **Affected components:** installer, configuration registry, package-manager configuration.
- **Dependencies:** platform facilities.
- **Sequencing:** step six.
- **Guardrails:** consent boundary; exact restoration.
- **Signals:** any host configuration difference from the snapshot after a failed install.
- **Open questions:** none.

---

## Risk
**R-19. Accepted residual: modified-client retention and common piece-group keys.** A modified client can retain a decrypted credential past its interval. Every credential recovers the same encapsulated values, so the piece-group keys are common to every holder and exportable as a payload-proportional key file whose leak is unattributable.

## Impact
Medium for priced content; negligible for the MVP's permissively licensed corpus. A published key set decrypts the swarm copy for everyone at zero marginal cost and attributes to no one.

## Likelihood
High that the capability exists; it is a consequence of one canonical ciphertext under a symmetric cipher without trusted hardware and is recorded as not resolvable with known practical methods.

## Mitigation
Accepted and stated. The piece-group size bounds how much one leaked key unlocks; a leaked credential names its entitlement; the license makes a non-conforming client a violation; the decapsulation-to-cipher seam is preserved so a future multi-key suite can close the residual as a successor deployment; per-entitlement variance addresses plaintext attribution against unmodified clients in V2.

## Seed Examples
- A holder extracts and publishes the piece-group keys for a priced deployment; anyone holding the ciphertext decrypts it.

## Mitigation Plan
1. Choose the piece-group size with this exposure in view.
2. Do not describe the protocol as preventing it; the specification claims bounded operation for the conforming client, not hostile DRM.

## Notes
- **Affected components:** payload cipher, KEM, piece-group size.
- **Dependencies:** none.
- **Sequencing:** none; not open.
- **Guardrails:** none.
- **Signals:** a published key set for a priced asset.
- **Open questions:** none; reopen only if a practical multi-key PRF suite becomes available.

---

## Risk
**R-20. Accepted residual: pre-claim grant liveness and escrow-era attribution.** Under the escrow suite any holder can author a grant, so a new identity needs at least one holder online. That is a stricter condition than the swarm having a seeder: a conforming holder seeds, but a blind seeder holds no credential and authors nothing, so a swarm can hold every encrypted byte while no new user can obtain a grant. Escrow-era credentials carry no entitlement-level attribution because the suite uses the asset identity scope. Availability differs per operation and is stated separately in the system architecture's operation table: a cached plaintext read needs nobody; an existing holder's encrypted read needs no online party, only a quorum state view; a new escrow grant needs one online holder; post-claim issuance is the publisher's policy; metadata, chain state, and sponsorship each carry their own condition.

## Impact
Low while the registry is available, since an identity without a credential is served by the registry and its request is fulfilled later, in its absence if need be; Medium during a registry outage, when a healthy swarm with no holder online is a real state and the new user is told truthfully that no grant author is reachable, with a retry succeeding once one appears. Attribution is meaningless while entitlements are $0.00.

## Likelihood
High that the condition exists; Low that it bites for popular packages; Medium for the long tail, which is where the availability differentiation is claimed.

## Mitigation
Accepted and stated. An identity without a credential is served by the registry while it is available and its request stays pending until any holder fulfils it, offline requesters included; the conforming client serves pending requests automatically as it seeds; the project seed host holds an entitlement, and so a credential, for every asset it seeds, so its availability promise covers grants and not only bytes for the core closure; a claim never needs the First Finder.

## Seed Examples
- A rarely used package's only holder is offline; blind seeders hold every byte; a new identity cannot obtain a grant until a holder returns, and the client says so.

## Mitigation Plan
1. Verify grant fulfilment with the First Finder permanently offline (AS-26).
2. Verify that a swarm with only non-holder seeders reports the missing grant author accurately and is not counted as new-user availability.
3. Monitor grant fulfilment latency as a signal, separately from swarm health.

## Notes
- **Affected components:** credential delivery, seed host, grant service, availability pilot.
- **Dependencies:** none.
- **Sequencing:** step seven.
- **Guardrails:** First Finder absence affects nothing.
- **Signals:** grant fulfilment slowing while swarm health holds.
- **Open questions:** none.

---

## Risk
**R-21. Remote head rendezvous as a control surface over claimed daemons.** Deferred to V2 under MVP Scope's Website, Account, and Remote Head. When built, a rendezvous through which a browser reaches a user's local daemon becomes, if compromised, a path to every daemon claimed to an account.

## Impact
High if realized in that tier; none in the MVP, which ships no rendezvous and no remote head.

## Likelihood
Low by design once built: the daemon dials outbound only, browser and daemon communicate end to end under a key the daemon pins, commands go to the daemon and keys never leave it, and value-moving operations require local presence. A compromised rendezvous can deny service but cannot read or forge.

## Mitigation
The constraints above are recorded in MVP Scope as the tier's architectural protection, so they are design conditions rather than later hardening. The daemon's authenticated IPC under XA-02 is the only surface a remote head drives.

## Seed Examples
- A compromised rendezvous replays a captured command to a daemon; the daemon rejects it because the session key is pinned and the command is bound to it.
- A rendezvous operator attempts to initiate a transfer from a claimed daemon; the daemon refuses because transfer requires local presence.

## Mitigation Plan
1. When the tier is scoped, author its requirements with the four constraints as acceptance criteria before any rendezvous code exists.
2. Extend the consent trace to record the authorizing surface for every operation.

## Notes
- **Affected components:** daemon IPC, a future rendezvous service, a future remote head.
- **Dependencies:** none in the MVP.
- **Sequencing:** post-MVP.
- **Guardrails:** no operation that moves value or exports keys is reachable remotely.
- **Signals:** none until the tier exists.
- **Open questions:** who runs the rendezvous; whether the project seed host and site is that operator.

---

## Risk
**R-22. Account-to-identity mapping as a deanonymizing record.** Deferred to V2 with the account service. A familiar sign-in linked to an on-chain identity is the single record that retroactively unmasks that identity's public install history, which the specification's Dependency Graph Privacy section identifies as the individual developer's exposure.

## Impact
High for the individuals affected if the mapping leaks; none in the MVP, which has no account service.

## Likelihood
Medium once the tier exists: an account service is an ordinary web service with ordinary breach exposure, and the mapping is exactly what an attacker would take.

## Mitigation
Recorded in MVP Scope as the tier's protection: the mapping is held only by the account service, never on chain or in the DID Document; linking is optional and an identity with no account loses nothing; no key is ever derived from a provider identifier; and the account never owns an entitlement, so the mapping is not on any authorization path.

## Seed Examples
- An account-service breach publishes the table of provider identifiers to on-chain identities; every linked developer's dependency history is now attributable to a name.
- A user links and later wishes to unlink; the mapping must be deletable and its deletion must leave the identity functional.

## Mitigation Plan
1. When the tier is scoped, treat the mapping store as the most sensitive data the project holds, with deletion on request and no export path.
2. Carry the first-run disclosure through to account creation, so the user knows what linking discloses before linking.

## Notes
- **Affected components:** a future account service; onboarding disclosure.
- **Dependencies:** R-06.
- **Sequencing:** post-MVP.
- **Guardrails:** mapping never on chain; identity functional without an account.
- **Signals:** none until the tier exists.
- **Open questions:** whether the account blob for seed sync is offered at all, given that it makes the account service a custodian of an encrypted identity.

---

## Risk
**R-23. First-run disk and bandwidth doubling.** In the default mode a first run fetches and holds both the plaintext and the deployment's ciphertext for every dependency, so disk and download roughly double against npm alone. That is the price of independence and of seeding before holding, and it is the one observable cost of the first-run rule that is not latency.

## Impact
Medium for adoption on constrained machines and metered or slow connections; none for the foreground install, which the prefetch never delays.

## Likelihood
High. The condition is inherent in the default mode.

## Mitigation
Disclose the cost at install; run the prefetch in the background at low priority under the bandwidth, metered, and battery settings and the ciphertext store's quota; stop the prefetch, never the install, when the quota is reached; offer cache-only mode, which prefetches nothing.

## Seed Examples
- A developer on a metered connection installs a large project; the prefetch waits for an unmetered connection while the install completes at npm's speed.
- A machine with a small disk hits the ciphertext quota; later assets stay pending with the reason shown, and the plaintext install is unaffected.

## Mitigation Plan
1. Verify the prefetch obeys each setting and the quota in turn and that the foreground install's latency is unchanged with it stalled (PR-08, PR-11).
2. Record first-run disk and bandwidth per closure in the acceptance run alongside idle footprint (RO-07).

## Notes
- **Affected components:** resolution orchestrator, prefetch job, ciphertext store, installer consent flow.
- **Dependencies:** R-06 for the consent step.
- **Sequencing:** with the CAS and package host milestone.
- **Guardrails:** foreground isolation.
- **Signals:** pending assets accumulating with quota or metered reasons.
- **Open questions:** none.

---

# Additional Content

## Summary table

| ID | Risk | Impact | Likelihood | Status | Owner by role |
| --- | --- | --- | --- | --- | --- |
| R-01 | Unauthorized redistribution and irrevocability of ingested packages | Medium | Medium | Open, restate principle; optional policy narrowing | Project lead |
| R-02 | Cryptographic cost exceeds latency budget | High | Medium | Open, measurement | Cryptography implementer |
| R-03 | Delivery proof or verifier flaw | High | Low / Medium | Open, review | Cryptography implementer |
| R-04 | Execution breadth | High | High | Open, planning | Project lead |
| R-05 | Beachhead parity with existing tools | High | Medium | Open, positioning | Project lead |
| R-06 | Consumption privacy for developers | High / Medium | High | Open, inherited | Project lead |
| R-07 | Daemon as supply-chain surface | High | Medium | Mitigated by requirements | Daemon implementer |
| R-08 | Free-path abuse and subsidy | Medium | Medium | Open, funding; eased by the Base launch decision | Relayer operator |
| R-09 | Blocking selections | Medium | High | Open, decisions | Project lead |
| R-10 | Legal framings and regulation | Medium / High | Medium | Open, legal review | Project lead |
| R-11 | First Finder race and job failure | Medium | Low | Mitigated by requirements | First Finder implementer |
| R-12 | Chain properties change | Medium | Medium | Mitigated by adapters | Chain adapter implementer |
| R-13 | Escrow-salt custodian | Medium | Closed | Resolved: no commitment, no custodian | Project lead |
| R-14 | Adapter registry governance and scaffolding | Medium | Medium | Open, inherited | Project lead |
| R-15 | Settlement reversal | Low | Low | Stated property | Chain adapter implementer |
| R-16 | Verifier key compromise | High | Low | Mitigated by requirements | Claim verifier operator |
| R-17 | Secret leakage | High | Low / Medium | Mitigated by requirements | All implementers |
| R-18 | Installation misconfiguration | High | Low | Mitigated by requirements | Installer implementer |
| R-19 | Modified-client retention; common keys | Medium | High | Accepted residual | None |
| R-20 | Pre-claim grant liveness; escrow attribution | Low / Medium | High / Low | Accepted residual; upstream serves and requests pend | None |
| R-21 | Remote head rendezvous over claimed daemons | High when built / none in MVP | Low | Deferred; constraints recorded | Project lead |
| R-22 | Account-to-identity mapping as deanonymizing record | High when built / none in MVP | Medium | Deferred; constraints recorded | Project lead |
| R-23 | First-run disk and bandwidth doubling | Medium | High | Mitigated by disclosure, settings, quota, cache-only mode | Daemon implementer |

## Relationship to the other documents

The business case's Risks & Mitigation and SWOT, the critique's Threats and Problems, the technical approach's Risk Mitigation table, and the success metrics' Guardrails and Risk Signals all name subsets of these risks. This register is the consolidated list; where a companion document names a risk not here, the register is incomplete and should be extended rather than the companion trimmed. R-01, R-06, R-10, and R-14 are inherited from the source documents rather than introduced by the planning documents, and are recorded so that the decision to proceed is made with them visible.

## What this register does not cover

Deferred V2 items, seeder compensation, retention enforcement, Sybil resistance, per-entitlement variance, streaming, private content, and the post-claim pricing paradox, carry their own risks that the MVP does not encounter. They are held in the workplan To-Do list and are out of this register's scope by the same boundary that keeps them out of the MVP.
