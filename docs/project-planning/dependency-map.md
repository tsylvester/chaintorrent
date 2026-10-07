<!-- Template: antithesis_dependency_map.md -->
# Dependency Map

## Overview

How the parts of the ChainTorrent MVP fit together and in what order they are built, drawn from the [revised business case](business-case-revised.md), the [technical approach](technical-approach.md), the [workplan](../workplans/current/ChainTorrent%20MVP.md)'s build sequence, and the [MVP Application Requirements](../research/MVP%20Application%20Requirements.md). Groupings are addressed by delivery role: **foundation**, **cryptographic validation harness**, **protocol core and contract suite**, **local daemon and package serving**, **onboarding shells and services**, and **acceptance and release**.

Resolution decays with distance, deliberately. The foundation and harness groupings are mapped at **ticket** resolution: one ticket per source file, which is the node template's unit, so that each ticket is a candidate node the workplan author can promote. Inside the protocol core, the hashing, signature, and swarm milestones are also at ticket resolution, because they depend on nothing the harness measures and run beside it; the remaining protocol core milestones are mapped as **sprints** named by dependency role. Beyond that the map holds **epics**, then **milestones**, then **objectives**. The reason is that implementation of the nearest work will produce discoveries that revise the anticipated order of everything after it, and detail authored now for distant groupings would be rewritten rather than used. When the harness closes, the remaining protocol core milestones are re-mapped to ticket resolution from what was learned, and the decay shifts outward.

Tickets are candidates, not nodes. No node is authored here; a ticket names the source file's role and its dependencies so a node can be written from it through the ordinary authoring path. Every adapter family is a crate holding its `factory` module, which owns the generic interface, the capability declaration, and the factory function, and its concretes as private modules beneath it, and the map names tickets accordingly: a family's factory ticket is `crate/factory`, a concrete's `crate/concrete`, a function a concrete owns `crate/concrete/function`, a function the family owns `crate/function`, a module of the domain or workflows crate `crate/module` with its functions beneath it, a contract `contracts/<suite>/Contract`, and a configuration-only ticket its directory; the [technical requirements](technical-requirements.md)' file tree spells every path, crate directories hyphenated and module directories underscored. A concrete adapter is one node, since its operations are the methods of one adapter in one file. Within a family the first ticket, its first concrete or a family-owned function that precedes it, authors the generic interface, the declaration, and the family's mock as its producers, since it is the first source file that requires them, and creates the crate; each further concrete and family-owned function follows; the factory ticket, which constructs the concretes and so consumes them, follows every concrete of the family in this workplan and is revised in place when a later milestone adds a concrete; and every consumer ticket depends on the factory and never on a concrete. The declarations the composition resolver reads therefore exist from the family's first ticket. A family's factory ticket closes a producer, implementation, consumer chain, so it carries the family's integration test across the factory and its concretes; the commit sits on the last ticket of the milestone's chain, which for a milestone that closes at a family is that family's factory.

## Components

Grouped by role. Within a grouping at ticket resolution each row is a ticket; elsewhere each row is a sprint, epic, milestone, or objective as the decay dictates.

### Foundation, at ticket resolution

The workspace every node builds on, the secret type that cannot be formatted, the randomness family every key-generating ticket draws from, and the continuous integration that proves each ticket where the completion boundary requires. The encoding family is not a foundation member, since nothing encodable exists here; it follows the first encodable domain type at the head of the harness grouping. No crate is created ahead of the module that first lives in it; each ticket below creates its crate when that crate does not yet exist. A configuration file is authored once, complete, by the ticket that creates it, and no later ticket amends it; a crate's manifest and its `lib.rs` barrel are the exceptions by the topics' own design, since each module's ticket adds its dependencies to the manifest as its deps element and its re-export line to the barrel as its provides element.

| Ticket | Owns | Depends on | Proves |
| --- | --- | --- | --- |
| `workspace/cargo` | The virtual workspace manifest with the glob members `crates/*`, `adapters/*`, and `apps/*`, the lint table, and the `[patch.crates-io]` overlay pointing `librqbit` at the project's overlay branch `chaintorrent-overlay` in `https://github.com/tsylvester/rqbit`, which exists at a tagged upstream release before the bootstrap as external setup; `rust-toolchain.toml`; `deny.toml` with the license allowlist and that repository in `allow-git`; `.gitignore`; configuration files with no types and no tests, each authored once and complete | nothing | The root configuration every later ticket builds within; its build proof arrives with the first member |
| `domain/secret` | The secret-typed value: no formatting or serialization trait, an explicit accessor for the cryptographic operations and custody wrapping that consume it, zeroization on drop through `zeroize`, a crate dependency; creates the `domain` crate | `workspace/cargo` | CR-07 exclusion of secrets from logs, proven at compile time; the lint table's rejection of `unsafe_code`, `unwrap_used`, `expect_used`, `panic`, and `as_conversions` in the first production crate |
| `random/os` | The concrete over the operating system's generator; authors, as its producers, the randomness family's interface that fills bytes, the declaration, and the family's mock; creates the `adapters/random` crate; pins `rand_core` and `getrandom` | `workspace/cargo` | CR-05 contract; repeated draws of a fixed width pairwise distinct across a fixed count, and a filled buffer of a fixed length holding more than one distinct byte value; CR-05's per-bootstrap independence is proven at the First Finder engine and its reproducible publisher derivation at the explicit publisher path |
| `random/factory` | The randomness family's factory function, constructing the concrete a configuration names with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete and the grouping's commit | `random/os` | CR-05: the one surface every production draw passes through |
| `workspace/ci` | The Rust continuous-integration definition `.github/workflows/rust.yml`, authored once and complete: `cargo fmt --check`, `cargo check`, `cargo clippy`, the unit and integration tests, `cargo-deny`, and `cargo-audit` on Windows, macOS, and Linux; a configuration file with no types and no tests; the Solidity, shell, fuzz, and end-to-end checks are separate workflow files, each created once by the ticket that first needs it | `workspace/cargo` | XA-07 facilities run in CI; NF-M07; `deny.toml` read and enforced on every supported platform |

### Cryptographic validation harness, at ticket resolution

The harness begins with the domain's identifier modules and the derivation context, the first encodable domain types, followed by the encoding family that describes and encodes them, and builds the pairing, key-derivation, hash-to-scalar, credential KEM, envelope, and delivery proof families, each from its first ticket, which carries the family's interface, to its factory, which follows every concrete, together with the chain family's EVM forms concrete and its family-owned forms function, the harness's typed configuration with its shipped defaults, the harness run that composes every family through its factory over the consumers it owns, the output that writes the run's outcome, a success or a refusal, to the destinations the configuration names, and the harness binary that reads the configuration, calls the run, and hands its outcome to the output, exercises the deployed verifier on Base Sepolia, and records the measurements against the configured piece-group size and curve defaults with any adjustment the data supports (CD-07, AS-21). It constructs every curve and verifier-form combination through the factories, so an invalid combination is refused in the harness exactly as the daemon's resolver will refuse it. Every ticket is one Rust source file with its full support system except where the row says otherwise; a family's first ticket owns the family's generic interface, declaration, and mock beside its own implementation, a further concrete ticket is one adapter whose operations are its methods, and the factory ticket owns the factory function and the crate's public surface.

