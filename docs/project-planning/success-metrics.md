<!-- Template: thesis_success_metrics.md -->
# ChainTorrent MVP Success Metrics

Draft, 2026-09-23. Derived from the Cost Instrumentation section of [MVP Scope](../research/MVP%20Scope.md), the Relaying and Observability requirements of [MVP Application Requirements](../research/MVP%20Application%20Requirements.md), and the acceptance scenarios both documents gate release on. Those sources are explicit that no cost, latency, or size figure may be asserted from formulas before it is measured, and that the interactive latency budget is declared before the measurement run so instrumentation produces a pass or a failure rather than a number. This document therefore defines every metric, its direction, its source, and the point at which its target is set. It states no target the sources do not, and it marks each threshold that is set at budget declaration as such.

# Outcome Alignment

The MVP's objective, as MVP Scope states it, is a working install path for JavaScript dependencies that serves popular packages from a peer swarm, reuses across every project on a machine whatever that machine has already fetched, survives an upstream registry outage for packages already ingested, and exercises the full identity, entitlement, credential-delivery, encryption, and transfer lifecycle end to end. The MVP validates distribution and identity. It deliberately does not validate willingness to pay or seeder compensation.

The individual developer is the adopting population, and the benefits are ordered accordingly: cross-project reuse first, swarm retrieval second, registry-outage survival third. Metrics follow the same order. The one thing a developer at a terminal notices is a slower install, so interactive install wall-clock is the binding constraint and the guardrail every other metric is subordinate to.

Three outcomes therefore define success, and each has a metric family below.

| Outcome | What it means | Metric family |
| --- | --- | --- |
| The benefit is felt | A developer reuses what the machine already has, and installs are not noticeably slower | North star, primary KPIs, latency guardrail |
| The lifecycle is proven | Identity, entitlement, verified delivery, encryption, per-attempt authorization, and priced transfer work end to end through the packaged applications | Acceptance scenarios, delivery and settlement KPIs |
| The economics are measurable | Cost per install and cryptographic cost per operation exist as measured numbers | Cost instrumentation, validation harness output |

Everything the MVP defers, price discovery, resale volume, seeder compensation, Sybil resistance, and the load profile of streaming content, has no metric here because the MVP produces no observation of it.

# North Star Metric

**Plaintext CAS reuse: the fraction of package installs served from the local plaintext cache without network, chain, swarm, or authorization, and the number of projects on one machine sharing each artifact.**

MVP Scope names this the MVP's headline adoption metric because it is the benefit the adopting population actually experiences. It is measured on every install (RO-03) and is the observable outcome of the resolution order in PR-03: an install that hits the plaintext CAS is one the developer never waits on. Direction: up. Target: none is stated in the sources; the metric is a rate to be observed and reported, and the first measured value on the dogfood population is the baseline every later release is compared against.

Two things the north star deliberately excludes. It does not count installs served from the local encrypted store, which still require a decryption attempt under a current state view, so a growing encrypted-hit rate is a leading indicator rather than the outcome itself. It does not count anything about revenue, because the MVP is $0.00 by design.

# Primary KPIs

| KPI | Definition | Direction | Target | Source |
| --- | --- | --- | --- | --- |
| Interactive install wall-clock | Time from package-manager request to served artifact for a local developer, and the fraction of it that authorization adds | Down | Pass against the latency budget declared before the run; the budget states the acceptable fraction authorization may add | RO-06, MVP Scope Cost Instrumentation |
| Clean-install success | Both entry points on a clean supported machine reach ready and install a package after only permitted consent | Up | Every supported OS and architecture, both paths | SI-01, SI-02, SI-19; AS-01, AS-02 |
| Registry-outage install success | Installs of ingested assets that succeed with npm unavailable, from ledger, swarm, and an escrow grant alone | Up | Complete for the ingested set | FF-08; AS-11, AS-24 |
| Acceptance scenario pass rate | Scenarios AS-01 through AS-27 passing through the packaged applications with no test-only bypass | Up | All twenty-seven, which is the release condition | MVP Acceptance Scenarios, Completion Boundary |
| Cost per install | Relayer gas per free install and swarm bandwidth per install | Down | None; the first unit-economics datapoint, to be measured and reported | RO-03, MVP Scope Cost Instrumentation |
| Delivery-verification gas | On-chain cost for a mint and for a transfer on the resolved curve, reported as two components on the launch L2: L2 execution gas and the L1 data-posting fee, since the latter dominates on Base | Down | None; input to the curve choice and to grant-pool sizing, measured by the validation harness | CD-07, RO-03; AS-21 |
| Cross-project reuse | Number of projects on one machine sharing a given artifact | Up | None; reported with the north star | RO-03 |

