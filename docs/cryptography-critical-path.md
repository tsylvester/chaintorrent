# Entitlement cryptography critical path

Current as of ledger log 36 (2026-09-22). If the ledger's latest log row is higher than this number, this file is stale and must be re-synced before work resumes.

## Role and rules

This file is the current best statement of the construction and what remains. It is rewritten in place whenever understanding improves; it carries no dates in its body. The ledger, [cryptography-research-notebook.md](cryptography-research-notebook.md), is append-only and holds provenance: settled reasoning (S), exclusions (X), evidence (E), the candidate register (C) as derivation history, the dated research increments, and the decision log. Every claim here cites a ledger ID or section. Where the two disagree, this file is authoritative for current belief and the ledger for how that belief was reached. Neither is an adopted specification; the ledger's authority statement still governs, and adoption into cryptography.md and MVP Scope.md is separate work.

Status vocabulary: **closed** means the construction supplies the step and the ledger records the derivation; **accepted residual** means the step is met only under a user-accepted limit, cited; **open** means a stated obligation remains, keyed to its owner.

## Chain walk: the nine steps against the chosen construction

The construction is the depth-one BB1 KEM in type-3 pairing groups with a per-suite identity scope (user decisions, logs 28 and 36): explicit-publisher suites use the entitlement scope, C11a, and escrow suites use the asset scope, C11c. A native key is issued at mint and rerandomized by the seller at each sale; capsules are common and KEM-form; delivery at mint, grant, and sale is an ElGamal envelope with a chained source-group proof verified by the settlement contract through the chain's pairing precompiles. Under the escrow suite any current holder authors a grant's credential, so no asset depends on its First Finder, which holds the master scalar only for an optional handover at claim, and a claim never depends on it.

### Step 1. Observable result — accepted residual

An entitled holder decrypts any piece to its original bytes using material assigned to its ownership interval. Supplied: each sale produces a tuple with a fresh total exponent, and the buyer may rerandomize again privately, so no two intervals share material (C11, S19). Plaintext is byte-identical (R03). The window is the ownership interval; no subwindow keys exist (Q05 section). Residual: a modified prior bearer's retained tuple still decrypts; loss at transfer is readings (a) and (a′) of interpretation 7, not (b) (P15, S01, log 24).

### Step 2. Decryption interface — closed

`Decrypt(C, pieceIndex, D_window, H_optional)` is instantiated with `C` the common encrypted payload, `H_optional` the common authenticated capsule sidecar, a universal public helper under P02 and P12, and `D_window` the holder's secret tuple. No per-holder helper exists; the only holder-specific object is the constant-size envelope in the settlement record (C11; Q09 third increment).

### Step 3. Payload compatibility — accepted residual

Every valid tuple for the entitlement's identity recovers the same encapsulated value `K = e(A, U) / e(V · W^I, B)`; the entitlement-specific blinding cancels (C11; Q09 third increment). No deployment-wide shared key exists. Residual: the per-piece `K` values are common derived material, payload-proportional at one per capsule, accepted under reading (B) of R04 (S13, log 14–15).

### Step 4. Authorization binding — closed

The settlement record stores, per interval, the holder, its two envelope keys and the digest of its envelope; the envelope's plaintext is unique under those keys, so the digest binds the delivered tuple without a separate commitment. The proof's challenge hashes the full context: chain, contract, asset, parameter-set digest, NFT, both interval counters, purpose, both parties, all four keys, both envelopes and the expiry (Q09 fourth increment, context schema). A proof is valid for exactly one settlement.

### Step 5. Private input — closed

Only the buyer can open its envelope: ElGamal in each source group under keys `pk1 = g1^x`, `pk2 = g2^y` with independent secrets, CPA-secure under SXDH; observers see ciphertexts and zero-knowledge proofs. No holder ever learns its own total exponent, so no coalition of holders can strip a key or recover the master scalar without solving co-CDH (C11; Q09 third increment). The protocol exposes no decryption oracle, so CPA plus proof of knowledge of the coins is the target of the security statement, not CCA (Q09 fourth increment, claim (ii) and "not claimed").

### Step 6. Resolution without threshold release — closed

An authorized secret participant creates the material at an allowed lifecycle event and nobody acts at read time: the issuer at mint under the master scalar (S17, R08), the seller at a voluntary sale from its own tuple (P13, log 15). Both are non-deniable because the escrow-ordered settlement needs no live action from anyone once payment is locked and the envelope is posted (Q09 fourth increment, T1–T4; S20). Reads are local (P11). No public-update mechanism and no read-path secret service exist (R07).

