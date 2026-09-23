<!-- Template: synthesis_product_requirements_document.md -->
# Executive Summary

Draft, 2026-09-23. The gating product requirements document for the ChainTorrent MVP. It synthesizes the [revised business case](business-case-revised.md), the [feature specifications](feature-spec.md), the [success metrics](success-metrics.md), the [technical approach](technical-approach.md), the [dependency map](dependency-map.md), the [risk register](risk-register.md), the [non-functional requirements review](non-functional-requirements.md), the [technical feasibility assessment](technical-feasibility.md), and the [business case critique](business-case-critique.md), all of which derive from the research and MVP documents under [docs/research](../research/) at ledger log 36 and the [workplan](../workplans/current/ChainTorrent%20MVP.md). Where a section's detail lives in one of those documents, this one states the position and cites the source rather than repeating it. Claims from outside the repository appear only where labelled external context. No figure the sources leave unmeasured is estimated.

**The product.** A protocol that distributes encrypted assets over a peer swarm and resolves the right to decrypt from the holder's own credential and a fresh view of public ledger state, with no service on the read path. The access right is a transferable, irrevocable bearer asset. The MVP applies it to JavaScript dependencies: a developer installs one extension or one npm package, gives one visible consent, and thereafter installs from a machine-wide cache, a peer swarm, or the registry, in that order, at $0.00, with the swarm holding what anyone fetched and serving when the registry does not.

**The state of the work.** The cryptographic problem that made the design impossible is closed at the research level and adopted into the specifications. Technical feasibility is high; every component is conventional. Delivery feasibility is undetermined: no code, team, timeline, or budget exists, and the surface is wide. The first phase of work, a validation harness of thirty-two source files, is designed to calibrate that. The launch network, curve, platform matrix, custody, and the absence of an escrow-salt custodian are decided; the remaining selections proceed on accepted defaults, and one, the multi-device sync mechanism, awaits confirmation of its recommendation. Compliance items on the release path are unstarted.

**The decision this document supports.** Build the harness now; run legal and cryptographic review in parallel; declare the latency budget before measuring; re-map the next phase from the harness's measured throughput and decide the wide work then, against the stop criteria stated here.

# MVP Description

The MVP delivers a working install path for JavaScript dependencies that serves packages from a peer swarm, reuses across every project on a machine whatever that machine has fetched, survives an upstream registry outage for packages already ingested, and exercises the full identity, entitlement, credential-delivery, encryption, and transfer lifecycle end to end. It validates distribution and identity. It does not validate willingness to pay or seeder compensation. Monetization is $0.00 throughout, with one priced transaction on a package the project publishes itself to prove settlement.

**Deployable applications.** A Visual Studio Code extension and an npm bootstrap package as the two onboarding paths, converging on one Rust installation coordinator; a Tauri desktop application and a CLI as control surfaces; a single-instance local daemon and package host; a cryptographic validation harness; a relayer or paymaster; a claim verifier; a Solidity contract suite; a demonstration harness; and a project seed host and site. All Rust except the thin host shells and the contracts.

**What a developer experiences.** Install, consent, and then `npm install` works as before. A package the machine has installs from local plaintext with nothing else reachable. A package the swarm has installs from peers with the registry down. A package nobody has ingested installs from the registry at registry speed while the daemon encrypts, registers, and seeds it in the background. The daemon runs across reboots, its seeding archive is visible, its footprint is bounded by the developer, and one repair operation restores it. Uninstall restores npm exactly and keeps the developer's identity and plaintext.

**What the MVP proves.** Clean-machine onboarding through either path after only permitted consent; registry-outage survival for ingested assets; non-blocking, race-safe bootstrap; on-chain verification of credential delivery with invalid proofs rejected; and a priced transfer settling with the buyer decrypting at the declared tier and the seller destroying at hard finality. Completion is all twenty-seven acceptance scenarios passing through packaged applications on clean machines with no test-only bypass.

**Boundary.** Deferred and architecturally protected: Git commit wrapping, an escrow email bot, general paid monetization, a version alignment engine, media and streaming, content flagging, per-entitlement variance, composable containers, partial encryption, seeder compensation, retention enforcement, Sybil-resistant stake, and the website account, remote head, and hosted instance. Each is recorded in MVP Scope with the protection that keeps it out of the MVP's dependencies. The project site itself is in scope as the education tier: explanation, a browser demonstration built from the client's own crates compiled to WebAssembly against a sample deployment, and the download, with no account and no service on the read path.

# User Problem Validation

Validation is by the public record and by the structure of the systems involved, not by interviews. The MVP's own instrumentation validates the developer population: the north star is measured on the dogfood population before the adopting population is asked, and the latency guardrail fails the run if authorization makes installs noticeably slower.

The problems the MVP addresses for developers: one registry operator serves every build and its outage halts everything; a machine's local cache holds only what that machine fetched, so the first fetch anywhere still needs the registry; and a publisher's unpublishing breaks builds. The problems the protocol's shape addresses for later content classes, evaporating access, non-transferable licenses, and failed hostile DRM, are the reason the design exists and are invisible to a developer installing a free package; the MVP proves the mechanism and does not sell it. The full argument is in the revised business case's User Problem Validation section.

# Market Opportunity

