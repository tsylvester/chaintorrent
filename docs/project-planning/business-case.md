<!-- Template: thesis_business_case.md -->
# ChainTorrent Business Case

Draft, 2026-09-23. Grounded in the research and MVP documents under [docs/research](../research/) as they stand at ledger log 36, and structured to the business case template in [docs/templates/proposal](../templates/proposal/thesis_business_case.md). Where a figure has not been measured, this document says so rather than estimating it; the MVP's cost instrumentation exists to replace those gaps with numbers.

# Executive Summary

ChainTorrent is a protocol for distributing encrypted digital assets over a peer swarm and resolving the right to decrypt them from public ledger state and the holder's own credential, with no service on the read path. The access right is a transferable bearer asset, an entitlement, that the seller cannot revoke and the buyer can resell. That is digital first sale, the property right platforms structured licensing to avoid, restored as a protocol invariant.

The first product is a drop-in distribution layer for JavaScript dependencies. A developer installs a Visual Studio Code extension or runs `npm install chaintorrent` and thereafter installs packages from a peer swarm and a machine-wide plaintext cache, with every package they have ever fetched reused across every project on the machine and available even when the upstream registry is down. Package managers are extended, not replaced. Monetization is $0.00 throughout the MVP, with one deliberately priced transaction to prove settlement works.

The cryptographic problem that blocked the design, distinct native decryption material per ownership interval with no provisioning network, is closed at the research level as of 2026-09-22 and adopted into the specifications. No requirement remains unmet except two preferences the project has already classed as unavailable. What remains is engineering: a validation harness to measure the construction on the launch curve, the Rust application stack, the contract suite, and a small number of blocking selections recorded in the workplan.

The recommendation is to fund the MVP as specified, beginning with the cryptographic validation harness, and to treat the developer-tooling MVP as the proof of distribution and identity that every later content class, and the project's own chain and tokens, will build on.

# Market Opportunity

The opportunity is layered, and the layers are reached in order through adapters rather than through separate products. No market has been sized in the source documents, and the MVP is designed to validate distribution and identity rather than willingness to pay, so this section describes the shape of the opportunity and states plainly that its magnitude is unmeasured.

**Beachhead: JavaScript dependency distribution.** Every JavaScript build on earth pulls from one registry operator. The population is every developer with a terminal, every CI pipeline, and every enterprise build farm. The content is public, permissively licensed, immutable, and already attested by an upstream integrity hash, which makes it the content class with the least legal and economic friction and the cleanest bootstrap. Value to the developer is cross-project reuse, swarm retrieval, and registry-outage survival; value to CI and enterprises is the outage survival, which they adopt as a consequence of individual adoption rather than ahead of it.

**Adjacent: every other package ecosystem.** PyPI, crates.io, the Go module proxy, pnpm, Bun, and any publisher's own release endpoint are peer implementations of the same package-host and ingest interfaces. Each is the same product with a different adapter, and each is another population that already installs tools from a terminal and tolerates a daemon.

**Expansion: priced content.** Media, publications, and software licensing are the content classes where an irrevocable, resellable entitlement is a product in its own right. General monetization is deferred from the MVP because token economics and fiat rails carry regulatory and implementation drag the MVP does not need, but the contract already treats price as a parameter, and changing it from zero requires no client refactoring. The streaming pipeline, forensic variance, and container objects are architecturally protected V2 items.

**Platform: the project's own chain, tokens, keystore, and swarm client.** Chain, tokens, wallet, keystore, and seeder client are third-party implementations now and first-party at maturity. Every touchpoint is specified as an adapter with declared capabilities so that the project's own stack arrives as new implementations rather than as protocol changes. That is the long-run position: the protocol layer beneath distribution and rights for any content class on any chain an adapter exists for.

**Timing.** Three conditions have recently aligned. Pairing precompiles on every EVM chain, and BLS12-381 precompiles on mainnet since the Pectra upgrade, make on-chain verification of credential delivery affordable. The software supply chain is converging on transparency logs and provenance attestations, which the protocol consumes as publisher proofs. And courts have drawn the line for machine-learning corpora between lawful acquisition and shadow-library acquisition, which tells a protocol exactly where its ingest policy has to sit.