| Ticket | Owns | Depends on | Proves |
| --- | --- | --- | --- |
| `pairing/bn254_arkworks` | The BN254 concrete on arkworks, precompile-matching encodings, declaring the curve, first-group-only arithmetic at the verifier, and its target-group encoding identifier, proven by executing the identifier's definition in its tests; authors, as its producers, `IPairingAdapter` with generators, add, mul, MSM, subgroup check, and the pairing-product check, the reference trait `IPairingReference`, beside the generic interface, with the scalar field's order and the precompile encoding of the defined point outside each source group's prime-order subgroup, the group-element and scalar associated types, the sampling bound on the scalar associated type, sample from uniform bytes, the declaration of the curve, of second-group arithmetic at the verifier, of precompile encodings, and of the target-group encoding identifier, and the family's mock; owns its scalar and group-element types, holding the library's elements as private fields behind fallible constructors that perform the subgroup and encoding checks, names them as the associated types, implements the sampling bound on its scalar type, and proves the in-range draw; creates the `adapters/pairing` crate; pins `ark-ec`, `ark-ff`, and `ark-bn254` | `workspace/cargo`, `random/factory` | CR-10 contract and CR-10 on BN254; the declaration the proof and chain factories read |
| `pairing/bn254_halo2curves` | The BN254 concrete on `halo2curves`, precompile-matching encodings, declaring the curve, first-group-only arithmetic at the verifier, and its target-group encoding identifier, proven by executing the identifier's definition in its tests; owns its scalar and group-element types over its library's elements behind fallible constructors, implements the sampling bound on its scalar type, and proves the in-range draw; pins `halo2curves` | `pairing/bn254_arkworks` | CR-10 on BN254 |
| `pairing/bls12_381_arkworks` | The BLS12-381 concrete on arkworks, precompile-matching encodings, subgroup checks on every input, declaring the curve, second-group arithmetic at the verifier, and its target-group encoding identifier, proven by executing the identifier's definition in its tests and against the CFRG draft's published value; owns its scalar and group-element types over its library's elements behind fallible constructors, implements the sampling bound on its scalar type, and proves the in-range draw; pins `ark-bls12-381` | `pairing/bn254_arkworks` | CR-10 on BLS12-381 |
| `pairing/bls12_381_halo2curves` | The BLS12-381 concrete on `halo2curves`, precompile-matching encodings, subgroup checks on every input, declaring the curve, second-group arithmetic at the verifier, and its target-group encoding identifier, proven by executing the identifier's definition in its tests and against the CFRG draft's published value; owns its scalar and group-element types over its library's elements behind fallible constructors, implements the sampling bound on its scalar type, and proves the in-range draw | `pairing/bn254_arkworks` | CR-10 on BLS12-381 |
| `pairing/factory` | The pairing family's factory function, admitting a curve the chain's declared precompile sets support and a concrete only when it declares the target-group encoding identifier the suite requires, constructing the default concrete for that curve, the arkworks concrete per the tech stack's assumption, unless the composition names one, with a branch per concrete, and the crate's public surface; the benchmark's record revises this ticket in place if it selects otherwise; carries the family's integration test across factory and concretes, which proves the concretes of a curve return the same scalar field order and the same outside-the-subgroup encodings | `pairing/bn254_arkworks`, `pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, `pairing/bls12_381_halo2curves` | The surface the hash-to-scalar, KEM, envelope, proof, and chain tickets consume |
| `harness-crypto/benchmark` | The measurement of scalar multiplication, multi-scalar multiplication, and pairing over every pairing concrete, each constructed through the pairing factory, recording the default per curve, which revises the factory ticket in place if it differs from the arkworks assumption; names no library; creates the `apps/harness-crypto` crate | `pairing/factory` | The recorded default per curve |
| `domain/asset_identity` | The canonical asset identity, the package name and version, with its fallible `try_new` constructor enforcing its invariants | `domain/secret` | The identity every record, hash-card, and derivation names |
| `domain/deployment_identity` | The registry-assigned deployment identity with its fallible `try_new` constructor enforcing its invariants | `domain/secret` | The deployment every derivation and object is bound to |
| `domain/suite_identifier` | The cryptographic suite identifier and version with its fallible `try_new` constructor enforcing its invariants | `domain/secret` | The suite every derivation is bound to |
| `domain/parameter_set_identifier` | The parameter-set identifier with its fallible `try_new` constructor enforcing its invariants | `domain/secret` | The set every capsule and wrapping key is bound to |
| `domain/group_index` | The piece-group index with its fallible `try_new` constructor enforcing its bounds | `domain/secret` | The group every piece-group key is bound to |
| `domain/piece_geometry` | The piece size, piece-group size, and addressable extent with their fallible `try_new` constructor enforcing alignment | `domain/secret` | The geometry every derivation and the cipher are bound to |
| `domain/asset_identity_hash` | The asset identity hash, the registry key the hash-card fixes as the BLAKE3 hash of the asset's coordinates, with its fallible `try_new` constructor refusing the all-zero value | `domain/secret` | The key every asset record, transcript, and asset-scope identity mapping names |
| `domain/derivation_context` | The derivation context composed of the asset identity, deployment identity, suite identifier, parameter-set identifier, group index, and geometry, with its fallible `try_new` constructor enforcing its invariants; the first encodable domain type | `domain/asset_identity`, `domain/deployment_identity`, `domain/suite_identifier`, `domain/parameter_set_identifier`, `domain/group_index`, `domain/piece_geometry` | The context every wrapping key and lineage derivation is domain-separated by |
| `encoding/derivation_context` | The family-owned description of the derivation context, implementing the encoding contract as its canonical field sequence, stated once and format-free; authors, as its producer, the repo-owned encoding contract in the encoding factory's interface, which names no format and no vendor; creates the `adapters/encoding` crate | `domain/derivation_context` | The contract, and the first description every encoding concrete encodes |
| `encoding/abi` | The Ethereum ABI concrete through `alloy`'s sol types, implementing both interfaces as one adapter; untrusted bytes to the owned type or its error; authors, as its producers, `IEncoderAdapter` and `IDecoderAdapter` with the versioned encoding identifier, the encoding types, the capability declaration, and the family's mock; its known-answer vectors are the derivation context's and its identifiers' encodings authored from the ABI specification, which the generator later mirrors, and its round trip and its rejections of truncated input, trailing bytes, non-canonical padding, and a wrong element count run over the same values through the description; adds to the encoding contract the byte-string kind, which carries a pairing group element's precompile encoding, encoded as ABI `bytes` against its own known-answer vector, and, for the proof family's transcript descriptions, the fixed twenty-byte and 256-bit unsigned kinds, each named as a width, mapped to its ABI type by this concrete, and encoded against its own known-answer vector with the decoder rejecting a word outside the width; pins `alloy` | `encoding/derivation_context` | ABI encoding of the derivation context against its vectors; round trip; malformed input rejected; the hash-card's encoding identifier; the byte string a statement over group elements is encoded as; the widths a transcript's parties and entitlements are encoded as |
| `encoding/factory` | The encoding family's factory function, constructing the concrete the configuration or a hash-card names with a branch per concrete, and the crate's public surface; carries the family's integration test across factory, concrete, and description | `encoding/abi` | The generic surface every hashed, signed, stored, or framed value passes through |
| `kdf/blake3_keyed` | The BLAKE3 keyed-derivation concrete; context strings, serialization, and output lengths frozen with known-answer vectors from an independent implementation; authors, as its producers, the interface that derives a key of a stated length from key material under an encoded context, the KDF identifier declaration, and the family's mock; creates the `adapters/kdf` crate; pins `blake3` | `encoding/factory`, `domain/derivation_context` | CR-05, CR-11 derivation contract and vectors |
| `kdf/factory` | The key-derivation family's factory function with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete | `kdf/blake3_keyed` | The surface every off-chain derivation passes through |
| `hash-to-scalar/keccak256` | The keccak256 concrete under domain tags, reduced modulo the group order, with vectors the generator mirrors; authors, as its producers, the interface that maps domain-tagged bytes to a scalar of the resolved pairing's field, the domain-tag types with their fallible constructors, the identifier declaration, and the family's mock; creates the `adapters/hash-to-scalar` crate | `encoding/factory`, `pairing/factory` | CR-08, CR-09 identity-mapping and challenge contract and vectors |
| `hash-to-scalar/factory` | The hash-to-scalar family's factory function with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete | `hash-to-scalar/keccak256` | The surface every value a contract recomputes passes through |
| `kem/bb1_depth_one` | The depth-one Boneh–Boyen concrete as one adapter: random master scalar and parameter set for escrow lineage, issuance with trivial identity-element refusal, seller-side rerandomization without the master scalar, the public validity check with encoding and subgroup validation, encapsulation under both identity scopes, well-formedness, and decapsulation to the encapsulated value; owns its parameter-set, credential, capsule, and identity-element types; authors, as its producers, `ICredentialKemAdapter` with setup, deriveIdentity, issue, rerandomize, isValid, encapsulate, isWellFormed, and decapsulate, the parameter-set, master-scalar, credential, capsule, identity-element, and encapsulated-value types as associated types, the identity mapping over caller-supplied canonical identity bytes, the identity-scope declaration carrying the identity mapping's domain tag, and the family's mock; creates the `adapters/kem` crate | `pairing/factory`, `hash-to-scalar/factory`, `random/factory`, `domain/secret`, `domain/asset_identity_hash`, `encoding/factory` | CR-08 contract and algebraic properties, LC-08, EC-06, CD-08 declaration |
| `kem/factory` | The credential KEM family's factory function, admitting a concrete by the suite's declared scope, with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete | `kem/bb1_depth_one` | CD-08 admission; the surface the sidecar, vector, and delivery tickets consume |
| `envelope/possession_statement` | The family-owned description of a proof of possession's statement, the key and commitment in the selected pairing's typed groups, flattened to precompile encodings only at the encoding boundary and validated with that pairing on reconstruction, implementing the encoding contract as its canonical field sequence; creates the `adapters/envelope` crate | `encoding/abi`, `pairing/factory` | LC-08 and CR-09, the proof of possession's statement frozen as one field sequence |
| `envelope/pairing_elgamal` | The pairing ElGamal concrete as one adapter: two independently keyed envelope keys generated from caller-supplied secret draws, each with a Schnorr proof of possession whose challenge is the hash-to-scalar mapping of the encoded possession statement, identity-element and shared-secret rejection, encryption of a credential's `CredentialComponents` with independent coins drawn through the randomness family, decryption under the recipient's key pair; owns its key-pair, public-key, possession, and envelope types; authors, as its producers, `IKeyAgreementAdapter` with key generation under proofs of possession, wrap, and unwrap, the key-pair, public-key, possession, and envelope types as associated types with their fallible constructors, a recipient's public keys admitted only with their proof of possession, the envelope-algebra declaration carrying the proofs of possession's domain tags, and the family's mock | `envelope/possession_statement`, `pairing/factory`, `hash-to-scalar/factory`, `random/factory`, `domain/secret`, `encoding/factory`, `kem/factory` | CR-04 contract; LC-08 at the algebra level |
| `envelope/factory` | The envelope family's factory function with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete | `envelope/pairing_elgamal` | The surface every envelope passes through |
| `proof/mint_statement` | The family-owned description of the mint transcript under delivery-statement version one, implementing the encoding contract as its canonical field sequence: the suite identifier, the chain, the entitlement contract, the asset identity hash, the parameter-set digest, the source and target entitlements, the old and new interval counters, the purpose, the issuer's identity in the seller field, the buyer's identity, the buyer's typed envelope keys, the typed new envelope, the expiry, and the mint relation's typed first messages, flattening group elements to precompile encodings only at the encoding boundary and validating them on reconstruction; relation-specific purpose variants with derived sixteen-bit codes and relations; the version constant; generic over the selected pairing and the chain family's identity, entitlement, interval, and chain-identifier forms, the entitlement contract taking the identity form, and authors, as its producers, the encoding family's canonical-field contract `ICanonicalField` in the encoding factory's interface with its mock form in the encoding factory's mock, a form's declared field kind, its conversion to one canonical field value, and its fallible reconstruction from one with a typed refusal, and the chain family's form interface, the trait carrying those forms as associated types each bounded by that contract, and the forms' mock, in `adapters/chain`'s factory module, creating that crate with that interface alone and a dependency on `adapters/encoding`, with the forms identifier and `ChainFormsDeclaration` defined before `IChainForms`, while `ISettlementAdapter`, `IEntitlementStateAdapter`, and the remainder of the chain declaration wait for their first consumer in the chain and submission sprint; creates the `adapters/proof` crate | `encoding/abi`, `encoding/factory`, `pairing/factory`, `envelope/factory`, `domain/asset_identity_hash`, `domain/suite_identifier`, `domain/parameter_set_identifier` | CR-09 statement binding under the version the hash-card names; the mint transcript frozen as one field sequence; the chain family's declaration the resolver reads |
| `proof/transfer_statement` | The family-owned description of the transfer transcript under delivery-statement version one, serving sale and grant: the mint transcript's context with the seller's identity and typed envelope keys, the typed old envelope, and the transfer relation's typed first messages, the source entitlement equal to the target for a sale and the author's entitlement for a grant with the old counter the author's current interval and the new counter zero ; reconstruction validates group encodings with the selected pairing | `proof/mint_statement`, `pairing/factory`, `envelope/factory` | CR-09; CD-05's grant statement frozen as one field sequence |
| `proof/schnorr_fs/challenge` | The function the concrete owns: the Fiat–Shamir challenge as the hash-to-scalar mapping, under the proof's domain tag, of a transcript's encoding through the encoding and hash-to-scalar concretes the hash-card's identifiers name; creates the `schnorr_fs` module | `proof/mint_statement`, `proof/transfer_statement`, `hash-to-scalar/factory`, `encoding/factory` | CR-09 statement binding |
| `proof/schnorr_fs` | The generalized Schnorr concrete as one adapter: the mint prover, the transfer prover from the seller's fresh decryption and total offset, and the reference verifier in both forms, second-group arithmetic and the hash-weighted pairing product with first-group arithmetic only; declares both forms; authors, as its producers, `IDeliveryProofAdapter` with proveMint, proveTransfer, and verify, the algebraic mint and transfer statement types over the pairing's group elements, the proof type as an associated type, the declaration of supported envelope algebras, verifier forms, and delivery-statement versions and of the challenge and weight domain tags, and the family's mock | `proof/schnorr_fs/challenge`, `pairing/factory`, `envelope/factory`, `kem/factory`, `random/factory` | CR-09, CD-01, CD-02 |
| `proof/factory` | The delivery proof family's factory function, admitting a concrete only when it declares the resolved envelope's algebra, the verifier form the resolved pairing declares, and the delivery-statement version the hash-card names, with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete | `proof/schnorr_fs` | CR-09 contract; the admission rule the harness exercises |
| `chain/evm_forms` | The EVM suite's forms as a private concrete of the chain family: the identity, entitlement, interval, and chain-identifier forms over the twenty-byte, 256-bit unsigned, 64-bit unsigned, and 256-bit unsigned kinds the suite's schemas fix, each implementing the canonical-field contract, and the type implementing the form interface that names them; implements the forms declaration type and identifier already produced with `IChainForms` by `proof/mint_statement`, setting its associated declaration constant; the family's first concrete, private beneath the factory as every concrete is | `proof/mint_statement`, `encoding/factory` | CR-09: the forms a transcript's chain, entitlement, interval, and identity fields take, declared and admitted rather than assumed |
| `chain/create_chain_forms` | The family-owned function constructing the forms concrete the configuration names, admitting it only when its declaration carries the forms identifier the hash-card carries as an explicit field, supplied by the configuration in the harness and read from the deployment record in the daemon, with a branch per concrete, and handing it to a consumer generic over the form interface; its own module, named for its one function as the Schnorr challenge module is, because the factory module holds one function and that function constructs the chain concrete; carries the family's integration test across the function and the forms concrete | `chain/evm_forms` | The surface the generator, verifier client, vectors, and measurement obtain their forms through, and later every consumer of the chain family's forms |
| `workflows/sidecar/wrap` | The wrapping key derived through the KDF family from an encapsulated value and the context, and the XOR wrap of a piece-group key under it; creates the `workflows` crate | `kdf/factory`, `kem/factory`, `encoding/factory` | CR-11 wrap |
| `workflows/sidecar/unwrap` | The unwrap of a piece-group key from its wrapped value under the derived wrapping key | `workflows/sidecar/wrap` | CR-11 cross-set agreement |
| `harness-crypto/generate/evm/render` | The function the EVM generate concrete owns that renders Solidity library source, opening with the license identifier and the compiler version pragma its params carry: a constants library from a library name and an ordered list of named entries, each a byte string, a 256-bit, sixty-four-bit, or sixteen-bit unsigned value, a boolean, or a string; and a vector-record library from a library name and, per vector kind, a struct name, the struct's members as names with canonical field kinds in canonical order, a decode function name, a path constant name, and the path of the kind's vector file relative to the Foundry project root, rendering each member's type through the EVM suite's table from canonical field kinds to ABI type names, which this function holds, a fixed twenty-byte kind as `bytes20`, each decode function as `abi.decode` of one line's bytes over the members' types in canonical order returning the struct, and each path constant as a string; owns the entry type, the record-declaration type, the validated name types, and the kind-to-type table the concrete's other functions, the concrete, and the configuration consume; pure, reading no adapter and touching no operating-system facility; creates the `generate` area and the `evm` module | `harness-crypto/benchmark` | CR-09 parity inputs: one rendering for every constant and every vector record type |
| `harness-crypto/generate/evm/constants` | The function the concrete owns that returns the constants library's entries, generic over the form interface: each family's domain tags read from its declaration, the identity mapping's, the two proofs of possession's, and the delivery proof's challenge and weight, each a byte string; the delivery-statement version it is handed and the delivery purposes' codes from the proof family's declared purposes, each a sixteen-bit unsigned value; the mint and transfer transcripts' field sequences read from their descriptions over the forms in play, each rendered as one string of ABI type names through the kind-to-type table `harness-crypto/generate/evm/render` holds, a fixed twenty-byte kind as `bytes20`, as the encoding concrete encodes it, never `address`; the scalar field's order from the pairing family's reference trait, a 256-bit unsigned value; the two generators' precompile encodings; and whether the verifier has second-group arithmetic, a boolean read from the pairing's declaration | `harness-crypto/generate/evm/render`, `pairing/factory`, `kem/factory`, `envelope/factory`, `proof/factory`, `chain/create_chain_forms` | CR-09 parity inputs; CR-11 the tags and the modulus a contract hashes under |
| `harness-crypto/generate/evm/group_vectors` | The function the concrete owns that returns the pairing library's group-operation and pairing-product vector records, each kind drawn and holding the count its params carry for that kind, over scalars it draws through the randomness family, each record carrying its draws, its points in the precompile encodings the pairing declares, and the pairing-product verdict read from the reference: first-group addition, first-group multiplication, and the pairing-product check for every pairing, and second-group addition and both groups' multi-scalar multiplication where the pairing declares second-group arithmetic at the verifier; each record type with its canonical description implementing the encoding contract and the member names the record declaration `render` owns takes; owns the per-kind vector count type the other vector functions, the generate family's per-kind settings, and the configuration consume | `harness-crypto/generate/evm/render`, `encoding/factory`, `pairing/factory`, `random/factory` | CR-10 on-chain vectors |
| `harness-crypto/generate/evm/rejection_vectors` | The function the concrete owns that returns the decode-rejection vector records, an enumerated kind holding exactly the candidates it enumerates: one record per candidate encoding, for each source group and for a scalar, each naming what it decodes into and holding the candidate's bytes, every record an input the suite must refuse, the outside-the-subgroup candidates being the pairing family's reference encodings, absent where a group is the whole curve, and the non-canonical scalar candidate being the scalar field's order; the record type with its canonical description and member names | `harness-crypto/generate/evm/render`, `encoding/factory`, `pairing/factory` | CR-10 subgroup rejection and encoding edge cases on chain |
| `harness-crypto/generate/evm/mapping_vectors` | The function the concrete owns that returns the hash-to-scalar vector records, a drawn kind holding the count its params carry for it, each a message it draws and the scalar that message maps to under each family's domain tag read from that family's declaration, the identity mapping's, the two proofs of possession's, and the delivery proof's challenge and weight, the tags the constants library carries, so one message under every tag proves the tag is bound; and the identity-mapping vector records, a kind over configured inputs holding one record per identity-bytes value its payload carries, each mapped through the credential KEM under the admitted identity scope, the scalar read from the identity element's components; each record type with its canonical description and member names | `harness-crypto/generate/evm/render`, `encoding/factory`, `hash-to-scalar/factory`, `kem/factory`, `envelope/factory`, `proof/factory`, `pairing/factory`, `random/factory` | CR-08, CR-11 identity-mapping and hash-to-scalar vectors |
| `harness-crypto/generate/evm/possession_vectors` | The function the concrete owns that returns the possession vector records, a drawn kind holding the count its params carry for it: envelope keys with their proofs of possession's components, generated through the key-agreement family from secret draws the function makes and records; the proofs' nonces are drawn inside the adapter and are not recorded; the record type with its canonical description and member names | `harness-crypto/generate/evm/render`, `encoding/factory`, `envelope/factory`, `pairing/factory`, `random/factory` | LC-08 at the algebra level, on chain |
| `harness-crypto/generate/evm/challenge_vectors` | The function the concrete owns that returns the challenge vector records, generic over the form interface, a drawn kind holding the count its params carry for it, each record carrying the values the function drew through the randomness family: mint and transfer transcripts whose forms are rebuilt from the payload's canonical context values, each with its encoding through its description and the encoding family and its challenge, the hash-to-scalar mapping of that encoding under the challenge tag the proof family's declaration carries; the transcripts stand alone, since the proof family's challenge function is private to its crate and a proof in the second-group form carries no first messages; the record type with its canonical description and member names | `harness-crypto/generate/evm/render`, `proof/factory`, `encoding/factory`, `hash-to-scalar/factory`, `chain/create_chain_forms`, `pairing/factory`, `random/factory` | CR-09, CR-11 challenge vectors |
| `harness-crypto/generate/evm/proof_vectors` | The function the concrete owns that returns the mint-proof and transfer-proof vector records, generic over the form interface, each a drawn kind holding the count its params carry for it, each record a statement with its proof's components: a parameter set set up under the admitted identity scope, the identity element of the payload's identity bytes, an issued credential wrapped to the seller's admitted keys and proved by the mint prover, then the seller's fresh decryption rerandomized, wrapped to the buyer's admitted keys, and proved by the transfer prover; each statement as its context values and its elements' precompile encodings, the first-group-only form carrying its second-group first messages, and each record carrying the draws the function made; each record type with its canonical description and member names | `harness-crypto/generate/evm/render`, `encoding/factory`, `kem/factory`, `envelope/factory`, `proof/factory`, `pairing/factory`, `random/factory`, `chain/create_chain_forms` | CR-09 parity inputs |
| `harness-crypto/generate/evm` | The EVM concrete of the generate family, one adapter generic over the form interface, constructed over the borrowed pairing, hash-to-scalar adapter, encoder, randomness source, credential KEM, key agreement, and delivery proof `harness-crypto/run`'s consumers obtained through their factories over the forms `create_chain_forms` handed it, with their declarations, the admitted identity scope, the delivery-statement version, the per-kind settings, the library names, the Foundry project root and the generated directory relative to it, and the Solidity compiler version and license identifier its libraries open with: calls each function it owns with each drawn kind's count, renders the constants library through `render`, encodes each vector record through the encoder under its canonical description into one line, `0x` and the encoding's lowercase hex digits and a line feed, writes one vector file per kind its declaration names under the file name its per-kind settings carry, renders through `render` the vector-record library declaring each kind's struct, decode function, and path constant under the names its per-kind settings carry with each file's path relative to the Foundry project root, and writes both libraries, all under the generated directory in the directory named from the curve the pairing's declaration carries, the statements' context values and the identity bytes arriving as payload from the configuration; the family's only module that touches the filesystem, an app module in the adapter role as the benchmark is over the clock; authors, as its producers, the generate family's interface that emits a target suite's constants, vector-record types, and vector files from the Rust reference under a caller-supplied destination, the vector-kind identifier and the per-kind settings type, its declaration naming the vector kinds it emits with each kind's sizing, drawn, enumerated, or over configured inputs, and gating each kind that needs second-group arithmetic on the verifier form the pairing declares, and its mock; calls no factory and names no chain's forms, no path, no file, library, or symbol name, no count, no curve, no profile, no compiler version, and no license | `harness-crypto/generate/evm/constants`, `harness-crypto/generate/evm/group_vectors`, `harness-crypto/generate/evm/rejection_vectors`, `harness-crypto/generate/evm/mapping_vectors`, `harness-crypto/generate/evm/possession_vectors`, `harness-crypto/generate/evm/challenge_vectors`, `harness-crypto/generate/evm/proof_vectors`, `pairing/factory`, `random/factory`, `kem/factory`, `envelope/factory`, `proof/factory`, `hash-to-scalar/factory`, `encoding/factory`, `chain/create_chain_forms` | CR-09 parity inputs; CR-10 and CR-11 on-chain vectors |
| `harness-crypto/generate/factory` | The generate family's factory function, constructing the concrete the configuration names over the borrowed adapters and declarations its deps carry and the configured values its params carry, the per-kind settings, library names, Foundry project root, generated directory, Solidity compiler version, and license identifier among them, admitting the configuration only when its per-kind settings name exactly the kinds the concrete declares, with a count for each drawn kind and none for any other kind, with a branch per concrete, and its public surface; carries the family's integration test across factory and concrete | `harness-crypto/generate/evm` | CR-09 parity-input contract |
| `harness-crypto/config/layers` | The function the configuration owns that produces the configuration document from layered sources: parses the command line through `clap` into its repeatable `--config <PATH>` and `--set <KEY=VALUE>` arguments, reads each named file through `std::fs`, parses the defaults text it is handed, each file's text, and each assignment as TOML, merges them over the defaults in command-line order, a table merging key by key and any other value replacing the earlier one whole, and deserializes the merged table into the configuration document; owns the argument and document types, each document table refusing a key it does not declare; returns the argument error, a file's read error with its path, a text's parse error, and an assignment that is not a key and a value, each carried unchanged; an app module in the adapter role, as the benchmark is over the clock; creates the `config` module; pins `clap` with its derive feature, `toml`, and `serde` with its derive feature, the one place the harness names any of them | `workspace/cargo` | Every configured value overridable from a configuration file and from a command-line flag, applied in order over the shipped defaults |
| `harness-crypto/config/admit_selections` | The function the configuration owns that admits the document's selections into the adapter families' own types: the randomness source, the encoding, hash-to-scalar, chain-forms, KEM, key-agreement, and delivery-proof concretes with their identifiers, the identity scope, the envelope algebra, and the delivery-statement version, each curve to run with its pairing concrete and the target-group encoding identifier the suite requires for it, each pairing concrete the benchmark measures with the target-group encoding identifier its curve requires, and the network profile's precompile encodings, each text value mapped to the variant it names and an unknown one refused with its document path; owns the admitted selection types; adds `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq` where they are absent to `RandomSourceKind`, `PairingConcrete`, `PrecompileEncoding`, `HashToScalarConcrete`, `HashToScalarIdentifier`, `KemConcrete`, `KemIdentifier`, `KeyAgreementConcrete`, `KeyAgreementIdentifier`, and `DeliveryProofConcrete` in their families' factory interfaces, so the configuration holds them and the run hands each curve and each benchmarked concrete its own copy | `harness-crypto/config/layers`, `random/factory`, `pairing/factory`, `encoding/factory`, `hash-to-scalar/factory`, `chain/create_chain_forms`, `kem/factory`, `envelope/factory`, `proof/factory` | The concrete and identifier per family and the pairing per curve every factory call takes, configured and never literal |
| `harness-crypto/config/admit_statements` | The function the configuration owns that admits the document's network profile and statements: the chain identifier and the entitlement contract as canonical field values, each a kind and a value the document carries, a fixed-width value as `0x`-prefixed hex of its exact width and an unsigned value within its kind's range; the mint and transfer contexts, their suite identifier, asset identity hash, and parameter-set identifier through the domain constructors, their entitlements, intervals, seller, and buyer as canonical field values, their expiry, and their chain identifier and entitlement contract the network profile's; the mint and transfer purposes through their declared codes; the identity bytes the identity mapping covers and the identity the proofs are made for, each `0x`-prefixed hex; each refusal carried unchanged with its document path; owns the admitted network-profile and statement types | `harness-crypto/config/layers`, `domain/suite_identifier`, `domain/asset_identity_hash`, `domain/parameter_set_identifier`, `encoding/factory`, `proof/factory`, `harness-crypto/generate/evm` | The context values, purposes, and identities every statement the harness proves carries, configured and never literal |
| `harness-crypto/config/admit_generate` | The function the configuration owns that admits the document's generate settings into `GenerateConcrete::Evm { settings }`: each vector kind's text value mapped to its `VectorKind`, its size through `VectorCount` and `MsmTermCount` with its message length, and its file, struct, decode function, and path constant names through `GeneratedFileName` and the render name types; the constants and vector-record libraries' file and library names, the Foundry project root, the generated directory's segments, the curve directories, the Solidity compiler version, and the license identifier through their own constructors; each refusal carried unchanged with its document path; admits each kind it is given and leaves the admission of the kind set against the concrete's declaration to `harness-crypto/generate/factory` | `harness-crypto/config/layers`, `harness-crypto/generate/evm/render`, `harness-crypto/generate/factory` | The per-kind settings, names, paths, compiler version, and license the generator takes, configured and never literal |
| `harness-crypto/config/admit_output` | The function the configuration owns that admits the document's output destinations, the places the run's outcome is written: standard output, a file at a path the document carries, or both, each destination's text mapped to the variant it names and a file's path carried; an unknown destination and an empty destination list each refused with its document path; owns the admitted output types | `harness-crypto/config/layers` | The run's outcome written where the caller configures, from a shipped default, a file, or a flag, never literal |
| `harness-crypto/config` | The configuration's composing function: hands the shipped defaults and the command line to `harness-crypto/config/layers`, then the document's parts to `harness-crypto/config/admit_selections`, `harness-crypto/config/admit_statements`, `harness-crypto/config/admit_generate`, and `harness-crypto/config/admit_output`, and admits the benchmark's iteration count, returning the typed configuration every factory call and every write in the harness takes or the first refusal unchanged; owns the typed configuration and its error; carries `apps/harness-crypto/harness.toml`, the shipped defaults embedded in the binary, a default for every value the document declares, the pairing concretes the benchmark measures and the output destinations among them; revised in place, with its defaults, when the verifier, vectors, measurement, and report tickets add the values they first consume | `harness-crypto/config/admit_selections`, `harness-crypto/config/admit_statements`, `harness-crypto/config/admit_generate`, `harness-crypto/config/admit_output`, `harness-crypto/benchmark` | The values every factory call and every write in the harness takes, configured and never literal |
| `harness-crypto/run/generate_consumer` | The run's consumer of the generate family's factory, one type implementing `IGenerateConsumer` generic over the forms: handed the generate concrete the factory admitted, calls `emit` with the configured mint and transfer contexts, purposes, identities, and proof identity, and returns the files written or the concrete's refusal unchanged; names no concrete and calls no factory | `harness-crypto/generate/factory`, `harness-crypto/config` | The generated directory written from the configured statements through a concrete the run never names |
| `harness-crypto/run/delivery_proof_consumer` | The run's consumer of the delivery proof family's factory, one type implementing `IDeliveryProofConsumer`: handed the delivery proof concrete, calls `create_generate` over the adapters and declarations it carries with the configured generate concrete, the scope the KEM factory admitted, and the configured delivery-statement version, its consumer `harness-crypto/run/generate_consumer`; revised in place when the verifier, vectors, and measurement tickets add the calls they first need | `harness-crypto/run/generate_consumer`, `harness-crypto/generate/factory`, `proof/factory` | The composed adapters handed to the generate factory with the configured values its params carry |
| `harness-crypto/run/key_agreement_consumer` | The run's consumer of the envelope family's factory, one type implementing `IKeyAgreementConsumer`: handed the key-agreement concrete, calls `create_delivery_proof` with the configured delivery-proof concrete and statement version, the envelope algebra the key-agreement selection carries, and the verifier form the selected pairing declares, its consumer `harness-crypto/run/delivery_proof_consumer` | `harness-crypto/run/delivery_proof_consumer`, `proof/factory`, `envelope/factory` | The proof factory's admission exercised from the configured selection and the pairing's declared form |
| `harness-crypto/run/kem_consumer` | The run's consumer of the credential KEM family's factory, one type implementing `IKemConsumer`: handed the KEM concrete and the scope it admitted, calls `create_key_agreement` with the configured key-agreement selection, its consumer `harness-crypto/run/key_agreement_consumer` | `harness-crypto/run/key_agreement_consumer`, `envelope/factory`, `kem/factory` | The admitted scope carried to every later stage |
| `harness-crypto/run/chain_forms_consumer` | The run's consumer of the chain family's forms function, one type implementing `IChainFormsConsumer`: handed the forms concrete, calls `create_kem` with the configured KEM selection, its consumer `harness-crypto/run/kem_consumer` | `harness-crypto/run/kem_consumer`, `kem/factory`, `chain/create_chain_forms` | The forms every later stage is generic over, admitted by the configured identifier |
| `harness-crypto/run/encoding_consumer` | The run's consumer of the encoding family's factory, one type implementing `IEncodingConsumer`: handed the encoding concrete, calls `create_chain_forms` with the configured chain-forms selection, its consumer `harness-crypto/run/chain_forms_consumer` | `harness-crypto/run/chain_forms_consumer`, `chain/create_chain_forms`, `encoding/factory` | The encoder every later stage borrows, admitted by the configured identifier |
| `harness-crypto/run/pairing_consumer` | The run's consumer of the pairing family's factory, one type implementing `IPairingConsumer`: handed a curve's pairing concrete, obtains the hash-to-scalar adapter through `create_hash_to_scalar` with the configured selection and calls `create_encoding` with the configured encoding selection, its consumer `harness-crypto/run/encoding_consumer` | `harness-crypto/run/encoding_consumer`, `encoding/factory`, `hash-to-scalar/factory`, `pairing/factory` | Each configured curve composed through every family's factory |
| `harness-crypto/run` | The harness run, the composing function over the consumers it owns and the harness's composition root: obtains the randomness source through `create_random_source`; for each configured benchmark concrete, calls `create_pairing` with that concrete, its target-group encoding identifier, and the network profile's precompile encodings, its consumer a `PairingBenchmark` over the configured iteration count; then for each configured curve, calls `create_pairing` likewise, its consumer `harness-crypto/run/pairing_consumer`; returns the run's outcome, the benchmark's measurements per concrete and the files written per curve, or the first refusal unchanged; owns the outcome and its error; with its consumers, the harness's only caller of a family's factory; revised in place when the verifier, vectors, measurement, and report tickets add their results to the outcome | `harness-crypto/run/pairing_consumer`, `harness-crypto/benchmark`, `harness-crypto/config`, `random/factory`, `pairing/factory` | Every family composed through its factory from the configuration's values; the benchmark's measurements per configured concrete |
| `harness-crypto/output` | The function the harness owns that writes the run's outcome, a success or a refusal, rendered as text, to each destination the configuration names, standard output, the file at the configured path, or both, in the configured order; an app module in the adapter role over the console and the filesystem, as the generate concrete is over the filesystem; returns a destination's write error with the destination it concerned | `harness-crypto/run`, `harness-crypto/config/admit_output` | The run's outcome, success or refusal, reaching every destination the caller configured |
| `harness-crypto/main` | The harness binary's entry point, `apps/harness-crypto/src/main.rs`, holding only what a binary's `main` keeps: calls `harness-crypto/config` with the shipped defaults and the command line, `harness-crypto/run` with the typed configuration, and `harness-crypto/output` with the run's outcome and the configured destinations, and exits with a success status only when the run succeeded and every destination was written; a configuration refusal, which arrives before any destination is admitted, is written to standard error; declares no type and holds no test, its correctness checked by reading, and so carries no element files; holds no literal | `harness-crypto/config`, `harness-crypto/run`, `harness-crypto/output` | The generated directory written before `contracts/evm/PairingLib` and carried by the milestone's commit; the harness invoked from its shipped defaults, a file, and a flag |
| `contracts/evm/PairingLib` | The Solidity library over the precompiles in both forms, with contract-side subgroup checks where the precompile does not perform them, its constants read from the committed constants library and its test vectors from the committed vector files, each file at the path and decoded by the function the committed vector-record library carries, all of which the harness binary wrote; creates `contracts/evm/foundry.toml`, granting `fs_permissions` read access to the configured generated directory's shipped default and to nothing else, leaving `ffi` disabled, and setting `via_ir = true` so a vector-record library's decode function over a record of any member count compiles, and the Solidity continuous-integration definition `.github/workflows/contracts.yml` with `forge build`, `forge fmt --check`, and `forge test`, each authored once and complete | `workspace/ci`, `harness-crypto/main`; mirrors the pairing family's declared precompile encodings | CR-10 on chain |
| `contracts/evm/DeliveryVerifier` | Solidity mint and transfer verification over the statement fields, bit-for-bit with `proof/schnorr_fs`, its constants read from the committed constants library and its test vectors from the committed vector files through the committed vector-record library | `contracts/evm/PairingLib`, `harness-crypto/main` | CD-03, CR-09 |
| `contracts/evm/deploy` | The deployment script for the verifier on each curve form to Anvil and Base Sepolia, emitting addresses to configuration; exempt from the full support structure as a deployment script | `contracts/evm/DeliveryVerifier` | CD-07 |
| `harness-crypto/verifier/evm` | The EVM concrete over the deployed verifier through `alloy`, generic over the form interface, constructed over the adapters and declarations `harness-crypto/run/delivery_proof_consumer` hands the verifier factory and calling no factory, its statements built over the forms `create_chain_forms` hands it and their context values from the configuration's network profile and deployment record; authors, as its producers, the verifier family's interface that reaches a suite's deployed delivery verifier, submits a statement and proof, and reads acceptance and cost as L2 execution gas and L1 data fee, its declaration, and its mock; pins `alloy` for the harness | `contracts/evm/deploy`, `proof/factory`, `encoding/factory`, `chain/create_chain_forms` | CD-03 cross-verification |
| `harness-crypto/verifier/factory` | The verifier family's factory function, constructing the concrete the configuration names over the borrowed adapters and declarations its deps carry, with a branch per concrete, and its public surface; carries the family's integration test across factory and concrete | `harness-crypto/verifier/evm` | CD-03 cross-verification contract |
| `harness-crypto/vectors` | Algebraic, mutation, and admission vector sets over the adapters and declarations `harness-crypto/run/delivery_proof_consumer` hands it, generic over the form interface, apart from the generate concrete's parity inputs, which mirror the reference to Solidity where these exercise it: cross-holder agreement, cross-set agreement with two independently generated parameter sets unwrapping one piece-group key, rerandomized validity, non-convertibility, malformed capsules, mutated statement fields, replay, and the refusal the proof factory and the forms function return for every curve, verifier-form, and forms combination they must refuse, each attempted by the run and handed to this module to record | `kem/factory`, `envelope/factory`, `proof/factory`, `chain/create_chain_forms`, `workflows/sidecar/unwrap`; `random/factory`, whose production concrete supplies the independent draws, the drawn values recorded with each emitted vector | CR-08, CR-09, CR-11 vectors; AS-17 for the cryptographic families |
| `harness-crypto/measure` | Size, timing, and gas capture per curve, over the adapters and declarations `harness-crypto/run/delivery_proof_consumer` hands it: capsule, envelope, proof bytes; decapsulation per group; prove and verify time; mint and transfer cost as L2 execution and L1 data fee | `harness-crypto/vectors`, `harness-crypto/verifier/factory` | CD-07, RO-03 |
| `harness-crypto/report` | The release-evidence report recording the measurements, the piece-group size and curve defaults in force, and any adjustment the data supports, selecting nothing against a threshold; the grouping's integration test across the whole chain and its commit | `harness-crypto/measure` | AS-21 |

### Protocol core and contract suite

The hashing, signature, and swarm milestones are at ticket resolution; they depend on nothing the harness measures and run beside it from `encoding/factory` onward, since their first tickets consume the encoding family, and each begins with the first concrete of its family, which carries the family's interface, and closes each family at its factory. The remaining milestones are sprints, re-mapped to tickets when the harness closes.

#### Hashing and commitments, at ticket resolution

| Ticket | Owns | Depends on | Proves |
| --- | --- | --- | --- |
| `hashing/blake3_bao` | The BLAKE3/Bao concrete as one adapter: root and outboard construction, verification, challenge and response, and keyed mode for the keyed plaintext-root disclosure mode; authors, as its producers, the commitment interface with root and outboard construction over a byte stream, streaming and random-access verification against a root with an authentication path, and random chunk challenge and response, the root, outboard, path, and chunk types with their fallible constructors, the commitment-scheme identifier the hash-card carries, the commitment-scheme and chunk-granularity declaration, and the family's mock; creates the `adapters/hashing` crate; pins `blake3` and `bao` | `encoding/factory` | CR-02 contract, construction, verification, and challenges; EC-02; SW-04 |
| `hashing/factory` | The hashing family's factory function with a branch per concrete, and the crate's public surface; carries the milestone's integration test and commit | `hashing/blake3_bao` | The surface every content root passes through |

#### Signature services, at ticket resolution

| Ticket | Owns | Depends on | Proves |
| --- | --- | --- | --- |
| `signature/ed25519` | The Ed25519 concrete with domain separation and complete-message binding; authors, as its producers, `ISignatureAdapter` with sign, isValidSignature, recoverSigner, and canonicalAddress, the signature types with their fallible constructors, the per-layer declaration of scheme and layers served, and the family's mock; creates the `adapters/signature` crate; pins `ed25519-dalek` | `encoding/factory`, `random/factory` | CR-03 contract and CR-03 |
| `signature/secp256k1` | The secp256k1 concrete through `alloy`, with EIP-712 typed data for registrations, bindings, and signed intents; pins `alloy` for the signature crate | `signature/ed25519` | CR-03, IW-07, LC-13 intents |
| `signature/factory` | The signature family's factory function, resolving a scheme per layer, with a branch per concrete, and the crate's public surface; carries the milestone's integration test and commit | `signature/ed25519`, `signature/secp256k1` | The surface every layer signs through |

#### Swarm transport, discovery, storage, and seed host, at ticket resolution

| Ticket | Owns | Depends on | Proves |
| --- | --- | --- | --- |
| `transport/rqbit` | The `librqbit` concrete over the overlay branch the workspace manifest names: the piece-completion hook, the peer-injection call, and the storage-backend trait on that branch with each hook's upstream issue; root-to-infohash translation; torrent creation per object with the ciphertext and each sidecar as its own object and locator; Bao verification of every completed piece after the library's check, through the hashing family; seeder-map peer injection; the engine capability; authors, as its producers, `ISwarmTransportAdapter`, the locator types with their fallible constructors, the declaration of the integrity structure on the wire and of the engine capability a transport may expose for library-backed discovery, and the family's mock; creates the `adapters/transport` crate; pins `librqbit` | `hashing/factory`, `encoding/factory` | SW-01 contract, SW-01, SW-06, EC-02 |
| `transport/factory` | The transport family's factory function with a branch per concrete, and the crate's public surface, including the engine capability; carries the family's integration test across factory and concrete | `transport/rqbit` | The surface the seed host, resolution, and library-backed discovery consume |
| `discovery/local` | Local-network discovery as a source; authors, as its producers, `IPeerDiscoveryAdapter`, the peer types with their fallible constructors, the declaration, and the family's mock; creates the `adapters/discovery` crate | `encoding/factory` | SW-02 contract; SW-02 |
| `discovery/dht` | The library's DHT as a source, through the engine capability the transport factory's surface exposes | `discovery/local`, `transport/factory` | SW-02 |
| `discovery/pex` | Peer exchange as a source, through the same engine capability | `discovery/local`, `transport/factory` | SW-02 |
| `discovery/tracker` | Trackers as a source, through the same engine capability | `discovery/local`, `transport/factory` | SW-02 |
| `discovery/aggregate` | The family-owned aggregation: concurrent sources unioned and deduplicated, none authoritative | `discovery/local` | SW-02 aggregation under overlapping, conflicting, unavailable, and malicious results |
| `discovery/factory` | The discovery family's factory function, resolving every source the composition admits behind the aggregation, with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concretes; revised in place when the chain and submission sprint adds `discovery/seeder_map` | `discovery/local`, `discovery/dht`, `discovery/pex`, `discovery/tracker`, `discovery/aggregate` | The surface the swarm engine consumes |
| `storage/redb` | The `redb` concrete; authors, as its producers, the key-value store interface with tables, transactions, and checkpoints, its types with their fallible constructors, the declaration, and the family's mock; creates the `adapters/storage` crate; pins `redb` | `encoding/factory` | XA-03 and SW-05 store contract; transactions and checkpoints under interruption |
| `storage/factory` | The storage-engine family's factory function with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete | `storage/redb` | The surface every index, the job engine, and the configuration registry consume |
| `seed-host/owned` | The owned concrete: persistent seeding independent of sessions, the root-keyed ciphertext store with holding reason, quota, and eviction, its index through the storage family, settings passthrough and enforcement, archive enumeration; authors, as its producers, `ISeedHostAdapter` with held-and-served, take custody, release, capacity and usage, and enumerate, the holding types with the obligated-versus-voluntary distinction and their fallible constructors, the capacity and holding-reason declaration, and the family's mock; creates the `adapters/seed-host` crate | `hashing/factory`, `encoding/factory`, `transport/factory`, `storage/factory` | SW-05 contract; SW-03, SW-05, SW-07, ST-06; PR-05 separation |
| `seed-host/rqbit` | The delegated concrete: the rqbit application driven through its HTTP API, root-to-identifier translation, and Bao custody challenges through the hashing family | `seed-host/owned`, `hashing/factory` | SW-04 |
| `seed-host/factory` | The seed host family's factory function with a branch per concrete, and the crate's public surface; carries the milestone's integration test across owned and delegated retrieval and its commit | `seed-host/owned`, `seed-host/rqbit` | The surface resolution and the First Finder consume |

#### Remaining milestones, as sprints

| Sprint by role | Contents | Depends on |
| --- | --- | --- |
| Domain model | The protocol's remaining types, each authored in the node of the file that first consumes it and housed in the module that implements it, fixed when that module's ticket is written, the identifier modules and the derivation context being harness tickets; the `domain` crate holds only the types its own modules implement; for each encoded type, its canonical description as the encoding family's module `encoding/<type>`, implementing the factory's contract for that type, one module per type, authored after the type and before the first ticket that encodes it | the types the harness families own; `domain/derivation_context`; `encoding/factory` |
| Payload cipher and sidecar layer | `cipher/aes_ctr` with the counter layout and extent rules, authoring `IPayloadCipherAdapter`, the counter-layout and addressable-extent declarations, and the family's mock as its producers and creating the `adapters/cipher` crate, then `cipher/factory`; `workflows/sidecar/build`, per live set a capsule and wrapped piece-group key per group committed by that set's Bao root, each sidecar its own object, and `workflows/sidecar/validate`, the manifest, hash-card, and sidecar validation ordering; the `sample_deployment` generator for the site demonstration, which needs the cipher and so lives here rather than in the harness and carries the configured piece-group size default | domain model; `hashing/factory`; `workflows/sidecar/wrap`; `kem/factory` |
| Registry and entitlement contracts, the EVM suite | Under `contracts/evm`: asset records with the per-name version index, deployments and hash-cards, attestation verification through the `IAttestationVerifier` adapter bound per source with the npm P-256 verifier as the first concrete and the source-key table it reads, later escrow deployments, parameter-set liveness with the sidecar-coverage rule and sidecar addition, the envelope-key registry, `IEntitlement` with the ERC-721 concrete carrying interval state and the ownership override that disables standard transfers, issuance and transfer calling the delivery verifier, batched grant requests readable by holders and closed by grant or withdrawal, escrow records per deployment, identity binding, the identity contract account under ERC-1271 with its device registry and the `isDeviceAdmitted` view, claim-set state layout, batch and paginated views; every identity-bound mutation taking the acting identity and a signed intent verified through the account against a signer whose permissions cover it, the contracts enforcing lock expiry and no volume limit | harness contracts; domain model |
| Chain and submission families | `chain/base` with the contract bindings, Base's tier mapping, `evaluateAuthorization` and batch with per-context bindings, and the event and calldata reader, authoring the chain interface, `ISettlementAdapter`, `IEntitlementStateAdapter`, the remainder of the declaration, and their mock as its producers in the factory module `proof/mint_statement` created, and binding as its associated forms the forms `create_chain_forms` resolves, its declaration naming the forms identifier it binds; `chain/quorum_view`, the family-owned aggregation of two of three configured nodes at a common reference, stale ignored, divergence and views older than `τ_soft` failing closed; `chain/factory`, the chain factory function constructing the chain concrete the configuration names, admitting it only when its declaration lists the forms identifier the resolved forms carry, with its `base` branch, authored once after the concrete; `submission/self_funded`, the path the daemon uses on Base Sepolia until the relayer exists, authoring the interface for submitting a signed intent and its declaration and creating the `adapters/submission` crate; `submission/factory`, revised in place when the relayer milestone adds `submission/paymaster` and `submission/relayer`; `discovery/seeder_map`, the on-chain seeder map as a discovery source through the chain factory, with `discovery/factory` revised in place to carry its branch | registry and entitlement contracts; `signature/factory`; `discovery/factory` |
| Adapter registry and factory, on chain | The `AdapterRegistry` governed by a single project-held key and the factory that constructor-injects the identity adapters, the signature adapters, the attestation verifier per source, and the entitlement token form, each immutable once bound | registry and entitlement contracts |

### Local daemon and package serving, as epics

| Epic by role | Contents | Depends on |
| --- | --- | --- |
| Durable jobs and configuration | `telemetry/local_metrics` over the storage family, authoring the exporter interface, the declaration, and the family's mock as its producers and creating the `adapters/telemetry` crate, then `telemetry/tracing`, the family-owned tracing setup carrying per-request correlation identifiers, then `telemetry/factory`, revised in place when observability adds `telemetry/opentelemetry`; `workflows/jobs`, the job engine over the storage family with checkpointing and idempotent restart; `workflows/config`, the versioned configuration registry over the storage and encoding families, and the settings catalogue; `platform-paths/linux`, authoring the application-directory interface and creating the crate, `platform-paths/macos`, `platform-paths/windows`, and `platform-paths/factory` for the store-root defaults; `workflows/compose`, capability resolution across every factory, failing closed; the `ipc` family's `ipc/framing` and `ipc/principals`, family-owned and creating the crate, `ipc/unix_socket` authoring the IPC transport interface, `ipc/named_pipe`, `ipc/server` with the control-principal and local-presence rules, and `ipc/factory`; `lifecycle/lock`, the single-instance lock as a family-owned function creating the `adapters/lifecycle` crate, the family's concretes and factory following in the installation coordinator; the daemon binary | domain model; `storage/factory` |
| Identity and custody | `custody/holder_seed` with the control and envelope branches, family-owned and creating the `adapters/custody` crate; `custody/device_key`; `custody/local_keystore` over `keyring` with version-headed blobs, authoring `IKeyCustodyAdapter`, the versioned capability set including device roles, and the family's mock as its producers; `custody/upgrade`, the in-place migration between concretes verified against the chain, through the interface; `custody/factory`; `wallet/eip1193`, authoring the signing-only wallet interface and creating the crate, `wallet/walletconnect`, and `wallet/factory`; `identity/publisher_authority`, authoring `IIdentityAdapter` with the binding schema and creating the crate, `identity/escrow`, and `identity/factory`; `workflows/identity` with identity creation, relayer-paid binding and contract account creation, envelope-key registration, device pairing, signer admission, and revocation | the signature tickets; the chain and submission families; `envelope/factory`; `kdf/factory`; durable jobs and configuration |
| Plaintext CAS, resolution orchestrator, and package host | `cas/filesystem`, verified atomic CAS of tarballs extracted per project by the package manager with nothing linking into the store, quota, pinning, eviction, its index through the storage family, authoring the store interface and creating the crate, then `cas/factory`; `workflows/resolve` with the hedged source order under the catalogue's source deadline and grant wait, and upstream for any identity without a credential; `package-host/npm` with its metadata store, the npm registry protocol served locally with canonical tarball URLs, authoring `IPackageHostAdapter` and creating the crate, then `package-host/factory`; per-asset independence status in `workflows/health` | durable jobs and configuration; `hashing/factory` |
| First Finder ingest | `ingest/npm` with attestation validation or recorded absence, metadata capture, and availability as eligibility, authoring `IIngestSourceAdapter` and creating the crate, then `ingest/factory`; `workflows/first_finder` with the foreground serve, the background bootstrap job, the state-locked registration race, escrow custody of the master scalar, seeding, and the finder's grant of the asset's first entitlement to itself; `workflows/grant` | plaintext CAS and package host; the swarm tickets; the registry contracts; the chain and submission families; identity and custody; `kem/factory`, `envelope/factory`, and `proof/factory`; `workflows/sidecar/build` |
| Credential delivery and per-attempt authorization | Envelope-key registration on first run; `workflows/request`, the batched grant request job and its pickup from chain events; `workflows/prefetch` with pinning, seeding, and set matching; `workflows/acquire` with mint and grant delivery, holders fulfilling requests for absent requesters, and sale delivery; credential load and recovery; `workflows/attempt` with `τ_soft` and `τ_wallet`; `workflows/decrypt` with decapsulation through the KEM factory, unwrap through `workflows/sidecar/unwrap`, and decryption through the cipher factory; `workflows/interval_end`; `workflows/gate` | the chain and submission families; identity and custody; `kem/factory`, `envelope/factory`, and `proof/factory`; payload cipher and sidecar layer; First Finder ingest; the submission factory's relayer concrete once the relayer exists, its self-funded concrete on Base Sepolia until then |
| Demonstrable milestone | No new subsystem: a second identity on a second machine profile installs a pinned closure the First Finder ingested, at the registry's speed with requests registered and ciphertext prefetched, then with the registry unavailable, through the packaged daemon and package host over the swarm, on a grant fulfilled while it was offline; the north star, first-run independence, and install wall-clock measured on the dogfood population | credential delivery and per-attempt authorization; plaintext CAS and package host; First Finder ingest; the swarm tickets |

### Onboarding shells and services, as milestones

| Milestone by role | Contents | Depends on |
| --- | --- | --- |
| Installation coordinator | `workflows/install`, the durable install plan with platform detection, repair, update, uninstall, the health probe, the consent items, and the first-run cost disclosure; `lifecycle/systemd`, authoring the service-lifecycle interface, `lifecycle/launchd`, `lifecycle/windows_service`, `lifecycle/scheduled_task`, and `lifecycle/factory`; `artifact-verifier/sigstore`, authoring the signed-artifact verification interface and creating the crate, `artifact-verifier/authenticode`, `artifact-verifier/apple_notarization`, and `artifact-verifier/factory`; `redirect/npm`, authoring the detect, redirect, back-up, and restore interface and creating the crate, then `redirect/factory`; the installer binary | durable jobs and configuration; identity and custody |
| Visual Studio Code extension and npm bootstrap | Thin shells over the coordinator | installation coordinator |
| Desktop application and CLI | Tauri and Rust control surfaces | installation coordinator |
| Relayer or paymaster | The relayer binary with the chosen provider's bundler concrete, authoring the bundler-provider interface, and `relayer/bundler/factory`; free-path sponsorship under a global per-window budget and maximum liability with per-identity limits as one layer; the sponsorship mechanism for the identity contract account chosen here under LC-13, with `submission/paymaster` and `submission/relayer` authored as the chosen mechanism requires and `submission/factory` revised in place to carry their branches; sponsored device admission and revocation; cost and budget reporting | the chain and submission families, so it may start before the daemon grouping closes |
| Explicit publisher path | `workflows/publish` with the publisher-derived lineage, swarm-native publication of the dependency closure, and the issuance policy service; `publisher-proof/provenance`, authoring `IPublisherProofAdapter` and creating the crate, `publisher-proof/maintainer_oauth`, and `publisher-proof/factory` | First Finder ingest; identity and custody |
| Claim verifier and escrow claim | The claim verifier binary with its revocable key; `claim-verifier/attestor`, authoring `IClaimVerifierAdapter` and creating the crate, then `claim-verifier/factory`; `workflows/claim`; the claim set established from upstream metadata at verification; claim-set vouchers; the claimant parameter set; optional handover; sidecar addition; voluntary migration | registry contracts; explicit publisher path; credential delivery |
| Project seed host and site | Persistent first seeder of the core closure holding an entitlement and credential for every asset it seeds; static site; WebAssembly demonstration | `seed-host/factory`; explicit publisher path; payload cipher and sidecar layer, for the sample deployment |
| Observability | RO-03 metrics, RO-04 correlation, RO-05 health, RO-06 attempt latency and install wall-clock reported against the values in force; `telemetry/opentelemetry`, with `telemetry/factory` revised in place to carry its branch | every component that emits |
| Demonstration harness | Controlled participants, wallets, chain state, failures, and the deliberately incompatible adapter declarations the incompatibility scenario rejects, refused by the factories and by `workflows/compose` | every component under test |

### Acceptance and release, as objectives

| Objective by role | Contents | Depends on |
| --- | --- | --- |
| Transaction flow proof | Priced primary issuance and secondary transfer through the packaged applications on the Base mainnet pilot | explicit publisher path; credential delivery; relayer; external review dispositioned |
| Acceptance scenarios | Every acceptance scenario on clean machines through packaged applications | every milestone |
| External cryptographic review closed | Findings dispositioned before any priced deployment | harness report |
| Legal prerequisites | License text, regulatory review, export confirmation | none technical |
| Dogfood baseline and release | ChainTorrent published swarm-natively; baseline metrics recorded; release evidence complete | acceptance scenarios; review; legal |

## Integration Points

Where one component's output becomes another's input across a boundary that an integration test must cross, and the milestone whose integration test crosses it.

| Integration point | Producer | Consumer | Boundary crossed | Crossed in |
| --- | --- | --- | --- | --- |
| Factory admission | each family's concretes, through their declarations | the factory of every downstream family, and `workflows/compose` | A concrete admitted only when its declaration satisfies every downstream requirement; refused at construction with no side effect | The harness, for the cryptographic families; the daemon skeleton, for the whole composition |
| Verifier parity | `proof/schnorr_fs` | `contracts/evm/DeliveryVerifier` through `harness-crypto/verifier/evm` | Rust to chain; bit-for-bit acceptance on every vector | Delivery proof and Solidity verifier |
| Precompile encoding | the pairing factory's declared encodings, produced by every pairing concrete | `contracts/evm/PairingLib` | Rust encoding to precompile input | Delivery proof and Solidity verifier |
| Piece-group key | the KEM factory's decapsulate and `workflows/sidecar/unwrap` | the cipher factory | The decapsulation-to-cipher seam preserved for a future multi-key suite | Payload cipher and sidecar layer |
| Sidecar root | the KEM factory's encapsulate and `workflows/sidecar/build` | commitments and hash-card, through the hashing factory | Per live set, capsule and wrapped key per group committed by that set's Bao root; each sidecar its own object | Payload cipher and sidecar layer |
| Discovery engine | the transport factory's engine capability | `discovery/dht`, `discovery/pex`, `discovery/tracker` | Library-backed sources reach the engine through the generic surface, never through the transport concrete | Swarm transport and seed host |
| Store | the storage factory | `workflows/jobs`, `workflows/config`, the CAS index, the ciphertext index, the metadata store, `telemetry/local_metrics` | Tables, transactions, and checkpoints with no store library named by a consumer | Swarm transport and seed host, for the ciphertext index; the daemon skeleton, for the rest |
| Envelope in settlement | the envelope factory's wrap and the proof factory's provers | registry and entitlement contracts, through the submission factory | Calldata and event; digest stored | Registry and entitlement contracts |
| Signed intent | `signature/secp256k1`, through the signature factory | registry and entitlement contracts, through the submission factory | EIP-712 intent to ECDSA or ERC-1271 verification | Registry and entitlement contracts |
| Attestation verification on chain | the source's attestation, carried by `ingest/npm` | `IAttestationVerifier` bound per source by the adapter registry | The registration refused unless the source's verifier accepts, absence admitted and recorded | Registry and entitlement contracts |
| Attempt context | domain model | `chain/quorum_view` over `chain/base`, and credential delivery | View call at the declared tier from a quorum of configured nodes | Chain and submission families |
| Resolution order | resolution orchestrator | the CAS, seed-host, transport, and ingest factories | Local, encrypted, swarm, upstream, hedged | Plaintext CAS, resolution orchestrator, and package host |
| Package-manager protocol | `package-host/npm` | npm | HTTP registry protocol; resolution stays in npm; canonical tarball URLs so lockfiles stay portable | Plaintext CAS, resolution orchestrator, and package host |
| Persistent credential | the envelope and custody factories | credential delivery | Stored only under `IKeyCustodyAdapter` | Identity and custody |
| Device admission | identity manager on a root | identity contract account; the attempt engine's wallet-control assertion | Signer admitted and revoked on chain; admission read in the same state view as authorization | Identity and custody |
| Grant request to holder | request job, through the submission factory | registry request record; any holder's grant service | Batched sponsored request; holder reads open requests and fulfils them for absent requesters | Credential delivery and per-attempt authorization |
| Grant pickup from events | registry mint and grant events, through `chain/base` | request job and credential engine | Envelope recovered from chain history on the next run; set matched against held deployments | Credential delivery and per-attempt authorization |
| Installer to daemon | installation coordinator | daemon, package host, seed host, through the lifecycle and IPC factories | Service lifecycle and IPC | Installation coordinator |
| Artifact authentication | the release's signatures | the artifact-verifier factory | Every artifact and update verified before execution, under each scheme's concrete | Installation coordinator |
| Shells to coordinator | extension, npm bootstrap, desktop, CLI | installation coordinator | One coordinator, identical postcondition | Extension and npm bootstrap; desktop and CLI |
| Relayer to contracts | relayer, through its bundler factory | identity binding, requests, mint | Sponsored transactions under policy | Relayer or paymaster |
| Verifier voucher to contract | `claim-verifier/attestor` | escrow claim contract surface | Voucher bound to claimant, set, contract, chain, nonce, expiry | Claim verifier and escrow claim |
| Site demonstration | WebAssembly build of the client crates | browser | Same crates as the client; no service on the read path | Project seed host and site |

## Conflict Flags

- **Grouping membership versus dependency order.** The hashing, signature, and swarm tickets sit in the protocol core grouping and depend on no contract sprint, so they run in parallel with the harness and before the contracts.
- **Every factory follows its concretes.** A family's generic interface, declaration, and mock are authored in its first ticket, so the declarations the resolver reads exist from the family's first ticket; the factory function consumes the concretes it constructs and is authored once after every concrete of the family in this workplan; and no consumer ticket depends on a concrete. The chain family's forms are constructed by a family-owned function, `chain/create_chain_forms`, not by the factory, because the factory module holds one function and that function constructs the chain concrete; the function is authored in the harness after the forms concrete, and the factory is authored once in the chain and submission sprint after the Base concrete.
- **A concrete a later milestone adds revises its factory's ticket in place.** `discovery/seeder_map` in the chain and submission sprint, `submission/paymaster` and `submission/relayer` at the relayer, and `telemetry/opentelemetry` at observability each add a branch by revising the family's factory ticket, which remains one ticket, per the workplan-structure rule that an existing node is copied and revised rather than followed by a second node editing the same file; the lifecycle family's lock precedes its concretes and factory, which arrive together at the installation coordinator.
- **The harness needs a verifier client before the chain family exists.** The daemon never submits a proof to the verifier, since the contracts call it inside mint, grant, and delivery, so the verifier client is the harness's own `verifier` family with an `evm` concrete, and no chain concrete carries one.
- **The chain family's form interface precedes its concretes, and its forms concrete precedes its chain concrete by a sprint.** The proof family's transcript descriptions are generic over the chain family's identity, entitlement, interval, and chain-identifier forms, so `proof/mint_statement`, their first consumer, authors the family's form interface and the forms' mock in `adapters/chain`'s factory module and creates that crate with that interface alone. The generator, verifier client, vectors, and measurement need forms the deployed verifier reproduces and may name no concrete, so `chain/evm_forms`, the family's first concrete, supplies the EVM suite's forms and authors the forms portion of the declaration, and `chain/create_chain_forms`, a family-owned function module, follows it, admitting a forms concrete by the identifier the suite requires and handing it to a consumer generic over the form interface, since the factory module holds one function and that function constructs the chain concrete. The forms are the suite's, not the network's: the EVM suite is one suite shared by every EVM chain concrete, Ethereum mainnet being a later adapter of the same suite. `chain/base`, the first consumer of the settlement and entitlement-state operations, authors those and the remainder of the declaration in the chain and submission sprint, binds the forms the function resolves, and `chain/factory` is authored then, once, admitting a chain concrete only when its declaration lists the forms identifier the resolved forms carry.
- **The harness composes through configuration.** Nothing in the harness names a path, a curve, a concrete, a form, a version, or a network profile. `harness-crypto/config` carries every such value as a shipped default, overridable from a configuration file and from command-line flags, and `harness-crypto/run` and its consumers pass each as params through the factory it belongs to; the generator writes under the destination its params carry, an app module in the adapter role reached only through its factory, and the statements' context values arrive as payload from the configured profile.
- **The generate concrete is an adapter over functions it owns.** Rendering, the constants, and each kind of vector are functions beneath `harness-crypto/generate/evm`, each its own ticket ahead of the adapter, as `proof/schnorr_fs/challenge` is beneath `proof/schnorr_fs`, because one adapter file holding all of it exceeds the size at which a file is decomposed. `harness-crypto/generate/evm/render` owns the entry type the constants function returns, the record-declaration type, the validated name types, and the kind-to-type table; each vector function returns typed records with their canonical descriptions and member names; the adapter alone encodes records into lines, renders the libraries, and writes files.
- **The configuration is a composing function over functions it owns.** Layering the sources, admitting the family selections, admitting the network profile and statements, admitting the generate settings, and admitting the output destinations are functions beneath `harness-crypto/config`, each its own ticket ahead of it, because one file holding all of it exceeds the size at which a file is decomposed. `harness-crypto/config/layers` alone parses the command line, reads files, and names `clap`, `toml`, and `serde`, and owns the document every admission function reads; each admission function owns the admitted types it returns and maps the document's text values to the types their owning families declare; `harness-crypto/config` composes them, owns the typed configuration, and carries the shipped defaults. The selection types the configuration holds and the run hands each curve and each benchmarked concrete a copy of carry `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq`, added in their families' factory interfaces by `harness-crypto/config/admit_selections`, the first file that requires them.
- **Vectors are committed files, one encoded record per line.** Solidity holds no constant array, and a counted kind must be iterable by a Foundry test whatever count the configuration carries, so each vector kind is a file the adapter writes, each line the encoding family's encoding of one record as `0x`-prefixed hex; a test reads it with `vm.readLine`, converts each line with `vm.parseBytes`, and decodes it with the kind's function from the generated vector-record library, which also carries the file's path. One record per line needs no list kind in the encoding contract. The decode function decodes over the members' types, never over the struct, since a struct with a dynamic member encodes behind an offset and each line holds the parameter encoding of the fields.
- **Vector kinds, counts, and names are declared and configured.** The generate concrete's declaration names the kinds it emits and each kind's sizing: a drawn kind holds the count the configuration carries for it, an enumerated kind holds exactly what it enumerates, and a kind over configured inputs holds one record per input; every file, library, struct, function, and constant name is the configuration's, with shipped defaults; the generate factory admits a configuration only when its per-kind settings name exactly the declared kinds, with a count for each drawn kind and none for any other. A further target's generate concrete declares its own kinds, and no consumer holds a count, a kind list, or a name.
- **The harness run alone calls the factories.** `harness-crypto/run` and the consumers it owns nest each factory's consumer inside the previous one and hand the adapters and their declarations to the generate and verifier factories, which construct their concretes over them, and to the vectors and measurement modules; the run attempts each combination a factory must refuse and hands the refusal to `harness-crypto/vectors`; no other harness module calls a factory, and each reads a tag, a form, or a version from what it is handed.
- **The run is a composing function over consumers it owns.** A family's factory hands the concrete it constructs to a consumer rather than returning it, because the concrete's type carries the borrowed adapters' types, so composing the families is a chain of consumers, each implementing one factory's consumer trait and calling the next factory with the next consumer: pairing, then hash-to-scalar and encoding, chain forms, KEM, key agreement, delivery proof, and generate. One type per file admits one consumer per file, so each consumer is its own ticket beneath `harness-crypto/run`, authored innermost first since each outer consumer constructs the one inside it, as `harness-crypto/benchmark` is the pairing family's consumer in its own file. `harness-crypto/run` owns the randomness source's construction, the benchmark's calls, each curve's call to the pairing factory, and the outcome every call returns; the verifier, vectors, measurement, and report tickets revise `harness-crypto/run/delivery_proof_consumer` and `harness-crypto/run` in place, never the binary.
- **The binary holds only what a binary's `main` keeps.** A binary's `main` cannot be called by a test, so, after the Rust book's separation of concerns for binary projects, the logic lives in the library and `apps/harness-crypto/src/main.rs` calls `harness-crypto/config`, `harness-crypto/run`, and `harness-crypto/output` in turn and sets the exit status from their results; it declares no type and holds no test, so it carries no element files and is checked by reading.
- **The harness's output is configured, as its inputs are.** The run's outcome, a success with the benchmark's measurements and the files written or the first refusal, is written by `harness-crypto/output` to each destination the configuration names, standard output, a file at a configured path, or both, so a caller chooses where a run reports by the same shipped default, file, and flag that set every other value; the shipped default is standard output. A configuration refusal arrives before any destination is admitted, so the binary writes it to standard error, and every refusal ends the process with a failure status.
- **The benchmark's concretes are configured.** The benchmark measures every pairing concrete so the default per curve can be set from its measurements, and the run names no concrete, so the configuration carries the concretes the benchmark measures, each with the target-group encoding identifier its curve requires, every pairing concrete in the shipped defaults; the run hands each to the pairing factory with a `PairingBenchmark` as its consumer, and the outcome carries each concrete's measurements to the output.
- **The challenge vectors are standalone transcripts.** The proof family's challenge function is private to its crate, and a proof in the second-group form carries no first messages, so `harness-crypto/generate/evm/challenge_vectors` builds its own transcripts, encodes each through its public description, and maps the encoding under the challenge tag the declaration carries; the proofs' own challenges travel in the proof vectors.
- **An identity encodes as `bytes20`.** The encoding concrete encodes the fixed twenty-byte kind as a right-padded `bytes20`, so the constants render it as `bytes20` and `contracts/evm/DeliveryVerifier` encodes every identity field of a transcript as `bytes20`; an `address` is left-padded and yields another challenge.
- **A generated library's compiler version and license identifier are configured.** A Solidity source file opens with a license identifier and a compiler version pragma, neither of which is a library name or an entry, so `harness-crypto/config` ships a default for each, `harness-crypto/run/delivery_proof_consumer` passes them through the generate factory's params, the generate concrete holds them from construction, and `harness-crypto/generate/evm/render` takes them as params and holds neither as a literal.
- **The curve directory is read, not chosen.** The adapter names each generated directory from the curve the resolved pairing's declaration carries and selects no curve.
- **The harness's parameter-set digest and identity bytes are configured.** No harness ticket computes a parameter-set digest or describes an entitlement identifier, and the delivery verifier takes the identity element as an input, so both arrive as configured values; the registry and entitlement contracts sprint binds them to the record. The entitlement contract a harness transcript names is the network profile's configured address whether or not a suite is deployed there when the generator runs, since the harness exercises the delivery verifier apart from the registry; in operation the registry and entitlement contracts supply every statement field from their own records, the stored envelope digest, the registered keys, the identity element, and the interval counter among them, and refuse a field the record contradicts, which LC-05 and AS-13 prove at the registry and entitlement contracts milestone and again through the daemon at credential delivery.
- **The generated directory is committed.** The Solidity continuous-integration definition runs only `forge build`, `forge fmt --check`, and `forge test`, so the constants library, the vector-record library, and the vector files `contracts/evm/PairingLib` and `contracts/evm/DeliveryVerifier` read exist in the repository when it runs; `contracts/evm/foundry.toml` grants `fs_permissions` read access to the configured generated directory's shipped default and to nothing else, `ffi` stays disabled, and `via_ir = true` compiles every source through the IR pipeline, since a proof record's decode function destructures more values than the legacy pipeline's sixteen reachable stack slots hold; the harness binary, run once from its shipped defaults, writes them through the generator before `contracts/evm/PairingLib`, and the commit that closes the milestone carries them.
- **The wrap precedes the cipher.** The harness's cross-set vector needs the wrap and unwrap, so `workflows/sidecar/wrap` and `workflows/sidecar/unwrap` are harness tickets and create the workflows crate; `workflows/sidecar/build` and `workflows/sidecar/validate` wait for the cipher in the protocol core.
- **The benchmark follows the pairing concretes.** It constructs each through the pairing factory and names no library; what it records is the default concrete per curve, which the factory constructs when a composition names a curve alone, and every concrete remains beneath the factory whether or not it is the default.
- **The library-backed discovery sources share the transport's engine.** They reach it through the engine capability the transport factory's generic surface exposes, so no discovery concrete depends on `transport/rqbit`.
- **The demonstrable milestone needs a grant.** Resolving against the swarm with the registry down requires a second identity to hold a credential, so the milestone sits after credential delivery and per-attempt authorization, not at the package host.
- **Escrow claim is last by dependency.** The escrow record carries no maintainer commitment and the verifier establishes the claim set at verification time, so the milestone waits only on the publisher path, the registry contracts, and credential delivery.
- **Two verifier forms, one primary.** `contracts/evm/PairingLib` exists in both forms so the harness measures both; BLS12-381 ships as the primary form on Base and BN254 is retained.
- **The delivery verifier is built in the harness and consumed by the contract sprint.** It is the first shipped contract, written to production standard, with its constants and vectors generated from the Rust reference by `harness-crypto/generate/evm`.
- **The EVM suite is one concrete.** `contracts/evm` is the on-chain half of `chain/base`; a further chain form is a further suite beside it, and within the suite the adapter registry binds the entitlement token form, the attestation verifier per source, and the identity and signature adapters.
- **Identity and custody sits in the daemon grouping but the installer needs it.** The dependency runs the right way, coordinator depends on custody, and the epic is placed early in the grouping.
- **The identity is a contract account; its sponsorship mechanism is chosen at the relayer milestone.** The contract sprint builds the account with its device registry and takes every identity-bound mutation as a signed intent under LC-13, so `submission/paymaster`, `submission/relayer`, and `submission/self_funded` all remain open behind one factory.

## Dependencies

The dependency graph across the groupings. Edges point from producer to consumer. Identifiers are descriptive; nothing is numbered.

```mermaid
flowchart TB
    subgraph foundation["Foundation"]
        direction TB
        f_ws["workspace/cargo"]
        f_sec["domain/secret"]
        f_rand_os["random/os"]
        f_rand["random/factory"]
        f_ci["workspace/ci"]
        f_ws --> f_sec
        f_ws --> f_rand_os
        f_rand_os --> f_rand
        f_ws --> f_ci
    end

    subgraph harness["Cryptographic validation harness"]
        direction TB
        pair_bn_ark["pairing/bn254_arkworks"]
        pair_bn_h2c["pairing/bn254_halo2curves"]
        pair_bls_ark["pairing/bls12_381_arkworks"]
        pair_bls_h2c["pairing/bls12_381_halo2curves"]
        pair["pairing/factory"]
        bench["harness-crypto/benchmark"]
        dom_asset["domain/asset_identity"]
        dom_dep["domain/deployment_identity"]
        dom_suite["domain/suite_identifier"]
        dom_pset["domain/parameter_set_identifier"]
        dom_group["domain/group_index"]
        dom_geom["domain/piece_geometry"]
        dom_ctx["domain/derivation_context"]
        dom_hash["domain/asset_identity_hash"]
        enc_desc["encoding/derivation_context"]
        f_abi["encoding/abi"]
        f_enc["encoding/factory"]
        kdf_b3["kdf/blake3_keyed"]
        kdf["kdf/factory"]
        h2s_k["hash-to-scalar/keccak256"]
        h2s["hash-to-scalar/factory"]
        kem_bb1["kem/bb1_depth_one"]
        kem["kem/factory"]
        env_ps["envelope/possession_statement"]
        env_eg["envelope/pairing_elgamal"]
        env["envelope/factory"]
        proof_ms["proof/mint_statement"]
        proof_ts["proof/transfer_statement"]
        proof_ch["proof/schnorr_fs/challenge"]
        proof_sfs["proof/schnorr_fs"]
        proof["proof/factory"]
        sc_wrap["workflows/sidecar/wrap"]
        sc_unwrap["workflows/sidecar/unwrap"]
        sol_pair["contracts/evm/PairingLib"]
        gen_render["harness-crypto/generate/evm/render"]
        gen_const["harness-crypto/generate/evm/constants"]
        gen_group["harness-crypto/generate/evm/group_vectors"]
        gen_reject["harness-crypto/generate/evm/rejection_vectors"]
        gen_map["harness-crypto/generate/evm/mapping_vectors"]
        gen_poss["harness-crypto/generate/evm/possession_vectors"]
        gen_chal["harness-crypto/generate/evm/challenge_vectors"]
        gen_proof["harness-crypto/generate/evm/proof_vectors"]
        gen_evm["harness-crypto/generate/evm"]
        gen["harness-crypto/generate/factory"]
        sol_verify["contracts/evm/DeliveryVerifier"]
        sol_deploy["contracts/evm/deploy"]
        hv_evm["harness-crypto/verifier/evm"]
        hv["harness-crypto/verifier/factory"]
        h_vectors["harness-crypto/vectors"]
        h_measure["harness-crypto/measure"]
        h_report["harness-crypto/report"]
        chain_forms["chain/evm_forms"]
        chain_forms_fn["chain/create_chain_forms"]
        cfg_layers["harness-crypto/config/layers"]
        cfg_sel["harness-crypto/config/admit_selections"]
        cfg_stmt["harness-crypto/config/admit_statements"]
        cfg_gen["harness-crypto/config/admit_generate"]
        cfg_out["harness-crypto/config/admit_output"]
        h_cfg["harness-crypto/config"]
        run_gen["harness-crypto/run/generate_consumer"]
        run_proof["harness-crypto/run/delivery_proof_consumer"]
        run_ka["harness-crypto/run/key_agreement_consumer"]
        run_kem["harness-crypto/run/kem_consumer"]
        run_forms["harness-crypto/run/chain_forms_consumer"]
        run_enc["harness-crypto/run/encoding_consumer"]
        run_pair["harness-crypto/run/pairing_consumer"]
        h_run["harness-crypto/run"]
        h_out["harness-crypto/output"]
        h_main["harness-crypto/main"]

        pair_bn_ark --> pair_bn_h2c
        pair_bn_ark --> pair_bls_ark
        pair_bn_ark --> pair_bls_h2c
        pair_bn_ark --> pair
        pair_bn_h2c --> pair
        pair_bls_ark --> pair
        pair_bls_h2c --> pair
        pair --> bench
        dom_asset --> dom_ctx
        dom_dep --> dom_ctx
        dom_suite --> dom_ctx
        dom_pset --> dom_ctx
        dom_group --> dom_ctx
        dom_geom --> dom_ctx
        dom_ctx --> enc_desc
        enc_desc --> f_abi
        f_abi --> f_enc
        dom_ctx --> kdf_b3
        kdf_b3 --> kdf
        pair --> h2s_k
        h2s_k --> h2s
        pair --> kem_bb1
        h2s --> kem_bb1
        kem_bb1 --> kem
        f_abi --> env_ps
        env_ps --> env_eg
        pair --> env_eg
        h2s --> env_eg
        kem --> env_eg
        env_eg --> env
        f_abi --> proof_ms
        dom_hash --> proof_ms
        dom_suite --> proof_ms
        dom_pset --> proof_ms
        proof_ms --> proof_ts
        proof_ts --> proof_ch
        h2s --> proof_ch
        proof_ch --> proof_sfs
        pair --> proof_sfs
        env --> proof_sfs
        kem --> proof_sfs
        proof_sfs --> proof
        kdf --> sc_wrap
        kem --> sc_wrap
        sc_wrap --> sc_unwrap
        proof_ms --> chain_forms
        chain_forms --> chain_forms_fn
        bench --> gen_render
        gen_render --> gen_const
        gen_render --> gen_group
        gen_render --> gen_reject
        gen_render --> gen_map
        gen_render --> gen_poss
        gen_render --> gen_chal
        gen_render --> gen_proof
        pair --> gen_const
        kem --> gen_const
        env --> gen_const
        proof --> gen_const
        chain_forms_fn --> gen_const
        pair --> gen_group
        pair --> gen_reject
        pair --> gen_map
        h2s --> gen_map
        kem --> gen_map
        env --> gen_map
        proof --> gen_map
        pair --> gen_poss
        env --> gen_poss
        pair --> gen_chal
        proof --> gen_chal
        h2s --> gen_chal
        chain_forms_fn --> gen_chal
        pair --> gen_proof
        kem --> gen_proof
        env --> gen_proof
        proof --> gen_proof
        chain_forms_fn --> gen_proof
        gen_const --> gen_evm
        gen_group --> gen_evm
        gen_reject --> gen_evm
        gen_map --> gen_evm
        gen_poss --> gen_evm
        gen_chal --> gen_evm
        gen_proof --> gen_evm
        pair --> gen_evm
        kem --> gen_evm
        env --> gen_evm
        proof --> gen_evm
        h2s --> gen_evm
        chain_forms_fn --> gen_evm
        gen_evm --> gen
        cfg_layers --> cfg_sel
        cfg_layers --> cfg_stmt
        cfg_layers --> cfg_gen
        pair --> cfg_sel
        h2s --> cfg_sel
        chain_forms_fn --> cfg_sel
        kem --> cfg_sel
        env --> cfg_sel
        proof --> cfg_sel
        dom_suite --> cfg_stmt
        dom_hash --> cfg_stmt
        dom_pset --> cfg_stmt
        proof --> cfg_stmt
        gen_evm --> cfg_stmt
        gen_render --> cfg_gen
        gen --> cfg_gen
        cfg_sel --> h_cfg
        cfg_stmt --> h_cfg
        cfg_gen --> h_cfg
        cfg_layers --> cfg_out
        cfg_out --> h_cfg
        bench --> h_cfg
        gen --> run_gen
        h_cfg --> run_gen
        run_gen --> run_proof
        gen --> run_proof
        proof --> run_proof
        run_proof --> run_ka
        proof --> run_ka
        env --> run_ka
        run_ka --> run_kem
        env --> run_kem
        kem --> run_kem
        run_kem --> run_forms
        kem --> run_forms
        chain_forms_fn --> run_forms
        run_forms --> run_enc
        chain_forms_fn --> run_enc
        run_enc --> run_pair
        h2s --> run_pair
        pair --> run_pair
        run_pair --> h_run
        bench --> h_run
        h_cfg --> h_run
        pair --> h_run
        h_run --> h_out
        cfg_out --> h_out
        h_cfg --> h_main
        h_run --> h_main
        h_out --> h_main
        h_main --> sol_pair
        sol_pair --> sol_verify
        h_main --> sol_verify
        sol_verify --> sol_deploy
        sol_deploy --> hv_evm
        proof --> hv_evm
        chain_forms_fn --> hv_evm
        hv_evm --> hv
        kem --> h_vectors
        env --> h_vectors
        proof --> h_vectors
        chain_forms_fn --> h_vectors
        sc_unwrap --> h_vectors
        h_vectors --> h_measure
        hv --> h_measure
        h_measure --> h_report
    end

    subgraph core["Protocol core and contract suite"]
        direction TB
        hashing_b3["hashing/blake3_bao"]
        hashing["hashing/factory"]
        sig_ed["signature/ed25519"]
        sig_k1["signature/secp256k1"]
        sig["signature/factory"]
        tr_rq["transport/rqbit"]
        tr["transport/factory"]
        disc_local["discovery/local"]
        disc_dht["discovery/dht"]
        disc_pex["discovery/pex"]
        disc_trk["discovery/tracker"]
        disc_agg["discovery/aggregate"]
        disc["discovery/factory"]
        disc_map["discovery/seeder_map"]
        st_redb["storage/redb"]
        st["storage/factory"]
        sh_owned["seed-host/owned"]
        sh_rqbit["seed-host/rqbit"]
        sh["seed-host/factory"]
        s_domain["Sprint: domain model"]
        s_cipher["Sprint: payload cipher and sidecar layer"]
        s_registry["Sprint: registry and entitlement contracts, the EVM suite"]
        s_chain["Sprint: chain and submission families"]
        s_adreg["Sprint: adapter registry and factory, on chain"]

        hashing_b3 --> hashing
        sig_ed --> sig_k1
        sig_ed --> sig
        sig_k1 --> sig
        hashing --> tr_rq
        tr_rq --> tr
        disc_local --> disc_dht
        disc_local --> disc_pex
        disc_local --> disc_trk
        disc_local --> disc_agg
        tr --> disc_dht
        tr --> disc_pex
        tr --> disc_trk
        disc_local --> disc
        disc_dht --> disc
        disc_pex --> disc
        disc_trk --> disc
        disc_agg --> disc
        st_redb --> st
        hashing --> sh_owned
        tr --> sh_owned
        st --> sh_owned
        sh_owned --> sh_rqbit
        hashing --> sh_rqbit
        sh_owned --> sh
        sh_rqbit --> sh
        s_domain --> s_cipher
        hashing --> s_cipher
        s_domain --> s_registry
        s_registry --> s_chain
        sig --> s_chain
        s_registry --> s_adreg
        s_chain --> disc_map
        disc_local --> disc_map
        disc_map -.->|"revises in place"| disc
    end

    subgraph daemon["Local daemon and package serving"]
        direction TB
        e_jobs["Epic: durable jobs and configuration"]
        e_identity["Epic: identity and custody"]
        e_cas["Epic: plaintext CAS, resolution orchestrator, and package host"]
        e_ff["Epic: First Finder ingest"]
        e_deliver["Epic: credential delivery and per-attempt authorization"]
        e_demo["Epic: demonstrable milestone"]

        e_jobs --> e_cas
        e_jobs --> e_identity
        e_cas --> e_ff
        e_identity --> e_ff
        e_identity --> e_deliver
        e_ff --> e_deliver
        e_deliver --> e_demo
        e_cas --> e_demo
        e_ff --> e_demo
    end

    subgraph shells["Onboarding shells and services"]
        direction TB
        m_installer["Milestone: installation coordinator"]
        m_ext["Milestone: extension and npm bootstrap"]
        m_desktop["Milestone: desktop application and CLI"]
        m_relayer["Milestone: relayer or paymaster"]
        m_publisher["Milestone: explicit publisher path"]
        m_claim["Milestone: claim verifier and escrow claim"]
        m_seedhost["Milestone: project seed host and site"]
        m_obs["Milestone: observability"]
        m_demo["Milestone: demonstration harness"]

        m_installer --> m_ext
        m_installer --> m_desktop
        m_publisher --> m_claim
        m_publisher --> m_seedhost
    end

    subgraph release["Acceptance and release"]
        direction TB
        o_txflow["Objective: transaction flow proof"]
        o_accept["Objective: acceptance scenarios"]
        o_review["Objective: external cryptographic review closed"]
        o_legal["Objective: legal prerequisites"]
        o_release["Objective: dogfood baseline and release"]

        o_txflow --> o_accept
        o_accept --> o_release
        o_review --> o_release
        o_legal --> o_release
    end

    f_ws --> pair_bn_ark
    f_rand --> pair_bn_ark
    f_ci --> sol_pair
    f_sec --> dom_asset
    f_sec --> dom_dep
    f_sec --> dom_suite
    f_sec --> dom_pset
    f_sec --> dom_group
    f_sec --> dom_geom
    f_sec --> dom_hash
    f_enc --> kdf_b3
    f_enc --> h2s_k
    f_enc --> kem_bb1
    f_enc --> env_eg
    f_enc --> proof_ch
    f_enc --> gen_evm
    f_enc --> gen_chal
    f_enc --> gen_group
    f_enc --> gen_reject
    f_enc --> gen_map
    f_enc --> gen_poss
    f_enc --> gen_proof
    f_rand --> gen_group
    f_rand --> gen_map
    f_rand --> gen_poss
    f_rand --> gen_chal
    f_rand --> gen_proof
    f_enc --> hv_evm
    f_enc --> hashing_b3
    f_enc --> sig_ed
    f_enc --> disc_local
    f_enc --> st_redb
    f_enc --> sc_wrap
    f_enc --> chain_forms
    f_ws --> cfg_layers
    f_rand --> cfg_sel
    f_enc --> cfg_sel
    f_enc --> cfg_stmt
    f_rand --> gen_evm
    f_rand --> h_run
    f_enc --> run_enc
    f_enc --> run_pair
    chain_forms_fn --> s_chain
    f_rand --> kem_bb1
    f_rand --> env_eg
    f_rand --> proof_sfs
    f_rand --> sig_ed
    f_sec --> s_domain
    f_sec --> kem_bb1
    f_sec --> env_eg
    st --> e_jobs

    kem --> s_domain
    h_report --> s_registry
    sol_verify --> s_registry
    kem --> s_cipher
    sc_wrap --> s_cipher

    s_domain --> e_jobs
    hashing --> e_cas
    st --> e_cas
    sh --> e_ff
    tr --> e_ff
    s_registry --> e_ff
    s_chain --> e_ff
    kem --> e_ff
    proof --> e_ff
    env --> e_ff
    s_cipher --> e_ff
    s_chain --> e_deliver
    s_cipher --> e_deliver
    proof --> e_deliver
    env --> e_deliver
    kem --> e_deliver
    sig --> e_identity
    s_chain --> e_identity
    env --> e_identity
    kdf --> e_identity
    sh --> e_demo

    e_jobs --> m_installer
    e_identity --> m_installer
    s_chain --> m_relayer
    e_ff --> m_publisher
    e_identity --> m_publisher
    s_registry --> m_claim
    e_deliver --> m_claim
    sh --> m_seedhost
    s_cipher --> m_seedhost
    e_cas --> m_obs
    e_deliver --> m_obs
    e_cas --> m_demo

    m_publisher --> o_txflow
    e_deliver --> o_txflow
    m_relayer --> o_txflow
    o_review --> o_txflow
    m_ext --> o_accept
    m_desktop --> o_accept
    m_claim --> o_accept
    m_seedhost --> o_accept
    m_obs --> o_accept
    m_demo --> o_accept
    h_report --> o_review
