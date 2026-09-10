# The MVP Execution Trace

When a package manager requests `somePackage` from the host adapter:

```mermaid
flowchart TD
    START([Package request]) --> LOCAL_CACHE{LOCAL_CACHE<br/>Plaintext in CAS?}
    LOCAL_CACHE -- Hit --> SERVE_RETAINED([SERVE_RETAINED<br/>Serve retained plaintext])
    LOCAL_CACHE -- Miss --> LEDGER_LOOKUP{LEDGER_LOOKUP<br/>Canonical asset present?}

    LEDGER_LOOKUP -- Present --> LOAD_DEPLOYMENT[LOAD_DEPLOYMENT<br/>Load authenticated descriptor and hash-card]
    LEDGER_LOOKUP -- Absent --> FF_FETCH[FF_FETCH<br/>Fetch exact NPM tgz]
    FF_FETCH --> FF_VERIFY{FF_VERIFY<br/>Upstream integrity valid?}
    FF_VERIFY -- No --> HALT([HALT<br/>Hard failure])
    FF_VERIFY -- Yes --> UPSTREAM_READY[UPSTREAM_READY<br/>Verified NPM plaintext available]
    UPSTREAM_READY --> CAS_COMMIT_UPSTREAM[CAS_COMMIT_UPSTREAM<br/>Persist upstream plaintext in global CAS]
    CAS_COMMIT_UPSTREAM --> SERVE_UPSTREAM([SERVE_UPSTREAM<br/>Serve initiating install])
    UPSTREAM_READY -. Start asynchronous bootstrap .-> FF_BUILD[FF_BUILD<br/>Allocate deployment identity, generate SCK and IV,<br/>encrypt, commit, and build hash-card]
    FF_BUILD --> REGISTRATION_RACE{REGISTRATION_RACE<br/>State-locked registration result}
    REGISTRATION_RACE -- Lost --> RACE_LOSS_DESTROY[RACE_LOSS_DESTROY<br/>Destroy local ciphertext and decrypt-capable state]
    RACE_LOSS_DESTROY --> BOOTSTRAP_DONE([BOOTSTRAP_DONE])
    REGISTRATION_RACE -- Won --> THRESHOLD_ESCROW[THRESHOLD_ESCROW<br/>Distribute SCK shares]
    THRESHOLD_ESCROW --> DURABLE_CUSTODY[DURABLE_CUSTODY<br/>Threshold custody durably acknowledged]
    DURABLE_CUSTODY --> FF_DESTROY[FF_DESTROY<br/>First Finder destroys local SCK and derivatives]
    FF_DESTROY --> FF_SEED[FF_SEED<br/>Hand ciphertext to seed host]
    FF_SEED --> BOOTSTRAP_DONE

    LOAD_DEPLOYMENT --> MANIFEST_GATE{MANIFEST_GATE<br/>Suite compatible and manifest valid?}
    MANIFEST_GATE -- No --> HALT
    MANIFEST_GATE -- Yes --> CIPHERTEXT_ACQUIRE[CIPHERTEXT_ACQUIRE<br/>Use local ciphertext or fetch from peers]
    CIPHERTEXT_ACQUIRE --> CIPHERTEXT_VERIFY{CIPHERTEXT_VERIFY<br/>Bao paths match ciphertext root?}
    CIPHERTEXT_VERIFY -- No --> HALT
    CIPHERTEXT_VERIFY -- Yes --> SEED_CIPHERTEXT[SEED_CIPHERTEXT<br/>Persist ciphertext in seed host]
    SEED_CIPHERTEXT --> ENTITLEMENT_CHECK{ENTITLEMENT_CHECK<br/>Asset-bound entitlement held?}

    ENTITLEMENT_CHECK -- No --> ENTITLEMENT_ACQUIRE[ENTITLEMENT_ACQUIRE<br/>Mint or acquire under issuance policy]
    ENTITLEMENT_CHECK -- Yes --> AUTH_CONTEXT
    ENTITLEMENT_ACQUIRE --> AUTH_CONTEXT[AUTH_CONTEXT<br/>Construct complete AuthorizationContext]
    AUTH_CONTEXT --> REFERENCE_AGREEMENT{REFERENCE_AGREEMENT<br/>Participants agree on one qualifying reference?}
    REFERENCE_AGREEMENT -- No --> HALT
    REFERENCE_AGREEMENT -- Yes --> AUTH_EVALUATE{AUTH_EVALUATE<br/>Evaluate current entitlement state}
    AUTH_EVALUATE -- PENDING_SETTLEMENT --> SETTLEMENT_WAIT[SETTLEMENT_WAIT<br/>Wait, then retry same agreed reference]
    SETTLEMENT_WAIT --> AUTH_EVALUATE
    AUTH_EVALUATE -- DENIED --> HALT
    AUTH_EVALUATE -- AUTHORIZED --> PROVISION[PROVISION<br/>Create recipient-wrapped response envelope]
    PROVISION --> RESPONSE_VERIFY{RESPONSE_VERIFY<br/>Envelope and bindings valid?}
    RESPONSE_VERIFY -- No --> HALT
    RESPONSE_VERIFY -- Yes --> WINDOW_ACTIVE[WINDOW_ACTIVE<br/>Authorized decryption window]

    WINDOW_ACTIVE --> DECRYPT_PIECE[DECRYPT_PIECE<br/>Decrypt and verify next piece]
    DECRYPT_PIECE -- More protected bytes --> WINDOW_BOUND{WINDOW_BOUND<br/>Ceiling or volume cap reached?}
    WINDOW_BOUND -- No --> WINDOW_ACTIVE
    WINDOW_BOUND -- Yes --> EXPIRY_DESTROY[EXPIRY_DESTROY<br/>Destroy envelope, material, and derivatives]
    EXPIRY_DESTROY -- More decryption required --> AUTH_CONTEXT
    EXPIRY_DESTROY -- No protected work remains --> DONE([DONE])
    DECRYPT_PIECE -- Asset complete --> PLAINTEXT_VERIFY{PLAINTEXT_VERIFY<br/>Plaintext root valid?}
    PLAINTEXT_VERIFY -- No --> HALT
    PLAINTEXT_VERIFY -- Yes --> CAS_COMMIT[CAS_COMMIT<br/>Persist plaintext in global CAS]
    CAS_COMMIT --> SERVE_DECRYPTED([SERVE_DECRYPTED<br/>Serve newly decrypted plaintext])
    SERVE_DECRYPTED -. Window reaches a bound later .-> EXPIRY_DESTROY

    DURABLE_CUSTODY -. Maintainer claim presented later .-> CLAIM_VERIFY{CLAIM_VERIFY<br/>Claim valid and settled?}
    CLAIM_VERIFY -- No --> DURABLE_CUSTODY
    CLAIM_VERIFY -- Yes --> AUTHORITY_TRANSFER[AUTHORITY_TRANSFER<br/>Transfer administration, issuance,<br/>custodial authority, and standing to supersede]
    AUTHORITY_TRANSFER --> CUSTODY_CONTINUES[CUSTODY_CONTINUES<br/>Threshold operational custody continues;<br/>no raw SCK delivery]
```

