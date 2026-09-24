<!-- Template: thesis_business_case.md -->
# ChainTorrent Business Case, Revised

Draft, 2026-09-23. A fresh authorship of the business case that synthesizes everything the planning work has established: the [original business case](business-case.md) and its [critique](business-case-critique.md), the [feature specifications](feature-spec.md), [success metrics](success-metrics.md), [technical approach](technical-approach.md), [risk register](risk-register.md), [non-functional requirements review](non-functional-requirements.md), [dependency map](dependency-map.md), and [technical feasibility assessment](technical-feasibility.md), all of which derive from the research and MVP documents under [docs/research](../research/) at ledger log 36 and the [workplan](../workplans/current/ChainTorrent%20MVP.md). Every claim traces to one of those documents except where a passage is labelled external context, which is the only place claims from outside the repository appear. No figure the sources leave unmeasured is estimated here; where a number is absent, the document says what would produce it and when.

# Executive Summary

ChainTorrent is a protocol that distributes encrypted digital assets over a peer swarm and lets the holder of an access right decrypt them from nothing but their own credential and a fresh view of public ledger state. The access right is a transferable bearer asset, an entitlement, that the seller cannot revoke and the buyer can resell. The protocol constructs that property directly; it does not borrow it from any doctrine. No service sits on the read path, no committee holds a content secret, and the plaintext a user decrypts is theirs to keep and use in any software.

The first product is a distribution layer for JavaScript dependencies that extends npm rather than replacing it. A developer installs a Visual Studio Code extension or runs `npx chaintorrent`, gives one visible and reversible consent to redirect their package manager, and thereafter installs packages from whichever of a machine-wide cache, a peer swarm, or the registry delivers first, never slower than the registry alone, while the daemon quietly obtains an entitlement and the swarm copy for every dependency so that a first run leaves the machine independent of the registry for everything it installed. What the swarm adds that nothing on the developer's machine already offers is this: it holds what anyone has fetched, not only what this machine has, and it keeps serving when the registry does not. Every install is $0.00. One priced transaction, on a package the project publishes itself, exists to prove that settlement works.

The case for building rests on three established facts and one honest limit. The hard cryptographic problem, distinct decryption material per ownership interval with no provisioning network, was open on 2026-09-15 and closed at the research level on 2026-09-22 with a construction, proof sketches with concrete loss terms, and an independent re-derivation; it is adopted into the specifications. Every engineering component is conventional Rust, Solidity, and platform work; the feasibility assessment rates technical feasibility high. The design is abstracted so that every later content class, every other package ecosystem, and the project's own chain and tokens arrive as adapters rather than as protocol changes. The limit is delivery: no code, team, timeline, or budget exists, the surface is wide, and the completion boundary is strict. The cryptographic validation harness is designed to be the calibration that turns delivery from unknown to estimated.

The recommendation is to build the harness now, run legal and cryptographic review in parallel with it, adopt the stop criteria stated below, and treat the MVP as the proof of distribution and identity that everything after it builds on.

# Market Opportunity

The opportunity is layered and reached in order through adapters. No market has been sized in any source and the MVP is designed not to test willingness to pay, so this section describes shape and states magnitude as unmeasured.

**Beachhead: JavaScript dependency distribution, adopted by individual developers.** Every JavaScript build pulls from one registry operator. The adopting population is developers at a terminal, and the benefit they feel is ordered: swarm availability of the long tail that no local cache holds, registry-outage survival for anything already ingested, and machine-wide reuse across projects. The workplan's problem statement supplies the adoption mechanism the original case lacked. Build platforms running the same few thousand popular installs continuously on always-on machines hold the ideal cache as a by-product of their own economics, and seeding it costs them almost nothing once it exists; they become superseeders for their own reasons rather than by recruitment. CI and enterprise adoption follow that stage as a consequence. The sequence is individual developers, then build platforms, then CI and enterprise, and it determines what the MVP measures: interactive install latency against a developer's tolerance, not a pipeline's.