# User Problem Validation

The problems below are established by the public record and by the structure of the systems involved rather than by user interviews, which the project has not conducted and does not treat as a gate. The MVP's cost instrumentation is the validation instrument: it measures whether the benefit the adopting population is promised is actually felt.

**Distribution is centralized where it need not be.** The npm registry is one operator serving every JavaScript build. A registry outage halts installs and CI pipelines globally; the 2016 unpublishing of a single eleven-line package broke builds across the ecosystem. The same single-operator dependence holds for every package ecosystem, every app store, and every streaming catalog. The content is immutable and public; the bottleneck is an accident of where the bytes live.

**Retained content is not retained.** Platforms sell access that evaporates: a subscription lapses and the library is gone, a store delists a title and the purchase is gone, a registry unpublishes and the build breaks. The transient copy is the platform's business model, not a technical necessity.

**Access rights are not transferable.** A purchased digital good cannot be resold, lent, or bequeathed, because licenses were structured specifically to escape the first-sale doctrine that governs physical goods. The user holds a permission, not a property.

**Hostile DRM has failed on its own terms.** Hardware enclaves, bespoke runtimes, and OS-level plaintext policing add friction for paying users while every protected asset still circulates unprotected. Piracy is a distribution and friction problem; making legitimate access seamless and inexpensive is the only mitigation that has ever worked.

**Existing swarm distribution has no identity or rights layer.** BitTorrent solved distribution twenty years ago and never solved who is allowed to read, who paid, and who may resell. It is infrastructure without economics.

**Developers already pay for local reuse.** pnpm's content-addressable store exists because installing the same package into every project separately is a felt cost. That store is machine-local and registry-dependent; the protocol generalizes it to a swarm and makes it survive the registry.

**How the MVP validates.** The plaintext CAS hit rate and cross-project reuse are the headline adoption metric because they are the benefit a developer at a terminal experiences. Interactive install wall-clock is measured against a latency budget declared before the run, so the instrumentation produces a pass or a failure rather than a number. If authorization makes installs noticeably slower for the local developer, the problem is not validated as solved and the parameters change before release.

# Competitive Analysis

No existing system offers the combination the protocol targets: one canonical encrypted swarm object, transferable and irrevocable on-chain entitlements, per-holder decryption capability with no service on the read path, and plaintext the user keeps. Each incumbent or adjacent system solves one layer and forecloses another.

| System or class | What it solves | What it lacks or forecloses |
| --- | --- | --- |
| Centralized registries (npm, PyPI, crates.io) | Canonical publication, metadata, integrity attestation | Single operator on the distribution path; outage halts every consumer; no rights layer; unpublishing breaks builds |
| Private registry proxies and mirrors (Verdaccio, Artifactory, Nexus, Cloudsmith) | Enterprise caching and outage insulation for one organization | Another central operator, per organization; no cross-organization sharing; no identity, entitlement, or transfer |
| Local content-addressable stores (pnpm, Yarn Berry cache) | Cross-project reuse on one machine through hardlinks | Machine-local; still depends on the registry for every first fetch; no swarm, no rights layer |
| BitTorrent and IPFS/Filecoin | Peer distribution, content addressing, durable storage | No encryption discipline, no identity, no entitlement, no transfer; anyone holding bytes can read them |
| Threshold-key access networks (Lit Protocol and similar NFT-gated decryption) | Token-gated decryption of encrypted content | A key-provisioning committee on the read path, which is exactly the threshold construction the research excluded: a liveness, centralization, and trust dependency at every read, and a shared content key wrapped per requester |
| Platform DRM (Widevine, FairPlay, PlayReady) | Rights enforcement acceptable to major rights holders | Hardware enclaves and bespoke runtimes; content usable only inside environments the platform controls; no transfer, no resale, no retained plaintext; the walled garden the protocol exists to dismantle |
| App stores and streaming platforms | Discovery, commerce, presentation | Revocable access, non-transferable licenses, single authoritative operator over content, entitlement, and transfer state |

