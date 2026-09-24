<!-- Template: antithesis_non_functional_requirements.md -->
# Non-Functional Requirements Review

## Overview

Draft, 2026-09-23. A review of the non-functional requirements the ChainTorrent MVP must meet, drawn from [cryptography.md](../research/cryptography.md), [MVP Scope](../research/MVP%20Scope.md), [MVP Application Requirements](../research/MVP%20Application%20Requirements.md), the [workplan](../workplans/current/ChainTorrent%20MVP.md), and the planning documents written against them, in particular the [success metrics](success-metrics.md) and [risk register](risk-register.md). The source documents state most of these requirements as operational requirements with proof obligations rather than as a separate non-functional list, so this review extracts them, gives each an identifier of the form NF-nn, cites where it is stated, and assigns a disposition: **stated** where a source fixes the requirement and its proof; **partial** where a source names the property but not a threshold, mechanism, or proof; **gap** where nothing in the sources addresses it and this review proposes it. Proposed requirements are marked as proposals and add nothing to the specifications until adopted there.

Two conventions from the sources govern this document. First, every requirement in the Application Requirements is simultaneously an operational requirement and an integration or end-to-end test obligation, so a non-functional requirement here is not satisfied by a unit test where its proof crosses a process, adapter, storage, network, custody, or chain boundary. Second, no threshold that the sources leave to measurement is given a value here; where a threshold is set at declaration, the review says so.

## Security

The protocol's security posture is stated in cryptography.md's Security Properties and Security Considerations, and the application's in the Application Requirements' cross-cutting and per-family rows. The review groups them by what they protect.

| ID | Requirement | Source | Disposition |
| --- | --- | --- | --- |
| NF-S01 | Confidentiality: no party holding no credential can derive decryption capability from the ciphertext, the header sidecar, the ledger's envelopes, or the delivery proofs; holds under SXDH, decisional BDH-3b, and the random-oracle model | cryptography.md, Security Properties; research notebook Q09 | Stated at the research level; external review pending |
| NF-S02 | Delivery soundness: a settlement the contract accepts has delivered a valid credential for the exact entitlement to the registered recipient keys; a seller cannot be paid without delivering and a buyer cannot obtain the envelope before payment is locked | cryptography.md, Security Properties; LC-05, CD-01 through CD-03 | Stated |
| NF-S03 | Replay and non-malleability: every delivery proof hashes its complete statement into its challenge and is valid for exactly one settlement; the contract rejects an advanced counter | cryptography.md, Replay Attacks; CR-09 | Stated |
| NF-S04 | Authorization is per attempt from a fresh state view at the declared tier, never from a cached view, a prior session, or a credential for another interval | cryptography.md, Cryptographic Authorization Invariants; EC-04, EC-05 | Stated |
| NF-S05 | No selective withholding: reading needs no party but the holder, transfer needs no party but seller and buyer, and no step of use or transfer requires the live action of any single third party | cryptography.md, Architectural Invariants | Stated |
| NF-S06 | Decrypt-capable material is memory-only, excluded from logs and diagnostics, and zeroized at every success, interval-end, cancellation, loss, and error transition; the persistent credential is held only under custody | EC-08, CR-07, IW-04 | Stated |
| NF-S07 | Every external input, descriptor, adapter response, IPC request, network message, and chain event is untrusted until validated at its boundary, and validation precedes any credential exercise | XA-01, EC-01, CR-06; cryptography.md, Manifest Bounds Validation | Stated |
| NF-S08 | Local APIs are authenticated and least-privileged; a project process gains no custody, credentials, keys, or authority by requesting a package | XA-02 | Stated |
| NF-S09 | Downloaded artifacts and updates are authenticated before execution; tampering halts without replacing the working version | SI-04, IC-02, SI-17 | Stated |
| NF-S10 | Envelope keys are never wallet keys; the two envelope secrets are independent; identity-element keys and trivial identity elements are rejected | IW-07, CR-04, LC-08 | Stated |
| NF-S11 | Signing-key custody for artifact signing, its rotation, and who holds it | XA-08 | Stated: artifact signing keys are held under the same custody discipline as issuance material, with a documented rotation and revocation path, before the first signed release |
| NF-S12 | Seeder agnosticism: possession of encrypted pieces or the sidecar confers no access | cryptography.md, Seeder Agnosticism | Stated |
| NF-S13 | Blast radius: a leaked credential exposes one asset; a leaked piece-group key exposes one group of one deployment; no number of credentials recovers the master scalar | cryptography.md, Blast Radius Containment | Stated |
| NF-S14 | Front-running resistance: claim proofs bound to the claimant's chain identity; verifier key revocable on chain | cryptography.md, Escrow Claim Front-Running; PC-03, PC-07 | Stated |
| NF-S15 | Failure messages identify the failed capability and a safe recovery action without exposing secrets or suggesting insecure bypasses | XA-04 | Stated |
| NF-S16 | Threat model for the daemon as a supply-chain surface on developer machines, stated as such | XA-09; risk register R-07 | Stated: a threat-model statement for the local package host covering same-user processes, local privilege escalation, artifact substitution, and redirect abuse |
| NF-S17 | Accepted residuals stated, not overclaimed: modified-client retention, common piece-group keys, settlement reversal | cryptography.md, Modified Clients, Common Piece-Group Keys, Settlement Reversal | Stated |