# Leading Indicators

Indicators that move before the north star does and predict whether it will.

- **Local encrypted-hit rate.** Installs served from local ciphertext with a successful decryption attempt. A rising rate means the machine is accumulating swarm-native content that will later be plaintext hits (PR-03, AS-07).
- **Remote encrypted-hit rate.** Installs served from the swarm rather than upstream. Growth means the swarm carries what developers need (PR-03, AS-08).
- **First Finder bootstrap completion.** Fraction of upstream-fallback installs whose background job reaches registration and persistent seeding, and the time it takes. Every completion is a future swarm hit for someone else (FF-07; AS-10).
- **Swarm-native publication coverage.** Fraction of a published package's dependency closure present in the swarm afterward (FF-09; AS-22, AS-23).
- **Seeding archive size and continuity.** Objects seeded per identity and retrieval success by another participant across the host's restart (SW-03, SW-07).
- **State-read volume and latency per install.** The empirical input to `τ_soft` and to whether reading at the declared tier is a noticeable fraction of install time (RO-03, MVP Scope Cost Instrumentation).
- **Decapsulation time per piece group.** The per-attempt cryptographic cost, which bounds how the protocol will feel for content classes that cross the attempt boundary continuously (RO-03).
- **Identity creation and binding.** One binding transaction and one envelope-key registration per identity across restart and reinstall, indicating the free onboarding path is working (IW-03; AS-14).

# Lagging Indicators

Indicators that confirm an outcome after it has happened.

- **North star trend over time** on the dogfood population and then the adopting population, compared against the first measured baseline.
- **Priced settlement completed.** One primary and one secondary priced settlement through the explicit-publisher path with every boundary assertion passing: proof verified and payment released in one transaction, buyer decrypting at the declared tier, seller destroying at `HARD` (AS-15).
- **Escrow claim completed with the First Finder absent**, with escrow-era credentials still working and a successor deployment readable under every live parameter set (AS-16).
- **Requirement-to-proof reconciliation.** Every requirement identifier mapped to a passing proof in CI on a clean machine (XA-07).
- **Measured piece-group size and curve recorded in release evidence**, derived from harness measurements rather than asserted (CD-07; AS-21).
- **Repair, update, and uninstall fidelity.** State preserved and package-manager configuration restored exactly across the recovery matrix (SI-16, SI-17, SI-18; AS-05).

# Guardrails

Metrics that must not degrade regardless of how the north star moves. A guardrail breach blocks release.

- **Latency budget.** Attempt latency and interactive install wall-clock stay within the budget declared before the run. The instrumentation is required to produce a failure, not an unqualified measurement, when the budget is exceeded (RO-06).
- **No protected material in telemetry, logs, diagnostics, storage, or crash artifacts.** Decrypted credentials, piece-group keys, cipher state, keystream, master scalars, and envelope secrets never appear (CR-07, IW-04, RO-03, EC-08; AS-20).
- **No authorization without a current view.** Zero decryptions under a stale view, an altered context, a missing wallet assertion, or below the declared tier (EC-04, EC-05).
- **No accepted invalid delivery.** Zero malformed, replayed, cross-entitlement, or record-contradicting proofs accepted on chain (CD-03, LC-05; AS-13).
- **No protected side effects from invalid compositions.** Zero network, chain, authorization, or secret activity from a composition that fails resolution (IC-05; AS-17).
- **Package-manager parity.** Resolved graphs and lockfile behavior identical to upstream across the representative project set (PR-01).
- **Consent boundary.** No clean install performs an unexplained manual download, account creation, endpoint entry, or component-install instruction (SI-19).
- **Subsidy bound.** Free mints and grants rate-limited per identity; the grant pool is not drained by a flood (LC-10; AS-27).
- **Ciphertext under obligation is never evicted** by plaintext cache policy (PR-05).
- **Foreground install independence.** The initiating install's latency ends with the upstream response and CAS commit regardless of background bootstrap state (FF-02, PR-08; AS-09).

# Measurement Plan

**Stage 1, validation harness.** Before any parameter is fixed, the cryptographic validation harness runs on each candidate curve against a verifier deployed on Base Sepolia and records capsule, envelope, and proof sizes; decapsulation time per piece group; proof generation and verification time; and delivery-verification cost for a mint and a transfer, split into L2 execution gas and the L1 data-posting fee. The piece-group size and curve are chosen from these and recorded in release evidence (CD-07; AS-21). This is the first measurement produced and the only one that has no dependency on the rest of the build.