**Where competition is welcome.** Discovery, presentation, commerce, indexing, and storage are intentionally permitted to be centralized businesses on top of the protocol. Their requirement is replaceability, not decentralization: none may become authoritative over content identity, entitlement ownership, or transfer state. Existing BitTorrent clients participate in the swarm unmodified through the compatibility adapter, and existing registries remain the ingest source the protocol bootstraps against.

**Where the protocol is structurally different.** Every competitor that enforces rights does so by putting a service or a device on the read path, and every competitor that avoids the read path enforces no rights. The credential KEM removes that trade-off: a sale delivers the buyer's credential from the seller's own, the contract verifies rather than decides, and reading is local against a state view. No committee holds a content secret and no service fields read-time traffic.

# Differentiation & Value Proposition

**For the developer:** install once, reuse everywhere, survive the registry. One install and one visible, reversible consent. No cryptocurrency to acquire for a free package; first-run gas is relayer-paid. Plaintext lands in a machine-wide cache and is theirs, usable with any toolchain, outside the authorization boundary by design. Their seeding contribution is visible like a torrent library, not hidden as a system cache.

**For the entitlement holder, in every content class:** digital first sale. An entitlement the publisher cannot revoke, deny, or block from transfer once minted; a credential delivered once and exercised locally; resale to any identity with settlement that pays the seller only on verified delivery and gives the buyer the credential only after payment is locked. The seller's participation in a sale is non-deniable by ordering, and no third party's availability sits on the use or transfer path.

**For the publisher:** unbounded, discretionary issuance with no protocol-imposed supply ceiling, acting as market maker over their own asset. Recoverable keys from a seed phrase. Authority rotation that never versions ciphertext or strands holders. Escrow that bootstraps their asset before they arrive and hands them complete protocol-level ownership when they claim it, with no rights drift.

**For the ecosystem:** a public entitlement ledger that makes applications running known-vulnerable versions visible, and makes paid distributions' consumption of open-source packages visible. Advisories that follow content across re-encryption because they target the plaintext fingerprint, so flags cannot be shed by repackaging.

**The design commitments that make the differentiation credible.** No hardware enclaves. No bespoke runtime; decrypted content runs in any compatible software. No takedown; governance is advisory metadata against a locally chosen trust set. No party may selectively withhold a holder's ability to decrypt or transfer. Every layer decentralized where authority matters and replaceable where it does not. Everything an adapter to an interface.

# Risks & Mitigation

**Technical residuals, accepted and stated.** A modified client can retain a decrypted credential past its interval, and the piece-group keys are common to every holder and exportable as a payload-proportional key file whose leak is unattributable. Both are consequences of one canonical ciphertext under a symmetric cipher without trusted hardware. Mitigations: the piece-group size bounds how much one leaked key unlocks; a leaked credential names its entitlement; the license makes producing a non-conforming client a violation; and the seam between decapsulation and the payload cipher is preserved so a future multi-key suite can close the residual as a successor deployment. The protocol claims bounded operation for the conforming client, not hostile DRM, and says so.

**Settlement reversal.** A buyer who read its envelope from a settlement that was then reorganized holds one entitlement's credential without a settled transfer. This is the same bounded quantity a seller could produce by leaking its own credential; it is forward-only and self-healing, and the declared settlement tier is the lever. Public free packages read at inclusion; priced deployments declare and justify a deeper tier.

**Abuse of the free path.** Relayer drain by flooding free mints, payment-lock griefing, RPC censorship of state reads, and compromise of the claim verifier's attestation key are each named in scope with a control: per-identity rate limits under relayer policy, short published lock expiries with refund, state views read from more than one node with inconsistency failing closed, and on-chain verifier-key rotation and revocation.

**Adoption.** The value proposition rests on cross-project reuse and outage survival being felt. Mitigation is the latency budget: if authorization makes installs noticeably slower for the local developer, the instrumentation fails the run rather than reporting a number, and the parameters change before release. The retention obligation, seeder compensation, and Sybil resistance are deferred until value is at risk, which is the point at which stake becomes available to condition them on.