**Adjacent: every other package ecosystem.** PyPI, crates.io, the Go module proxy, pnpm, Bun, and a publisher's own release endpoint are peer implementations of the same package-host and ingest interfaces. Each is another population that already installs tools from a terminal.

**Expansion: priced content.** Media, publications, and software licensing are where an irrevocable, resellable entitlement is a product in itself and where the per-attempt authorization mechanism, which install-once content exercises lightly, is exercised continuously. General monetization is deferred because token economics and fiat rails carry regulatory and implementation drag the MVP does not need; the contract already treats price as a parameter, and changing it from zero requires no client refactoring.

**Platform: the project's own chain, tokens, keystore, and swarm client.** Chain, tokens, wallet, keystore, and seeder client are third-party implementations now and first-party at maturity. That is why every touchpoint is an adapter with declared capabilities from the first implementation.

**Timing.** Pairing precompiles exist on every EVM chain, and Base carries the BLS12-381 precompiles the delivery verifier uses as its primary form. The software supply chain is converging on provenance attestations and transparency logs, which the protocol consumes as publisher proofs. And the project's own research has just closed the problem that made the design impossible a week earlier.

# User Problem Validation

Validation is by the public record and by the structure of the systems involved, not by interviews, which the project has not conducted. The MVP's instrumentation is the validation instrument for the developer population: it measures whether the promised benefit is felt and fails the run if authorization makes installs noticeably slower.

**The problems the MVP addresses for developers.**

- **One operator serves every build.** A registry outage halts installs and CI globally. The content is immutable and public; the bottleneck is where the bytes live.
- **A machine's cache holds only what that machine fetched.** Local content-addressable stores solve reuse within a machine and nothing across machines. The first fetch of any package on any machine still needs the registry.
- **Unpublishing breaks builds.** A publisher can withdraw a package the world depends on. The swarm forecloses that for consumers, and forecloses it for publishers too, which is stated under Risks as the cost it is.

**The problems the protocol addresses for later content classes, and does not claim to solve in the MVP.** Access that evaporates when a subscription lapses; licenses structured so a purchased digital good cannot be resold or lent; hostile DRM that adds friction for paying users while every protected asset still circulates. The original case listed these as validated problems for a dependency product. They are the reason the protocol has the shape it has, and they are invisible to a developer installing a free package. The MVP proves the mechanism that addresses them; it does not sell it.

**What the MVP's own north star tests.** The success metrics make plaintext CAS reuse the north star and interactive install wall-clock the guardrail. The first measured values, on the dogfood population, are the baseline. If the benefit is not felt there, the case does not proceed to the adopting population.

**External context, not from the repository.** The 2016 removal of one eleven-line package from npm broke builds across the ecosystem, which is the canonical illustration of the unpublishing problem. pnpm's content-addressable store exists because per-project installation of the same package is a felt cost, and it is prior art for the MVP's cache. Neither claim is in the source documents.

# Competitive Analysis

No existing system offers one canonical encrypted swarm object, transferable and irrevocable on-chain entitlements, per-holder decryption with nothing on the read path, and plaintext the user keeps. Each adjacent system solves one layer and forecloses another. The structural claim that matters: every system that enforces rights puts a service or a device on the read path, and every system that avoids the read path enforces no rights. The credential construction removes that trade-off.

| System or class | What it solves | What it lacks or forecloses |
| --- | --- | --- |
| Centralized registries | Canonical publication, metadata, integrity attestation | Single operator on the distribution path; unpublishing breaks consumers; no rights layer |
| Private registry proxies and mirrors | Caching and outage insulation for one organization | Another central operator per organization; no cross-organization sharing; the same redistribution posture as the swarm but serving one organization's licensees rather than the public |
| Local content-addressable stores | Cross-project reuse on one machine | Machine-local; registry-dependent for every first fetch; no swarm, no rights layer. This is parity for the MVP's reuse benefit, and the case says so |
| BitTorrent and content-addressed storage networks | Peer distribution, content addressing, durability | No encryption discipline, identity, entitlement, or transfer; anyone holding bytes reads them |
| Threshold-key access networks | Token-gated decryption of encrypted content | A key-provisioning committee on the read path, which is the construction the project's research excluded as a liveness, centralization, and trust dependency at every read, wrapping a shared content key per requester |
| Platform DRM | Rights enforcement acceptable to major rights holders | Hardware enclaves and bespoke runtimes; content usable only inside environments the platform controls; no transfer, resale, or retained plaintext |
| App stores and streaming platforms | Discovery, commerce, presentation | Revocable access, non-transferable licenses, one authoritative operator over content, entitlement, and transfer state |