**Assessment.** The cryptographic and application security requirements are unusually complete and each carries a proof obligation. Signing-key custody for the release channel and the package host's threat-model statement are XA-08 and XA-09. External review of the composition and the verifier is a stated prerequisite for any priced deployment and is the one security requirement whose satisfaction depends on a party outside the project.

## Performance

| ID | Requirement | Source | Disposition |
| --- | --- | --- | --- |
| NF-P01 | Interactive install wall-clock for the local developer, with the fraction authorization may add, within a latency budget declared before the measurement run; instrumentation must produce a pass or failure | RO-06; MVP Scope, Cost Instrumentation | Stated; threshold set at declaration |
| NF-P02 | A plaintext CAS hit serves with no network, chain, swarm, or authorization work | PR-06 | Stated |
| NF-P03 | The initiating install's latency ends with the response of the source that served it, independent of background bootstrap, request registration, ciphertext prefetch, and grant pickup | FF-02, PR-03, PR-08 | Stated |
| NF-P04 | Per-attempt state reads are view calls, batched and paginated across a dependency closure with the ceiling advertised by the adapter, never one request per package and never one request the recipient cannot accept | LC-06; MVP Scope, Entitlement Ledger | Stated |
| NF-P05 | Decapsulation cost per piece group and the piece-group size chosen from measurement against the latency budget | CD-07, AS-21 | Stated; value set by measurement |
| NF-P06 | Seekable, order-independent decryption: any piece decrypts from its index alone, with no per-piece expansion and no padding | cryptography.md, Cryptographic Primitives | Stated |
| NF-P07 | Incremental verification at Bao-chunk granularity without downloading the remainder | cryptography.md, Incremental Verification | Stated |
| NF-P08 | Delivery-verification gas per mint and transfer measured on the resolved curve and used to select curve and verifier form | CD-07, RO-03 | Stated; value set by measurement |
| NF-P09 | `PENDING_SETTLEMENT` handled by bounded wait and retry rather than failure, so an install that reads state it just wrote does not abort | EC-05; cryptography.md, Phase 2 | Stated; backoff bounds unspecified |
| NF-P10 | Resource footprint of the daemon on a developer machine: CPU at idle, memory, and bandwidth consumed by seeding and by the background prefetch of ciphertext, which roughly doubles a first run's disk and bandwidth | RO-07, ST-06, PR-11, SI-20 | Stated: the daemon exposes and respects configurable upload bandwidth and idle CPU limits, the prefetch obeys them and the ciphertext quota, the first-run cost is disclosed at install, and the acceptance run records idle footprint on each supported platform |
| NF-P11 | Cold-start time of the daemon after reboot before the package host is serving | RO-08 | Stated: measured and reported in the acceptance run; no threshold |

**Assessment.** The performance model is correctly built around one declared budget and a set of measurements that feed it. The developer's side of the ledger, the daemon's idle and seeding footprint, is RO-07; it is the cost a developer notices after install latency.

## Reliability

