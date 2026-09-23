# Entitlement cryptography requirements

Current as of ledger log 36 (2026-09-22). Rewritten in place only when the user decides; each row cites the log entry or source that set its current reading. The ledger, [cryptography-research-notebook.md](cryptography-research-notebook.md), holds the dated history of every amendment. The construction and open work are in [cryptography-critical-path.md](cryptography-critical-path.md).

## Objective

Replace the adopted MVP's two divergences from the target: individually wrapped responses that carry a common shared content key become distinct native decryption material per entitlement window, and the threshold provisioning network on the read path becomes access resolved from settled entitlement state and wallet control with no such network (log 1; ledger "Current state described by the documents").

## Requirements

| ID | Requirement | Current reading | Set by |
| --- | --- | --- | --- |
| R01 | One common ciphertext payload per deployment | Every holder obtains the same payload. No personalized payload or replacement torrent per holder. | Log 1, reaffirmed log 7 |
| R02 | Seekable, incremental decryption | Random access and out-of-order piece delivery are preserved. | Cryptographic specification |
| R03 | Recover the original plaintext | Byte-identical plaintext is preferred. No fingerprint or watermark is required. | Log 4 |
| R04 | Distinct native decryption material per entitlement window | A transport envelope around a common key does not satisfy it. Reading (B), issued distinctness, is sufficient: the material a holder is issued or generates is per-holder even though every decrypter can derive compact common objects such as per-piece keys. Reading (C), custody, is excluded. Reading (A), attributability, has available prerequisites and is optional. The operational criterion (i)–(iii) is acceptable if it yields a method, not obligate. | Log 14, log 15 |
| R05 | Authenticate entitlement and holder at every attempt | Before every conforming decryption attempt: valid issuance lineage, exact NFT identity, current settled ownership, and control of the bound wallet identity. | Log 5 |
| R06 | Enforce expiry in the conforming protocol | Conforming clients stop decrypting and destroy window material on expiry or entitlement loss. Cryptographic expiry against a modified client is not mandatory. Loss at transfer is an escalation chain: conforming-protocol loss is the minimum, distinguishable stale material is the working target, cryptographic loss is the unavailable preference (P15). | Log 9, log 22, log 24 |
| R07 | No threshold decryption or key-provisioning network | Consensus may authenticate public state under its BFT assumptions. Secret-sharing a key among validators is still threshold provisioning. | Log 1; X12 |
| R08 | No publisher dependency during reads | Once a valid entitlement exists, its holder needs no publisher cooperation to decrypt. The issuer may be on the decryption or transfer pathway of an existing entitlement only without selective deniability: material pre-published at publication or mint, or delivered atomically with settlement and publicly verified. Publisher discretion is a right over new issuance and forbidden over existing entitlements. Unavailability is indistinguishable from refusal, so no single party's live action may be required at such a step. | Log 14, log 15, log 25 (S20) |
| R09 | Transfer and future recipients | Transfers support owners unknown when the ciphertext was created; the buyer needs its own material. The seller's participation in a voluntary sale is an accepted dependency. Publisher participation at a transfer is acceptable only if non-deniable; a replacement mutates a member of the minted set without growing it. | Log 14, log 15, log 22 |
| R10 | Dynamic entitlement issuance | The publisher may mint an arbitrary, unknown number of entitlements on demand and is never pre-bound to a count; a supply bound fixed at publication is excluded (X19). Issuance is discretionary: the protocol must not oblige it, and publisher-driven scarcity is obligate. | Log 15, log 22 |
| R11 | Asset-bound entitlement | A valid entitlement authorizes every live deployment of the asset. A new deployment cannot silently strand existing rights. | Source documents |
| R12 | First Finder and escrow | Support bootstrap before the publisher arrives, later claim, and transfer of administrative authority without granting publisher rights to the bootstrapper. Escrow and claim confer no special benefit on the First Finder; the claimant's outcome equals an original publisher's, the sole acceptable difference being that the claimant did not price the keys issued during escrow. | Source documents; log 22 |
| R13 | Nonselective availability | No particular publisher, helper operator or provisioner acquires a unilateral read-path veto. State the actual consensus and data-availability assumptions. | Source documents |
| R14 | Evaluate the complete lifecycle | Identify who creates every secret and helper at encryption, mint, access, transfer, recovery and escrow claim. Moving an unresolved issuer into another step is not a solution. A retained root that can generate every later window key must be identified explicitly; erasing a leaf is not total erasure. | Source documents; interpretation 3 |
| R15 | Modular composition | Every cryptographic component is specified as an adapter to an interface with declared capabilities, and the deployment suite is the unit of composition: the pairing group, the credential KEM, the envelope encryption, the delivery proof, the on-chain verifier, the payload cipher, the KDF, the settlement view and the entitlement-state access are each replaceable by any implementation meeting its interface. This governs how the research conclusions are written into the specifications; it does not change what they conclude. | cryptography.md Architectural Invariants; user restatement, log 34 |