**Stage 2, budget declaration.** Before the acceptance run, the project declares the interactive latency budget for the local developer population, including the fraction of install wall-clock that authorization may acceptably add. Declaring it after measurement would let the number set the threshold; the source documents forbid that ordering (MVP Scope, Cost Instrumentation; RO-06).

**Stage 3, acceptance run.** All twenty-seven scenarios run through the packaged applications on clean machines. Every metric in RO-03 is captured and reconciled against the activity each scenario induces (AS-20). Telemetry is scanned for sensitive fields. The run yields the first values of the north star, the primary KPIs, and every leading indicator, on the dogfood population.

**Stage 4, dogfood baseline.** The project's own package is published swarm-natively and consumed through a second identity, and the priced primary and secondary settlements execute. The measured values from this stage are the baseline against which the adopting population is later compared.

**Stage 5, adopting population.** After release, the same instrumentation runs on every install by the adopting population. Metrics are reported without secrets and without identifying data beyond what the public entitlement ledger already publishes. The north star trend, cost per install, and state-read cost are reported against the baseline.

**Instrumentation requirements.** Metrics record CAS hit and cross-project reuse, install wall-clock, state-read volume and latency, decapsulation time per group, proof generation and verification time and gas, swarm bytes, ingest bytes, relayer gas, job outcome, and adapter health (RO-03). Structured logs and traces correlate one package request across package host, CAS, swarm, chain, credential custody, decryption, and background First Finder work (RO-04). Health and diagnostics expose the same facts through CLI, desktop, extension, and API (RO-05).

# Risk Signals

Observations that indicate a metric is about to fail or an assumption is wrong.

- **Authorization fraction of install wall-clock rising toward the declared budget** on the dogfood population, before the adopting population sees it. Indicates `τ_soft`, the settlement tier, or the state-read path needs adjustment.
- **Decapsulation time per group multiplied by groups per package approaching the interactive budget.** Indicates the piece-group size is too small for the curve chosen, or the curve choice should be revisited.
- **Delivery-verification gas making the free path's relayer cost per install unsustainable.** Indicates the curve or verifier form choice should be revisited before the grant pool is committed.
- **First Finder job completion falling below the ingest rate.** The swarm is not accumulating what developers fetch, and future hits will not materialize.
- **Seeding discontinuity across restart.** Retrieval by other participants fails after a host restarts, which undermines outage survival.
- **Grant fulfilment slowing when the First Finder is offline.** Indicates too few holders are online or the automatic grant service is rate-limited too tightly.
- **Rising `PENDING_SETTLEMENT` dwell time.** Installs are waiting on tier attainment; the declared tier or the freshness bound is mismatched to the chain.
- **Any sensitive field in a telemetry scan.** A guardrail breach; release blocks.
- **Lockfile or resolved-graph divergence from upstream.** The package host is altering resolution, which it must not do.
- **Any node inconsistency causing attempts to halt** more often than one unreachable node should. Indicates the multi-node state view is not aggregating correctly.
- **Relayer subsidy consumption rising faster than identity creation.** A flood or a rate-limit misconfiguration.

# Next Steps

1. Build the validation harness and produce the Stage 1 measurements on both candidate curves.
2. Choose and record the piece-group size and curve from those measurements.
3. Declare the interactive latency budget, including the acceptable authorization fraction, before the acceptance run.
4. Implement the RO-03 metrics, RO-04 correlation, and RO-05 health surfaces as each feature lands, so no scenario runs uninstrumented.
5. Run the acceptance scenarios and reconcile every metric against induced activity; scan telemetry for sensitive fields.
6. Publish the dogfood package, execute the priced settlements, and record the dogfood baseline.
7. Release, and begin reporting the adopting-population metrics against the baseline.

# Data Sources

| Source | What it supplies |
| --- | --- |
| Local daemon metrics store | CAS hit and reuse, install wall-clock, resolution path, state-read volume and latency, decapsulation time, swarm and ingest bytes, job outcomes, adapter health (RO-03) |
| Structured logs and traces | Per-request correlation across package host, CAS, swarm, chain, custody, decryption, and background work (RO-04) |
| Health and diagnostics API | Installed, configured, degraded, incompatible, and ready states; recovery actions (IC-08, RO-05) |
| Cryptographic validation harness | Sizes, timings, and gas on each candidate curve (CD-07) |
| Chain state and events | Settlements, interval counters, envelope digests, registrations, claims, and relayer-paid transactions; delivery-verification gas |
| Relayer or paymaster reports | Gas spent per sponsored transaction, rate-limit events, subsidy exhaustion (RO-01) |
| Seed host archive | Objects, sidecars, sizes, obligated versus voluntary holdings (SW-07) |
| Demonstration harness | Controlled participants, wallets, chain state, and induced failures for reproducible acceptance runs (XA-07) |
| Telemetry scans | Presence or absence of sensitive fields (RO-03, AS-20) |