**Where the protocol invites competition.** Discovery, presentation, commerce, indexing, and storage are intentionally permitted to be centralized businesses on top of the protocol. Their requirement is replaceability: none may become authoritative over content identity, entitlement ownership, or transfer state. Existing BitTorrent clients participate unmodified through the compatibility adapter, and existing registries remain the ingest source the protocol bootstraps against.

**The honest competitive position at the beachhead.** For machine-wide reuse the MVP is at parity with the best local store. For enterprise outage insulation it is at parity with a private proxy. Its advantage for the developer is the long tail and the accumulation of the swarm with adoption, and its advantage for the ecosystem is a rights layer that no developer will notice at $0.00. The case does not pretend otherwise.

**External context, not from the repository.** The product names behind the classes above, private proxies such as Verdaccio and Artifactory, local stores such as pnpm and Yarn Berry, threshold networks such as Lit Protocol, and platform DRM such as Widevine and FairPlay, are drawn from general knowledge. The structural characterizations are accurate; the names are not sourced from the project's documents.

# Differentiation & Value Proposition

**For the developer.** Install once through either entry point, give one visible consent, and thereafter install from whichever of cache, swarm, or registry delivers first, never slower than the registry alone, with no cryptocurrency to hold and no account to create; first-run gas is relayer-paid. A first run leaves the machine independent of the registry and of every other participant for everything it installed, because the daemon requests an entitlement for every dependency and fetches the swarm copy in the background while the registry serves the install. Packages already in the swarm install when the registry is down. Plaintext lands in a machine-wide cache, is reusable by any toolchain, and is outside the authorization boundary by design. What the developer contributes, and what they should know before they consent: the daemon runs continuously, holds a plaintext cache and a separate ciphertext store on their disk, which roughly doubles a first run's disk and bandwidth, seeds ciphertext upstream to peers, and records their entitlements and, at first run, their requests for them on a public ledger, which publishes the dependency closure at once; the request and prefetch behaviors are consent items and a cache-only mode with no chain identity exists for a developer who wants only the local cache and the registry. The seeding archive is visible as their contribution; the ciphertext they hold is unreadable to them and to anyone without a credential; and the ledger exposure is stated plainly under Risks. The daemon exposes bandwidth and idle limits so the contribution is theirs to bound.

**For build platforms and CI.** A platform running popular installs continuously holds the swarm's ideal cache as a by-product and can seed it at almost no marginal cost, which makes it a superseeder without being asked. What is not yet designed, and is recorded as open: how an ephemeral CI runner obtains an identity, whether each needs a relayer-paid binding, and how per-identity rate limits on free grants interact with fleets. These are onboarding-phase questions and the case does not claim they are answered.

**For the entitlement holder, in every content class.** An entitlement the publisher cannot revoke, deny, or block from transfer once minted; a credential delivered once, inside the settlement that records the entitlement, and exercised locally; resale to any identity with settlement that pays the seller only on verified delivery and gives the buyer the credential only after payment is locked. The seller's participation in a sale is made non-deniable by ordering, and no third party's availability sits on the use or transfer path. This is the protocol's construction of what first sale gives owners of physical goods; it is an analogy to that doctrine, not an invocation of it.

**For the publisher.** Unbounded, discretionary issuance with no protocol-imposed supply ceiling. Keys recoverable from a seed phrase. Authority rotation that never versions ciphertext or strands holders. Escrow that bootstraps an asset before the publisher arrives and hands over complete protocol-level ownership on claim with no rights drift, and no dependency on the party that bootstrapped it. What the publisher gives up, for content the swarm carries: the ability to withdraw it. Governance is advisory metadata against a locally chosen trust set, and a publisher's objection reaches conforming clients through that channel rather than through takedown.