**Legal.** Rights-holder exposure concentrates on the ingest path and is fenced by adapter-selection policy: only archives already free to use until the post-claim pricing paradox has an answer. Entitlement records are public, which for public dependencies is a security benefit and for private or personal content is a disclosure the specification treats as a separate case that no MVP adapter targets. Jurisdictional obligations for gateway and indexer operators are raised in the specification so the architecture is not retrofitted under pressure. The license conformance clause is a deliberate departure from OSI openness and requires legal drafting.

**Centralization scaffolding.** The relayer, the project seed host, and the claim verifier are centralized conveniences inside a decentralization thesis. Each is bounded: no client depends on the seed host, the verifier is one implementation behind an interface with a revocable key, and the relayer covers only the free path. They are named as scaffolding in the specification and expected to be replaced.

**Execution.** The surface is large and the completion boundary is strict: a mocked adapter, an off-chain-only verifier, or a manually prepared machine is development evidence, not completion evidence. The mitigation is the workplan discipline already in force, test-first and bottom-up, and the decision to build the validation harness before anything that depends on its measurements.

**Cryptographic review.** The research has proof sketches with concrete loss terms and an independent re-derivation, not a paper's proofs or an external audit. Mitigation: commission external review of the composition and the contract verifier before any priced deployment, and treat it as prudent before public First Finder ingest.

# SWOT

## Strengths

- The hard cryptographic problem is solved at the composition level, adopted into the specifications, and its residuals are stated rather than hidden.
- No service on the read path: reading is local, transfer is two-party, and the contract verifies rather than decides. No competitor that enforces rights has this property.
- Digital first sale as a protocol invariant, which is a product consumers want and platforms cannot offer without dismantling their model.
- A beachhead content class with public plaintext, permissive licenses, immutable artifacts, and a single registry to bootstrap against.
- Package managers extended rather than replaced, so dependency resolution, lockfiles, and workspaces remain the ecosystem's own solved work.
- Everything an adapter, so new chains, curves, transports, ecosystems, and the project's own stack arrive without protocol change.
- A specification discipline that classifies every rule as invariant, defined, unresolved, or implementation choice, and a requirements document whose every requirement carries an end-to-end proof obligation.

## Weaknesses

- No code exists; the repository holds specifications, research provenance, and a contract sketch.
- Cost is unmeasured: no byte, latency, proof, gas, or settlement figure has been produced, and the piece-group size and curve wait on the harness.
- The engineering surface is large for the first release, spanning cryptography, networking, durable jobs, platform installation on every supported OS, contracts, and four control surfaces.
- The MVP validates the mechanism but not its load profile; install-once content crosses the attempt boundary rarely, so streaming costs remain unobserved.
- Modified-client retention and common piece-group keys are accepted residuals that a sophisticated rights holder will notice.
- First-run friction depends on a subsidized relayer, a centralized chokepoint the project must fund and eventually replace.
- Several blocking selections remain open, including key custody, launch network, and the escrow-salt custodian.

## Opportunities

- Every other package ecosystem is a peer adapter and a new population.
- Priced content classes, where an irrevocable resellable entitlement is a product in itself, once price moves from zero.
- Seeder compensation and retention enforcement over a Sybil-resistant participant set, turning distribution into an economy.
- Forensic attribution against unmodified clients through per-entitlement variance, with no change to the swarm object.
- The project's own chain, tokens, keystore, and swarm client, arriving as adapters.
- Supply-chain security as a byproduct: a public dependency inventory that makes known-vulnerable versions visible and makes paid products' open-source consumption visible.
- Provenance attestations and transparency logs becoming standard, which the publisher-proof adapter consumes directly.

## Threats

- Rights-holder legal action on the ingest path if adapter-selection policy is ever crossed before the post-claim pricing paradox is answered.
- Regulatory drag on token economics and fiat rails, which is why general monetization is deferred.
- Consumption privacy: the entitlement ledger is a permanent, correlatable map of what each identity runs, and a solo participant's install history is a behavioral profile.
- A registry operator or platform mounting a hostile response to redirection of package managers, or shipping an outage-insulation feature that captures the beachhead benefit.
- A published key-set leak for a high-value deployment, which decrypts the swarm copy for everyone and attributes to no one.
- Dependence on chain properties the project does not control: precompile availability on the launch L2, settlement latency, RPC censorship.
- Loss of momentum if the harness measurements show authorization cost exceeding the latency budget for the local developer.

