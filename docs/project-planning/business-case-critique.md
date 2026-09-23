<!-- Template: antithesis_business_case_critique.md -->
# ChainTorrent Business Case Critique

Draft, 2026-09-23. A critical review of [business-case.md](business-case.md), read against the source documents it claims to derive from ([cryptography.md](../research/cryptography.md), [MVP Scope](../research/MVP%20Scope.md), [MVP Application Requirements](../research/MVP%20Application%20Requirements.md), [MVP Execution Trace](../research/MVP%20Execution%20Trace.md), the three cryptography research files, and the [workplan](../workplans/current/ChainTorrent%20MVP.md)) and against the companion planning documents ([feature-spec.md](feature-spec.md), [success-metrics.md](success-metrics.md), [technical-approach.md](technical-approach.md)). The review treats the business case's own reasoning as untrusted and re-derives each claim from the sources. Severity is stated per finding: **high** means a reader relying on the claim would make a wrong decision; **medium** means the claim is misleading or incomplete in a way that matters; **low** means precision or presentation.

# Executive Summary

The business case is faithful to the source documents on the protocol, the MVP boundary, the research state, and the risks, and it is disciplined about not inventing figures. Its structure now matches the template. Its central argument is sound as far as it goes: the hard cryptographic problem is closed, the beachhead content class carries the least friction, and the design is abstracted so everything after the MVP is an adapter.

Its weaknesses are of a different kind. The case argues the protocol well and the business thinly. It does not confront the sharpest objection to the beachhead: at $0.00, the benefit a developer actually feels, cross-project reuse, is already available from pnpm's content-addressable store, and the protocol's genuine novelty, transferable irrevocable entitlements, is invisible to that developer. It leans on two legal framings, digital first sale and the fair-use line drawn for machine-learning corpora, that do not transfer to verbatim redistribution as cleanly as the prose implies; what actually limits the MVP's exposure is that every package it ingests was already distributed to the public at no charge by its publisher's own choice, and the case should say that rather than borrow authority from rulings about different conduct. It treats the public entitlement ledger as a benefit while the specification itself says the individual developer, who is the adopting population, is not the party that argument covers. It states no cost, no timeline, no team, and no funding ask, which the template does not demand but a reader deciding whether to fund will. And it carries two factual errors and a handful of claims sourced from general knowledge rather than from the repository.

None of this undermines the recommendation to build the validation harness first. It does mean the business case should be read as a protocol brief with a business case attached, and revised before it is used to persuade anyone outside the project.

# Fit to Original User Request

The request was to read the cryptography and MVP documents, draft a business case in the project-planning folder, and then revise it to the template. The document does both. It draws every protocol claim from the named documents, it places itself in the requested folder, and its top-level sections follow the template in order, with the material that predates the template restructuring preserved under Additional Content.

Two departures from a strict reading of the request. First, the Competitive Analysis and parts of User Problem Validation and Market Opportunity cite products, incidents, and rulings that appear nowhere in the repository; the request was to draft from the named files, and these additions, while appropriate for a business case, are the only claims a reader cannot verify against the repository. The document flags this in one place but not consistently. Second, the Additional Content section is longer than the template's fixed sections combined, which inverts the template's emphasis: a reader following the template's structure meets the business argument first and the protocol argument second, but the weight of the document is the reverse.

# Strengths

- **Fidelity to the sources.** Every protocol, research-state, and risk claim traces to a named document, and the accepted residuals are stated rather than softened. The document does not claim revocation, fair exchange, or hostile DRM, and it says why.
- **Discipline about unmeasured quantities.** No market size, cost model, latency, gas, or byte figure is invented. The document says what the sources say: measure first.
- **The ordering argument is correct.** Validation harness first, because the piece-group size and curve gate everything that encrypts; external cryptographic review before anything priced; deferred items held deferred because the architecture protects them.
- **The threat model is placed correctly.** Rights-holder exposure is located on the ingest path, not the swarm, entitlements, or transfer, and the mitigation is an adapter-selection policy that exists in the specification.
- **The competitive table is structurally accurate.** Every system that enforces rights puts a service or device on the read path; every system that avoids the read path enforces no rights. That is the real differentiation and the document states it plainly.
- **The SWOT is honest.** No code, unmeasured cost, large surface, accepted residuals, subsidized chokepoint, and open blocking selections all appear under Weaknesses rather than being omitted.