No data source records secrets, and none records identifying data beyond what the public entitlement ledger already publishes. Per-attempt state reads are view calls served by configured nodes; which nodes see a client's read traffic is a disclosure the specification records under Dependency Graph Privacy and this plan does not add to.

# Reporting Cadence

The source documents do not fix a cadence. The following is proposed.

- **Per harness run:** the Stage 1 measurement table, with the resulting piece-group size and curve selection once chosen.
- **Per acceptance run:** the full scenario pass table, every RO-03 metric reconciled against induced activity, the telemetry scan result, and the latency-budget pass or fail.
- **At release:** the dogfood baseline for the north star, primary KPIs, and leading indicators, recorded in release evidence alongside the harness measurements.
- **After release:** the north star, cost per install, state-read cost, and guardrail status reported on a regular interval to be set at release, with the baseline as the comparison in every report.

# Ownership

The sources name roles, not people. Ownership is assigned by role.

| Metric family | Owner |
| --- | --- |
| Validation harness measurements, piece-group size, and curve selection | Cryptography implementer |
| Latency budget declaration | Project lead, before the acceptance run |
| North star, resolution-path metrics, install wall-clock | Package host and CAS implementer |
| State-read, attempt, and destruction metrics | Encrypted consumption implementer |
| Delivery, settlement, and gas metrics | Contract suite and chain adapter implementer |
| Bootstrap completion and seeding continuity | First Finder and seed host implementer |
| Relayer cost and subsidy guardrails | Relayer operator |
| Telemetry scans and correlation | Observability implementer |
| Acceptance run, reconciliation, and release evidence | Project lead |

# Escalation Plan

- **Guardrail breach in an acceptance run.** Release blocks. The breaching requirement is reopened in the workplan, the fix lands, and the affected scenarios rerun. No guardrail may be waived by adjusting the metric.
- **Latency budget failure.** The declared budget is not relaxed to admit the measurement. The parameters that feed it, `τ_soft`, the settlement tier, the piece-group size, and the state-read path, are revisited, and the run repeats. If no parameter within the specification's bounds meets the budget, that is reported to the project lead as a finding against the design, not absorbed as a slower install.
- **Harness measurement outside expectation.** If measured sizes, timings, or gas contradict the by-formula estimates in the research, the critical path's measurement obligation is reopened, the curve and verifier form choices are revisited, and the whole-target second pass records the result before any dependent parameter is fixed.
- **Sensitive field in telemetry.** Treated as a security defect: the emitting component is identified through trace correlation, the secret lifecycle service is corrected, and the scan is rerun over the full acceptance path.
- **Accepted invalid delivery or authorization under a stale view.** Treated as a specification-level defect, since both are invariants. The contract verifier or the attempt rule is corrected, cross-verified against the Rust implementation, and the delivery and interval-end scenarios rerun.
- **Post-release north star decline against baseline.** Investigated through the leading indicators in order: resolution-path distribution, encrypted-hit rates, bootstrap completion, seeding continuity. The finding is reported with the indicator that explains it.
- **Post-release cost per install rising.** The relayer operator reports gas and rate-limit events; if the curve or verifier form is implicated, the harness reruns and the finding goes to the project lead.

# Additional Content

**Why no targets appear for cost and cryptographic metrics.** The research closed with proof sketches and by-formula cost estimates and states that no byte, latency, proof, or settlement cost has been measured. MVP Scope requires that the piece-group size and curve be chosen from measurement, not asserted from formulas. Putting a target on an unmeasured quantity would reverse that ordering. These metrics are reported as observed values, the first observation becomes the baseline, and the only threshold in this document that can fail a run is the latency budget, which the project declares before measuring.

**What the metrics do not cover, by design.** Willingness to pay, price discovery, resale volume, seeder compensation, Sybil resistance, retention enforcement, and the load profile of streaming content are deferred under MVP Scope and produce no observation in the MVP. The per-attempt mechanism is validated while its load profile is not, because install-once content crosses the attempt boundary rarely; decapsulation time per group is recorded precisely so that a later content class has a baseline.

**The economic half of protocol overhead.** MVP Scope records that what a single-digit fee would have to cover is not discoverable at this stage and carries no threshold. Cost per install is the MVP's contribution to that question; revenue per install is not produced.