Layered and reached through adapters, unsized by design. Beachhead: JavaScript dependencies, adopted by individual developers, with build platforms following as superseeders for their own economic reasons and CI and enterprise following them. Adjacent: every other package ecosystem as a peer adapter. Expansion: priced content classes where an irrevocable, resellable entitlement is the product and per-attempt authorization is exercised continuously. Platform: the project's own chain, tokens, keystore, and swarm client, arriving as adapters. Timing rests on pairing precompiles being available on every EVM chain, provenance attestations becoming standard, and the research having closed the blocking problem. Detail in the revised business case.

# Competitive Analysis

No existing system combines one canonical encrypted swarm object, transferable irrevocable on-chain entitlements, per-holder decryption with nothing on the read path, and plaintext the user keeps. Every system that enforces rights puts a service or device on the read path; every system that avoids the read path enforces no rights. At the beachhead the honest position is parity: machine-wide reuse matches the best local store and enterprise outage insulation matches a private proxy, and the MVP's advantage is the long tail, swarm accumulation, and a rights layer no developer notices at $0.00. Discovery, presentation, commerce, indexing, and storage are invited to compete on top of the protocol under a replaceability requirement. The table and the labelled external context are in the revised business case.

# Differentiation & Value Proposition

For the developer: install once, consent once, and install from cache, swarm, or registry in that order with no cryptocurrency and no account; packages in the swarm install when the registry is down; plaintext is theirs; the daemon's disk, bandwidth, and public-ledger footprint are disclosed and bounded. For build platforms: a superseeder role at no marginal cost, with CI identity mechanics recorded as open. For entitlement holders in every content class: an irrevocable, resellable right with credential delivered inside settlement and exercised locally, the protocol's construction of what first sale gives physical goods, as analogy not doctrine. For publishers: unbounded discretionary issuance, recoverable keys, rotation that strands nobody, escrow that hands over complete ownership on claim, at the cost of the ability to withdraw. For the ecosystem: on-chain provenance, advisories that follow content, and a public dependency inventory that is also a privacy exposure carried under Risks. The design commitments: no enclaves, no bespoke runtime, no takedown, no selective withholding, everything an adapter.

# Risks & Mitigation

The [risk register](risk-register.md) holds twenty risks with ratings, affected components, signals, and mitigation plans. Those bearing on the decision to proceed, at the register's ratings:

| Risk | Impact / likelihood | Mitigation in one line |
| --- | --- | --- |
| Delivery breadth (R-04) | High / High | Harness at ticket resolution as calibration; platform matrix decided explicitly; demonstrable milestone at the package host |
| Cryptographic cost against the latency budget (R-02) | High / Medium | Harness before any node that encrypts; budget declared before measurement; instrumentation fails rather than reports |
| Beachhead parity (R-05) | High / Medium | Lead with the long tail and accumulation; superseeders; measure the north star on dogfood first |
| Consumption privacy for developers (R-06) | High for adoption / High | Onboarding disclosure; own-node state reads; constructions recorded as the V2 design question gating private adapters |
| Verifier and composition soundness (R-03) | High / Low to Medium | Rust reference verifier with generated Solidity constants; mutation and replay vectors; external review during the harness, gating the priced deployment |
| Daemon as supply-chain surface (R-07) | High / Medium | Authenticated artifacts, verified serving, least-privilege IPC, fuzzing, signing-key custody discipline |
| Unauthorized redistribution and irrevocability (R-01) | Medium / Medium | State the eligibility principle and its residual; optional policy narrowing for redistribution-forbidding licenses; advisory objection channel |
| Legal framings and regulation (R-10) | Medium / Medium | First sale as analogy; legal review in parallel with the harness scoped to license, eligibility, entitlements, paymaster, export |
| Free-path subsidy (R-08) | Medium / High cost growth | Rate limits, lock expiry, gas per install, pool sized from measurement; replacement path recorded as open |
| Blocking selections (R-09) | Medium / High | Recorded defaults; custody as first adapter; salt commitment dropped; harness on BN254 until the network is chosen |
| Adapter registry governance and scaffolding (R-14) | Medium / Medium | Governance recorded as a workplan item; multiple discovery and state-read sources by default; documented replacement paths |

Accepted residuals, not open: modified-client retention of a decrypted credential, and common piece-group keys exportable as an unattributable key file. Both follow from one canonical ciphertext without trusted hardware; the protocol claims bounded operation for the conforming client and says so.

# SWOT Overview

## Strengths

- Cryptographic problem closed, adopted, independently re-derived, residuals stated.
- Nothing on the read path; transfer two-party; the contract verifies rather than decides.
- A constructed first-sale-equivalent property for digital goods.
- A beachhead corpus its publishers already distribute to the public at no charge, immutable, attested, with one registry to bootstrap against.
- Package managers extended, not replaced.
- Everything an adapter, including the parts the project will own.
- Every requirement carries an end-to-end proof; the completion boundary forbids shortcuts.
- Build platforms as superseeders, requiring no recruitment.

## Weaknesses

- No code, team, timeline, or budget; delivery feasibility undetermined until the harness calibrates it.
- Cost unmeasured; piece-group size and curve wait on the harness.
- Reuse is parity with local stores at the beachhead; the rights layer is invisible at $0.00.
- Wide surface; platform breadth the largest delivery risk; supported platforms unlisted.
- Mechanism validated, load profile not.
- Adopting population is the one the public-ledger argument does not cover; no privacy construction ships.
- Free onboarding rests on an unfunded, unsized, unreplaced relayer.
- Six blocking selections open, two with no determinant in the sources.
- Compliance items unstarted on the release path.

## Opportunities