# Weaknesses

- **The beachhead value proposition is not defended against its obvious substitute.** pnpm already gives a developer a machine-wide content-addressable store with hardlinks into `node_modules`, which is the cross-project reuse the business case names as the north star benefit. Verdaccio and offline mirrors already give enterprises outage insulation. The business case's Competitive Analysis lists both and says they lack a swarm and a rights layer, which is true, but it never asks what the developer gains from the swarm and the rights layer at $0.00 for permissively licensed content. The honest answer is registry-outage survival for the long tail, which the document itself ranks third, and participation in a network whose value arrives later. That is a weaker beachhead argument than the document presents, and it should be made explicitly rather than left for a skeptical reader to construct.
- **Two legal framings are stronger in the prose than in the law.** The document presents digital first sale as a property right restored, and it invokes the fair-use line drawn for machine-learning corpora to locate rights-holder exposure. Neither transfers directly. Courts have declined to extend first sale to digital copies where a transfer necessarily makes a new reproduction, so "digital first sale" is a property the protocol constructs, not a doctrine it can invoke. And the machine-learning holding concerned transformative use; the protocol redistributes verbatim copies, which is not transformative. What limits the MVP's exposure is that every package on the public npm registry was placed there by its publisher for anyone to download at no charge, so the swarm gives no user access they did not already have; the document should say that rather than borrow authority from rulings about different conduct.
- **The eligibility principle is stated imprecisely, and the residual it leaves is misdescribed.** The business case says ingest is confined to archives whose content is already free to use, and the specification uses the same phrase. MVP Scope implements it as npm availability with no license classification. Those are closer than the wording suggests: npm serves every public package to anyone at no charge regardless of its license, so what availability actually tests is that the publisher chose free public distribution, and a user obtains nothing from the swarm they could not already obtain from npm. The residual is therefore not denied revenue, which is what the post-claim pricing paradox concerns and which does not arise for content the publisher gives away. It is two narrower things. First, First Finders and seeders redistribute to the public without the grant the publisher gave npm, and some restrictive licenses forbid redistribution even of freely obtainable bytes; public npm mirrors carry the same exposure today and it is largely uncontested, but a swarm serving strangers is a different posture from a proxy serving one organization's licensees. Second, the swarm is irrevocable by design, so a publisher loses the ability to unpublish that npm gave them. The document should state the principle as "publicly distributed at no charge by the rights holder's own choice," name npm availability as its implementing test, and name these two residuals rather than a license gap.
- **The adopting population is the population the privacy argument excludes.** The business case presents the public entitlement ledger as a security and economic benefit. The specification's own Dependency Graph Privacy section says that argument concerns organizations concealing inherited risk, that a solo participant's install history is a behavioral profile, that pseudonymity only partially helps because a dependency set approaches a fingerprint, and that state-read traffic sharpens the exposure. The individual developer is the MVP's adopting population. The business case should carry this tension into Risks and Differentiation rather than presenting only the favorable half.
- **The business case has no business figures.** No cost estimate, no timeline, no team size or composition, no funding ask, no runway, and no criteria for stopping. The template does not require them, and the sources supply none, but a document titled business case that cannot answer "how much and how long" will be read as a technical brief.
- **The developer's cost of participation is absent.** Running a daemon, holding ciphertext, seeding upstream, and consuming disk for a plaintext CAS plus a ciphertext store are costs the developer pays, and upstream bandwidth is metered or capped for many of them. The value proposition lists only benefits.
- **CI and enterprise adoption are asserted, not designed.** The document says CI and enterprises adopt as a consequence of individual adoption. It does not say how a CI runner obtains an identity, whether ephemeral runners each need a relayer-paid binding, or how per-identity rate limits on free mints interact with fleets. The workplan's problem statement expects build platforms to become superseeders for their own economic reasons, which is a stronger and more specific argument than the business case makes.