```

**Reading the graph.** Everything in the foundation and harness subgraphs, and every ticket-named node in the protocol core, is one source file, except the configuration tickets `workspace/cargo` and `workspace/ci`, each of which carries its configuration files together, and the deployment script. A factory node is the family's `factory` module; a concrete node is one adapter beneath it, private to the crate; a family's first ticket, its first concrete or a family-owned function preceding it, carries the family's interface and creates the crate, and the factory follows every concrete. Every edge out of a family runs from its factory, never from a concrete: the registry sprint consumes the harness's `contracts/evm/DeliveryVerifier`, and the First Finder, credential delivery, and identity epics consume the KEM, envelope, proof, and KDF factories, which construct the concretes the harness proved. The chain family's forms function sits in the harness beside its one concrete; the generator, verifier client, vectors, and measurement consume the function, so no harness ticket names a form set; the chain factory is authored in the chain and submission sprint after the Base concrete, and the sprint's edge from the forms function is the Base concrete binding the forms it resolves. The harness run is the grouping's composition root, consuming the configuration ticket, the benchmark, and every factory through the consumers it owns, each consumer following the one it constructs; the output follows the run, whose outcome it writes, and the binary follows the configuration, the run, and the output, which it calls in turn; the pairing library follows the binary because the committed generated directory is what running it writes. The generate concrete's functions each consume the factories whose adapters they are handed, and the vector functions consume the encoding family's contract for their records' descriptions; `harness-crypto/generate/evm/render` precedes every one of them, and the adapter follows them all. The configuration's layering function precedes its admission functions, each of which consumes the families whose types it admits, the generate settings' admission following `render` and the generate factory, whose validated names and per-kind settings it admits; the composing configuration ticket follows every admission function and the benchmark, whose iteration count it admits. The hashing, signature, and swarm tickets have no incoming edge from any contract sprint, which is the point that bytes can move before any chain exists. The relayer milestone depends only on the chain and submission families and can be built as soon as they exist. The demonstrable milestone has incoming edges from credential delivery, the package host, First Finder ingest, and the seed-host family, and nothing depends on it.

## Sequencing

`workspace/cargo` precedes everything. Within the harness the starting points are `pairing/bn254_arkworks`, which carries the pairing interface with its sampling bound, and the domain's identifier modules on `domain/secret`; the derivation context follows the identifiers, `encoding/derivation_context` carries the encoding contract and precedes `encoding/abi` and `encoding/factory`, and `kdf/blake3_keyed`, the first ticket that encodes, follows the encoding factory; every other ticket has a producer, each family's first ticket precedes its further concretes and its factory follows them all, `chain/evm_forms` follows `proof/mint_statement` and `encoding/factory` and `chain/create_chain_forms` follows it, `harness-crypto/generate/evm/render` precedes every other function the generate concrete owns, the adapter `harness-crypto/generate/evm` follows them all and `harness-crypto/generate/factory` follows it, `harness-crypto/config/layers` precedes the configuration's admission functions, `harness-crypto/config/admit_generate` follows `harness-crypto/generate/factory`, whose per-kind settings it admits, `harness-crypto/config` follows every admission function, `harness-crypto/config/admit_output` follows `harness-crypto/config/layers` and precedes `harness-crypto/config`, the run's consumers follow `harness-crypto/config` and `harness-crypto/generate/factory` innermost first, from `harness-crypto/run/generate_consumer` to `harness-crypto/run/pairing_consumer`, `harness-crypto/run` follows them and the benchmark, `harness-crypto/output` follows `harness-crypto/run`, `harness-crypto/main` follows `harness-crypto/output`, `contracts/evm/PairingLib` follows `harness-crypto/main`, whose run writes the generator's output into the committed generated directory, the tracks converge at `harness-crypto/verifier/factory` and `harness-crypto/vectors`, and the grouping closes at `harness-crypto/report`, whose node carries the integration test across the whole chain and the commit. The hashing and signature tickets start from `hashing/blake3_bao` and `signature/ed25519` on `encoding/factory`, and the swarm tickets from `transport/rqbit`, `discovery/local`, and `storage/redb` on `hashing/factory` and `encoding/factory`; each of those milestones closes at the factory its table names.

Across groupings the sequence is the [milestones](milestones.md)' order, with the parallelism the graph makes visible: the hashing, signature, and swarm tickets run beside the harness and the contract sprints from `encoding/factory` onward; the identity and custody epic begins as soon as the signature tickets and the chain and submission families exist, before the CAS epic, because the installation coordinator needs it; the relayer milestone begins on the chain and submission families; the demonstrable milestone follows credential delivery. The chain and submission sprint authors `chain/factory` once, after the Base concrete, since the harness authored the forms constructor as the family-owned module `chain/create_chain_forms` rather than in the factory.

The decay rule governs re-mapping. When the `harness-crypto/report` node commits, the remaining protocol core milestones are re-mapped from sprints to tickets using what the harness taught, the daemon grouping from epics to sprints, and so on outward. Each re-mapping is a revision of this document in place.

## Risk Mitigation

| Risk from the register | How the map addresses it |
| --- | --- |
| R-02, cost makes installs noticeably slower | The harness grouping is entirely at ticket resolution; `harness-crypto/report` records the measurements against the configured piece-group size and curve defaults and any adjustment the data supports, and every ticket that encrypts reads the piece-group size from the deployment record and waits on nothing |
| R-03, verifier flaw | `proof/schnorr_fs` and `contracts/evm/DeliveryVerifier` are separate tickets with `harness-crypto/verifier/evm` as the parity integration point and `harness-crypto/generate/evm` as the source of the contract's constants and vectors; `harness-crypto/vectors` carries the mutation and replay sets |
| R-04, execution breadth | Decaying resolution means no distant detail is authored to be rewritten; the demonstrable milestone is reachable before publishing and claims; a concrete adapter is one node, so a family's ticket count is its factory plus its concretes |
| R-09, open selections | The identity epic carries custody recovery UX; the license gates release and no ticket |
| R-12, chain properties | Every pairing concrete and both verifier forms are harness tickets beneath the pairing and proof factories, so a precompile change on Base selects the retained form at resolution rather than rebuilding; a further chain is a further `chain` concrete with its own suite under `contracts/` |
| R-13, escrow record | No commitment and no custodian; the claim milestone has no policy gate |
| R-14, adapter registry governance | The on-chain adapter registry sprint is separate from the registry contracts sprint, so its single project-held key can be replaced without touching entitlements; the same registry binds the attestation verifier per source and the entitlement token form |
| A consumer bound to a concrete | Every family crate exposes its factory alone, every consumer ticket follows the factory it names, and the harness constructs through the factories, so no ticket can name a concrete it does not own |

## Decisions

**The adapter family form.** Every family is a factory crate owning the generic interface, the capability declaration, and the types the generic surface needs; each concrete is a private module beneath the factory with its own interface implementing the generic one and owning only the types it alone produces; consumers depend on the factory's surface alone; the layout is general responsibility, then functional need, then concrete implementation. A family exists even where the MVP ships one concrete.

**Crate and path layout.** One workspace with a domain crate, a workflows crate, a crate per adapter family holding its `factory` module and its private concretes, and a crate per deployable, each created by the ticket of the first module that lives in it, which for a family is its first ticket; the workspace manifest lists members by glob; tickets are named by crate, factory, concrete, and function as the technical requirements' file tree spells them.

**Canonical encoding.** The `encoding` factory owns `IEncoderAdapter` and `IDecoderAdapter` under one versioned encoding identifier, the repo-owned encoding contract, and the declaration; the family owns one description module per encoded domain type, `encoding/<type>`, implementing the contract for that type, so the domain crate depends on nothing and the encoding crate depends on the domain crate; the family follows the first encodable domain type, the derivation context, at the head of the harness grouping, its first description authoring the contract and creating the crate; `encoding/abi` implements both through `alloy`'s sol types and is the only module naming `alloy` for encoding; one encoding for everything hashed, signed, stored, or framed over IPC, including the configuration registry's records.

**Tools and dependencies.** A configuration file is authored once, complete, by the ticket that creates it: the workspace manifest with the `librqbit` overlay and `deny.toml` with the overlay repository's source at `workspace/cargo`, the Rust continuous-integration definition at `workspace/ci`, `contracts/evm/foundry.toml`, granting `fs_permissions` read access to the configured generated directory's shipped default alone with `ffi` disabled and `via_ir = true`, and the Solidity continuous-integration definition, with `forge build`, `forge fmt --check`, and `forge test`, at `contracts/evm/PairingLib`, and the shell, fuzz, and end-to-end continuous-integration definitions each at the ticket that first needs it, as separate files. A crate's manifest and its `lib.rs` barrel accumulate one entry per module, each added by that module's ticket as its deps and provides elements. A vendor library is named only by the concrete that wraps it; `harness-crypto/benchmark` names none and reaches each pairing library through the pairing factory; `harness-crypto/config/layers` pins `clap`, `toml`, and `serde`, the one place the harness names any of them. `pairing/bn254_arkworks` pins `num-bigint` as the pairing crate's dev-dependency, at the version `Cargo.lock` already resolves through `ark-ff`, for the integer exponent its definition test computes; no tool outside the workspace takes part in the target-group proof.

**Secrets.** The secret-typed value is a `domain` ticket pinning `zeroize` as a domain crate dependency; the telemetry family carries correlation and tracing setup and no redaction layer.

**Ticket granularity for a concrete adapter.** One node: the concrete's operations are the methods of one adapter in one file, and a function the concrete owns, such as the Schnorr challenge or the EVM generate concrete's rendering, constants, and vector functions, is a node beneath it.

**Interfaces first, factories last.** A family's generic interface, declaration, and mock are authored in the ticket of its first concrete, the first source file that requires them; each further concrete and family-owned function follows; the factory function, which consumes the concretes, is authored once after every concrete of the family in this workplan and revised in place when a later milestone adds one; every consumer follows the factory, and the harness constructs its combinations through the factories. A family whose forms are consumed before its adapter exists constructs them through a family-owned function, `chain/create_chain_forms`, authored after the forms concrete, because the factory module holds one function and that function constructs the family's adapter; the factory is still authored once, after every concrete.

**The harness configuration.** A typed configuration the harness owns, `harness-crypto/config`, with a shipped default for every value the harness passes as params and overrides from a configuration file and from command-line flags, composed over a layering function and admission functions it owns, each its own ticket ahead of it; the shipped defaults are `apps/harness-crypto/harness.toml`, carried by the composing ticket, and the overrides are `--config <PATH>` files and `--set <KEY=VALUE>` assignments applied in command-line order over them; a value a later ticket first consumes is added by that ticket revising the composing ticket in place; the binary reads it, every factory call the run makes carries its values, and the output writes the run's outcome to the destinations it names, so no harness module holds a literal for a concrete, a curve, a benchmarked concrete, an identifier, a form, a version, a profile, an identity scope, a statement's context value, an identity, an iteration count, a vector kind's count, a path, a file, library, or symbol name, a compiler version, a license identifier, or an output destination.

**The harness run, its output, and its binary.** The run is a composing function over one consumer per factory it calls, each its own ticket, and the harness's composition root; the output writes the run's outcome, a success or a refusal, to each configured destination, standard output, a file, or both; the binary holds only what a binary's `main` keeps, calling the configuration, the run, and the output in turn and setting the exit status, and carries no element files because it declares no type and holds no test.

**The canonical target-group value.** The encapsulated value the KDF consumes is a target-group element's encoding, and two libraries of one curve are not assumed to compute the same element or serialize it the same way. The pairing family's declaration carries a target-group encoding identifier naming the pairing map, the generators, and the coefficient serialization; the suite requires it as an explicit hash-card field beside the forms identifier; the factory admits a concrete only when it declares the required one; and every concrete is proven by executing the identifier's definition in its own tests, which fixes the map, the generators, the tower, the exact exponent `(p^12 - 1) / r`, and the serialization: the test computes the library's Miller loop over the generators, raises it to the integer exponent, serializes it, and requires the concrete's pairing product to encode to the same bytes, and on BLS12-381 asserts the CFRG draft's published value as well; a library's own reduced pairing is never the proof, since several libraries compute a fixed multiple of the exact exponent for speed. A concrete wrapping such a library multiplies the library's output by the multiple's inverse in the scalar field, computed at construction from the multiple the library's own final exponentiation states; the multiple is a fact about that library recorded in the concrete's node, and the definition test proves it; the family's integration test then proves the two concretes of a curve equal to each other as a consequence.

**The harness's own families.** The harness's `generate` and `verifier` families, each with an `evm` concrete carrying the family's interface and a factory following it, because emitting a suite's vectors and reaching a suite's verifier are suite-specific and the daemon needs neither.

**The storage engine and the engines above it.** The `storage` factory with `storage/redb`; the job engine and the configuration registry are workflows modules over it, since nothing external remains in them.

**BitTorrent library.** `librqbit` compiled in as a soft fork through the workspace manifest's `[patch.crates-io]` overlay onto the project's overlay branch `chaintorrent-overlay` in `https://github.com/tsylvester/rqbit`, which tracks tagged upstream releases and exists before the bootstrap, with the hooks authored inside `transport/rqbit`, submitted upstream, and the commit pinned; no owned transport in the MVP; the library's discovery sources are `discovery` concretes reached through the transport factory's engine capability; the hard-fork trigger is recorded in the product requirements.