The graph's state identifiers define the execution trace; the descriptions below specify what each state does:

* **`LOCAL_CACHE` / `SERVE_RETAINED`.** If the plaintext CAS contains the asset, serve it without network access or authorization. Retained plaintext is outside the authorization boundary.
* **`LEDGER_LOOKUP` / `LOAD_DEPLOYMENT`.** Compute `BLAKE3(packageName@version)` and resolve the canonical asset record. An existing record supplies the authenticated deployment descriptor, hash-card, and transport locator set.
* **`FF_FETCH` / `FF_VERIFY` / `UPSTREAM_READY`.** If the asset is absent, require only that the supported NPM adapter can retrieve it. Fetch the exact `.tgz` and verify its NPM integrity attestation as provenance rather than a safety attestation. Successful verification makes the upstream plaintext available simultaneously to the foreground install and the asynchronous First Finder bootstrap.
* **`CAS_COMMIT_UPSTREAM` / `SERVE_UPSTREAM`.** Persist the verified NPM plaintext in the global CAS and serve the initiating installation. This foreground path waits only for the NPM response and its integrity verification; it does not wait for encryption, registration, threshold custody, seeding, entitlement acquisition, authorization, provisioning, or decryption.
* **`FF_BUILD`.** Asynchronous to the initiating install, obtain a registry-assigned deployment identity under state lock, generate a random content key and IV, encrypt with `AesCtrAdapter`, and construct the commitments and authenticated hash-card.
* **`REGISTRATION_RACE` / `RACE_LOSS_DESTROY` / `BOOTSTRAP_DONE`.** The first state-locked escrow registration wins. A loser destroys its local ciphertext, content key, expanded cipher state, buffered keystream, and other decrypt-capable derivatives, then ends its bootstrap attempt. Neither result affects the plaintext already supplied to the initiating install.
* **`THRESHOLD_ESCROW` / `DURABLE_CUSTODY` / `FF_DESTROY` / `FF_SEED`.** The winner distributes content-key shares into the real threshold suite and waits for durable acknowledgment. It then destroys every local content-key copy and derivative, hands ciphertext to `ISeedHostAdapter`, and reaches `BOOTSTRAP_DONE`. The First Finder never obtains publisher authority, and none of these states block the initiating install.
* **`MANIFEST_GATE`.** Authenticate the canonical record and hash-card, resolve every suite component, and validate piece geometry, addressable extent, and index bounds. Any failure reaches `HALT` before acquisition or authorization.
* **`CIPHERTEXT_ACQUIRE` / `CIPHERTEXT_VERIFY` / `SEED_CIPHERTEXT`.** Use locally produced ciphertext or fetch it through the active transport and discovery adapters, verify Bao authentication paths against the ciphertext root, and persist the verified object in the seed host.
* **`ENTITLEMENT_CHECK` / `ENTITLEMENT_ACQUIRE`.** Check for an asset-bound entitlement and mint or acquire one only when absent. First Finder/NPM assets mint at $0.00; the explicit-publisher dogfood deployment follows its declared paid settlement policy.
* **`AUTH_CONTEXT` / `REFERENCE_AGREEMENT`.** Construct the complete `AuthorizationContext`. Threshold participants must authenticate and agree on one exact settlement reference satisfying the deployment's tier; disagreement reaches `HALT`.
* **`AUTH_EVALUATE` / `SETTLEMENT_WAIT`.** Call `evaluateAuthorization` or its semantics-preserving paginated batch form. `PENDING_SETTLEMENT` waits and cycles back against the same agreed reference, `DENIED` reaches `HALT`, and only `AUTHORIZED` reaches provisioning.
* **`PROVISION` / `RESPONSE_VERIFY`.** Produce a fresh response envelope wrapped to the requester's key-agreement public key. Verify the context commitment, recipient, suite, keyset, decryption-material type, authentication, and both window bounds before entering `WINDOW_ACTIVE`.
* **`WINDOW_ACTIVE` / `DECRYPT_PIECE` / `WINDOW_BOUND`.** Decrypt pieces at their continuous-stream offsets and verify them. More protected bytes cycle through the active window while neither bound has been reached. Reaching the settlement-reference ceiling or volume cap moves to destruction; completing the asset moves to plaintext verification.
* **`PLAINTEXT_VERIFY` / `CAS_COMMIT` / `SERVE_DECRYPTED`.** Verify the plaintext root, store the plaintext in the global CAS, and serve it. Ciphertext remains separately in the seed host; the live window still proceeds to `EXPIRY_DESTROY` when a bound is reached.
* **`EXPIRY_DESTROY`.** At the first window bound, destroy the response envelope, decryption material, expanded cipher state, verification derivatives, buffered keystream, and every live context. Further protected decryption cycles back to `AUTH_CONTEXT`; otherwise the trace reaches `DONE`.
* **`CLAIM_VERIFY` / `AUTHORITY_TRANSFER` / `CUSTODY_CONTINUES`.** A later valid, settled maintainer claim transfers custodial authority, administration, issuance control, and standing to supersede. Operational threshold custody continues uninterrupted under the claimant's authority; no raw content key is delivered, and the First Finder has no remaining copy to transfer.