| ID | Requirement | Source | Disposition |
| --- | --- | --- | --- |
| NF-R01 | Long-running operations are durable, restartable, idempotent jobs that survive process and machine failure with exactly-once effects or safe compensation | XA-03, FF-03, IC-07 | Stated |
| NF-R02 | The daemon and seed host survive reboot and editor or terminal closure without losing, duplicating, or corrupting durable work | SI-06, SW-03, AS-18 | Stated |
| NF-R03 | Installation, update, repair, and uninstall are checkpointed with rollback to the prior coherent state | SI-08, SI-15, SI-16, SI-17, SI-18 | Stated |
| NF-R04 | Transport and storage failures leave resumable acquisition and seeding jobs without exposing unauthenticated partial data | SW-06 | Stated |
| NF-R05 | The per-attempt state view is a quorum of two of three configured nodes agreeing at a common reference within the freshness bound; one unreachable or stale node does not deny; divergent state at one reference and an over-age view fail closed | XA-06 | Stated, as a decision table |
| NF-R06 | No single operator's unavailability denies reading, transfer, grant, or claim: reads are local; grants under the escrow suite come from any holder; claims never need the First Finder | cryptography.md, Intermediary Independence; CD-05, CD-06, PC-04 | Stated |
| NF-R07 | A reorganized transfer never strands a still-owner: attempts stop at the declared tier and destruction waits for `HARD` | EC-08, LC-04 | Stated |
| NF-R08 | Readiness is an active end-to-end health probe, not the presence of files or processes | SI-13 | Stated |
| NF-R09 | A lost envelope is recoverable from chain history through any honest full-history node | CD-04, IW-05 | Stated |
| NF-R10 | Data availability of envelopes and proofs: carried in settlement calldata and events, not by a hash of bytes held elsewhere | cryptography.md, Credential Delivery; research notebook Q09 fourth increment | Stated |
| NF-R11 | Availability of the swarm object itself: replication is a constructed factor through retention assignment | MVP Scope, Retention Obligation | Partial. The shape ships; nothing is enforced and no replication factor is set. In the MVP, availability of an ingested asset rests on the project seed host for the core closure and on voluntary seeding for the rest |
| NF-R12 | Backoff bounds and retry limits for `PENDING_SETTLEMENT`, discovery, transport, and queued request submission | XA-10; EC-05, SW-06, PR-10 | Stated: bounded exponential backoff with declared ceilings per adapter, recorded in the configuration registry |

**Assessment.** Reliability of the local system is thoroughly specified through durable jobs and checkpointed lifecycle operations. Reliability of the network is where the MVP is honest about its limits: the retention obligation is unenforced, so long-tail availability is voluntary, and the seed host covers only the core closure. This is a stated MVP boundary, not an oversight, and the review records it so it is not mistaken for a guarantee.

## Scalability

| ID | Requirement | Source | Disposition |
| --- | --- | --- | --- |
| NF-SC01 | Credential size is constant across any number of sales; capsule cost is independent of the number of entitlements; issuance is unbounded with no setup-time count | cryptography.md, Credential KEM; research requirements R10 | Stated |
| NF-SC02 | Sidecar overhead is proportional to payload at one capsule and one wrapped group key per piece group per live parameter set, estimated between roughly 0.8 and 1.4 percent for a 16 KiB group with one live set depending on curve and identity scope, and a few hundredths of a percent for 1 MiB | critical path, Costs by formula | Stated by formula; unmeasured |
| NF-SC03 | On-chain state per entitlement is constant per interval: holder, two keys, one digest; envelopes and proofs are calldata and events | research notebook Q09 fourth increment | Stated |
| NF-SC04 | Batch reads paginate with an adapter-advertised ceiling, so a thousand-package closure is one traversal | MVP Scope, Entitlement Ledger | Stated |
| NF-SC05 | Deployments may be multi-homed across transports as one object; discovery aggregates across concurrent sources | SW-01, SW-02 | Stated |
| NF-SC06 | Relayer cost scales with identities and free mints and is bounded by a global per-window budget and maximum sponsored liability, with per-identity limits as one layer | LC-10, RO-01 | Stated. The budget bounds exposure; the grant pool's size and its growth with adoption remain to be set from measured gas |
| NF-SC07 | Chain throughput: one binding transaction per identity and one settlement per mint or transfer; per-attempt reads are view calls | MVP Scope, Gas Relayer | Stated; no capacity analysis against a chosen network |
| NF-SC08 | Load profile of continuous-decryption content classes | MVP Scope, What the MVP Does and Does Not Exercise | Stated as out of scope; decapsulation time per group is recorded as the baseline |
| NF-SC09 | Plaintext CAS and ciphertext store are sized and retained independently, with quota and eviction policy exposed | PR-05, SW-05 | Stated |

**Assessment.** The cryptographic scaling properties are the construction's strengths and are stated exactly. The unbounded quantities are economic rather than technical: relayer cost with adoption and chain capacity on Base. Both resolve with the harness measurements and the dogfood run.