- Long-tail availability and accumulation as the developer argument no substitute matches.
- Build platforms as the swarm's backbone at no cost to the project.
- Other package ecosystems as peer adapters.
- Supply-chain security as a first-class benefit.
- A demonstrable milestone before the last third of the work.
- Harness measurements as a publishable result inviting review.
- Priced content with no client refactoring; seeder compensation over a stake layer; the first-party stack.

## Threats

- An incumbent ships a native peer cache or outage insulation.
- The daemon and its update channel targeted as a supply chain.
- Developers decline a permanent public install history.
- Objections from rights holders of freely distributed but redistribution-restricted packages.
- Regulatory treatment of priced entitlements or the paymaster.
- Subsidy scaling with success while unfunded.
- Chain properties outside the project's control.
- A published key set for a high-value deployment.
- Harness measurements exceeding the developer's latency tolerance.

# Feature Scope

Sixteen features constitute the MVP, each specified in the [feature spec](feature-spec.md) with objective, user stories, acceptance criteria citing requirement identifiers, dependencies, and success metrics. In scope:

| Feature | Requirement families | Delivery grouping by role |
| --- | --- | --- |
| Cryptographic services and validation harness | CR, CD-07 | Cryptographic validation harness |
| Adapter composition and capability resolution | SI-10, IC-05, CD-08 | Protocol core and contract suite |
| Entitlement ledger, contracts, and settlement | LC | Protocol core and contract suite |
| Swarm transport, peer discovery, and seed hosting | SW | Protocol core and contract suite, in parallel |
| Package serving and local CAS | PR | Local daemon and package serving |
| Encrypted content consumption and per-attempt authorization | EC | Local daemon and package serving |
| First Finder bootstrap and escrow | FF, PC-02 | Local daemon and package serving |
| Credential delivery at mint, grant, and sale | CD | Local daemon and package serving |
| Identity, wallet, and key custody | IW | Local daemon and package serving |
| Self-installing onboarding | SI, IC | Onboarding shells and services |
| Relaying and free-path subsidy | RO-01, LC-10 | Onboarding shells and services |
| Explicit publishing and escrow claim | PC | Onboarding shells and services |
| Project seed host and site | FF-09, LI-01 | Onboarding shells and services |
| Observability, diagnostics, and cost instrumentation | RO-03 to RO-06, XA-01, XA-04 | Onboarding shells and services |
| Test facilities and demonstration harness | XA-03, XA-05, XA-07 | Onboarding shells and services |
| Transaction flow proof | LC-03, RO-02, LI-01 | Acceptance and release |

Out of scope, deferred and protected: the items listed under MVP Description, including the website account service, rendezvous, remote head, and hosted instance. Nothing in scope depends on any of them. The project seed host and site feature carries the education tier and reserves the account-link step in the installer's consent flow so that the deferred tier is a later feature rather than a redesign.

# Feature Details

Compact statements; the feature spec holds user stories and acceptance criteria.

- **Cryptographic services and validation harness.** AES-256-CTR payload cipher with the specified counter layout; BLAKE3/Bao commitments and challenges; Ed25519 and secp256k1 signatures per layer; the depth-one Boneh–Boyen KEM with declared identity scope; pairing ElGamal envelopes; chained Schnorr delivery proofs in both verifier forms; pairing adapters on BN254 and BLS12-381 matching precompile encodings; a harness that measures sizes, timings, and gas on both curves against a deployed verifier and records the piece-group size and curve. Thirty-two tickets in the dependency map; the delivery verifier is production code.
- **Adapter composition.** Every adapter declares capabilities, version, and compatibility; composition validates before any operation and fails closed; the immutable deployment suite fixes every cryptographic component together.
- **Entitlement ledger and contracts.** Canonical identity, authenticated deployments and hash-cards, parameter-set liveness with the sidecar-coverage rule, envelope-key registry with proofs of possession, entitlements with interval state and envelope digests, issuance and transfer with delivery verification through the precompiles and atomic payment release, escrow, identity binding, claims, batch and paginated views, lock expiry, rate limits.
- **Swarm, discovery, and seed hosting.** Transport by authenticated root with BitTorrent compatibility as one implementation; concurrent non-authoritative discovery; a session-independent seed host with delegation verified by Bao challenges; a root-keyed ciphertext store distinguishing obligated from voluntary holdings; a visible seeding archive.
- **Package serving and CAS.** npm's registry protocol served locally, resolution left to npm; deterministic coordinate-to-identity mapping; fixed resolution order; verified atomic CAS with quota, pinning, and eviction warnings; plaintext hits with no remote work; authenticated least-privilege local API.
- **Encrypted content consumption.** Authentication of record, hash-card, suite, bounds, and sidecar before any credential is exercised; Bao-verified acquisition; the attempt rule over the full context at the declared tier within the freshness bound; three-state authorization handling; memory-only decrypt-capable material destroyed at hard finality on transfer out; multi-node state views failing closed.
- **First Finder bootstrap and escrow.** As-is ingest with attestation validation; foreground serve before background bootstrap; durable idempotent job; random master scalar and parameter set; state-locked registration race with loser destruction; escrow custody; later-seeker install without the registry; dependency-closure ingestion on publication.
- **Credential delivery.** First credential in the mint; buyer's credential in the sale from the seller's fresh decryption; any holder authors escrow grants under the asset scope; persistent credential recoverable from chain history; claim registers the claimant's parameter set with optional handover and sidecar-per-live-set.
- **Identity, wallet, and custody.** Separable capabilities bound to one principal; declared custody capabilities; one identity, one relayer-paid binding, one envelope-key registration; recovery without silent principal change; EIP-1193, WalletConnect, EIP-712; local default identity; envelope keys never wallet keys.
- **Self-installing onboarding.** Both paths converge on one coordinator: detection, signed artifacts, stores, daemon lifecycle across reboot, explicit reversible redirect, backup and rollback, idempotent reinstall, health probe, repair, signed reversible update, uninstall preserving identity and plaintext.
- **Relaying.** Sponsored free acquisition and one binding per identity with rate limits, explicit failure states, and gas reporting; paid acquisition requires real funds.
- **Explicit publishing and escrow claim.** Publisher authority by strongest available proof; seed-derived keys; claim-set vouchers bound to claimant, set, contract, chain, nonce, expiry; claim independent of any unavailable party; complete authority transfer; verifier key rotation and revocation.
- **Project seed host and site.** Persistent first seeder of the core closure; a convenience no client depends on.
- **Observability.** The metrics of RO-03 without secrets; per-request correlation; consistent health; attempt latency evaluated against a pre-declared budget.
- **Test facilities and demonstration harness.** Unit, integration, and end-to-end facilities; every requirement mapped to a proof class; controlled participants, wallets, chain state, and failures; the packaged MVP runs without a developer toolchain.
- **Transaction flow proof.** Priced primary issuance and secondary transfer on the project's own package, asserting the settlement boundary.