# Next Steps

1. **Build the cryptographic validation harness first.** Implement the credential KEM, envelope, and delivery proof on both candidate curves, exercise the deployed verifier on a test chain, and record capsule, envelope, and proof sizes, decapsulation time per piece group, proof generation and verification time, and delivery gas for a mint and a transfer.
2. **Resolve the measurement-gated selections from the harness output:** the launch network and curve, the piece-group size, and the attempt-rule parameters against a latency budget declared before the run. Re-run the whole-target evaluation with measured costs in hand.
3. **Resolve the remaining blocking selections** recorded in the workplan: the default key-custody implementation against its declared capability list, the escrow-salt custodian, the license text and its OSI question, custody recovery UX, and the funding and sizing of the relayer grant pool.
4. **Author workplan nodes** from the adopted specifications in test-first, bottom-up dependency order, beginning with the protocol and domain ring.
5. **Commission external cryptographic review** of the composition and the contract verifier before any priced deployment.
6. **Build in phases:** protocol core and contract suite; local daemon and package serving; onboarding shells and services; acceptance and release against all twenty-seven packaged scenarios on clean machines.
7. **Dogfood at release:** publish ChainTorrent itself through the explicit-publisher path, which seeds the swarm with its first content, exercises First Finder for its dependencies, and supplies the only assets that may carry a price.

# References