### Step 7. Future-state coverage — closed

The ciphertext is encrypted to a fixed identity, so any tuple issued later under the master scalar, or rerandomized from any existing tuple, decrypts it; entitlements are unbounded and never pre-counted (R10, X19). Later owners are covered by rerandomization. Escrow: the First Finder authors the first grant (P14, S0–S1) and every later grant is authored by any current holder under the asset scope (F2, log 36), so pre-claim liveness needs one holder online, the same condition as one seeder; claim registers the claimant's own parameter set and never needs the First Finder (S21, S2); handover is an optional shortcut (S3); later bodies carry one sidecar per live parameter set, with voluntary migration and retirement (S4). The registry accepts a deployment only if it covers every live set, which is the R11 rule (Q10 section).

### Step 8. Operational lifecycle — closed at the research level

An attempt is one piece-key derivation from one capsule. Before it, the conforming client holds a SOFT-tier state view no older than `τ_soft` showing the wallet as owner at the credential's interval, a wallet-control assertion renewed every `τ_wallet`, and the authenticated payload descriptor. On a transfer out it stops at SOFT and destroys the credential only at HARD, so a reorganized transfer never strands an owner; on a transfer in it uses the credential once settlement is at SOFT and the tuple passes the local validity check. No attempts offline, which matches the user's stated always-online client. The persistent credential is `(x, y, E_o, interval index)`, constant size; the reading tuple is decrypted from `E_o` at start-up and may be rerandomized in memory, and a sale always starts from the fresh decryption of `E_o`. Losing `(x, y)` ends the ability to transfer (P13). `τ_soft` and `τ_wallet` are implementation parameters against the chain adapter's tiers (Q05 section). Adoption into the specifications is pending and separate.

### Step 9. Established components and composition proof — open

Components, all reviewed: the depth-one BB1 scheme (E46, symmetric; its selective-identity theorem is more than the KEM needs); ElGamal under SXDH; generalized Schnorr with Fiat–Shamir (E44), simulation-sound and weakly simulation-extractable by E48's Theorems 2 and 3 since responses are perfectly unique and first messages uniform; the EVM precompile interface (E47: boolean output only, second-group subgroup checks enforced by the BN254 pairing precompile and by every BLS12-381 MSM and pairing input); standard KDF, AEAD and hash-to-scalar. Proof sketches of delivery soundness, ledger confidentiality and non-malleability exist with their extraction and hybrid bookkeeping and concrete loss terms (Q09 fifth and seventh increments). Non-holder content confidentiality holds under decisional BDH-3b, the named Type-3 assumption of E49, through the exact intermediate statement A1 and the embedding in the Q09 sixth increment; DBDH-3c does not suffice by that route, and no SXDH-only reduction is expected. All relations, the proof system and the reductions were independently re-derived and confirmed by a second agent (Q09 eighth increment). Open: every measurement.

## What the walk shows

- **Closed:** steps 2, 4, 5, 6, 7, and 8 at the research level; step 9 except measurement.
- **Accepted residuals:** step 1 under P15; step 3 under S13.
- **Open:** measurement only.

No step needs a primitive of unknown existence or efficiency. The Q06 first pass finds every requirement met at the protocol-composition level under the recorded residuals (Q06 section). What separates this from a buildable, adoptable design is proofs, source inspection, re-derivation, implementation and measurement.

## The construction, stated once

All algebra is the ledger's C11 section and Q09 third and fourth increments; nothing here is new.