# Feasibility Insights

From the [feasibility assessment](technical-feasibility.md). Technical feasibility is high: every cryptographic operation is scalar multiplication, pairing, hashing, and a KDF over standard components with named assumptions; every engineering component is conventional; no integration requires an external party's cooperation or an undocumented interface. Delivery feasibility is undetermined: no team, timeline, or budget, and the harness phase is the calibration unit. Four components are Medium for breadth or an open selection rather than difficulty: transport, custody, installation, and the demonstration harness. Platform breadth is the largest delivery risk and the supported platform matrix is unlisted. The Rust and Solidity verifiers must agree bit for bit for the contract's lifetime, which argues for generating the Solidity constants and vectors from the Rust reference. The transaction flow proof is reached last and depends on external review, so review starts during the harness. The hard problem is behind the project; the wide problem is ahead.

# Non-Functional Alignment

The [non-functional requirements review](non-functional-requirements.md) extracted sixty-seven requirements with dispositions. Security, performance, reliability, scalability, and maintainability are well covered by the sources as operational requirements with proof obligations. Compliance is the least specified. Nine gaps are proposed for adoption into the Application Requirements so each acquires a proof class: artifact signing-key custody and rotation; a threat-model statement naming the local package host as a supply-chain surface; daemon idle and seeding footprint limits; cold-start time; backoff bounds; a per-release suite compatibility statement; regulatory review of priced entitlements and the paymaster; export constraints on a packaged cryptographic binary; accessibility baselines for the Tauri surfaces; and a first-run disclosure of what is public and what the daemon uses. No conflicts between non-functional requirements were found. A proposal in that review is not a requirement until it is a row in the Application Requirements.

# Score Adjustments & Tradeoffs

The tradeoffs the planning work made explicit, with what was given up and why.

| Tradeoff | Chosen | Given up | Reason |
| --- | --- | --- | --- |
| Monetization in the MVP | $0.00 everywhere, one nominal priced proof on project-owned assets | Revenue data; willingness-to-pay validation | Content bootstrap confers no publisher rights, so nobody may price what they do not own; the priced proof exercises the settlement boundary where it is cheap to assert |
| Escrow suite identity scope | Asset scope: any holder authors grants | Entitlement-level attribution of leaked escrow-era credentials | A First Finder that vanishes must strand nothing; attribution is meaningless at $0.00 |
| Retained plaintext | The user keeps what they decrypt | Any claim over plaintext after decryption | The transient copy is the platform behavior the protocol exists to replace |
| Public entitlement ledger | Public records | Consumption privacy for individuals | A stated property for public content; the individual case is recorded as open and mitigated by disclosure and own-node reads |
| Ingest eligibility | Public npm availability as the test of free public distribution | A license check at ingest | Availability tests the publisher's choice; the residual is redistribution and irrevocability, which public mirrors share; a license check is an optional narrowing held as a policy decision |
| Settlement tier for free packages | Read at inclusion | Protection against reversal for free content | The exposure is one entitlement's credential, self-healing, on content already public; priced deployments declare deeper tiers |
| Client license | Source-available with a conformance clause | OSI openness | A non-conforming client becomes a legal problem for the modifier as well as an accepted edge |
| Planning resolution | Tickets for the nearest phase, decaying outward | A full plan now | Implementation of the nearest phase will revise everything after it; detail authored early would be rewritten |
| Harness delivery verifier | Production code from the start | A quick benchmark | The registry contract calls it; writing it twice is the expensive path |
| Custody default | Local daemon-held keystore as the first adapter | A browser or platform authenticator integration first | The capability list is not known to be satisfied by those APIs; a later implementation is a second adapter |
| Escrow-salt custodian default | Drop the on-chain commitment | A recorded maintainer binding | The only candidate with no lost-salt failure mode that satisfies the rule that claim verification depend on no unavailable party |

# Outcome Alignment & Success Metrics