- [cryptography.md](../research/cryptography.md), the Transactable Key Protocol cryptographic specification: invariants, primitives, adapters, protocol flow, security considerations, and open problems.
- [MVP Scope.md](../research/MVP%20Scope.md), the boundary of the MVP and the items deferred to V2 with their architectural protection.
- [MVP Application Requirements.md](../research/MVP%20Application%20Requirements.md), the deployable applications, module catalogue, and acceptance scenarios with their proof obligations.
- [MVP Execution Trace.md](../research/MVP%20Execution%20Trace.md), the state machine a package request follows.
- [cryptography-critical-path.md](../research/cryptography-critical-path.md), the current statement of the construction, its accepted residuals, and the next unit of work.
- [cryptography-requirements.md](../research/cryptography-requirements.md), the fourteen research requirements and fifteen preferences at their current readings.
- [cryptography-research-notebook.md](../research/cryptography-research-notebook.md), the append-only ledger of settled reasoning, exclusions, evidence, candidates, and decisions.
- [The Foundations of Internet 3.0](https://medium.com/@TimSylvester/the-foundations-of-internet-3-0-237df6013582), the originating essay.
- [EIP-196](https://eips.ethereum.org/EIPS/eip-196), [EIP-197](https://eips.ethereum.org/EIPS/eip-197), and [EIP-2537](https://eips.ethereum.org/EIPS/eip-2537), the EVM pairing precompile interfaces the delivery proof is verified through.
- Bartz v. Anthropic, the ruling distinguishing use of lawfully acquired works from shadow-library acquisition, which locates rights-holder exposure on the ingest path.

# Additional Content

## The Solution

ChainTorrent composes four existing things and adds the one that was missing.

- **A content-addressed swarm carries encrypted objects.** One canonical ciphertext per deployment, byte-identical for every participant, verified piece by piece with BLAKE3/Bao so that any peer can serve any piece and no peer can serve a bad one. Seeders hold opaque bytes and learn nothing; possession of ciphertext confers no access.
- **A public ledger records who holds what.** Each entitlement is a single-owner bearer asset, an ERC-721 token in the MVP, bound to the asset rather than to any one encryption of it. Ownership and transfer are consensus facts, not entries in a platform database.
- **A per-interval native credential decrypts.** Every ownership interval receives its own cryptographic credential, delivered inside the settlement that creates the interval and verified by a proof the contract checks. The credential is constant size, decrypts every live deployment of the asset, and is rerandomized by the seller at each sale without any publisher or service present.
- **The client authorizes itself.** Before every decryption attempt the client reads a fresh view of chain state, confirms it holds the entitlement at its credential's interval, proves control of its own key to its own session, and decrypts locally. No party is asked for anything at read time. When the entitlement transfers away, the conforming client stops and destroys its decrypt-capable material.
- **What was missing:** distinct decryption capability per holder against one shared ciphertext, without a key-provisioning committee, a publisher on the read path, or trusted hardware. That is the construction the research closed.

The protocol governs the transition from ciphertext to plaintext and makes no claim over plaintext thereafter. A user who has decrypted content keeps it and uses it in whatever software they choose. That is the deliverable, not a leak.

## Strategic Thesis

**Markets follow value; incumbents do not consent.** Netflix's DVD business needed no license because first sale conferred the rental right. Newspapers, advertising, and most recently machine-learning corpora were rebuilt without the incumbents' agreement. ChainTorrent does not seek rights-holder buy-in as a precondition and treats rights-holder reaction as a threat model, not a gate.

**The threat lands on the ingest path, and the ingest path is already fenced.** The swarm carries ciphertext, entitlements are auditable bearer assets, transfer is ordered and irrevocable; none of those is where a claim attaches. The claim attaches to how bytes were acquired, which is the distinction courts drew against shadow-library acquisition while sustaining use of lawfully acquired works. The specification therefore confines ingest adapters to archives whose content is already free to use until the post-claim pricing question has an answer. The MVP targets npm, where the plaintext is public, the license is permissive, and an arriving maintainer was denied no revenue by free entitlements.

**Digital first sale is the product.** An entitlement that the seller cannot revoke and the buyer can resell is a property right, not a license. That property is what makes the protocol attractive to consumers without asking publishers to volunteer for it, and it is the reason the design forbids selective withholding at every layer.

**Developer tooling is the beachhead because it is the content class with the fewest obstacles.** Public plaintext, permissive licenses, immutable artifacts, a single dominant registry to bootstrap against, and an adopting population that installs tools from a terminal and tolerates a daemon. It validates distribution and identity without touching willingness to pay. The same protocol, with different adapters and priced entitlements, is the media, publishing, and software-licensing product.

## What the MVP Proves

The MVP is releasable only when twenty-seven packaged acceptance scenarios pass against real application boundaries. In business terms it proves five things.

- **The install path works for a stranger.** A clean machine, either entry point, permitted consent only, and a supported package installs. Reinstall is idempotent, interruption resumes or rolls back, repair restores service, uninstall restores the prior registry configuration.
- **Distribution survives the registry.** After an asset is ingested, a second clean identity installs it with npm unavailable, using only the ledger, the swarm, and a grant authored by any online holder.
- **Bootstrap is non-blocking and race-safe.** An initiating install completes as soon as upstream bytes are verified; encryption, registration, and seeding continue as durable background jobs, and concurrent First Finders resolve to one canonical winner.
- **The cryptography holds end to end on chain.** Mint and transfer proofs verify through the chain's pairing precompiles; malformed, replayed, and cross-entitlement proofs are rejected; a lapsed payment lock refunds; a seller cannot be paid without delivering and a buyer cannot obtain the credential before payment is locked.
- **A priced transfer settles.** The project publishes its own package, prices it trivially above gas, a funded buyer acquires it, resells it, and the settlement boundary is asserted: the buyer decrypts at the declared tier and the seller's client destroys at hard finality. This is the specification's most contested invariant and the one whose failure mode is silent.

**What the MVP deliberately does not prove:** willingness to pay, price discovery, resale volume, seeder compensation, Sybil resistance, and the load profile of streaming content. Per-attempt authorization is built in full but exercised lightly, because a package decrypts once and lives in the cache. Cost instrumentation exists so the MVP still yields a measured baseline for the content classes that will exercise it hard.

## Current State

**Research is closed on the critical path.** The entitlement cryptography research ran from 2026-09-15 to 2026-09-22 across two maintainers and an independent re-derivation agent. The chosen construction is a depth-one Boneh–Boyen identity-based KEM in Type-3 pairing groups with seller-side rerandomization, ElGamal credential envelopes, and a chained Schnorr delivery proof verified through EVM pairing precompiles. Every one of the fourteen requirements is met at the protocol-composition level; the composition claims have proof sketches with concrete loss terms under SXDH, decisional BDH-3b, and the random-oracle model; the relations and reductions were independently re-derived and reconciled. Two preferences are recorded as unavailable rather than open: cryptographic loss of capability against a modified client, and removal of the common piece-group keys. Both are consequences of one canonical ciphertext without trusted hardware, and both are stated in the specification rather than hidden.

**The specifications are adopted.** cryptography.md, MVP Scope, MVP Application Requirements, and the Execution Trace carry the construction as the MVP posture: native credentials, verified delivery, no provisioning committee, per-attempt authorization on the settlement adapter's tiers, parameter sets with a sidecar-coverage rule, and First Finder escrow that no claim ever depends on. Three previously open problems are marked resolved in place.

**No code exists.** The workplan holds a pre-WBS note and a To-Do list and no authored nodes. The repository contains specifications, research provenance, a Solidity sketch, and agent process rules.

**Blocking selections are enumerated, not hidden.** Held in the workplan's To-Do list: the default key-custody implementation; the launch network and therefore the pairing curve; the escrow-salt custodian; the attempt-rule parameters and piece-group size, which the validation harness measures; the license text and its OSI question; and custody recovery UX. Each has a determinant path recorded in the specifications and none is a research question.

## Delivery Plan

The Application Requirements fix the implementation language as Rust with Tauri for GUI surfaces, thin TypeScript only where Visual Studio Code and npm impose it, and Solidity for the EVM contract suite. Dependencies point inward through three rings: protocol and domain, application workflows, adapters and host shells. Every adapter declares capabilities and composition fails closed at resolution.

**Phase 1, cryptographic validation harness.** The first thing built. It implements the credential KEM, envelope, and delivery proof on the resolved pairing adapter, exercises the deployed verifier on a test chain, and produces capsule, envelope, and proof sizes, decapsulation time per piece group, proof generation and verification time, and delivery-verification gas for a mint and a transfer. The piece-group size and the curve are chosen from those measurements, not asserted from formulas. This phase also runs the whole-target second pass with measured costs in hand.

**Phase 2, protocol core and contract suite.** The authoritative Rust domain types and state machines; the contract suite covering canonical identity, authenticated deployments, parameter sets, envelope-key registration, entitlements with interval state, issuance and transfer with delivery verification, escrow, identity binding, and claims; the chain, settlement, and entitlement-state adapters; the cryptographic services for AES-256-CTR, BLAKE3/Bao, signatures, and the pairing adapter on both candidate curves.

**Phase 3, local daemon and package serving.** The single-instance package host, plaintext CAS with atomic commit and symlinking, ciphertext store under the seed host adapter, resolution orchestrator, First Finder durable jobs, swarm transport and discovery adapters including BitTorrent compatibility, and per-attempt authorization and decryption.

**Phase 4, onboarding shells and services.** The Visual Studio Code extension, the npm bootstrap package, the Tauri desktop application, the CLI, the relayer or paymaster, the claim verifier, the project seed host and site, and the demonstration harness.

**Phase 5, acceptance and release.** All twenty-seven scenarios through the packaged applications on clean machines, metrics reconciled against induced activity, and the completion boundary of the Application Requirements met. Dogfooding is part of release: publishing ChainTorrent itself through the explicit-publisher path is what seeds the swarm with its first content and supplies the only assets that may carry a price.

Ordering within phases follows the workplan's test-first, bottom-up dependency discipline. Nodes are authored from the specifications once the harness measurements resolve the parameters that gate them.

## Resources and Costs

No cost model exists yet and this document does not invent one. What can be stated is the shape of the spend.

- **Engineering.** Rust systems work across cryptography, networking, storage, durable jobs, platform installation on every supported OS, and Tauri; Solidity contract development and audit; a thin TypeScript extension. The Application Requirements enumerate on the order of one hundred fifty numbered operational requirements, each carrying an integration or end-to-end proof obligation, which is the honest measure of the surface.
- **Cryptographic review.** External review of the composition and of the contract verifier is a release prerequisite for anything that carries a price, and a prudent one before public First Finder ingest.
- **Operating subsidy.** A developer grant pool funds the relayer that pays for free mints and the one-time identity binding per new identity. This is acknowledged scaffolding: a centralized chokepoint and an unmetered subsidy that exists to remove first-run friction and is expected to be replaced. Per-attempt reads are view calls and cost no gas. The MVP records relayer gas per install so this number stops being a guess.
- **Infrastructure.** The project seed host and site, an optional discovery endpoint, and test-chain deployment. The seed host is the first seeder, not a privileged one; nothing depends on it.
- **Legal.** License drafting for a source-available license whose conformance clause makes a non-conforming client a violation, and the content-terms field carried per asset.

## Unit Economics and Success Metrics

The MVP's cost instrumentation is designed to produce the first real unit-economics datapoints rather than to assume them. Success is defined against these, with the latency budget declared before the run so instrumentation produces a pass or a failure rather than an unqualified number.

| Metric | Why it matters | Source |
| --- | --- | --- |
| Plaintext CAS hit rate and cross-project reuse | The benefit the adopting population experiences; the headline adoption metric | Every install |
| Interactive install wall-clock, and the fraction authorization adds | The binding constraint for a developer at a terminal; a doubled install is noticed | Every install, against a declared budget |
| State-read volume and latency per install | Empirical input to the freshness bound and to whether reading at the declared tier is noticeable | Every install |
| Relayer gas per install and swarm bandwidth per install | Cost per install, the first unit-economics datapoint, nearly free to capture | Every free acquisition |
| Capsule, envelope, and proof bytes; decapsulation, proof, and verification time; delivery gas per mint and transfer | Inputs to the piece-group size and curve choice; the cost of the cryptography at the point it is actually exercised | Validation harness and every settlement |
| Swarm-native publication coverage | Whether publishing a package leaves its whole dependency closure in the swarm | Dogfood publication |

The economic half of the question, what a single-digit protocol fee would have to cover, is not discoverable at this stage and carries no threshold. The MVP produces cost per install; it does not produce revenue per install, by design.

## Path to Value

The MVP is a proof of distribution and identity. Value follows in an order the architecture already protects.

1. **Adoption in the developer population** through the benefits above, measured by CAS reuse and install counts, and extended to other package ecosystems through peer package-host and ingest adapters.
2. **Paid monetization** by changing an entitlement's price parameter from zero, which requires no client refactoring. General monetization is deferred because token economics and fiat rails carry regulatory and implementation drag the MVP does not need.
3. **Seeder compensation and retention enforcement**, which resolve together over a Sybil-resistant participant set and attach to the discretionary retention tier the store interface already records.
4. **Per-entitlement variance** for forensic attribution against unmodified clients, added as a variant root on the entitlement and confidential material in the envelope, with no change to the swarm object.
5. **Other content classes**: media and streaming through a new client pipeline, publications and generic content through identity adapters, containers through reference nesting, and private content only once a consumption-privacy answer exists.
6. **The project's own chain and tokens**, arriving as adapters alongside a first-party keystore and swarm client, which is why every touchpoint is abstracted now.

## Recommendation

Proceed with the MVP as specified. Fund the cryptographic validation harness immediately, since every parameter that gates the rest of the build depends on its measurements and it is the first cryptographic artifact the project produces. Author the workplan nodes from the adopted specifications in dependency order once the harness resolves the curve and piece-group size. Commission external cryptographic review of the composition and the contract verifier before any priced deployment. Hold the deferred items deferred: the architecture protects them, and pulling them forward would spend effort on economics the MVP is designed not to test.

The case rests on three facts established in the research: the hard cryptographic problem is solved at the composition level with its residuals stated; the first content class carries the least legal and economic friction while exercising the full identity, entitlement, delivery, encryption, and transfer lifecycle; and the design is abstracted so that everything after the MVP is an adapter, including the project's own stack.