**For the ecosystem.** Provenance attestations recorded on chain. Advisories that follow the plaintext fingerprint across repackaging, so a flag cannot be shed by re-uploading under a new name. A public record of entitlement acquisition and ownership, from which bounded inferences follow: which versions have been acquired and by which identities, never which project runs them, because retained plaintext and cache reuse sit outside authorization by design and an entitlement can be acquired and never installed, or transferred away while its plaintext stays in use. Whether a known-vulnerable version is installed anywhere, or what a paid product actually consumes, would need build or project attestations the MVP does not produce. The same record is a privacy exposure for individuals, and the case carries it under Risks rather than presenting only this half.

**The design commitments that make the differentiation credible.** No hardware enclaves. No bespoke runtime. No takedown. No party may selectively withhold a holder's ability to decrypt or transfer. Every layer decentralized where authority matters and replaceable where it does not. Everything an adapter to an interface, including the parts the project intends to own.

# Risks & Mitigation

The [risk register](risk-register.md) holds the identified risks with impact, likelihood, affected components, and signals. This section carries the ones that bear on the decision to proceed, at the register's ratings.

**Delivery breadth, High impact, High likelihood.** Every operational requirement with a boundary-crossing proof, every acceptance scenario on clean machines, four control surfaces, signed installation and service lifecycle on every supported platform, a contract suite, three services, and a demonstration harness that the completion boundary makes load-bearing. No code, team, timeline, or budget exists. Mitigation: the foundation and harness groupings are mapped at ticket resolution, with the swarm and signature milestones beside them, and resolution decays outward from there; the harness's measured throughput is the calibration; the platform matrix is Windows, macOS, and Linux; a demonstrable milestone after credential delivery produces something to show and measure before publishing and claims.

**Cryptographic cost against the latency budget, High impact, Medium likelihood.** Nothing is measured. Mitigation: the harness measures sizes, timings, and gas on both curves before any parameter is fixed; the latency budget is declared before the acceptance run so instrumentation produces a failure, not a number; install-once content crosses the attempt boundary rarely.

**Beachhead parity, High impact, Medium likelihood.** Machine-wide reuse is available today from local stores. Mitigation: the value proposition leads with the long tail and accumulation; the superseeder mechanism is the adoption engine; the north star is measured on the dogfood population before the adopting population is asked.

**Consumption privacy for the adopting population, High impact for adoption, High likelihood.** The entitlement ledger is a permanent public record of what each identity runs, and state-read traffic shows a serving node the same associations in real time. The specification's argument that public consumption is a benefit concerns organizations concealing inherited risk; it says a solo participant's install history is a behavioral profile and that pseudonymity only partially helps. Individual developers are the adopting population. Mitigation in the MVP: disclose the exposure at onboarding; prefer state reads from the client's own node where available; record the candidate constructions, ephemeral wallets, blinded reads, aggregate settlement, zero-knowledge entitlement proofs, as the design question that gates any private-registry adapter.

**Verifier and composition soundness, High impact, Low to Medium likelihood.** The research has proof sketches with loss terms and one independent re-derivation, not a paper's proofs or an audit; encoding, subgroup checks, and context binding are where such systems fail in implementation. Mitigation: the Rust verifier is the reference and the Solidity verifier's constants and vectors are generated from it; every statement field is mutated and every proof replayed in the harness vectors; external review starts during the harness on the composition claims and gates the priced deployment.

**The daemon as a supply-chain surface, High impact, Medium likelihood.** A local registry endpoint redirecting the package manager on every developer machine is exactly what an attacker would target, and the protocol's own signed update channel is a second surface. Mitigation: authenticated artifacts and updates before execution, verified content addressing before anything is served, authenticated least-privilege IPC, fuzzing at every boundary, and a signing-key custody and rotation discipline that the non-functional review found missing and proposes.