## Maintainability

| ID | Requirement | Source | Disposition |
| --- | --- | --- | --- |
| NF-M01 | Every cryptographic component and every external touchpoint is an adapter to an interface with declared capabilities; composition is validated at resolution and fails closed | cryptography.md, Adapter Composition; research requirements R15; SI-10, IC-05 | Stated |
| NF-M02 | Three dependency rings with dependencies pointing inward; the protocol and domain ring depends on no host, chain SDK, wallet, transport, or storage engine | Application Requirements, Composition Boundary | Stated |
| NF-M03 | One Rust implementation of protocol rules, workflows, cryptography, storage, networking, and delivery shared by every surface; TypeScript only where a host imposes it and owning no protocol logic | Application Requirements, Implementation Language and Runtime Policy | Stated |
| NF-M04 | Every requirement identifier maps to a proof class and a test facility; unit, integration, and end-to-end facilities ship with the repository | XA-07; MVP Scope, Test Facilities | Stated |
| NF-M05 | Immutable deployment suites: an incompatible cryptographic change is a successor deployment with a sidecar per live parameter set, never a reinterpretation of existing ciphertext | cryptography.md, Adapter Composition; LC-09 | Stated |
| NF-M06 | Versioned configuration registry with schema-aware, reversible migrations | IC-04, SI-17 | Stated |
| NF-M07 | Repository authoring discipline: test-first, bottom-up, one file per turn, strict typing, guards at every boundary, `Success | Error` returns | docs/agents topics; workplan node template | Stated |
| NF-M08 | Specification discipline: every rule classified as invariant, defined behavior, required-but-unresolved, or implementation choice; undetermined decisions held in one To-Do list; no fact in two places | cryptography.md, Statement Classes; workplan To-Do | Stated |
| NF-M09 | Observability for maintenance: structured logs and traces correlating one request across every subsystem; health states consistent across surfaces | RO-04, RO-05, IC-08 | Stated |
| NF-M10 | Contract upgradeability: constructor-injected adapters from a governance-controlled registry, immutable once bound, with optional EIP-1967 proxies | cryptography.md, Population Strategy | Stated: a single project-held key governs the registry in the MVP, recorded as scaffolding with its replacement path held in the workplan (risk register R-14) |
| NF-M11 | Adapter and suite versioning across releases: how a client with an older suite implementation treats a newer deployment | IC-09 | Stated: a compatibility statement per release listing the suite identifiers and adapter versions it can resolve |

**Assessment.** Maintainability is where the project has invested most deliberately: adapters, rings, one implementation, a proof class per requirement, and an authoring discipline enforced by the repository's own rules. The adapter registry's governance is scaffolding under a single project-held key, and IC-09 carries the compatibility statement.

## Compliance

| ID | Requirement | Source | Disposition |
| --- | --- | --- | --- |
| NF-C01 | Ingest eligibility: adapters target archives whose content is publicly distributed at no charge by the rights holder's choice, implemented for the MVP as public npm availability; the residual is unauthorized public redistribution and irrevocability, not denied revenue | cryptography.md, Ingest Source Eligibility; MVP Scope, Canonical Identity; risk register R-01 | Stated |
| NF-C02 | Software license: source-available with a conformance clause making a non-conforming client a violation; per-asset content terms; the project's packages obligating an entitlement for use and a license per copy sold or bundled | MVP Scope, Distribution and Client License; LI-01 | Partial. Required and unstarted; legal drafting is on the release path |
| NF-C03 | Maintainer privacy in escrow records: no maintainer-derived field published on chain at all; the claim set is established from upstream metadata at verification | MVP Scope, Entitlement Ledger and Escrow Claim; PC-02, PC-04 | Stated |
| NF-C04 | Consumption privacy: entitlement records are public by design for public content; a first run in the default mode publishes the identity's dependency closure as grant requests at once, consented at install, and cache-only mode leaves no chain footprint; the individual and private-content cases are recorded as open with candidate constructions; no private-registry adapter ships before a consumption-privacy answer | cryptography.md, Dependency Graph Privacy; SI-20, PR-10 | Stated as open; onboarding disclosure and cache-only mode in the requirements |
| NF-C05 | Telemetry records no secrets and no identifying data beyond what the ledger publishes | RO-03; success metrics, Data Sources | Stated |
| NF-C06 | Consent: user interaction limited to unavoidable OS permissions, explicit package-manager redirect consent, the request and prefetch consent items with the first-run cost disclosure, custody user presence, recovery confirmation, and funding for the paid proof; a consent trace fails for anything else | SI-19, SI-20 | Stated |
| NF-C07 | Content governance: no takedown; advisories are signed, append-only, and evaluated against a local trust set; jurisdictional obligations for gateway and indexer operators are raised, not answered | cryptography.md, Content Governance | Stated as framed; not implemented in the MVP |
| NF-C08 | Regulatory treatment of priced transferable entitlements and of the paymaster | LI-02 | Stated: legal review scoped to these, in parallel with the harness, before the transaction flow proof |
| NF-C09 | Export or cryptographic-distribution constraints on shipping a client containing AES and pairing cryptography to every supported platform and jurisdiction | LI-02 | Stated: confirmed during legal review; standard open cryptographic libraries typically qualify for exemption but the project ships a packaged binary |
| NF-C10 | Accessibility of the desktop application and extension | IC-10 | Stated: platform accessibility baselines for the Tauri surfaces and the extension's webviews, recorded in release evidence |