# Opportunities

- **Make the long-tail outage argument the beachhead argument.** pnpm's cache holds what one machine fetched; the swarm holds what anyone fetched. That is the benefit no substitute offers, and it grows with adoption. The document should lead with it for the developer rather than with reuse, which is a parity feature.
- **Build platforms as superseeders.** The workplan's problem statement argues that a build platform running the same few thousand popular installs on always-on machines holds the ideal cache as a by-product and can seed it at almost no cost. That is the most concrete adoption mechanism in any of the source documents and the business case does not use it.
- **Supply-chain security as a first-class benefit.** Provenance attestations recorded on chain, advisories that follow the plaintext fingerprint across repackaging, and a verifiable dependency inventory are benefits the security-conscious part of the developer population values now, independent of the swarm.
- **A thinner first demonstration.** The workplan's sequence reaches "an ordinary `npm install` resolves against the swarm" at its sixth step, before credential delivery, publishing, or claims. The Application Requirements forbid a reduced demonstration path as a release, but an internal milestone at that point produces something to show and measure months before the completion boundary.
- **The validation harness as a publishable result.** Measured sizes, timings, and gas for the construction on both curves are a contribution to the applied cryptography record in their own right, and publishing them invites the external review the document says it needs.

# Threats

- **A registry operator or package-manager maintainer ships outage insulation or a peer cache natively.** The beachhead benefit is capturable by an incumbent without a rights layer.
- **The daemon becomes an attractive supply-chain target.** A local registry redirect on every developer machine is precisely the component an attacker would want to compromise. The Application Requirements address authenticated artifacts, least-privilege local APIs, and untrusted inputs; the business case does not mention that the protocol's own distribution channel is now a supply-chain surface, and a security reviewer will.
- **Consumption privacy becomes a reason not to adopt.** If the individual developer population learns that its install history is a permanent public record, the beachhead population is the one most able to walk away.
- **Regulatory treatment of transferable priced entitlements and of the relayer.** The document says token economics and fiat rails carry regulatory drag and defers them. It does not consider whether tradeable ERC-721 access rights sold above zero, even in the dogfood proof, or a paymaster subsidizing transactions, attract securities, consumer-protection, or money-transmission scrutiny in any jurisdiction. This is outside the sources and should be stated as an open legal question rather than left unmentioned.
- **The relayer subsidy scales with adoption and is unfunded.** Every new identity costs a binding transaction and every free mint verifies a proof on chain. Success makes the subsidy more expensive, and the business case names no bound.
- **Chain properties the project does not control.** Precompile availability on the launch L2, settlement latency, and RPC censorship are named; a chain reorganization policy change or a precompile repricing is not.

# Problems

Substantive issues with the argument, as distinct from errors of fact.

1. **The differentiation the document is proudest of is invisible at the beachhead.** Transferable, irrevocable, resellable entitlements are the protocol's distinguishing property, and at $0.00 on permissively licensed packages no developer will notice or use them. The MVP proves the mechanism; the business case should not imply the beachhead population values it.
2. **"Retained content is not retained" is a problem for media consumers, not for npm users.** The User Problem Validation section lists five problems; three of them, evaporating access, non-transferable licenses, and failed DRM, are media and software-licensing problems the MVP does not address. Listing them as validated problems for a JavaScript dependency product overstates the fit.
3. **The eligibility argument is right for the reason it does not give.** The document says a free archive makes the post-claim pricing paradox inert because an arriving maintainer was denied no revenue. That holds for every public npm package, including restrictively licensed ones, because npm charged nobody for the download; the license governs use, which travels with the plaintext under either channel. What the argument leaves out is that redistribution and irrevocability are separate from revenue, and it is those, not pricing, that a rights holder of a freely distributed package could object to.
4. **The subsidy is described as scaffolding and also as the onboarding path.** Both are true, and the document does not resolve them: if the relayer is to be replaced, the business case should say by what, or say that free onboarding ends when it is.