**Unauthorized redistribution and irrevocability of ingested packages, Medium impact, Medium likelihood.** The ingest principle is that adapters target content publicly distributed at no charge by the rights holder's own choice, and the MVP implements it as public npm availability. The two are close: npm serves every public package to anyone at no charge regardless of its license, so a user obtains nothing from the swarm they could not already obtain from npm, and no revenue is denied, which is why the post-claim pricing paradox does not arise for this corpus. What changes is who distributes and whether it can be withdrawn. First Finders and seeders redistribute to the public without the grant the publisher gave npm, which some restrictive licenses forbid, and the swarm is irrevocable by design. Public npm mirrors carry the first of those today and it is largely uncontested; a swarm serving strangers and refusing withdrawal is a different posture. Mitigation: the principle and its residual are stated in the specification and MVP Scope; the MVP ingests whatever npm serves, and a license check at ingest is a later policy decision; publishers get a visible objection channel through advisories.

**Legal framings and regulation, Medium impact for the MVP, Medium likelihood.** Digital first sale and the fair-use line drawn for machine-learning corpora are analogies; neither transfers to verbatim redistribution: first sale for digital copies has been rejected where transfer makes a reproduction, and the fair-use holding concerned transformative use. What limits the MVP's exposure is that every package it ingests was distributed to the public at no charge by its publisher. Separately, tradeable priced entitlements, even the nominal dogfood transaction, and a paymaster subsidizing transactions may attract securities, consumer-protection, or money-transmission scrutiny that no document examines. Mitigation: use first sale as the analogy the specification uses; commission legal review scoped to the license text, the eligibility rule, the regulatory questions, and export constraints on a packaged cryptographic binary, in parallel with the harness. This paragraph's characterizations of case law are external context, not from the repository.

**Free-path subsidy, Medium impact, High likelihood of cost growth.** Every new identity costs a binding transaction and every free grant verifies a proof on chain at relayer expense; the grant pool is unfunded and grows with success. Mitigation: per-identity rate limits, short lock expiries with refund, gas recorded per install, the pool sized from measured gas after the harness and dogfood run. What ends free onboarding, and what replaces the relayer, is not decided; the case records that as open rather than implying the subsidy is permanent.

**Open selections, Medium impact, Medium likelihood.** The attempt-rule parameters and piece-group size, the license text, and custody recovery UX remain open; custody, launch network and curve, and the escrow record's form are decided. Mitigation: the parameters are fixed from harness measurement before any node that encrypts a registered deployment; the license is a release prerequisite; recovery UX is decided at the identity milestone.

**Adapter registry governance and scaffolding, Medium impact, Medium likelihood.** Adapters are resolved from a governance-controlled on-chain registry, immutable once bound, governed in the MVP by a single project-held key recorded as scaffolding. The relayer, project seed host, and claim verifier are centralized conveniences the project operates. Mitigation: hold the governance's replacement path as a workplan item; ship default configuration with more than one discovery source and more than one state-read node; keep every service behind an interface with a documented replacement path.

**Accepted residuals, stated so they are not mistaken for open items.** A modified client can retain a decrypted credential past its interval. Every credential recovers the same encapsulated values, so the piece-group keys are common to every holder and exportable as a payload-proportional key file whose leak is unattributable. Both follow from one canonical ciphertext under a symmetric cipher without trusted hardware, and the research records the second as not resolvable with known practical methods. The piece-group size bounds exposure, a leaked credential names its entitlement, the license makes a non-conforming client a violation, and the decapsulation-to-cipher seam is preserved for a future multi-key suite. The protocol claims bounded operation for the conforming client, not hostile DRM, and says so.

# SWOT

## Strengths