**Assessment.** Compliance is the least specified area. The license, the regulatory review, and the export question are on the release path and none has started.

## Outcome Alignment

The non-functional requirements serve the same three outcomes the success metrics define. The benefit is felt: NF-P01 through NF-P03 and NF-P10 govern whether a developer notices anything but a faster second install. The lifecycle is proven: NF-S02 through NF-S06, NF-R06, and NF-R07 are the invariants whose failure would be silent from the outside and whose proof is the MVP's hardest deliverable. The economics are measurable: NF-P05, NF-P08, NF-SC02, and NF-SC06 are what the harness and instrumentation exist to produce. The compliance set aligns with the business case's strategic thesis: rights-holder exposure is confined to the ingest path by NF-C01, and the protocol's refusal to withhold or take down is NF-S05 and NF-C07 stated as design rather than as risk.

## Primary KPIs

The [success metrics](success-metrics.md) hold the full KPI set. The non-functional subset that a release is gated on:

| KPI | Requirement served | Target |
| --- | --- | --- |
| Interactive install wall-clock and authorization fraction | NF-P01 | Pass against the declared budget |
| Zero protected material in telemetry, logs, storage, crash artifacts | NF-S06, NF-C05 | Zero |
| Zero accepted invalid deliveries; zero authorizations under a stale or altered view | NF-S02, NF-S03, NF-S04 | Zero |
| Zero protected side effects from invalid compositions or untrusted inputs | NF-S07, NF-M01 | Zero |
| Durable-job recovery at every persisted transition | NF-R01, NF-R02 | Every checkpoint |
| Clean-install, reinstall, repair, and uninstall fidelity across the platform matrix | NF-R03 | Every supported platform |
| Delivery-verification gas and decapsulation time per group | NF-P05, NF-P08 | Measured and recorded; no target |

## Leading Indicators

- State-read volume and latency per install, the empirical input to `τ_soft` (NF-P01, NF-P04).
- Decapsulation time per group multiplied by groups per package on the dogfood corpus (NF-P05).
- Daemon idle CPU, memory, and upload bandwidth on each platform (NF-P10).
- Bootstrap job completion rate and time after induced termination (NF-R01).
- Requirement-to-proof reconciliation progress against node completion (NF-M04).
- Adapter capability declarations present for every implementation before its consumer is authored (NF-M01).

## Lagging Indicators

- Every acceptance scenario passing on clean machines.
- External cryptographic review completed with findings dispositioned (NF-S01, NF-S02).
- License text adopted and content-terms field populated for the project's packages (NF-C02).
- Legal review of priced entitlements, the paymaster, and export complete (NF-C08, NF-C09).
- Piece-group size, curve, `τ_soft`, and `τ_wallet` recorded in release evidence (NF-P05, NF-P08).

## Measurement Plan

The [success metrics](success-metrics.md) measurement plan applies unchanged: budget declaration, validation harness, acceptance run, dogfood baseline, adopting population. Non-functional additions to that plan:

- **Security proofs run as scenarios, not as audits.** Every NF-S row with a requirement identifier is proven by its named integration or end-to-end test; the acceptance run includes the fuzzing, mutation, replay, and telemetry-scan cases.
- **Platform matrix.** Reliability and installation requirements are measured on Windows, macOS, and Linux, not on one reference machine.
- **Footprint capture.** Idle footprint and cold-start time are captured in the same acceptance run on each platform.
- **Compliance checklist.** NF-C02, NF-C08, and NF-C09 are checked as release evidence items with a document reference each, since they have no runtime measurement.
- **Review closure.** External cryptographic review findings are tracked to disposition and recorded in release evidence before the transaction flow proof.

## Risk Signals

The [risk register](risk-register.md) and success metrics carry the full set. Those specific to non-functional properties:

- Authorization fraction of install wall-clock rising toward the budget (NF-P01).
- Any sensitive field in a telemetry scan (NF-S06).
- Any divergence between the Rust and on-chain verifier on the harness vector set (NF-S02).
- Any IPC call from an unprivileged principal succeeding beyond a package request (NF-S08).
- Host configuration differing from the pre-install snapshot after a failed install (NF-R03).
- Attempts halting on node disagreement more often than one failed node explains (NF-R05).
- Grant fulfilment slowing with the First Finder offline (NF-R06).
- Relayer subsidy consumption rising faster than identity creation (NF-SC06).
- A node's `deps` element naming an adapter with no capability declaration (NF-M01).
- Daemon upload bandwidth or idle CPU exceeding the configured limits (NF-P10).

## Guardrails

Non-functional guardrails that block release. They are the same as the success metrics' guardrails, restated against requirement identifiers:

- Latency budget pass (NF-P01).
- No protected material anywhere it should not be (NF-S06, NF-C05).
- No authorization without a current view; no accepted invalid delivery (NF-S02 through NF-S04).
- No side effects from invalid compositions or untrusted inputs (NF-S07, NF-M01).
- Consent boundary honored; package-manager configuration restored exactly (NF-C06, NF-R03).
- Free path rate-limited; grant pool not drained (NF-SC06).
- Ciphertext under obligation never evicted by plaintext policy (NF-SC09).
- Foreground install independent of background bootstrap (NF-P03).
- No priced deployment before external cryptographic review and legal review (NF-S01, NF-C08).

## Next Steps

- Write the local package host's threat-model statement for release evidence (NF-S16).
- Record artifact signing-key custody in the release process before the first signed artifact (NF-S11).
- Start legal drafting and review in parallel with the harness (NF-C02, NF-C08, NF-C09).
- Hold the adapter registry governance key's replacement path as a workplan item (NF-M10).
- Carry NF-P10 and NF-P11 into the acceptance run's measurement set.

# Additional Content

## Usability and user experience

The document descriptor names usability and the template has no section for it. The sources state it through the consent and installation requirements rather than as a separate property.

| ID | Requirement | Source | Disposition |
| --- | --- | --- | --- |
| NF-U01 | Either entry point yields a working system with no manual component installation, endpoint entry, or account creation | SI-01, SI-02, SI-19 | Stated |
| NF-U02 | Package-manager redirect is explicit, visible, and reversible; a toolchain is never silently rerouted | SI-07; MVP Scope, Package Manager Integration | Stated |
| NF-U03 | One repair operation from any surface restores service without discarding state | SI-16 | Stated |
| NF-U04 | Health, diagnostics, and failure messages are consistent across CLI, desktop, extension, and API and name a safe recovery action | RO-05, XA-04, IC-08 | Stated |
| NF-U05 | The seeding archive is visible as the user's contribution; the plaintext CAS is private; eviction warns before removing anything not re-authorizable | SW-07; MVP Scope, Local CAS | Stated |
| NF-U06 | A free package installs without the user holding or knowing about cryptocurrency | MVP Scope, Gas Relayer; RO-01 | Stated |
| NF-U07 | Onboarding discloses what is public about the user's entitlements and what the daemon will do with their disk and bandwidth | None | Gap. Proposed: a plain-language disclosure at first run covering the public ledger, seeding, and the two local stores, with the consent trace verifying it was shown |
| NF-U08 | Recovery and multi-device use never silently create a different principal and expose rotation semantics to the user | IW-05 | Stated |

## How to read the dispositions

"Stated" means a source fixes the requirement and its proof; it does not mean the requirement is built or proven. Nothing is built. "Partial" and "gap" are findings for the specifications and the workplan, not for the code, and each proposal is worded so that it can be copied into the Application Requirements as a row with a proof column. The review found no non-functional requirement in the sources that conflicts with another; the gaps are absences, and most are on the compliance and operational side rather than the protocol side.