# Obstacles

Things that will slow or block the plan whether or not the argument is right.

- **Six blocking selections precede node authoring**, and two of them, custody and the escrow-salt custodian, have no determinant in the sources; the technical approach records default assumptions, but they are assumptions.
- **The completion boundary is unusually strict**: twenty-seven scenarios through packaged applications on clean machines, no mocked adapter, no off-chain-only verifier. That is correct and it is expensive.
- **Platform breadth.** Signed installation, service lifecycle, credential stores, and firewall handling on every supported OS and architecture are each a body of work with no shared shortcut.
- **External cryptographic review** has a lead time and a cost the document does not estimate.
- **Legal drafting** of a source-available license with a conformance clause is on the release path and unstarted.
- **The relayer grant pool** must exist before the first free onboarding and its source is unnamed.

# Errors

Statements in the business case that are wrong against the sources or unverifiable from them.

| Location in business case | Claim | Finding | Severity |
| --- | --- | --- | --- |
| References | "the fourteen research requirements and fifteen preferences" | The requirements file lists R01 through R15, fifteen requirements. The whole-target evaluation covered R01 through R14 because R15 was added afterward as a composition rule, which is why "fourteen" appears in Current State; the References line is wrong. | Low |
| Market Opportunity, Timing | BLS12-381 precompiles on mainnet "since the Pectra upgrade" | The research notebook's evidence entry says EIP-2537 was included in the 2025 Pectra upgrade and, in the same row, that the page inspected does not name the upgrade that shipped it and deployment must be confirmed per launch L2. The business case states as settled what the evidence register marks as to be confirmed. | Medium |
| Resources and Costs | "on the order of one hundred fifty numbered operational requirements" | Counting the Application Requirements tables gives 115 operational requirements (SI 19, IC 8, PR 8, EC 8, FF 9, SW 7, LC 10, IW 7, CR 10, CD 8, PC 7, LI 1, RO 6, XA 7) plus 27 acceptance scenarios. The figure is only right if scenarios are counted as requirements; the sentence says operational requirements. | Low |
| User Problem Validation | "the 2016 unpublishing of a single eleven-line package broke builds across the ecosystem" | Not in any repository document. Accurate as a matter of public record but unsourced within the repository, and the document does not mark it as such. | Low |
| Competitive Analysis | Characterizations of Verdaccio, Artifactory, Nexus, Cloudsmith, pnpm, Yarn Berry, IPFS/Filecoin, Lit Protocol, Widevine, FairPlay, PlayReady | Not in any repository document. The structural claims are accurate in general terms, but the document is presented as derived from the named sources and these are not. | Low |
| References, Market Opportunity, Strategic Thesis | Bartz v. Anthropic as locating rights-holder exposure on the ingest path | The ruling is not in the repository; the specification's Ingest Source Eligibility section makes the same point without naming a case. More importantly, the holding concerned transformative use of lawfully acquired works, and the protocol's conduct is verbatim redistribution; the case supports where exposure lands, not that the protocol's use is protected. | Medium |
| Executive Summary and Differentiation | "digital first sale, the property right platforms structured licensing to avoid, restored as a protocol invariant" | First sale for digital copies has been rejected where transfer creates a reproduction. As a description of the protocol's guarantee the sentence is accurate; as an implied legal basis it is not. The sources use the first-sale framing as an analogy, and the business case should too. | Medium |

# Omissions

What the business case should contain and does not.