- Outcome Alignment: three outcomes define success, per the [success metrics](success-metrics.md). The benefit is felt: a developer reuses what the machine has and installs are not noticeably slower. The lifecycle is proven: identity, entitlement, verified delivery, encryption, per-attempt authorization, and priced transfer work end to end through packaged applications. The economics are measurable: cost per install and cryptographic cost per operation exist as measured numbers. Deferred items produce no observation in the MVP and have no metric.
- North Star Metric: plaintext CAS reuse, the fraction of installs served from local plaintext with no network, chain, swarm, or authorization work, and the number of projects per machine sharing each artifact. Direction up. No target is stated; the first measured value on the dogfood population is the baseline.

## Primary KPIs

Interactive install wall-clock and the fraction authorization adds, passing against a budget declared before the run. Clean-install success on every supported platform through both paths. Registry-outage install success for the ingested set. All twenty-seven acceptance scenarios passing, which is the release condition. Cost per install, relayer gas and swarm bandwidth, measured and reported. Delivery-verification gas per mint and transfer on the resolved curve, measured. Cross-project reuse count.

## Leading Indicators

Local and remote encrypted-hit rates; First Finder bootstrap completion rate and time; swarm-native publication coverage; seeding archive size and continuity across restart; state-read volume and latency per install; decapsulation time per piece group; one binding and one envelope-key registration per identity.

## Lagging Indicators

North star trend against baseline; a completed priced primary and secondary settlement with every boundary assertion; an escrow claim completed with the First Finder absent; requirement-to-proof reconciliation complete; piece-group size, curve, and attempt-rule parameters recorded in release evidence; repair, update, and uninstall fidelity across the recovery matrix.

## Guardrails

Latency budget pass. No protected material in telemetry, logs, storage, or crash artifacts. No authorization without a current view; no accepted invalid delivery. No side effects from invalid compositions or untrusted inputs. Package-manager parity. Consent boundary honored; configuration restored exactly. Free path rate-limited; grant pool not drained. Obligated ciphertext never evicted by plaintext policy. Foreground install independent of background bootstrap. No priced deployment before external cryptographic review and legal review. A guardrail breach blocks release and no guardrail may be waived by adjusting the metric.

## Measurement Plan

Five stages: the harness on both curves against a deployed verifier; budget declaration before the acceptance run; the acceptance run with every metric reconciled against induced activity and telemetry scanned; the dogfood baseline from swarm-native publication and the priced settlements; the adopting population reported against the baseline. Security requirements are proven as scenarios, reliability on every supported platform, compliance as release-evidence items, and review findings tracked to disposition.

## Risk Signals

Authorization fraction rising toward the budget; decapsulation time per group times groups per package approaching the budget; delivery gas making the free path unsustainable; bootstrap completion below the ingest rate; seeding discontinuity across restart; grant fulfilment slowing with the First Finder offline; rising pending-settlement dwell; any sensitive field in telemetry; lockfile divergence from upstream; attempts halting on node disagreement beyond one failed node; subsidy consumption outpacing identity creation.

# Decisions & Follow-Ups

## Resolved Positions

Positions reached during the planning work, each recorded in the document named.