- The hard cryptographic problem is closed at the composition level, adopted into the specifications, independently re-derived, and its residuals are stated.
- Nothing on the read path: reading is local, transfer is two-party, the contract verifies rather than decides. No competitor that enforces rights has this property.
- A constructed property equivalent to first sale for digital goods, which consumers want and platforms cannot offer without dismantling their model.
- A beachhead content class whose publishers already distribute it to the public at no charge, with immutable artifacts, upstream integrity attestations, and one registry to bootstrap against.
- Package managers extended rather than replaced, so resolution, lockfiles, and workspaces remain the ecosystem's solved work.
- Everything an adapter, including the parts the project intends to own, so new chains, curves, transports, ecosystems, and the first-party stack arrive without protocol change.
- A requirements document in which every requirement carries an end-to-end proof, a completion boundary that forbids the usual shortcuts, and an authoring discipline enforced by the repository.
- An adoption mechanism, build platforms as superseeders, that requires no recruitment.

## Weaknesses

- No code, no team, no timeline, no budget; delivery feasibility is undetermined until the harness calibrates it.
- Cost is unmeasured: no byte, latency, proof, gas, or settlement figure exists, and the piece-group size and curve wait on the harness.
- At the beachhead, machine-wide reuse is parity with existing local stores, and the rights layer is invisible at $0.00.
- The engineering surface is wide and platform breadth across Windows, macOS, and Linux is the largest delivery risk.
- The MVP validates the mechanism but not its load profile; streaming costs are unobserved.
- The adopting population is the population the public-ledger argument does not cover, and the MVP ships no privacy construction.
- Free onboarding depends on a subsidized relayer with no funding source, no size, and no replacement path.
- The attempt-rule parameters, piece-group size, license text, and custody recovery UX remain open.
- Compliance is the least specified area: eligibility restatement, license, regulatory review, and export confirmation are all unstarted and on the release path.

## Opportunities

- Long-tail availability and network accumulation as the developer argument no substitute can match.
- Build platforms as superseeders, turning the most demanding population into the swarm's backbone at no cost to the project.
- Every other package ecosystem as a peer adapter and a new population.
- Supply-chain security as a first-class benefit: on-chain provenance, advisories that follow content, a verifiable dependency inventory.
- A demonstrable milestone at the package host, early enough to show and measure before the last third of the work.
- The harness measurements as a publishable applied-cryptography result that invites the external review the project needs.
- Priced content classes once price moves from zero, with no client refactoring.
- Seeder compensation and retention enforcement over a Sybil-resistant participant set, attaching to a store interface that already records why it holds what it holds.
- The project's own chain, tokens, keystore, and swarm client, arriving as adapters.

## Threats

- A registry operator or package-manager maintainer ships outage insulation or a peer cache natively and captures the beachhead benefit without a rights layer.
- The daemon and its update channel become a supply-chain target on every developer machine.
- Individual developers decline a permanent public record of their install history.
- Rights holders of freely distributed but redistribution-restricted packages object to public redistribution or irrevocability, an exposure public mirrors share but a swarm sharpens.
- Regulatory treatment of priced transferable entitlements or of the paymaster in some jurisdiction.
- The relayer subsidy scaling with success while unfunded.
- Chain properties outside the project's control: precompile availability and pricing on the launch L2, sequencer behavior, RPC censorship.
- A published piece-group key set for a high-value deployment, decrypting the swarm copy for everyone and attributing to no one.
- The harness showing authorization cost above the developer's tolerance, which is the first stop criterion and should be met as a finding rather than absorbed as a slower install.

# Next Steps

Ordered by dependency, named by role, not numbered. Each has an owner by role and a stop criterion where one applies.