R09–R13 are preserved from the source documents and earlier research scope. Do not drop them because a later message concentrates on expiry, tracing or helper size.

## Preferences, concessions and non-goals

| ID | Topic | Current disposition | Set by |
| --- | --- | --- | --- |
| P01 | No personalized helper | Preferred if a viable construction exists; not a reason to reject an otherwise conforming compact-helper construction. | Log 10 |
| P02 | Compact helper or variant object | Acceptable as a fallback when it is a small fraction of total object size. The NFT may define, authenticate or reference it; a separate torrent colocated in the swarm is acceptable. | Log 10 |
| P03 | Full-sized personalized helper | Excluded; it defeats the distribution value of the common ciphertext. | Log 7 |
| P04 | Cryptographic dependence on current NFT state | Preferred over a client-only state check; client enforcement is acceptable for the interim construction. | Log 5 |
| P05 | Modified-client revocation | Desired stronger property, not an acceptance condition. Retained complete classical decryption inputs are outside the accepted expiry guarantee. | Log 9 |
| P06 | Traitor tracing | Nice to have. Do not block the access construction on unstrippable attribution or arbitrary-decoder tracing. | Log 8 |
| P07 | Plaintext watermarking | Optional, only if acceptable for the content; neither preferred nor required. | Log 4 |
| P08 | Plaintext redistribution | The project does not attempt to disallow it. Do not describe this as encouragement or add plaintext-control machinery. | Log 6 |
| P09 | Trusted hardware | Proprietary TEEs and hardware DRM are rejected. | Source specification |
| P10 | Speed of research | Accuracy, completeness and precision take priority. Independent mathematical investigation is authorized; do not ask the user to supply a cryptographic solution. | Log 2 |
| P11 | Decryption locality | Preferentially an algorithmic, independent act of the entitlement possessor with no network or request-resolution latency; latency is accepted only where it cannot be avoided. | Log 14 |
| P12 | Helper availability | A compact object required for decryption may be unique to one holder, shared or universal, and is expected to propagate through the swarm so a holder who loses it can recover it. It must not be decryption-capable without the holder's private material. | Log 14 |
| P13 | Forced transfer, lost keys, recovery | Not protocol obligations. Key custody is the possessor's liability, as with a physical object. Enabling forced transfer is neutral to negative; re-issuance or recovery is neutral to positive. | Log 14 |
| P14 | Single-participant First Finder bootstrap | The initial deployment must work with the First Finder as the sole available credential source. Requiring an independent custodian before publication is excluded. Additional sources may improve availability later; this does not make a sole custodian fault tolerant or give it publisher rights. | Log 21 |
| P15 | Cryptographic loss of decryption ability at transfer | The preferred property, that a prior bearer's retained material stops working even in a modified client, is not available under R01, P09 and R07. Distinguishable stale material is the working target; conforming-protocol loss is the minimum. Record any construction that approaches it under the unchanged constraints; do not relax the constraints to reach it. | Log 24 |

## Interpretations still open

Numbering follows the ledger. Interpretations 1, 3 and 5 were settled at the research level in the ledger's Q05 section (log 29): the ownership interval is the window, the persistent credential is named exactly, and attempt semantics run on the chain adapter's SOFT and HARD tiers with no offline attempts. Interpretations 2, 6 and 7 were settled earlier. Adoption of the Q05 conclusions into the specifications is separate work; if the user overrides any of them, this section reopens the row.

| No. | Interpretation | What is undecided | Owner |
| --- | --- | --- | --- |
| 4 | Helper budget | "Small fraction" is now settled by formula (Q05 section): three source-group elements per capsule, a constant per interval on chain, a constant per holder. What remains is byte measurement on the chosen curve, which no decision can replace. | Q08 remainder |