- **Ingest eligibility is free public distribution by the rights holder's choice, tested by public npm availability, with the residual being redistribution and irrevocability rather than denied revenue.** Reached in the licensing discussion; recorded in the critique, the register at R-01, and the revised business case. The critique's earlier High rating rested on a wrong premise and was re-rated Medium.
- **The build sequence follows the workplan's proposed order**, harness first, swarm transport in parallel with the registry contract, escrow claim last; recorded in the technical approach and dependency map. It remains a proposal until ratified.
- **Delivery groupings are addressed by dependency role, never by ordinal**; the original business case's numbered phases are superseded. Recorded in the dependency map and the revised business case.
- **The harness's delivery verifier is production code** consumed by the registry contract. Recorded in the dependency map and feasibility assessment.
- **Planning resolution decays with distance**: tickets for the nearest phase, sprints, epics, milestones, objectives outward, re-mapped one phase at a time. Recorded in the dependency map.
- **Legal framings are analogies.** First sale describes the protocol's constructed property; no fair-use holding is borrowed. Recorded in the critique and the revised business case.
- **Consumption privacy is carried as a risk for the adopting population**, with onboarding disclosure and own-node reads as the MVP's mitigations. Recorded in the register at R-06 and the revised business case.
- **The latency budget is the only threshold that fails a run and is declared before measurement.** Recorded in the success metrics.
- **Stop criteria exist**: five, stated in the revised business case and restated under Release Plan below.
- **External cryptographic review and legal work start in parallel with the harness**, not after it. Recorded in the feasibility assessment and the revised business case.
- **Factual corrections**: fifteen research requirements; BLS12-381 precompile availability confirmed per network; one hundred fifteen operational requirements and twenty-seven scenarios. Recorded in the critique and the revised business case.
- **Launch network is Base**, an Ethereum L2 on the OP Stack, as the launch chain adapter: Base Sepolia for the harness and the lifecycle scenarios, Base mainnet for the swarm pilot and the priced transaction flow proof. The pairing curve follows Ethereum compatibility and existing standards: BLS12-381 through the EIP-2537 precompiles as the primary verifier form, live on Base since the Isthmus upgrade activated on 9 May 2025, with BN254 through EIP-196 and EIP-197 retained as the second form. Ethereum mainnet is a later adapter. Settlement tiers map to Base as sequencer inclusion, the safe head posted to L1, L1-finalized L2 state, and fault-proof resolution; seller destruction triggers at the finalized tier. The sequencer is a write-path liveness dependency only; reads continue from any configured node. The harness measures L2 execution gas and the L1 data-fee component separately. Arbitrum is excluded until its EIP-2537 status is confirmed. User decision, 2026-09-23, revised the same day from Ethereum mainnet after peer review.
- **Supported platform matrix is Windows, macOS, and Linux.** User decision, 2026-09-23.
- **Default key custody is a local keystore under the OS credential store**, holding the default identity and the envelope keys as the first `IKeyCustodyAdapter` with keys derived from a holder seed; external wallets sign only, through EIP-1193 or WalletConnect; Frame is evaluated as the first desktop provider. No operating system provides an Ethereum wallet, and no Ethereum wallet can hold envelope keys, master scalars, or publisher seeds, so a custody adapter exists regardless and the wallet covers only the chain-signing key. Default accepted after analysis, 2026-09-23.
- **No escrow-salt custodian and no maintainer commitment in the escrow record.** The commitment confers nothing the attestor verifier can use, the contract cannot check it, checking it on chain would publish what it hides, and a snapshot at ingest conflicts with per-version claims resolved at verification; the verifier establishes the claim set from upstream metadata at verification time. Reopened only with a ZK-Email verifier, which can compute its own commitment at claim without a custodian. MVP Scope's Escrow Claim and Entitlement Ledger sections, PC-02, and PC-04 amended; the workplan To-Do entry removed. Default accepted after analysis, 2026-09-23.
- **Attempt-rule parameters and piece-group size** are chosen from harness measurement against the declared budget. Default accepted.
- **License and the OSI question**: build proceeds; license text is a release prerequisite; the content-terms field is a string supplied at publication. Default accepted.
- **Ingest eligibility policy narrowing**: no additional license check in the MVP; the principle and residual are restated in MVP Scope; the narrowing option is recorded as a policy decision. Default accepted.
- **Team composition**: a small team directing agent implementers, with applied cryptography and platform packaging present or contracted. Default accepted.
- **Build sequence**: the workplan's proposed order is ratified and the first node is a harness ticket. Default accepted.
- **Crate and path layout, KEM ticket granularity**: one workspace with a crate per adapter family and a domain crate; one function per file. Default accepted.
- **Solidity verifier** constants and vectors are generated from the Rust reference. Default accepted.
- **External cryptographic review** starts during the harness on the composition claims, concludes on harness output, and gates only the priced deployment. Default accepted.
- **The nine proposed non-functional requirements** are adopted as rows in the Application Requirements, each with a proof class. Default accepted.
- **Adapter registry governance**: a single project-held key for the MVP, recorded as scaffolding with a documented path to something else, held as a workplan item. Default accepted.
- **Relayer grant pool**: sized from measured gas after the dogfood run; the replacement path is recorded as open. Default accepted.
- **Scope statement of the authorization invariants**: left to the specification's next revision. Default accepted.
- **A pre-install web account owns onboarding state, preferences, a roster of linked identities and devices, and optionally an end-to-end encrypted holder seed blob under a user-held secret; it never owns an entitlement.** The principal is the identity, never the account; a sign-in provider cannot be translated into a key; an installation links to an account by a signed challenge proving possession of the identity's own keys; the mapping is held only by the account service, never on chain, and is optional. The website account, rendezvous, remote head, and hosted instance are deferred to V2 with architectural protection recorded in MVP Scope under Website, Account, and Remote Head; the MVP site carries the education tier with a WebAssembly demonstration built from the client's crates and reserves the link step in the installer's consent flow. User decision, 2026-09-23, after peer review.
- **Device roles under one identity, full, read-delegate, and signer-only, are capabilities the custody adapter declares and versions**, so a CI runner reads as a delegate of an always-on full device without holding the seed, and later roles are added without a protocol change. Recorded in MVP Scope under Website, Account, and Remote Head. User decision, 2026-09-23.
- **The MVP's sole swarm transport is BitTorrent through embedded `librqbit` behind `ISwarmTransportAdapter`, carried as a soft fork.** The library is compiled into the daemon, not installed separately. The workspace depends on it through a `[patch.crates-io]` overlay pointing at a project-controlled Git branch that applies the hooks the adapter needs on top of a tagged upstream release, with `Cargo.lock` pinning the commit; each hook is submitted upstream as a pull request, and the overlay is rebased onto each upstream release and removed when a hook merges. Development never waits on the maintainer. No owned transport is built; a BLAKE3-native transport is the V2 peer implementation the interface reserves. The adapter owns root-to-infohash translation, Bao verification of every completed piece after the library's own check, holding reason, quota and eviction, schedule and power policy, and seeder-map peer injection. The three hooks proposed upstream: a piece-completion hook or wrappable storage trait, a peer-injection call, and a stable storage backend trait. The rqbit application is not made ChainTorrent-aware; it is at most a delegated seed host through its existing HTTP API. Apache-2.0 permits redistributing the modified library inside the product; the release carries the license and NOTICE and marks changed files, and the project's conformance clause governs the project's code and the combined work while the Apache parts remain Apache, subject to the scoped legal review. **Hard-fork trigger, any one of which ends the upstream track, after which the project stops rebasing, stops submitting pull requests, renames the crate, and takes on the client's maintenance and security burden as a recorded decision:** a hook the adapter needs is rejected upstream, or an issue proposing it receives no maintainer response within ninety days; no upstream release or commit activity for six months; a security fix the daemon needs has no upstream response within fourteen days; or an upstream API change breaks the overlay and rebasing is not possible within one sprint. The trigger is checked at each release and its thresholds are the project's, set 2026-09-23. SW-01 amended accordingly. User decision, 2026-09-23.
- **Publisher discretion over minting is configuration served automatically under an issuance policy**, with per-request approval or override available as declared, versioned capabilities of the publisher authority adapter; issuance remains a right and never an obligation. Recorded in MVP Scope under Paid Monetization. User decision, 2026-09-23.