**The contract suite.** `contracts/evm` is the on-chain concrete of `chain/base`, with Foundry inside it; a further chain form is a further suite.

**The harness's delivery verifier.** The shipped contract, written to production standard from the start.

**Milestones at ticket resolution beside the harness.** Hashing, signatures, and the swarm, because they depend on nothing the harness measures; they start once `encoding/factory` closes, since their first tickets encode.

**The demonstrable milestone.** After credential delivery and per-attempt authorization.

**The delegated seed-host concrete** is `seed-host/rqbit`, driving the rqbit application through its HTTP API; a further external client or the project's own client is a further concrete beneath the seed-host factory; the swarm milestone's commit sits on it.

# Additional Content

**What a ticket is and is not.** A ticket here is a candidate for a workplan node: it names the source file's role, what it owns, what it depends on, and what requirement it proves. It is not a node. A node is authored through the ordinary path in the node template, with every element in the fixed order, and this document does not emit node structure. When a ticket is promoted, its row here is unchanged; the node is the instruction and this map is the overview.

**Why the harness has the tickets it has.** One source file per node, so each cryptographic family's concretes, one per curve per library for pairing, and its factory, the benchmark, the generate and verifier families with their EVM concretes, each function the generate concrete owns, the transcript descriptions, the Solidity library and verifier, the wrap and unwrap, and the measurement driver each get their own file with full support. The `sample_deployment` generator needs the payload cipher and belongs to the protocol core grouping. That list is the honest size of the harness grouping.

**Requirement coverage of the foundation and harness groupings.** CR-03 in part, CR-04, CR-05, CR-07 in part, CR-08, CR-09, CR-10, CR-11, CD-03, CD-07, CD-08, LC-08 in part, XA-07 in part, AS-17 for the cryptographic families, and AS-21. Everything else the requirements name is in a later grouping.