- **Cost, timeline, team, and funding ask.** No estimate of any of them, and no statement of why not beyond "no cost model exists." A business case can state ranges and assumptions; this one states none.
- **Stop criteria.** Nothing says what result would cause the project not to proceed past the harness, past the demonstrable milestone, or past acceptance. The success metrics document supplies a latency guardrail; the business case should name it as the first kill criterion.
- **The developer's costs of participation**: daemon, disk for two stores, upstream bandwidth, seeding.
- **CI and enterprise mechanics**: identities for runners, rate limits for fleets, the superseeder argument from the workplan.
- **The supply-chain surface the daemon creates**, and the Application Requirements' answer to it.
- **Regulatory questions for priced transferable entitlements and for the paymaster**, even as an explicit unknown.
- **The subsidy's replacement path.** What ends free onboarding, and when.
- **The precise statement of what ingest eligibility protects against.** Not denied revenue, for freely distributed content, but unauthorized public redistribution and the loss of the publisher's ability to withdraw; and the comparison to public npm mirrors that already carry the first of those.
- **Consumption privacy for the adopting population**, carried into Risks.
- **How the swarm reaches long-tail coverage.** The project seed host covers the core packages and their closure; everything else depends on someone having installed it first. The business case does not describe the cold-start dynamics.
- **Governance of the adapter registry.** The specification says adapters are resolved from a governance-controlled registry, immutable once bound. Who governs it is not stated anywhere, and for a decentralization thesis that is a material omission the business case inherits.

# Discrepancies

Where the business case disagrees with the sources or with the companion planning documents.

| Business case says | Source or companion says | Finding | Severity |
| --- | --- | --- | --- |
| Delivery Plan, Phase 2 builds protocol core and contracts before Phase 3 builds the daemon, package serving, and swarm transport | The workplan's proposed sequence builds swarm transport, seed host, and the ciphertext store before the registry contract, because it is provable with no chain and no cryptography; the technical approach adopts the workplan's order | The business case's phases are coarser and differently ordered. The technical approach notes the difference; the business case should be brought into line or should say its phases are groupings, not a sequence. | Medium |
| Strategic Thesis: ingest adapters are confined to archives whose content is already free to use, which makes the pricing paradox inert because the maintainer was denied no revenue | MVP Scope, Canonical Identity and As-Is Ingestion: MVP eligibility is npm availability, with no license or use-rights classification | The implementing test is a fair proxy for the principle, since public npm availability means the publisher chose free public distribution, and the revenue argument holds. What the business case does not state is the residual that survives it: unauthorized public redistribution, which some licenses forbid, and irrevocability. | Medium |
| Differentiation, For the ecosystem: the public entitlement ledger is a benefit | cryptography.md, Dependency Graph Privacy: individuals are not the party the property argues about; a solo participant's install history is a behavioral profile | The business case presents one side of a tension the specification states in full. | Medium |
| Current State: "two maintainers and an independent re-derivation agent" | The notebook's log also records a third agent whose three proposals were reviewed and excluded (log 26) | Minor incompleteness; the research involved more participants than stated. | Low |
| Market Opportunity: registry-outage survival "matters most to CI and enterprises, who adopt as a consequence of individual adoption" | Workplan problem statement: build platforms adopt next and for their own reasons as natural superseeders; CI and enterprise arrive after that | The workplan has a three-stage adoption sequence with a mechanism; the business case has two stages and no mechanism. | Medium |
| Executive Summary: "No requirement remains unmet except two preferences" | Requirements file: fifteen requirements; Q06 evaluated fourteen, and R15 governs how conclusions are written rather than what they conclude | Accurate in substance; the count in References is wrong. | Low |

# Areas for Improvement