## Open Questions

One decision remains open pending the user's response to the analysis recorded under Open Decisions: the multi-device custody sync mechanism. Every other decision carried in the technical approach, the dependency map, and the feasibility assessment is resolved above, either by an explicit user decision or by acceptance of the recorded default.

## Next Steps

By dependency, by role. Build the harness. In parallel, quote and start external review, start legal drafting and review, and record the adapter registry governance as a workplan item. Declare the latency budget. Resolve the measurement-gated selections from the harness output and confirm or overrule the recorded defaults for custody and the salt custodian. Re-map the protocol core phase to tickets from measured throughput and decide the wide work against the funding stop criterion. Build the protocol core with the swarm sprint in parallel; build the daemon to the demonstrable milestone and measure the north star and guardrail there; build credential delivery; build the shells and services with escrow claim last; run acceptance and release. Owners and stop criteria per step are in the revised business case's Next Steps.

# Release Plan

Release is a single event at the completion boundary: all twenty-seven acceptance scenarios passing through packaged applications on clean machines, every blocking selection resolved with a compatible implementation, both onboarding paths reaching the same postcondition, external cryptographic review dispositioned, legal prerequisites complete, and the dogfood baseline recorded in release evidence alongside the harness measurements and the chosen piece-group size, curve, and attempt-rule parameters.

Before release, two internal checkpoints exist and neither is a release. The harness report, which closes the first delivery grouping with its integration test and commit and produces the measurements everything else waits on. The demonstrable milestone at the package host, where an ordinary `npm install` resolves against the swarm and the north star and latency guardrail are first observed on the dogfood population.

Stop criteria, any one of which halts the plan at the point it is met: the harness shows no parameter within the specification's bounds meets the declared latency budget; the post-harness estimate of remaining work is beyond what the project can fund; the dogfood measurement at the demonstrable milestone shows the benefit is not felt; any release guardrail is breached in the acceptance run; external review finds a composition or verifier flaw that is not dispositioned.

After release, the adopting population's metrics are reported against the dogfood baseline on a cadence set at release, and the next planning cycle re-maps the deferred items against what was learned.

# Assumptions

Stated so they can be challenged.

- The individual developer adopts first, build platforms follow as superseeders for their own reasons, and CI and enterprise follow them. From the workplan.
- Public npm availability is an adequate test that a package's publisher chose free public distribution. From the licensing discussion.
- Install-once content exercises per-attempt authorization lightly enough that the MVP's latency budget is achievable; the load profile of continuous-decryption content is not assumed.
- The by-formula cost estimates in the research are directionally right and the harness will confirm rather than overturn them. Not assumed to be right in magnitude.
- The repository's one-file-per-turn, test-first discipline is the implementation method, and throughput under it is measurable.
- On Base both pairing precompile sets are available: BN254 through EIP-196 and EIP-197, and BLS12-381 through EIP-2537 since Isthmus. The dominant per-transaction cost is L1 data posting rather than L2 execution, and the harness measures both components.
- Entitlements and contract state do not move across chain adapters automatically; a migration mechanism is required before any later chain, including the project's own, and none is specified. Held in the workplan To-Do.
- At least one honest full-history node is reachable for envelope recovery, and more than one node is configured for state reads.
- Mature libraries exist for BLAKE3, Bao, AES, Ed25519, secp256k1, and both pairing curves in Rust.
- The MVP's corpus, packages their publishers distribute to the public at no charge, carries little rights-holder exposure, and the residual is comparable to that of public npm mirrors.
- The relayer grant pool can be funded at the scale the dogfood run and early adoption require; its long-run replacement is not assumed.

# Open Decisions

For the user to respond to. Every decision resolved on 2026-09-23 is recorded under Resolved Positions above. One remains, with the analysis and a recommendation; the recommendation is the default the build proceeds on if the Feedback block stays blank.

**Multi-device custody sync mechanism.** The specification's multi-device posture is that an identity's devices share its persistent credential, and IW-05 requires a device lacking the envelope to recover it from chain history without creating a new principal; chain history returns the encrypted envelope, never the envelope secrets, so a second device needs the secrets by some custody mechanism, and a keystore under the OS credential store does not cross machines. Analysis: deriving the holder's envelope secrets and handshake key from one holder seed through a domain-separated KDF, as the publisher hierarchy already does for parameter sets, reduces sync to moving one seed. Three transports are available and the custody adapter declares which it supports: paired transfer over the local network between two installations; an authenticator-wrapped or passkey-protected export; or the deferred account service's end-to-end encrypted blob under a user-held secret. Registering fresh envelope keys on a new device is not sync; it is a sale to oneself per entitlement. Recommendation: adopt the holder seed derivation as an implementation choice within the Anti-Derivability rule, and ship paired local transfer as the MVP's declared multi-device capability, with the authenticator export and the account blob as later custody adapters. Device roles, full, read-delegate, and signer-only, are capabilities the custody adapter declares and versions, per MVP Scope's Website, Account, and Remote Head; the MVP declares only the full role. Assumption if blank: as recommended.