- **Groups, parameters and assumptions.** Type-3 pairing groups `G1, G2, GT` of prime order with `e: G1 × G2 → GT`; the curve is BN254 or BLS12-381 as the launch chain's precompiles dictate (E47); Ethereum is the launch ecosystem and the network is held. Assumptions: SXDH for the envelopes, decisional BDH-3b (E49) for the capsule KEM, the random-oracle model for the proofs. The KEM's identity scope is fixed per suite: entitlement scope (`F_I = u0 · u1^I`, three-element capsule) for explicit-publisher suites, asset scope (fixed `F`, two-element capsule) for escrow suites. Public parameters `g1, u0, u1 ∈ G1` and `g2, hpub = g2^alpha ∈ G2`; the master scalar `alpha` is the issuer's. `F_I = u0 · u1^I ∈ G1` for the authenticated, domain-separated scalar mapping `I` of the exact entitlement identifier. The discrete logs among `g1, u0, u1` and between `g1` and any published element are never known.
- **Native key.** `(A, B) = (g1^alpha · F_I^r, g2^r)`, `r` disclosed to nobody. Validity `e(A, g2) = e(g1, hpub) · e(F_I, B)` with encoding and subgroup checks. Sale: `(A · F_I^s, B · g2^s)` with private `s`; the buyer may rerandomize again. Master-secret hiding and non-convertibility across identities are co-CDH.
- **Capsule, KEM form.** `(U, V, W) = (g2^t, u0^t, u1^t)`; encapsulated value `K = e(g1, hpub)^t`; decryption `K = e(A, U) / e(V · W^I, B)`, two pairings and one `G1` scalar multiplication. Well-formedness `e(V, g2) = e(u0, U)`, `e(W, g2) = e(u1, U)`, cacheable. Three source-group elements per capsule, about 128 bytes on BN254 and 192 on BLS12-381.
- **Payload.** Piece key = domain-separated KDF of `K` and the context (asset, deployment, suite, piece index, geometry); each piece encrypted once. Everyone receives the same encrypted pieces and the same sidecar; a body carries one sidecar per live parameter set.
- **Buyer keys.** `pk1 = g1^x ∈ G1`, `pk2 = g2^y ∈ G2`, independent nonzero secrets, each with a Schnorr proof of possession, the pair wallet-signed; the contract rejects identity-element keys; reusable across entitlements; persistent for the interval. Mint rejects an identity-element `F_I`.
- **Envelope.** `E = (g1^rho, A · pk1^rho, g2^sigma, B · pk2^sigma)`, two `G1` and two `G2` elements; the buyer recovers the tuple and checks validity locally with three pairings.
- **Mint proof.** Knowledge of `(alpha, r, rho, sigma)` with `hpub = g2^alpha`, `C1 = g1^rho`, `C2 = g1^alpha · F_I^r · pk1^rho`, `D1 = g2^sigma`, `D2 = g2^r · pk2^sigma`.
- **Transfer proof.** Knowledge of `(x_s, y_s, s, rho, sigma)` with `pk1_s = g1^x_s`, `C1_n = g1^rho`, `C2_n / C2_o = C1_o^(-x_s) · F_I^s · pk1_b^rho`, `pk2_s = g2^y_s`, `D1_n = g2^sigma`, `D2_n / D2_o = D1_o^(-y_s) · g2^s · pk2_b^sigma`. Soundness runs by induction from the mint proof along the entitlement's envelope history; only the total offset enters, so private rerandomization is preserved.
- **Proof form and verification.** Generalized Schnorr, Fiat–Shamir over the full context. Six scalars, about 192 bytes. With `G2` arithmetic at the verifier: sixteen source-group scalar multiplications in six multi-scalar multiplications, no pairing. With `G1` arithmetic only: the three `G2` first messages travel too, and the `G2` equations are checked either as three pairing products totalling twelve pairings, or as one hash-weighted product of ten pairs; the weighted form is the only single-call form, since no `G2` point repeats within an equation. A mint proof is ten pairings unweighted, eight weighted.
- **Settlement.** T1 register buyer keys, with proofs of possession and rejection of identity elements; T2 lock payment against the exact NFT, seller, price and expiry; T3 the seller posts envelope and proof, the contract verifies and atomically advances the counter, records the buyer and digest, transfers the NFT and releases payment; T4 refund on expiry. Mint is the same with the issuer as seller and registry-set price, zero pre-claim. Deterministic at T3; the only residual is the reorg race (X20).
- **Custody.** Parameter sets are the unit of issuance authority and are marked live per asset. S0 bootstrap under the First Finder's set; S1 escrow-live grants; S2 claim registers the claimant's own set; S3 optional handover; S4 every later body covers every live set, with voluntary migration of escrow-era holders and retirement of empty sets. Rotation is the publisher's recovery from a lost master scalar.
- **Costs, by formula.** Per capsule three source-group elements; per interval on chain a four-element envelope and a six-scalar proof, plus three `G2` elements on BN254; per holder a constant credential. Below one percent of payload for a 16 KiB piece group, a few hundredths of a percent for 1 MiB. No byte, latency, proof or settlement cost has been measured.

## Open obligations