1. **Rewrite the beachhead argument around what substitutes cannot do.** Lead the developer value proposition with long-tail outage survival and network accumulation, present cross-project reuse as parity with pnpm, and say plainly that entitlements are invisible at $0.00 and are being proven, not sold.
2. **Replace borrowed legal authority with the actual limit on exposure.** State that every package the MVP ingests was already distributed to the public at no charge by its publisher, so the swarm confers no access and denies no revenue; state the principle as free public distribution by the rights holder's choice with npm availability as its test; and name the two residuals, unauthorized public redistribution and irrevocability, with the observation that public npm mirrors already carry the first. Use first sale as the analogy the sources use, not as a doctrine.
3. **Carry the privacy tension into Risks and Differentiation**, and say what the MVP does about it for the individual developer, which today is nothing beyond reading state from the client's own node where possible.
4. **Add the missing business figures as ranges with stated assumptions**, or add a section that says what would have to be true to produce them and when.
5. **Add stop criteria** tied to the harness output, the latency guardrail, and the demonstrable milestone.
6. **Import the workplan's adoption sequence and superseeder mechanism.**
7. **Align the Delivery Plan with the workplan's proposed build order**, or relabel its phases as groupings.
8. **Add the developer's participation costs and the CI mechanics** to the value proposition and risks.
9. **Name the daemon as a supply-chain surface** and point at the Application Requirements' controls.
10. **State the regulatory unknown** for priced transferable entitlements and the paymaster.
11. **Mark every claim drawn from outside the repository** as such, consistently, or move them to a clearly labelled external-context section.
12. **Fix the three factual errors**: the requirement count in References, the Pectra claim, and the operational-requirement count.
13. **Rebalance the document** so the business argument carries its own weight before the protocol argument arrives.

# Feasibility

**Cryptographic feasibility: high.** The construction is closed at the research level with proof sketches, concrete loss terms, an independent re-derivation, and named assumptions. Every component is standard: a depth-one Boneh–Boyen KEM, ElGamal envelopes, generalized Schnorr under Fiat–Shamir, EVM precompiles. No primitive of unknown existence or efficiency is required. What is unmeasured is cost, and the harness is designed to measure it before anything depends on it.

**Engineering feasibility: moderate, with high delivery risk from breadth.** Nothing in the module catalogue is novel engineering, but the surface is wide: 115 operational requirements each carrying a boundary-crossing proof, four control surfaces, multi-platform signed installation and service lifecycle, a contract suite with on-chain pairing verification, three services, and a demonstration harness. No code exists and no team is named. The workplan's sequence produces a demonstrable result at its sixth step, which is the right internal checkpoint, and the completion boundary is strict enough that the last third of the work, publishing, claims, and the transaction proof, is where schedules slip.

**Economic feasibility: undetermined by design.** The MVP is $0.00 and does not test willingness to pay. Cost per install becomes measurable; revenue per install does not. The relayer subsidy grows with adoption and is unfunded. The business case is right not to invent these figures and wrong not to say what it would take to obtain them.

**Legal feasibility: adequate for the MVP corpus, with a stated residual.** Every package the MVP ingests was distributed to the public at no charge by its publisher, so the swarm confers no access a user lacked and denies no revenue, whatever the license. The residual is that First Finders and seeders redistribute publicly without a grant from the publisher, which some restrictive licenses forbid, and that the swarm removes the publisher's ability to withdraw. Public npm mirrors carry the first of those today and it is largely uncontested; the second is new and is the protocol's purpose. Priced transferable entitlements and the paymaster carry regulatory questions the sources do not address.

**Adoption feasibility: the least supported.** The developer benefit at the beachhead is partly parity with existing tools, the participation costs are unstated, the privacy exposure falls on the adopting population, and the cold-start dynamics for the long tail are undescribed. The workplan's superseeder argument is the strongest adoption mechanism available and the business case does not use it.

# Recommendations