Feedback:

# Implementation Risks

Distinct from the register's product and protocol risks, these are the risks of building it as planned.

- **The harness phase's thirty-two tickets, under one file per turn, take longer than expected and no throughput baseline exists to say so early.** Mitigation: track tickets closed against the map from the first one; the four independent starting points allow parallel authoring.
- **Bit-for-bit verifier parity is broken by an encoding or hash-to-scalar decision made in one language and not the other.** Mitigation: generate Solidity constants and vectors from Rust; cross-verify every vector in CI.
- **A node is authored against an unrecorded decision** and rewritten when the decision lands. Mitigation: no node authored against an open selection.
- **The demonstration harness is treated as test tooling and underbuilt**, and the completion boundary then cannot be met. Mitigation: ticket it as a component.
- **Platform work expands to fill whatever matrix is implied**. Mitigation: decide the matrix before the onboarding phase.
- **The decrypted credential or piece-group keys are cached to disk by an ordinary implementation habit.** Mitigation: inspect storage after decryption and after transfer out, as IW-04 specifies; treat any find as a security defect.
- **Two-space versus four-space, wrapped versus unwrapped, numbered versus role-named: repository conventions violated in planning documents propagate into nodes.** Mitigation: the workplan-structure canaries; the dependency map already flags the two documents that used ordinals.
- **Resolution decay is skipped and distant phases are ticketed early**, producing detail that implementation invalidates. Mitigation: re-map only after a phase closes.

# Stakeholder Communications

No external communication has occurred and none is planned before the demonstrable milestone. By audience:

- **Project lead.** This document, the revised business case, and the Open Decisions above; the stop criteria; the request to declare the latency budget and the platform matrix before the work that depends on them.
- **Implementers.** The technical approach, dependency map, and feature spec; the repository's agent process rules; the harness tickets as the first nodes to be authored; the defaults under Open Decisions as their fallback positions.
- **External cryptographic reviewer.** The critical path, the research notebook's Q09 increments, the Application Requirements' CR and CD families, and the harness output when it exists; scope is the composition claims and the verifier.
- **Legal counsel.** MVP Scope's licensing and ingest sections, the risk register's R-01 and R-10, and the non-functional review's compliance rows; scope is the license text, the eligibility principle and residual, priced entitlements, the paymaster, and export.
- **Prospective developers, before install.** The project site's education tier: the protocol explained, the browser demonstration running the client's own crates against a sample deployment, and the two onboarding paths. No account, no sign-in, nothing to manage.
- **Adopting developers, at first run.** A plain-language disclosure of what the daemon does with their disk and bandwidth and what is public about their entitlements, delivered before consent, with the consent trace verifying it was shown, and the reserved account-link step shown as unavailable in the MVP. Proposed in the non-functional review; not yet a requirement.
- **Build platforms.** Nothing before the demonstrable milestone; the superseeder argument is that they adopt for their own reasons once the swarm exists.
- **Rights holders.** No outreach; rights-holder reaction is a threat model, not a gate. Their channel, once built, is the advisory mechanism.

# References

- [business-case-revised.md](business-case-revised.md), the synthesized case.
- [business-case.md](business-case.md) and [business-case-critique.md](business-case-critique.md), the original and its review.
- [feature-spec.md](feature-spec.md), sixteen features with acceptance criteria.
- [success-metrics.md](success-metrics.md), north star, KPIs, guardrails, measurement plan.
- [technical-approach.md](technical-approach.md), architecture, components, data, deployment, sequencing.
- [dependency-map.md](dependency-map.md), tickets for the harness and decaying resolution outward, with the Mermaid graph.
- [risk-register.md](risk-register.md), twenty risks.
- [non-functional-requirements.md](non-functional-requirements.md), sixty-seven requirements with dispositions.
- [technical-feasibility.md](technical-feasibility.md), the feasibility verdict.
- [cryptography.md](../research/cryptography.md), [MVP Scope.md](../research/MVP%20Scope.md), [MVP Application Requirements.md](../research/MVP%20Application%20Requirements.md), [MVP Execution Trace.md](../research/MVP%20Execution%20Trace.md), [cryptography-critical-path.md](../research/cryptography-critical-path.md), [cryptography-requirements.md](../research/cryptography-requirements.md), [cryptography-research-notebook.md](../research/cryptography-research-notebook.md), the source documents.
- [ChainTorrent MVP workplan](../workplans/current/ChainTorrent%20MVP.md), the To-Do list and proposed build sequence.

# Additional Content

**What this document is for.** It is the single place a reader can find every position the planning work reached and every decision it left to the user, with a pointer to the document holding the detail. It adds no requirement to the specifications and authors no workplan node. When the Open Decisions are answered, the answers belong in the specifications and the workplan, and this document is revised in place to state them as resolved.

**What changed between the original business case and this synthesis, in one paragraph.** The beachhead argument was re-founded on what substitutes cannot do; the legal framings became analogies and the actual exposure limit was stated; the eligibility principle acquired its residual and lost a wrong High rating; consumption privacy moved from a benefit to a risk for the adopting population; participation costs, CI mechanics, the supply-chain surface, regulatory unknowns, subsidy replacement, cold-start dynamics, and adapter-registry governance were added; missing figures were replaced by a calibration method and stop criteria; phases were renamed by role and reordered to the workplan; nine non-functional gaps and thirty-two harness tickets were identified; and every claim from outside the repository was labelled.