- **Q09.** Closed at the research level (fourth to eighth increments): relations, proof system, composition claims with bookkeeping and loss terms, assumption identified as decisional BDH-3b, sources reviewed, and an independent re-derivation reconciled with its corrections adopted.
- **Q08 remainder, measurement.** Encoding sizes, capsule and proof bytes, decryption latency, proof generation and verification cost on the chosen curve, and settlement cost against E47's interface.
- **Q06, second pass.** Re-run the whole-target table once the proofs and measurements exist.
- **Handoff, done (log 35).** The Q05 attempt rules, the Q10 state machine and sidecar-coverage rule, and the Q09 settlement protocol are adopted into cryptography.md, MVP Scope.md, MVP Application Requirements.md and the Execution Trace, with the adapters `IPairingAdapter`, `ICredentialKemAdapter`, `PairingElGamalAdapter` and `IDeliveryProofAdapter` named there. Still open in the workplan's To-Do: the escrow salt custodian, a design decision with no determinant in the research; and the attempt-rule parameters and piece-group size, which the validation harness measures.

## Accepted residuals

Met only under a user-accepted limit. Not open items; do not re-open without the cited premise changing.

- Loss at transfer is readings (a) and (a′), not (b): a modified prior bearer's retained tuple still decrypts (P15, S01, log 24).
- Common per-piece encapsulated values exist and are payload-proportional; accepted under reading (B) of R04 (S13, log 14–15).
- Pre-claim grants depend on some holder of the escrow set being online, and escrow-era credentials carry no entitlement-level attribution, because the escrow suite uses the asset scope (P14, S21, S22, log 36). Claim itself depends on nobody.
- The piece-group keys are common to every holder and exportable as a payload-proportional key file; accepted as not resolvable with known practical methods, with the decapsulation-to-cipher seam preserved for a future multi-key PRF suite (S13, log 36).
- The First Finder may retain a copy of the master scalar; plaintext-equivalent and P08-class, not a benefit of ownership (custody section, R12 analysis).
- The publisher at mint and the seller at a voluntary sale are accepted, non-deniable dependencies (R08, P13, S20).
- Forced transfer, lost keys and recovery are non-obligations; a holder that loses its envelope secrets keeps reading but cannot transfer (P13).
- A buyer that reads a reverted step-3 envelope holds one entitlement's tuple without a settled transfer; S01-class (Q09 second increment, X20).
- Entitlement-level attribution of leaked tuples is available; interval-level attribution is optional and defeated by rerandomization (S19).

## Do not re-propose

Check S01–S22 and X01–X23 in the ledger before proposing anything. Titles only; the ledger holds the reason and the reopening condition.

| ID | Excluded |
| --- | --- |
| X01 | Wrapped SCK via threshold release |
| X02 | Wrapped SCK via one server, HPKE, or a conventional hybrid PRE/ABE/KEM |
| X03 | Full-sized personalized ciphertext or XOR helper |
| X04 | Hash of public NFT, owner or state fields as a secret key |
| X05 | NFT mutation as automatic destruction of old mathematical capability |
| X06 | Public chain witness alone as the decryption witness |
| X07 | Plain WE false-statement security as sufficient |
| X08 | Fixed-recipient broadcast or SWE as a complete dynamic-NFT solution |
| X09 | Public registration automatically unlocking old ciphertexts |
| X10 | The AFGH proxy re-encryption instance |
| X11 | Linear public key delta |
| X12 | Threshold MPC, FHE or KMS operated by validators |
| X13 | Time release or consensus randomness alone |
| X14 | Hardware-enforced erasure or a TEE gate |
| X15 | Cooperative watermark as a decryption credential |
| X16 | Proof extractor as an operational traitor tracer |
| X17 | Abstract, interface or benchmark treated as a complete composition |
| X18 | Reviewed RBE, RABE or distributed-broadcast instances as retroactive access |
| X19 | Broadcast encryption with a setup-fixed slot count |
| X20 | Time-lock or VDF envelope for the Q09 reorg race |
| X21 | Puncturable or forward-secure KEM as an O(1) (b)-route |
| X22 | Offset-linkage proof as a correctness requirement (adopted only for precompile verifiability) |
| X23 | Source-group substitutes for the target-group first message of the direct validity proof |

Also do not restart with: the impossibility of revoking retained secrets (S01); traitor-tracing difficulty (P06, S06); a survey of the same broad encryption families (C01–C08); registration-based encryption as an access path (S11, X18); the pre-2026-09-16 premise that no issuer may hold a master secret (S17); a contract method as a secret holder (S03, S08); holder-assisted issuance under C11a (S22); or a shared secret across the two envelope keys (Q09 third increment, hazard).

## Next unit of work

Measurement on the chosen curve (Q08 remainder), which needs an implementation and therefore an engineering decision; then the Q06 second pass. Adoption of the Q05, Q09 and Q10 conclusions into cryptography.md and MVP Scope.md is separate work and awaits the user's decision. No research question remains open on the critical path.