1. **Proceed with the harness regardless.** Nothing in this critique bears on the decision to build the validation harness first; it is the cheapest step, it resolves the most parameters, and it is publishable on its own.
2. **Revise the business case before external use**, applying the Areas for Improvement in order of severity: the legal framings and the eligibility residual, the privacy tension, the beachhead argument, the missing figures and stop criteria, then the factual corrections.
3. **State the eligibility principle precisely and record the residual.** Restate it in MVP Scope and the business case as free public distribution by the rights holder's choice, with public npm availability as the implementing test, and record that the accepted residual is unauthorized public redistribution and irrevocability rather than denied revenue. Whether to additionally refuse First Finder ingest for packages whose declared license forbids redistribution is a policy choice the workplan already holds behind a deliberate line; it is a narrowing of an already small exposure, not a repair of a gap, and should be decided on that footing.
4. **Adopt the workplan's adoption sequence and build order** in the business case so that all four planning documents agree.
5. **Add an internal demonstrable milestone** at the point an ordinary `npm install` resolves against the swarm, with its own measurement of the north star and the latency guardrail, without reducing the release boundary.
6. **Commission the legal review the document already implies**, scoped to the license text, the eligibility rule, and the regulatory treatment of priced entitlements and the paymaster, in parallel with the harness rather than after it.
7. **State a first privacy mitigation for the adopting population**, even if it is only reading state from the client's own node and documenting the exposure at onboarding.

# Notes

- This critique applies the repository's Reviewer posture: prior reasoning treated as untrusted, findings grouped by file and severity, and no file edited. The business case itself is left unchanged; the corrections above are proposed, not applied.
- The critique does not recommend validating demand with rights holders, consistent with the project's stated strategy that rights-holder reaction is a threat model rather than a gate. Validating the value proposition with the adopting population, individual developers, is a different matter and is recommended.
- Several findings, the eligibility residual, the privacy tension, the missing adapter-registry governance, and the regulatory questions, are inherited from the source documents rather than introduced by the business case.
- An earlier draft of this critique rated the eligibility finding High on the premise that restrictively licensed packages on npm were outside the "free to use" principle. That premise was wrong: npm distributes them to the public at no charge, so the revenue argument holds for them too. The finding was re-rated Medium and restated as a residual about redistribution and irrevocability. They are recorded here because the business case is the document that will be read by someone deciding whether to proceed, and it should not be more confident than its sources.
- The companion documents were checked for consistency with the business case. The feature spec and success metrics agree with it. The technical approach disagrees on build order and says so; that disagreement is recorded above as a discrepancy in the business case, not in the technical approach, because the technical approach follows the workplan.

# Additional Content

**Findings by file, for the workplan.**

| File | Finding | Severity | Proposed change |
| --- | --- | --- | --- |
| business-case.md | Eligibility principle stated as "free to use" with a revenue rationale; the surviving residual, redistribution and irrevocability, unstated | Medium | Restate the principle as free public distribution by the rights holder, name npm availability as its test, and name the residual |
| business-case.md | Fair-use and first-sale framings imply legal protection they do not supply | Medium | Restate as analogy; name licensing as the actual protection |
| business-case.md | Privacy tension presented one-sidedly for the adopting population | Medium | Carry into Risks and Differentiation |
| business-case.md | Delivery Plan order differs from the workplan and technical approach | Medium | Align or relabel as groupings |
| business-case.md | Pectra claim stated as settled where the evidence register says confirm per network | Medium | Qualify |
| business-case.md | Adoption sequence omits the workplan's build-platform superseeder stage | Medium | Import it |
| business-case.md | No cost, timeline, team, funding, or stop criteria | Medium | Add ranges with assumptions, or a section stating when they can be produced |
| business-case.md | Requirement count wrong in References; operational-requirement count imprecise | Low | Correct to fifteen; state 115 plus 27 scenarios |
| business-case.md | External-knowledge claims not consistently marked | Low | Mark or segregate |
| MVP Scope.md | Eligibility phrased as "free to use" where the implementing test is public availability; the residual it leaves is not stated | Medium | Restate the principle and record the residual; whether to also refuse licenses that forbid redistribution is a policy choice held in the workplan |
| cryptography.md | Adapter registry governance unstated | Medium | Outside this critique's scope; recorded for the workplan |