- **Build the cryptographic validation harness.** Mapped at ticket resolution in the dependency map, closing at the report node that records capsule, envelope, and proof sizes, decapsulation time per piece group, proof generation and verification time, and delivery gas for a mint and a transfer on both curves, and that carries the phase's integration test and commit. The Solidity verifier is production code. Owner: cryptography implementer. Stop criterion: none at this step; the harness produces the inputs to the next.
- **In parallel with the harness, start the external work that has no engineering dependency.** Quote and begin external cryptographic review on the composition claims, to conclude on the harness output. Begin legal drafting and review scoped to the license text, the eligibility principle and its residual, priced entitlements and the paymaster, and export constraints. Record the adapter registry's governance and the supported platform matrix as decisions. Owner: project lead.
- **Declare the interactive latency budget**, including the fraction of install wall-clock authorization may add, before any measurement is compared to it. Owner: project lead. This is the first stop criterion: if no parameter within the specification's bounds meets the budget on the harness's measurements, the finding is reported against the design rather than absorbed.
- **Resolve the measurement-gated selections from the harness output**: the piece-group size and attempt-rule parameters, and confirm the curve. Re-run the whole-target evaluation with measured costs. Owner: project lead with the cryptography implementer.
- **Re-map the remaining protocol core milestones to ticket resolution from what the harness taught**, including its measured throughput, and produce the first evidence-based estimate of timeline and engineering cost. Owner: project lead. Stop criterion: if the estimate is beyond what the project can fund, that is decided here, before the wide work begins.
- **Build the protocol core and contract suite, with the swarm and signature milestones in parallel from the foundation onward.** Domain model, payload cipher and commitments, registry and entitlement contracts calling the harness's verifier, chain and settlement adapters, adapter registry and factory; hashing, signatures, swarm transport, seed host, and ciphertext store alongside, since they need no chain. Owner: implementers by family.
- **Build the local daemon and package serving.** Durable jobs, plaintext CAS and resolution order, First Finder ingest, package host; identity and custody early because the installer needs it. At the package host an ordinary `npm install` is served at the registry's speed, registers its requests, and prefetches its ciphertext. Owner: daemon implementer.
- **Build credential delivery and per-attempt authorization**, closing the read path: mint delivery through the relayer, holders fulfilling requests for requesters present or absent, local decryption under the attempt rule, interval-end destruction, completing a first run's independence.
- **Reach the demonstrable milestone.** With the registry down, a second identity installs a closure against the swarm on a grant fulfilled in its absence. Measure the north star, first-run independence, and the latency guardrail on the dogfood population here. Owner: project lead. Stop criterion: the guardrail; if the benefit is not felt on the dogfood population, the case does not proceed to the adopting population.
- **Build the onboarding shells and services.** Installation coordinator, extension and npm bootstrap, desktop and CLI, relayer, explicit publisher path, claim verifier and escrow claim last, project seed host and site, observability, demonstration harness. Owner: platform and services implementers.
- **Run acceptance and release.** Every acceptance scenario through packaged applications on clean machines; the transaction flow proof after external review closes; the dogfood baseline recorded; legal prerequisites complete; release. Stop criteria: any guardrail breach blocks; no priced deployment before review and legal.

# References

Repository documents, on which every unlabelled claim rests:

- [cryptography.md](../research/cryptography.md), the protocol specification.
- [MVP Scope.md](../research/MVP%20Scope.md), the MVP boundary and deferred items.
- [MVP Application Requirements.md](../research/MVP%20Application%20Requirements.md), the operational requirements and acceptance scenarios.
- [MVP Execution Trace.md](../research/MVP%20Execution%20Trace.md), the request state machine.
- [cryptography-critical-path.md](../research/cryptography-critical-path.md), the construction and open obligations.
- [cryptography-requirements.md](../research/cryptography-requirements.md), the research requirements and preferences.
- [cryptography-research-notebook.md](../research/cryptography-research-notebook.md), the research ledger.
- [ChainTorrent MVP workplan](../workplans/current/ChainTorrent%20MVP.md), the To-Do list, adoption sequence, and proposed build order.
- The planning documents cited in the preamble: business case, critique, feature spec, success metrics, technical approach, risk register, non-functional review, dependency map, feasibility assessment.
- [EIP-196](https://eips.ethereum.org/EIPS/eip-196), [EIP-197](https://eips.ethereum.org/EIPS/eip-197), [EIP-2537](https://eips.ethereum.org/EIPS/eip-2537), the precompile interfaces the research notebook's evidence register cites.
- [The Foundations of Internet 3.0](https://medium.com/@TimSylvester/the-foundations-of-internet-3-0-237df6013582), the originating essay.

External context, labelled as such where used: the npm unpublishing incident of 2016, pnpm's store as prior art, the product names in the competitive table, and the characterizations of first-sale and fair-use case law under Risks. None is sourced from the repository.

# Additional Content

## What is being built

A deployment is one canonical ciphertext plus one public header sidecar per live parameter set, seeded to a content-addressed swarm and committed on chain by three BLAKE3/Bao roots. An entitlement is a single-owner bearer asset on the ledger, an ERC-721 token in the MVP, bound to the asset rather than to any deployment. Each ownership interval receives a native credential, delivered inside the settlement that creates the interval and verified by a proof the contract checks through the chain's pairing precompiles. The conforming client authorizes every decryption attempt itself from a fresh view of consensus state at the deployment's declared settlement tier, decrypts locally by decapsulating one capsule per piece group, and when the entitlement transfers away, stops at the declared tier and destroys its decrypt-capable material at hard finality. Seeders hold ciphertext they cannot read. Under the escrow suite any current holder can author a grant for a new identity, so no asset depends on the party that bootstrapped it. The protocol governs the transition from ciphertext to plaintext and makes no claim over plaintext thereafter.

The construction is a depth-one Boneh–Boyen identity-based KEM in Type-3 pairing groups with seller-side rerandomization, ElGamal credential envelopes, and a chained Schnorr delivery proof, secure under SXDH, decisional BDH-3b, and the random-oracle model. Every operation is scalar multiplication, pairing, hashing, and a KDF.

## Cost, timeline, and funding, stated as a method rather than a figure

No source supports a figure and none is invented. What the plan produces, and when: delivery-verification gas on both curves at the harness, which is the largest input to the relayer's per-install cost and to the network choice; relayer gas per install and per identity binding, and swarm bandwidth per install, at the first dogfood acquisition; engineering throughput, tickets closed per unit of time under the repository's discipline, across the harness grouping, which is the calibration for the first timeline and cost estimate; and external review and legal costs, which have market rates and can be quoted now. Spend categories are engineering across applied cryptography, Rust systems, Solidity, platform packaging, and Tauri; external cryptographic review; the relayer grant pool, sized from measured gas; infrastructure for the seed host, site, and test chain; and legal. The decision point for funding the wide work is after the harness, when the remaining protocol core milestones are re-mapped to tickets with a measured throughput behind it.

## Stop criteria

- The harness shows that no parameter within the specification's bounds meets the declared latency budget: reported as a finding against the design, not absorbed.
- The post-harness estimate of the remaining work is beyond what the project can fund: decided before the wide work begins.
- The north star and latency guardrail on the dogfood population at the demonstrable milestone show the benefit is not felt: the case does not proceed to the adopting population.
- Any release guardrail breached in the acceptance run: release blocks. No guardrail may be waived by adjusting the metric.
- External cryptographic review finds a composition or verifier flaw: no priced deployment until it is dispositioned.

## Delivery groupings, by dependency role

The technical approach and the dependency map hold the build order. The groupings, addressed by role, are the foundation; the cryptographic validation harness, at ticket resolution; the protocol core and contract suite, with the hashing, signature, and swarm milestones at ticket resolution and running in parallel because they need no chain; the local daemon and package serving, closing the read path with credential delivery and reaching the demonstrable milestone; the onboarding shells and services; and acceptance and release. Resolution decays with distance: when the harness closes, the next grouping is re-mapped to tickets from what was learned, and the decay shifts outward.

## What the MVP proves and does not

It proves that either entry point produces a working system on a clean machine after only permitted consent; that an ingested package installs from the swarm with the registry down; that bootstrap is non-blocking and race-safe; that mint and transfer proofs verify on chain and invalid ones are rejected; and that a priced transfer settles with the buyer decrypting at the declared tier and the seller destroying at hard finality. It does not prove willingness to pay, price discovery, resale volume, seeder compensation, Sybil resistance, or the load profile of continuous-decryption content. Per-attempt authorization is built in full and exercised lightly, and decapsulation time per group is recorded so a later content class has a baseline.
