<!-- Template: antithesis_dependency_map.md -->
# Dependency Map

## Overview

How the parts of the ChainTorrent MVP fit together and in what order they are built, drawn from the [revised business case](business-case-revised.md), the [technical approach](technical-approach.md), the [workplan](../workplans/current/ChainTorrent%20MVP.md)'s build sequence, and the [MVP Application Requirements](../research/MVP%20Application%20Requirements.md). Groupings are addressed by delivery role: **foundation**, **cryptographic validation harness**, **protocol core and contract suite**, **local daemon and package serving**, **onboarding shells and services**, and **acceptance and release**.

Resolution decays with distance, deliberately. The foundation and harness groupings are mapped at **ticket** resolution: one ticket per source file, which is the node template's unit, so that each ticket is a candidate node the workplan author can promote. Inside the protocol core, the signature and swarm milestones are also at ticket resolution, because they depend on nothing the harness measures and run beside it; the remaining protocol core milestones are mapped as **sprints** named by dependency role. Beyond that the map holds **epics**, then **milestones**, then **objectives**. The reason is that implementation of the nearest work will produce discoveries that revise the anticipated order of everything after it, and detail authored now for distant groupings would be rewritten rather than used. When the harness closes, the remaining protocol core milestones are re-mapped to ticket resolution from what was learned, and the decay shifts outward.

Tickets are candidates, not nodes. No node is authored here; a ticket names the source file's role and its dependencies so a node can be written from it through the ordinary authoring path. Every adapter family is a crate holding its `factory` module, which owns the generic interface, the capability declaration, and the factory function, and its concretes as private modules beneath it, and the map names tickets accordingly: a family's factory ticket is `crate/factory`, a concrete's `crate/concrete`, a function a concrete owns `crate/concrete/function`, a function the family owns `crate/function`, a module of the domain or workflows crate `crate/module` with its functions beneath it, a contract `contracts/<suite>/Contract`, and a configuration-only ticket its directory; the [technical requirements](technical-requirements.md)' file tree spells every path, crate directories hyphenated and module directories underscored. A concrete adapter is one node, since its operations are the methods of one adapter in one file. Within a family the first ticket, its first concrete or a family-owned function that precedes it, authors the generic interface, the declaration, and the family's mock as its producers, since it is the first source file that requires them, and creates the crate; each further concrete and family-owned function follows; the factory ticket, which constructs the concretes and so consumes them, follows the family's last concrete and precedes every consumer; and every consumer ticket depends on the factory and never on a concrete. The declarations the composition resolver reads therefore exist from the family's first ticket. A family's factory ticket closes a producer, implementation, consumer chain, so it carries the family's integration test across the factory and its concretes; the commit sits on the last ticket of the milestone's chain, which for a milestone that closes at a family is that family's factory.

A ticket depends on another only when its file needs the other's file in order to be written: a type, a trait, a function, or an interface it imports, or a configuration file it is built under. What a file needs only in order to run — output a program writes, a deployed contract, a measured value — is a runtime input; the [Runtime sequence](#runtime-sequence) section holds every runtime input, and none orders a ticket. Each ticket's row is its file's single, complete authoring.

## Components

Grouped by role. Within a grouping at ticket resolution each row is a ticket; elsewhere each row is a sprint, epic, milestone, or objective as the decay dictates.

### Foundation, at ticket resolution

The workspace every node builds on, the secret type that cannot be formatted, the randomness family every key-generating ticket draws from, and the continuous integration that proves each ticket where the completion boundary requires. The encoding family is not a foundation member, since nothing encodable exists here; it follows the hashing family's root type, the first encodable type, at the head of the harness grouping. No crate is created ahead of the module that first lives in it; each ticket below creates its crate when that crate does not yet exist. A configuration file is authored once, complete, by the ticket that creates it, and no later ticket amends it; a crate's manifest and its `lib.rs` barrel are the exceptions by the topics' own design, since each module's ticket adds its dependencies to the manifest as its deps element and its re-export line to the barrel as its provides element.

| Ticket | Owns | Depends on | Proves |
| --- | --- | --- | --- |
| `workspace/cargo` | The virtual workspace manifest with the glob members `crates/*`, `adapters/*`, and `apps/*`, the lint table, and the `[patch.crates-io]` overlay pointing `librqbit` at the project's overlay branch `chaintorrent-overlay` in `https://github.com/tsylvester/rqbit`, which exists at a tagged upstream release before the bootstrap as external setup; `rust-toolchain.toml`; `deny.toml` with the license allowlist and that repository in `allow-git`; `.gitignore`; configuration files with no types and no tests, each authored once and complete | nothing | The root configuration every later ticket builds within; its build proof arrives with the first member |
| `domain/secret` | The secret-typed value: no formatting or serialization trait, an explicit accessor for the cryptographic operations and custody wrapping that consume it, zeroization on drop through `zeroize`, a crate dependency; creates the `domain` crate | `workspace/cargo` | CR-07 exclusion of secrets from logs, proven at compile time; the lint table's rejection of `unsafe_code`, `unwrap_used`, `expect_used`, `panic`, and `as_conversions` in the first production crate |
| `random/os` | The concrete over the operating system's generator; authors, as its producers, the randomness family's interface that fills bytes, the declaration, and the family's mock; creates the `adapters/random` crate; pins `rand_core` and `getrandom` | `workspace/cargo` | CR-05 contract; repeated draws of a fixed width pairwise distinct across a fixed count, and a filled buffer of a fixed length holding more than one distinct byte value; CR-05's per-bootstrap independence is proven at the First Finder engine and its reproducible publisher derivation at the explicit publisher path |
| `random/factory` | The randomness family's factory function, constructing the concrete a configuration names with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete and the grouping's commit | `random/os` | CR-05: the one surface every production draw passes through |
| `workspace/ci` | The Rust continuous-integration definition `.github/workflows/rust.yml`, authored once and complete: `cargo fmt --check`, `cargo check`, `cargo clippy`, the unit and integration tests, `cargo-deny`, and `cargo-audit` on Windows, macOS, and Linux; a configuration file with no types and no tests; the Solidity, shell, fuzz, and end-to-end checks are separate workflow files, each created once by the ticket that first needs it | `workspace/cargo` | XA-07 facilities run in CI; NF-M07; `deny.toml` read and enforced on every supported platform |

### Cryptographic validation harness, at ticket resolution

The harness begins with the domain's identifier modules and the hashing family, whose root type is the first encodable type and depends on no encoding, followed by the encoding family, whose first description is the root type's, and the derivation context in the sidecar layer, which composes the root type with the identifier modules and is described beside itself in the workflows crate, and builds the pairing, key-derivation, hash-to-scalar, credential KEM, envelope, and delivery proof families, each from its first ticket, which carries the family's interface, to its factory, which follows every concrete, together with the chain family's EVM forms concrete and its family-owned forms function, the mint and transfer delivery functions every ticket that proves, mutates, encodes, replays, or times a delivery consumes, the harness's typed configuration with its shipped defaults, the harness run that composes every family through its factory over the consumers it owns, the output that writes the run's outcome, a success or a refusal, to the destinations the configuration names, and the harness binary that reads the configuration, calls the run, and hands its outcome to the output, exercises the deployed verifier on Base Sepolia, and records the measurements against the configured piece-group size and curve defaults with any adjustment the data supports (CD-07, AS-21). It constructs every curve and verifier-form combination through the factories, so an invalid combination is refused in the harness exactly as the daemon's resolver will refuse it. Every ticket is one Rust source file with its full support system except where the row says otherwise; a family's first ticket owns the family's generic interface, declaration, and mock beside its own implementation, a further concrete ticket is one adapter whose operations are its methods, and the factory ticket owns the factory function and the crate's public surface.

| Ticket | Owns | Depends on | Proves |
| --- | --- | --- | --- |
| `pairing/bn254_arkworks` | The BN254 concrete on arkworks, precompile-matching encodings, declaring the curve, first-group-only arithmetic at the verifier, and its target-group encoding identifier, proven by executing the identifier's definition in its tests; authors, as its producers, `IPairingAdapter` with generators, add, mul, MSM, subgroup check, and the pairing-product check, the reference trait `IPairingReference`, beside the generic interface, with the scalar field's order and the precompile encoding of the defined point outside each source group's prime-order subgroup, the group-element and scalar associated types, the sampling bound on the scalar associated type, sample from uniform bytes, the declaration of the curve, of second-group arithmetic at the verifier, of precompile encodings, and of the target-group encoding identifier, and the family's mock; owns its scalar and group-element types, holding the library's elements as private fields behind fallible constructors that perform the subgroup and encoding checks, names them as the associated types, implements the sampling bound on its scalar type, and proves the in-range draw; creates the `adapters/pairing` crate; pins `ark-ec`, `ark-ff`, and `ark-bn254` | `workspace/cargo`, `random/factory` | CR-10 contract and CR-10 on BN254; the declaration the proof and chain factories read |
| `pairing/bn254_halo2curves` | The BN254 concrete on `halo2curves`, precompile-matching encodings, declaring the curve, first-group-only arithmetic at the verifier, and its target-group encoding identifier, proven by executing the identifier's definition in its tests; owns its scalar and group-element types over its library's elements behind fallible constructors, implements the sampling bound on its scalar type, and proves the in-range draw; pins `halo2curves` | `pairing/bn254_arkworks` | CR-10 on BN254 |
| `pairing/bls12_381_arkworks` | The BLS12-381 concrete on arkworks, precompile-matching encodings, subgroup checks on every input, declaring the curve, second-group arithmetic at the verifier, and its target-group encoding identifier, proven by executing the identifier's definition in its tests and against the CFRG draft's published value; owns its scalar and group-element types over its library's elements behind fallible constructors, implements the sampling bound on its scalar type, and proves the in-range draw; pins `ark-bls12-381` | `pairing/bn254_arkworks` | CR-10 on BLS12-381 |
| `pairing/bls12_381_halo2curves` | The BLS12-381 concrete on `halo2curves`, precompile-matching encodings, subgroup checks on every input, declaring the curve, second-group arithmetic at the verifier, and its target-group encoding identifier, proven by executing the identifier's definition in its tests and against the CFRG draft's published value; owns its scalar and group-element types over its library's elements behind fallible constructors, implements the sampling bound on its scalar type, and proves the in-range draw | `pairing/bn254_arkworks` | CR-10 on BLS12-381 |
| `pairing/factory` | The pairing family's factory function, admitting a curve the chain's declared precompile sets support and a concrete only when it declares the target-group encoding identifier the suite requires, constructing the concrete the composition names, with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concretes, which proves the concretes of a curve return the same scalar field order and the same outside-the-subgroup encodings | `pairing/bn254_arkworks`, `pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, `pairing/bls12_381_halo2curves` | The surface the hash-to-scalar, KEM, envelope, proof, and chain tickets consume |
| `harness-crypto/benchmark` | The measurement of scalar multiplication, multi-scalar multiplication, and pairing over every pairing concrete, each constructed through the pairing factory, recording each concrete's measurements; names no library; creates the `apps/harness-crypto` crate | `pairing/factory`, `random/factory` | The measurements from which the configuration's shipped pairing concrete per curve is set |
| `domain/deployment_identity` | The registry-assigned deployment identity with its fallible `try_new` constructor enforcing its invariants | `domain/secret` | The deployment every derivation and object is bound to |
| `domain/suite_identifier` | The cryptographic suite identifier and version with its fallible `try_new` constructor enforcing its invariants | `domain/secret` | The suite every derivation is bound to |
| `domain/parameter_set_identifier` | The parameter-set identifier with its fallible `try_new` constructor enforcing its invariants | `domain/secret` | The set every capsule and wrapping key is bound to |
| `domain/group_index` | The piece-group index with its fallible `try_new` constructor enforcing its bounds | `domain/secret` | The group every piece-group key is bound to |
| `domain/piece_geometry` | The piece size, piece-group size, and addressable extent with their fallible `try_new` constructor enforcing alignment | `domain/secret` | The geometry every derivation and the cipher are bound to |
| `domain/signature_scheme_identifier` | The signature scheme identifier, the `uint8` the identity binding and the claim verifier's key record store, with its fallible `try_new` constructor refusing the zero value an unassigned slot reads as; the type every signature concrete's declaration carries as its scheme identifier and the chain family's identity form takes for address derivation, so neither family depends on the other for it | `domain/secret` | The scheme every signature declaration, binding, verifier key record, and address derivation names |
| `hashing/blake3_bao` | The BLAKE3/Bao concrete as one adapter: root and outboard construction, verification, challenge and response, and keyed mode for the keyed plaintext-root disclosure mode, which takes the plaintext-root key as a borrowed `Secret` the key-derivation family derives and computes keyed roots and keyed authentication paths through `blake3`'s keyed mode, `bao` serving the unkeyed roots and outboards; authors, as its producers, the commitment interface with root and outboard construction over a byte stream, streaming and random-access verification against a root with an authentication path, and random chunk challenge and response, the root, outboard, path, and chunk types with their fallible constructors, the root type being the object's BLAKE3/Bao root, the asset's identity, the plaintext root for a public asset and the ciphertext root for a private one, refusing the all-zero value, the commitment-scheme identifier the hash-card carries, the commitment-scheme and chunk-granularity declaration, and the family's mock; every hashing operation is over bytes and the family carries no digest operation and no identity-hash identifier, so it depends on no encoding; creates the `adapters/hashing` crate; pins `blake3` and `bao` | `workspace/cargo`, `domain/secret` | CR-02 contract, construction, verification, and challenges; EC-02; SW-04 |
| `hashing/factory` | The hashing family's factory function with a branch per concrete, admitting a concrete by the commitment-scheme identifier alone, and the crate's public surface; carries the family's integration test | `hashing/blake3_bao` | The surface every content root passes through |
| `encoding/abi` | The Ethereum ABI concrete through `alloy`'s sol types, implementing both interfaces as one adapter; untrusted bytes to the owned type or its error; authors, as its producers, `IEncoderAdapter` and `IDecoderAdapter` with the versioned encoding identifier, the encoding types, the capability declaration, the total operation mapping a canonical field kind to the validated `EncodingTypeName` it gives that kind, and the family's mock; its known-answer vectors are the root type's through its description, and the encodings of the identifiers it names authored from the ABI specification, which the generator later mirrors, and its round trip and its rejections of truncated input, trailing bytes, non-canonical padding, and a wrong element count run over the same values through the description; adds to the encoding contract the byte-string kind, which carries a pairing group element's precompile encoding, encoded as ABI `bytes` against its own known-answer vector, and, for the proof family's transcript descriptions, the fixed twenty-byte and 256-bit unsigned kinds, each named as a width, mapped by this concrete to the one ABI type name it declares: `bytes32`, `bytes20`, `uint16`, `uint32`, `uint64`, `uint256`, `string`, and `bytes` for the eight canonical kinds respectively; `FixedBytes20` is always `bytes20`, never `address`; pins `alloy` |  | ABI encoding of the root type through its description against its vectors; round trip; malformed input rejected; the hash-card's encoding identifier; every canonical field kind's ABI type name, including `FixedBytes20` as `bytes20`; the byte string a statement over group elements is encoded as; the widths a transcript's parties and entitlements are encoded as |
| `encoding/factory` | The encoding family's factory function, constructing the concrete the configuration or a hash-card names with a branch per concrete, and the crate's public surface; carries the family's integration test across factory, concrete, and description | `encoding/abi` | The generic surface every hashed, signed, stored, or framed value passes through |
| `kdf/blake3_keyed` | The BLAKE3 keyed-derivation concrete; context strings, serialization, and output lengths frozen with known-answer vectors from an independent implementation, over a context its caller encodes through the encoding family; authors, as its producers, the interface that derives a key of a stated length from key material under an encoded context, the KDF identifier declaration, and the family's mock; creates the `adapters/kdf` crate; pins `blake3` | `encoding/factory` | CR-05, CR-11 derivation contract and vectors |
| `kdf/factory` | The key-derivation family's factory function with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete | `kdf/blake3_keyed` | The surface every off-chain derivation passes through |
| `hash-to-scalar/keccak256` | The keccak256 concrete under domain tags, reduced modulo the group order, with vectors the generator mirrors; authors, as its producers, the interface that maps domain-tagged bytes to a scalar of the resolved pairing's field, the domain-tag types with their fallible constructors, the identifier declaration, and the family's mock; creates the `adapters/hash-to-scalar` crate | `encoding/factory`, `pairing/factory` | CR-08, CR-09 identity-mapping and challenge contract and vectors |
| `hash-to-scalar/factory` | The hash-to-scalar family's factory function with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete | `hash-to-scalar/keccak256` | The surface every value a contract recomputes passes through |
| `kem/bb1_depth_one` | The depth-one Boneh–Boyen concrete as one adapter: random master scalar and parameter set for escrow lineage, issuance with trivial identity-element refusal, seller-side rerandomization without the master scalar, the public validity check with encoding and subgroup validation, encapsulation under both identity scopes, well-formedness, and decapsulation to the encapsulated value; owns its parameter-set, credential, capsule, and identity-element types; authors, as its producers, `ICredentialKemAdapter` with setup, deriveIdentity, issue, rerandomize, isValid, encapsulate, isWellFormed, and decapsulate, the parameter-set, master-scalar, credential, capsule, identity-element, and encapsulated-value types as associated types, the identity mapping over caller-supplied canonical identity bytes, which under the asset scope are the asset's root, the root type's canonical field through its description, the identity-scope declaration carrying the identity mapping's domain tag, and the family's mock; creates the `adapters/kem` crate | `pairing/factory`, `hash-to-scalar/factory`, `random/factory`, `domain/secret`, `hashing/factory`, `encoding/factory` | CR-08 contract and algebraic properties, LC-08, EC-06, CD-08 declaration |
| `kem/factory` | The credential KEM family's factory function, admitting a concrete by the suite's declared scope, with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete | `kem/bb1_depth_one` | CD-08 admission; the surface the sidecar, vector, and delivery tickets consume |
| `envelope/possession_statement` | The family-owned description of a proof of possession's statement, the key and commitment in the selected pairing's typed groups, flattened to precompile encodings only at the encoding boundary and validated with that pairing on reconstruction, implementing the encoding contract as its canonical field sequence; creates the `adapters/envelope` crate | `encoding/factory`, `pairing/factory` | LC-08 and CR-09, the proof of possession's statement frozen as one field sequence |
| `envelope/pairing_elgamal` | The pairing ElGamal concrete as one adapter: two independently keyed envelope keys generated from caller-supplied secret draws, each with a Schnorr proof of possession whose challenge is the hash-to-scalar mapping of the encoded possession statement, identity-element and shared-secret rejection, encryption of a credential's `CredentialComponents` with independent coins drawn through the randomness family, decryption under the recipient's key pair; owns its key-pair, public-key, possession, and envelope types; authors, as its producers, `IKeyAgreementAdapter` with key generation under proofs of possession, wrap, and unwrap, the key-pair, public-key, possession, and envelope types as associated types with their fallible constructors, a recipient's public keys admitted only with their proof of possession, the envelope-algebra declaration carrying the proofs of possession's domain tags, and the family's mock | `envelope/possession_statement`, `pairing/factory`, `hash-to-scalar/factory`, `random/factory`, `domain/secret`, `encoding/factory`, `kem/factory` | CR-04 contract; LC-08 at the algebra level |
| `envelope/factory` | The envelope family's factory function with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete | `envelope/pairing_elgamal` | The surface every envelope passes through |
| `proof/mint_statement` | The family-owned description of the mint transcript under delivery-statement version one, implementing the encoding contract as its canonical field sequence: the suite identifier, the chain, the entitlement contract, the asset's root taken through the root type's description, the parameter-set digest, the source and target entitlements, the old and new interval counters, the purpose, the issuer's identity in the seller field, the buyer's identity, the buyer's typed envelope keys, the typed new envelope, the expiry, and the mint relation's typed first messages, flattening group elements to precompile encodings only at the encoding boundary and validating them on reconstruction; relation-specific purpose variants with derived sixteen-bit codes and relations; the version constant; generic over the selected pairing and the chain family's identity, entitlement, interval, and chain-identifier forms, the entitlement contract taking the identity form; creates the `adapters/proof` crate | `encoding/factory`, `pairing/factory`, `envelope/factory`, `chain/create_chain_forms`, `hashing/factory`, `domain/suite_identifier`, `domain/parameter_set_identifier` | CR-09 statement binding under the version the hash-card names; the mint transcript frozen as one field sequence |
| `proof/transfer_statement` | The family-owned description of the transfer transcript under delivery-statement version one, serving sale and grant: the mint transcript's context, its asset root field included, with the seller's identity and typed envelope keys, the typed old envelope, and the transfer relation's typed first messages, the source entitlement equal to the target for a sale and the author's entitlement for a grant with the old counter the author's current interval and the new counter zero ; reconstruction validates group encodings with the selected pairing | `proof/mint_statement`, `pairing/factory`, `envelope/factory`, `chain/create_chain_forms` | CR-09; CD-05's grant statement frozen as one field sequence |
| `proof/schnorr_fs/challenge` | The function the concrete owns: the Fiat–Shamir challenge as the hash-to-scalar mapping, under the proof's domain tag, of a transcript's encoding through the encoding and hash-to-scalar concretes the hash-card's identifiers name; creates the `schnorr_fs` module | `proof/mint_statement`, `proof/transfer_statement`, `hash-to-scalar/factory`, `encoding/factory`, `pairing/factory`, `chain/create_chain_forms` | CR-09 statement binding |
| `proof/schnorr_fs` | The generalized Schnorr concrete as one adapter: the mint prover, the transfer prover from the seller's fresh decryption and total offset, and the reference verifier in both forms, second-group arithmetic and the hash-weighted pairing product with first-group arithmetic only; declares both forms; authors, as its producers, `IDeliveryProofAdapter` with proveMint, proveTransfer, and verify, the algebraic mint and transfer statement types over the pairing's group elements, the proof type as an associated type, the declaration of supported envelope algebras, verifier forms, and delivery-statement versions and of the challenge and weight domain tags, and the family's mock | `proof/schnorr_fs/challenge`, `pairing/factory`, `envelope/factory`, `kem/factory`, `random/factory`, `chain/create_chain_forms` | CR-09, CD-01, CD-02 |
| `proof/factory` | The delivery proof family's factory function, admitting a concrete only when it declares the resolved envelope's algebra, the verifier form the resolved pairing declares, and the delivery-statement version the hash-card names, with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete | `proof/schnorr_fs`, `chain/create_chain_forms` | CR-09 contract; the admission rule the harness exercises |
| `chain/evm_forms` | See [chain/evm_forms ownership](#chainevm_forms-ownership) below this table | `encoding/factory`, `domain/signature_scheme_identifier` | CR-09: the forms a transcript's chain, entitlement, interval, and identity fields take, declared and admitted rather than assumed; the chain family's declaration the resolver reads |
| `chain/create_chain_forms` | The family-owned function constructing the forms concrete the configuration names over the borrowed encoder it is handed, admitting the forms identifier the hash-card carries as an explicit field, supplied by the configuration in the harness and read from the deployment record in the daemon, and admitting the encoder only when its declaration carries the encoding identifier the forms concrete declares it requires, otherwise returning a typed refusal with no side effect; it hands the admitted encoder to the forms concrete, with a branch per concrete, and hands the result to a consumer generic over the form interface; its own module, named for its one function as the Schnorr challenge module is, because the factory module holds one function and that function constructs the chain concrete; carries the family's integration test across the function and the forms concrete | `chain/evm_forms`, `encoding/factory` | The surface the proof family's transcript descriptions, challenge, concrete, and factory, the generator, verifier client, vectors, and measurement obtain their forms through, and later every consumer of the chain family's forms; incompatible encoder refusal |
| `workflows/sidecar/derivation_context` | The derivation context composed of the hashing family's root type as its asset component, the deployment identity, suite identifier, parameter-set identifier, group index, and geometry, with its fallible `try_new` constructor enforcing its invariants; housed in the sidecar layer, which composes it for derivation, and not in the `domain` crate; creates the `workflows` crate, which depends on the encoding family | `hashing/factory`, `domain/deployment_identity`, `domain/suite_identifier`, `domain/parameter_set_identifier`, `domain/group_index`, `domain/piece_geometry` | The context every wrapping key and lineage derivation is domain-separated by |
| `workflows/sidecar/wrap` | The wrapping key derived through the KDF family from an encapsulated value and the context, and the XOR wrap of a piece-group key under it; creates no crate | `workflows/sidecar/derivation_context`, `kdf/factory`, `kem/factory`, `encoding/factory` | CR-11 wrap |
| `workflows/sidecar/unwrap` | The unwrap of a piece-group key from its wrapped value under the derived wrapping key | `workflows/sidecar/wrap` | CR-11 cross-set agreement |
| `harness-crypto/delivery/mint` | The mint delivery, generic over the form interface, over the borrowed credential KEM, key agreement, delivery proof, and randomness source its caller hands it, from a `&DeliveryContext<F, MintPurpose>` and an `AdmittedKemIdentity` it is handed: sets up a parameter set, derives the identity element, issues a credential, generates and admits the recipient's keys, wraps the credential to them, and proves the mint statement, drawing master, `u0`, `u1`, issue, recipient `x`, and recipient `y` in that order; returns the statement, cloned from the borrowed context, and the proof, the parameter set, identity element, and credential, the mint prover's witnesses, `MasterScalarComponents`, the issue randomness, and the wrap's `EnvelopeCoins`, each copied through `Secret::try_new` over its exposed value since the prover consumes what it is given, the recipient as a `DeliveryHolder` of its key pair, public keys, and envelope, and the uniforms in draw order; each callee error carried unchanged; owns `AdmittedKemIdentity`, the KEM's `SetupScope` and `KemIdentity` paired and deriving `Clone` and `Copy`, `DeliveryHolder`, `MintDelivery`, and `MintWitnesses`; adds a `Clone` impl for `DeliveryContext` to the proof family's factory interface, manual since a derive would bound the forms, and `Clone` and `Copy` to `SetupScope` and `KemIdentity` in the KEM family's factory interface; creates the `delivery` module | `kem/factory`, `envelope/factory`, `proof/factory`, `pairing/factory`, `chain/create_chain_forms`, `random/factory`, `domain/secret` | CD-01 delivery at mint, with its witnesses held for every consumer that proves, mutates, encodes, or times it |
| `harness-crypto/delivery/transfer` | The transfer delivery, generic over the form interface, over the borrowed credential KEM, key agreement, delivery proof, and randomness source its caller hands it, from a `&DeliveryContext<F, TransferPurpose>`, the parameter set, the identity element, and the `DeliveryHolder` it transfers from: unwraps the holder's envelope under the holder's key pair, rerandomizes the credential, generates and admits the buyer's keys, wraps the credential to them, and proves the transfer statement, drawing buyer `x`, buyer `y`, and the rerandomization in that order; returns the statement and the proof, the rerandomized credential, the transfer prover's witnesses, the seller's `KeyPairComponents`, the offset, and the wrap's `EnvelopeCoins`, each copied as the mint copies its witnesses, the buyer as a `DeliveryHolder`, a valid holder for the next transfer, and the uniforms in draw order; each callee error carried unchanged; owns `TransferDelivery` and `TransferWitnesses` | `harness-crypto/delivery/mint`, `kem/factory`, `envelope/factory`, `proof/factory`, `pairing/factory`, `chain/create_chain_forms`, `random/factory`, `domain/secret` | CD-02 delivery at sale, chainable from any holder, with its witnesses held for every consumer that proves, mutates, encodes, replays, or times it |
| `harness-crypto/generate/solidity/render` | The function the Solidity generate concrete owns that renders Solidity library source, opening with the license identifier and the compiler version pragma its params carry: a constants library from a library name and an ordered list of named entries, each a byte string, a 256-bit, sixty-four-bit, or sixteen-bit unsigned value, a boolean, or a string; and a vector-record library from a library name and, per vector kind, a struct name, the struct's members as names with their canonical field kinds and already-resolved `SolidityAbiType` names in canonical order, a decode function name, a path constant name, and the path of the kind's vector file relative to the Foundry project root, rendering each member as the type name it carries and using the kind only to choose Solidity data location (`Text` and `Bytes` in `memory`), each decode function as `abi.decode` of one line's bytes over the members' types in canonical order returning the struct, and each path constant as a string; owns the entry type, the record-declaration type, and the validated name types; pure, reading no adapter and touching no operating-system facility; creates the `generate` area and the `solidity` module; the vector files the concrete writes stay ABI-encoded and language-neutral, and only the libraries it renders are Solidity | `harness-crypto/benchmark` | CR-09 parity inputs: one rendering for every constant and every vector record type |
| `harness-crypto/generate/solidity/constants` | The function the concrete owns that returns the constants library's entries, generic over the form interface: each family's domain tags read from its declaration, the identity mapping's, the two proofs of possession's, and the delivery proof's challenge and weight, each a byte string; the delivery-statement version it is handed and the delivery purposes' codes from the proof family's declared purposes, each a sixteen-bit unsigned value; the mint and transfer transcripts' field sequences read from their descriptions over the forms in play, each rendered as one string of ABI type names read from the borrowed encoder for each field kind and wrapped as `SolidityAbiType`; the scalar field's order from the pairing family's reference trait, a 256-bit unsigned value; the two generators' precompile encodings; and whether the verifier has second-group arithmetic, a boolean read from the pairing's declaration | `harness-crypto/generate/solidity/render`, `encoding/factory`, `pairing/factory`, `kem/factory`, `envelope/factory`, `proof/factory`, `chain/create_chain_forms` | CR-09 parity inputs; CR-11 the tags and the modulus a contract hashes under |
| `harness-crypto/generate/solidity/group_vectors` | The function the concrete owns that returns the pairing library's group-operation and pairing-product vector records, each kind drawn and holding the count its params carry for that kind, over scalars it draws through the randomness family, each record carrying its draws, its points in the precompile encodings the pairing declares, and the pairing-product verdict read from the reference: first-group addition, first-group multiplication, and the pairing-product check for every pairing, and second-group addition and both groups' multi-scalar multiplication where the pairing declares second-group arithmetic at the verifier; each record type with its canonical description implementing the encoding contract and the member names the record declaration `render` owns takes; owns the per-kind vector count type the other vector functions, the generate family's per-kind settings, and the configuration consume | `harness-crypto/generate/solidity/render`, `encoding/factory`, `pairing/factory`, `random/factory` | CR-10 on-chain vectors |
| `harness-crypto/generate/solidity/rejection_vectors` | The function the concrete owns that returns the decode-rejection vector records, an enumerated kind holding exactly the candidates it enumerates: one record per candidate encoding, for each source group and for a scalar, each naming what it decodes into and holding the candidate's bytes, every record an input the suite must refuse, the outside-the-subgroup candidates being the pairing family's reference encodings, absent where a group is the whole curve, and the non-canonical scalar candidate being the scalar field's order; the record type with its canonical description and member names | `harness-crypto/generate/solidity/render`, `encoding/factory`, `pairing/factory` | CR-10 subgroup rejection and encoding edge cases on chain |
| `harness-crypto/generate/solidity/mapping_vectors` | The function the concrete owns that returns the hash-to-scalar vector records, a drawn kind holding the count its params carry for it, each a message it draws and the scalar that message maps to under each family's domain tag read from that family's declaration, the identity mapping's, the two proofs of possession's, and the delivery proof's challenge and weight, the tags the constants library carries, so one message under every tag proves the tag is bound; and the identity-mapping vector records, a kind over configured inputs holding one record per `AdmittedKemIdentity` its payload carries, each mapped through the credential KEM under that identity's setup scope, the scalar read from the identity element's components; admits no identity, every identity it is handed being admitted already; each record type with its canonical description and member names | `harness-crypto/generate/solidity/render`, `harness-crypto/delivery/mint`, `encoding/factory`, `hash-to-scalar/factory`, `kem/factory`, `envelope/factory`, `proof/factory`, `pairing/factory`, `random/factory` | CR-08, CR-11 identity-mapping and hash-to-scalar vectors |
| `harness-crypto/generate/solidity/possession_vectors` | The function the concrete owns that returns the possession vector records, a drawn kind holding the count its params carry for it: envelope keys with their proofs of possession's components, generated through the key-agreement family from secret draws the function makes and records; the proofs' nonces are drawn inside the adapter and are not recorded; the record type with its canonical description and member names | `harness-crypto/generate/solidity/render`, `encoding/factory`, `envelope/factory`, `pairing/factory`, `random/factory` | LC-08 at the algebra level, on chain |
| `harness-crypto/generate/solidity/challenge_vectors` | The function the concrete owns that returns the challenge vector records, generic over the form interface, a drawn kind holding the count its params carry for it, each record carrying the values the function drew through the randomness family: mint and transfer transcripts over the `&DeliveryContext<F, MintPurpose>` and `&DeliveryContext<F, TransferPurpose>` its payload carries, each context cloned per record and its purpose read from it, each transcript with its encoding through its description and the encoding family and its challenge, the hash-to-scalar mapping of that encoding under the challenge tag the proof family's declaration carries; rebuilds no form; the transcripts stand alone, since the proof family's challenge function is private to its crate and a proof in the second-group form carries no first messages; the record type with its canonical description and member names; declares `CanonicalDeliveryContext` and `DeliveryContextField`, which it does not consume, until `harness-crypto/config/admit_statements` and `harness-crypto/run/admit_contexts` move them | `harness-crypto/generate/solidity/render`, `harness-crypto/delivery/mint`, `proof/factory`, `encoding/factory`, `hash-to-scalar/factory`, `chain/create_chain_forms`, `pairing/factory`, `random/factory` | CR-09, CR-11 challenge vectors |
| `harness-crypto/generate/solidity/proof_vectors` | The function the concrete owns that returns the mint-proof and transfer-proof vector records, generic over the form interface, each a drawn kind holding the count its params carry for it, each record a statement with its proof's components: a mint record is one mint delivery from `harness-crypto/delivery/mint` over the typed mint context and the `AdmittedKemIdentity` its payload carries, its draws that delivery's six uniforms; a transfer record is a mint delivery followed by a transfer delivery from `harness-crypto/delivery/transfer` over the typed transfer context, its draws those nine uniforms in draw order; each statement as its context values and its elements' precompile encodings, the first-group-only form carrying its second-group first messages; composes no delivery step, draws nothing, rebuilds no form, and admits no identity; each record type with its canonical description and member names | `harness-crypto/generate/solidity/render`, `harness-crypto/delivery/mint`, `harness-crypto/delivery/transfer`, `encoding/factory`, `kem/factory`, `envelope/factory`, `proof/factory`, `pairing/factory`, `chain/create_chain_forms` | CR-09 parity inputs |
| `harness-crypto/generate/solidity` | See [harness-crypto/generate/solidity ownership](#harness-cryptogeneratesolidity-ownership) below this table | `harness-crypto/generate/solidity/constants`, `harness-crypto/generate/solidity/group_vectors`, `harness-crypto/generate/solidity/rejection_vectors`, `harness-crypto/generate/solidity/mapping_vectors`, `harness-crypto/generate/solidity/possession_vectors`, `harness-crypto/generate/solidity/challenge_vectors`, `harness-crypto/generate/solidity/proof_vectors`, `harness-crypto/delivery/mint`, `pairing/factory`, `random/factory`, `kem/factory`, `envelope/factory`, `proof/factory`, `hash-to-scalar/factory`, `encoding/factory`, `chain/create_chain_forms` | CR-09 parity inputs; CR-10 and CR-11 on-chain vectors |
| `harness-crypto/generate/factory` | The generate family's factory function, constructing the concrete the configuration names over the borrowed adapters and declarations its deps carry and the configured values its params carry, the per-kind settings, library names, Foundry project root, generated directory, Solidity compiler version, and license identifier among them, admitting the configuration only when its per-kind settings name exactly the kinds the concrete declares, with a count for each drawn kind and none for any other kind, with a branch per concrete, and its public surface; its consumer trait generic over the forms, handing a consumer only a concrete whose `Forms` are the consumer's; takes no scope; carries the family's integration test across factory and concrete | `harness-crypto/generate/solidity`, `chain/create_chain_forms` | CR-09 parity-input contract |
| `contracts/evm/IDeliveryVerifier` | The contract suite's own node, in the suite's declared source language: authors, as its producers, the delivery verifier's interface, the language-neutral ABI description `contracts/evm/abi/IDeliveryVerifier.json` the suite owns, and its Solidity realization `contracts/evm/src/IDeliveryVerifier.sol`, with the builders and mock that stand in for the interface and its interface test, which proves the Solidity realization and the ABI description against the same selector texts; creates `contracts/evm/foundry.toml`, granting `fs_permissions` read access to the generated directory and the ABI description directory alone with `ffi` disabled and `via_ir = true`, and `.github/workflows/contracts.yml` with `forge build`, `forge fmt --check`, and `forge test`, each authored once and complete | `workspace/ci` | CD-03 interface; the ABI the verifier client binds and the toolchain every contract ticket builds under |
| `harness-crypto/verifier/evm` | Binds, as its producers, to the suite's ABI description `contracts/evm/abi/IDeliveryVerifier.json` through `alloy`'s `sol!` over that description and authors no contract source and no toolchain file; the EVM concrete, named for the EVM ABI it speaks, over the deployed verifier through `alloy`, generic over the form interface, constructed over the borrowed adapters and declarations its caller hands it and calling no factory, its statements built over the forms `create_chain_forms` hands it, their context values and the deployed verifier's address the canonical field values its params carry; authors, as its producers, the verifier family's interface that reaches a suite's deployed delivery verifier, submits a statement and proof, and reads acceptance and cost as L2 execution gas and L1 data fee, its declaration, and its mock; pins `alloy` for the harness | `contracts/evm/IDeliveryVerifier`, `proof/factory`, `encoding/factory`, `chain/create_chain_forms` | CD-03 cross-verification |
| `harness-crypto/verifier/factory` | The verifier family's factory function, constructing the concrete the configuration names over the borrowed adapters and declarations its deps carry, with a branch per concrete, and its public surface; carries the family's integration test across factory and concrete | `harness-crypto/verifier/evm` | CD-03 cross-verification contract |
| `harness-crypto/vectors/cross_holder` | The function the vectors own that returns the cross-holder agreement set over the credential KEM its caller hands it: per record a parameter set set up under the setup scope of the `AdmittedKemIdentity` it is handed, that identity's element, a credential issued and its rerandomized successor, a capsule encapsulated under the set, the value each credential decapsulates, and the uniforms it drew; admits no identity; owns the record type; creates the `vectors` module | `kem/factory`, `harness-crypto/delivery/mint`; `random/factory`, whose production concrete supplies the draws each record holds | CR-08 cross-holder agreement |
| `harness-crypto/vectors/rerandomization` | The function the vectors own that returns the rerandomized-validity set: per record a parameter set, the identity element of the `AdmittedKemIdentity` it is handed, an issued credential, its rerandomization, each credential's `is_valid` verdict, and its draws; admits no identity; owns the record type | `kem/factory`, `harness-crypto/delivery/mint`; `random/factory`, whose production concrete supplies the draws each record holds | CR-08 rerandomized validity |
| `harness-crypto/vectors/non_convertibility` | The function the vectors own that returns the non-convertibility set under the entitlement scope: per record a credential issued for one `AdmittedKemIdentity` and its `is_valid` verdicts against its own identity's element and the other `AdmittedKemIdentity`'s element, with its draws; the set empty when the identity's setup scope is the asset scope, whose declaration fixes one identity per set; admits no identity; owns the record type | `kem/factory`, `harness-crypto/delivery/mint`; `random/factory`, whose production concrete supplies the draws each record holds | CR-08 cross-entitlement non-convertibility |
| `harness-crypto/vectors/malformed_capsules` | The function the vectors own that returns the malformed-capsule set: per record a parameter set set up under the setup scope of the `AdmittedKemIdentity` it is handed, a capsule, that capsule rebuilt from its components with one source-group component displaced, both `is_well_formed` verdicts, and its draws; admits no identity; owns the record type | `kem/factory`, `harness-crypto/delivery/mint`; `random/factory`, whose production concrete supplies the draws each record holds | CR-08 malformed-capsule rejection |
| `harness-crypto/vectors/cross_set` | The function the vectors own that returns the cross-set set: per record two parameter sets set up independently under the setup scope of the `AdmittedKemIdentity` it is handed, a piece-group key drawn once, that key wrapped under each set's encapsulated value over the configured derivation context, each wrapped key unwrapped with the value a credential for that identity under its set decapsulates, and the draws; admits no identity; owns the record type | `kem/factory`, `workflows/sidecar/unwrap`, `harness-crypto/delivery/mint`; `random/factory`, whose production concrete supplies the draws each record holds | CR-08, CR-11 cross-set agreement |
| `harness-crypto/vectors/statement_mutations` | The function the vectors own that returns, generic over the form interface, per record one mint delivery from `harness-crypto/delivery/mint` and the transfer delivery from `harness-crypto/delivery/transfer` that follows it, over the typed contexts and the `AdmittedKemIdentity` its payload carries, each baseline statement verified against its proof, and every statement field and every proof scalar mutated alone, each mutation with the reference verifier's verdict, a form-typed field mutated through its form's `to_field` and `from_field`; its draws the deliveries' nine uniforms followed by its own mutation-offset uniform; composes no delivery step, rebuilds no form from a canonical context, and admits no identity; owns the record and field types | `harness-crypto/delivery/mint`, `harness-crypto/delivery/transfer`, `proof/factory`, `pairing/factory`, `envelope/factory`, `encoding/factory`, `chain/create_chain_forms`; `random/factory`, whose production concrete supplies the mutation offset each record holds | CR-09 mutated statement fields and responses |
| `harness-crypto/vectors/replay` | The function the vectors own that returns, generic over the form interface, per record a mint delivery from `harness-crypto/delivery/mint`, a transfer delivery from `harness-crypto/delivery/transfer` under the typed transfer context, and a transfer delivery from that buyer's holder under the typed next context, with both settlements' verdicts and the verdict of the first transfer's proof against the next statement, the replay; before any record, admits the next context as the next settlement of the same entitlement by comparing the forms' `to_field` values and the domain values, refusing the first differing field; its draws the deliveries' uniforms in order, drawing nothing itself; composes no delivery step, rebuilds no form, and admits no identity; owns the record type | `harness-crypto/delivery/mint`, `harness-crypto/delivery/transfer`, `proof/factory`, `chain/create_chain_forms` | CR-09 replay across settlements |
| `harness-crypto/vectors` | The composing function over the functions it owns: calls each with its configured count over the adapters and declarations its caller hands it, generic over the form interface, apart from the generate concrete's parity inputs, handing each set the typed mint, transfer, and next contexts, the identity and the other identity as `AdmittedKemIdentity` values, and the configured derivation context that set takes, and returns the vector sets; owns the sets' success and error types | `harness-crypto/delivery/mint`, `harness-crypto/vectors/cross_holder`, `harness-crypto/vectors/rerandomization`, `harness-crypto/vectors/non_convertibility`, `harness-crypto/vectors/malformed_capsules`, `harness-crypto/vectors/cross_set`, `harness-crypto/vectors/statement_mutations`, `harness-crypto/vectors/replay` | CR-08, CR-09, CR-11 vectors |
| `harness-crypto/measure` | Size, timing, and gas capture per curve, over the borrowed pairing, credential KEM, key agreement, delivery proof, randomness source, and verifier concrete its caller hands it, the typed mint and transfer contexts, the `AdmittedKemIdentity`, and the configured iteration count, an adapter over the operating system's monotonic clock as `harness-crypto/benchmark` is: for each iteration, fresh inputs from `harness-crypto/delivery/mint` and `harness-crypto/delivery/transfer`, untimed, then only the measured call timed, `prove_mint` over a fresh mint's statement and witnesses, `prove_transfer` over a fresh mint and transfer's statement and witnesses, `verify` over each relation's statement and proof, and `decapsulate` over the mint delivery's credential and identity element and a capsule encapsulated, untimed, from a drawn uniform, each timing the mean over the iteration count, one per operation per curve; the capsule, envelope, and each relation's proof bytes from the pairing's encoders over the objects it produced; its own mint and transfer submitted through the verifier concrete, an accepted delivery's cost read as L2 execution gas and L1 data fee and a rejected or reverted verdict returned with its relation | `harness-crypto/delivery/mint`, `harness-crypto/delivery/transfer`, `kem/factory`, `envelope/factory`, `proof/factory`, `pairing/factory`, `chain/create_chain_forms`, `random/factory`, `harness-crypto/verifier/factory` | CD-07, RO-03 |
| `harness-crypto/report` | The release-evidence report recording the measurements, the piece-group size and curve defaults in force, and any adjustment the data supports, selecting nothing against a threshold, written under the paths the configuration names | `harness-crypto/measure` | AS-21 |
| `harness-crypto/config/layers` | The function the configuration owns that produces the configuration document from layered sources: parses the command line through `clap` into its repeatable `--config <PATH>` and `--set <KEY=VALUE>` arguments, reads each named file through `std::fs`, parses the defaults text it is handed, each file's text, and each assignment as TOML, merges them over the defaults in command-line order, a table merging key by key and any other value replacing the earlier one whole, and deserializes the merged table into the configuration document; owns the argument and document types, each document table refusing a key it does not declare; returns the argument error, a file's read error with its path, a text's parse error, and an assignment that is not a key and a value, each carried unchanged; an app module in the adapter role, as the benchmark is over the clock; creates the `config` module; pins `clap` with its derive feature, `toml`, and `serde` with its derive feature, the one place the harness names any of them | `workspace/cargo` | Every configured value overridable from a configuration file and from a command-line flag, applied in order over the shipped defaults |
| `harness-crypto/config/admit_selections` | The function the configuration owns that admits the document's selections into the adapter families' own types: the randomness source, the encoding, hash-to-scalar, chain-forms, KEM, key-agreement, delivery-proof, and verifier concretes with their identifiers, the identity scope, the envelope algebra, and the delivery-statement version, each curve to run with its pairing concrete and the target-group encoding identifier the suite requires for it, each pairing concrete the benchmark measures with the target-group encoding identifier its curve requires, and the network profile's precompile encodings, each text value mapped to the variant it names and an unknown one refused with its document path; owns the admitted selection types; adds `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq` where they are absent to `RandomSourceKind`, `PairingConcrete`, `PrecompileEncoding`, `HashToScalarConcrete`, `HashToScalarIdentifier`, `KemConcrete`, `KemIdentifier`, `KeyAgreementConcrete`, `KeyAgreementIdentifier`, and `DeliveryProofConcrete` in their families' factory interfaces, so the configuration holds them and the run hands each curve and each benchmarked concrete its own copy | `harness-crypto/config/layers`, `random/factory`, `pairing/factory`, `encoding/factory`, `hash-to-scalar/factory`, `chain/create_chain_forms`, `kem/factory`, `envelope/factory`, `proof/factory`, `harness-crypto/verifier/factory` | The concrete and identifier per family and the pairing per curve every factory call takes, configured and never literal |
| `harness-crypto/config/admit_statements` | See [harness-crypto/config/admit_statements ownership](#harness-cryptoconfigadmit_statements-ownership) below this table | `harness-crypto/config/layers`, `harness-crypto/generate/solidity/challenge_vectors`, `domain/suite_identifier`, `hashing/factory`, `domain/parameter_set_identifier`, `encoding/factory`, `kem/factory`, `proof/factory`, `harness-crypto/generate/factory` | The context values, purposes, and identities every statement the harness proves carries, configured and never literal |
| `harness-crypto/config/admit_generate` | The function the configuration owns that admits the document's generate settings into `GenerateConcrete::Solidity { settings }`, the concrete named by its language: each vector kind's text value mapped to its `VectorKind`, its size through `VectorCount` and `MsmTermCount` with its message length, and its file, struct, decode function, and path constant names through `GeneratedFileName` and the render name types; the constants and vector-record libraries' file and library names, the Foundry project root, the generated directory's segments, the curve directories, the Solidity compiler version, and the license identifier through their own constructors; each refusal carried unchanged with its document path; admits each kind it is given and leaves the admission of the kind set against the concrete's declaration to `harness-crypto/generate/factory` | `harness-crypto/config/layers`, `harness-crypto/generate/solidity/render`, `harness-crypto/generate/factory` | The per-kind settings, names, paths, compiler version, and license the generator takes, configured and never literal |
| `harness-crypto/config/admit_output` | The function the configuration owns that admits the document's output destinations, the places the run's outcome is written, and the report's destinations: standard output, a file at a path the document carries, or both, each destination's text mapped to the variant it names and a file's path carried; an unknown destination and an empty destination list each refused with its document path; owns the admitted output types | `harness-crypto/config/layers`, `harness-crypto/report` | The run's outcome written where the caller configures, from a shipped default, a file, or a flag, never literal |
| `harness-crypto/config` | The configuration's composing function: hands the shipped defaults and the command line to `harness-crypto/config/layers`, then the document's parts to `harness-crypto/config/admit_selections`, then to `harness-crypto/config/admit_statements` with the identity scope `admit_selections` admitted, then to `harness-crypto/config/admit_generate` and `harness-crypto/config/admit_output`, and admits the benchmark's iteration count and the measurement's settings, returning the typed configuration every factory call and every write in the harness takes or the first refusal unchanged; owns the typed configuration and its error; carries `apps/harness-crypto/harness.toml`, the shipped defaults embedded in the binary, a default for every value the document declares, the generate concrete named by its language, `solidity`, the pairing concretes the benchmark measures, the generated directory `contracts/evm/foundry.toml` grants, the verifier's selection and the deployment record it reaches, the measurement's settings, the report's destinations, and the output destinations among them | `harness-crypto/config/admit_selections`, `harness-crypto/config/admit_statements`, `harness-crypto/config/admit_generate`, `harness-crypto/config/admit_output`, `harness-crypto/benchmark`, `harness-crypto/verifier/factory`, `harness-crypto/measure`, `harness-crypto/report` | The values every factory call and every write in the harness takes, configured and never literal |
| `harness-crypto/run/admit_contexts` | The function the run owns that admits a `CanonicalDeliveryContext` and its purpose into a `DeliveryContext<F, Purpose>`, generic over the form interface: the chain identifier, entitlement contract, source and target entitlements, old and new intervals, seller, and buyer rebuilt in transcript order through each form's `from_field`, the domain values and expiry taken from the canonical context; a form's refusal returned with its `DeliveryContextField`, or as the chain identifier's refusal, and the shared `FromFieldErrorReturn`, carried unchanged; the one place a canonical context becomes a typed one, so no later module rebuilds a form from a canonical context; moves `DeliveryContextField` out of `harness-crypto/generate/solidity/challenge_vectors`' interface into its own module, the type edit of the first file that consumes it | `harness-crypto/config/admit_statements`, `harness-crypto/generate/solidity/challenge_vectors`, `proof/factory`, `encoding/factory`, `chain/create_chain_forms` | Every context a statement carries admitted into the forms once, where the forms are resolved |
| `harness-crypto/run/generate_consumer` | The run's consumer of the generate family's factory, one type implementing `IGenerateConsumer` generic over the forms: handed the generate concrete the factory admitted, calls `emit` with the typed mint and transfer contexts, the mapping identities, and the proof identity as `AdmittedKemIdentity` values its constructor carries, and returns the files written or the concrete's refusal unchanged; names no concrete, calls no factory, and admits nothing | `harness-crypto/generate/factory`, `harness-crypto/delivery/mint`, `harness-crypto/config` | The generated directory written from the configured statements through a concrete the run never names |
| `harness-crypto/run/verifier_consumer` | The run's consumer of the verifier family's factory, one type implementing `IVerifierConsumer` generic over the forms: handed the verifier concrete, calls `harness-crypto/measure` over it and the adapters and declarations it carries, with the typed mint and transfer contexts, the proof identity as an `AdmittedKemIdentity`, and the configured iteration count, and returns the measurements or the first refusal unchanged; names no concrete, calls no factory, and admits nothing | `harness-crypto/verifier/factory`, `harness-crypto/measure`, `harness-crypto/delivery/mint`, `harness-crypto/config` | The deployed verifier's acceptance and cost reached through a concrete the run never names |
| `harness-crypto/run/delivery_proof_consumer` | The run's consumer of the delivery proof family's factory, one type implementing `IDeliveryProofConsumer`: handed the delivery proof concrete, calls `create_generate` over the adapters and declarations it carries with the configured generate concrete and the configured delivery-statement version, its consumer `harness-crypto/run/generate_consumer` constructed with the typed contexts and admitted identities it carries; then hands the adapters and declarations it carries, the typed mint, transfer, and next contexts, the identity and the other identity, and the configured derivation context to `harness-crypto/vectors`; then calls `create_verifier` with the configured verifier selection and deployment record, its consumer `harness-crypto/run/verifier_consumer` constructed with the typed contexts, the proof identity, and the configured iteration count; returns the files written, the vector sets, and the measurements, or the first refusal unchanged | `harness-crypto/run/generate_consumer`, `harness-crypto/delivery/mint`, `harness-crypto/generate/factory`, `proof/factory`, `harness-crypto/vectors`, `harness-crypto/verifier/factory`, `harness-crypto/run/verifier_consumer`, `harness-crypto/config` | The composed adapters handed to the generate factory with the configured values its params carry |
| `harness-crypto/run/key_agreement_consumer` | The run's consumer of the envelope family's factory, one type implementing `IKeyAgreementConsumer`: handed the key-agreement concrete, calls `create_delivery_proof` with the configured delivery-proof concrete and statement version, the envelope algebra the handed key-agreement concrete declares, and the verifier form the selected pairing declares, its consumer `harness-crypto/run/delivery_proof_consumer`; carrying the typed contexts, the admitted identities, and the configured verifier selection, deployment record, and measurement settings to the consumer it constructs | `harness-crypto/run/delivery_proof_consumer`, `harness-crypto/delivery/mint`, `proof/factory`, `envelope/factory` | The proof factory's admission of the configured selection exercised on every declared pairing |
| `harness-crypto/run/kem_consumer` | The run's consumer of the credential KEM family's factory, one type implementing `IKemConsumer`: handed the KEM concrete and the scope it admitted, builds an `AdmittedKemIdentity` for the proof identity, each mapping identity, and the other identity by borrowing the configuration's admitted identities, each paired with its setup scope, then calls `create_key_agreement` with the configured key-agreement selection, its consumer `harness-crypto/run/key_agreement_consumer`; carrying the typed contexts, the admitted identities, and the configured verifier selection, deployment record, and measurement settings to the consumer it constructs | `harness-crypto/run/key_agreement_consumer`, `harness-crypto/delivery/mint`, `envelope/factory`, `kem/factory` | The admitted scope and the admitted identities carried to every later stage |
| `harness-crypto/run/chain_forms_consumer` | The run's consumer of the chain family's forms function, one type implementing `IChainFormsConsumer`: handed the forms concrete, admits the configured mint, transfer, and next-settlement contexts with their purposes into typed contexts in those forms through `harness-crypto/run/admit_contexts`, a refusal returned unchanged, then calls `create_kem` with the configured KEM selection, its consumer `harness-crypto/run/kem_consumer`; carrying the typed contexts and the configured verifier selection, deployment record, and measurement settings to the consumer it constructs | `harness-crypto/run/kem_consumer`, `harness-crypto/run/admit_contexts`, `kem/factory`, `chain/create_chain_forms` | The forms every later stage is generic over, admitted by the configured identifier, and every context admitted into them once |
| `harness-crypto/run/encoding_consumer` | The run's consumer of the encoding family's factory, one type implementing `IEncodingConsumer`: handed the encoding concrete, calls `create_chain_forms` with the configured chain-forms selection, its consumer `harness-crypto/run/chain_forms_consumer`; carrying the configured verifier selection, deployment record, and measurement settings to the consumer it constructs | `harness-crypto/run/chain_forms_consumer`, `chain/create_chain_forms`, `encoding/factory` | The encoder every later stage borrows, admitted by the configured identifier |
| `harness-crypto/run/pairing_consumer` | The run's consumer of the pairing family's factory, one type implementing `IPairingConsumer`: handed a curve's pairing concrete, obtains the hash-to-scalar adapter through `create_hash_to_scalar` with the configured selection and calls `create_encoding` with the configured encoding selection, its consumer `harness-crypto/run/encoding_consumer`; carrying the configured verifier selection, deployment record, and measurement settings to the consumer it constructs | `harness-crypto/run/encoding_consumer`, `encoding/factory`, `hash-to-scalar/factory`, `pairing/factory` | Each configured curve composed through every family's factory |
| `harness-crypto/run` | The harness run, the composing function over the consumers it owns and the harness's composition root: obtains the randomness source through `create_random_source`; for each configured benchmark concrete, calls `create_pairing` with that concrete, its target-group encoding identifier, and the network profile's precompile encodings, its consumer a `PairingBenchmark` over the configured iteration count; then for each configured curve, calls `create_pairing` likewise, its consumer `harness-crypto/run/pairing_consumer`; returns the run's outcome, the benchmark's measurements per concrete and, per curve, the files written, the vector sets, the measurements, and the report `harness-crypto/report` records over them, or the first refusal unchanged; owns the outcome and its error; with its consumers, the harness's only caller of a family's factory; its public integration test runs the whole chain, its deployed-verifier cases over the configuration's deployment record | `harness-crypto/run/pairing_consumer`, `harness-crypto/report`, `harness-crypto/benchmark`, `harness-crypto/config`, `random/factory`, `pairing/factory` | Every family composed through its factory from the configuration's values; the benchmark's measurements per configured concrete |
| `harness-crypto/output` | The function the harness owns that writes the run's outcome, a success or a refusal, rendered as text, to each destination the configuration names, standard output, the file at the configured path, or both, in the configured order; an app module in the adapter role over the console and the filesystem, as the generate concrete is over the filesystem; returns a destination's write error with the destination it concerned | `harness-crypto/run`, `harness-crypto/config/admit_output` | The run's outcome, success or refusal, reaching every destination the caller configured |
| `harness-crypto/main` | The harness binary's entry point, `apps/harness-crypto/src/main.rs`, holding only what a binary's `main` keeps: calls `harness-crypto/config` with the shipped defaults and the command line, `harness-crypto/run` with the typed configuration, and `harness-crypto/output` with the run's outcome and the configured destinations, and exits with a success status only when the run succeeded and every destination was written; a configuration refusal, which arrives before any destination is admitted, is written to standard error; declares no type and holds no test, its correctness checked by reading, and so carries no element files; holds no literal | `harness-crypto/config`, `harness-crypto/run`, `harness-crypto/output` | The harness invoked from its shipped defaults, a file, and a flag; its first run writes the generated directory the contract suite builds against |
| `contracts/evm/PairingLib` | The Solidity library over the precompiles in both forms, with contract-side subgroup checks where the precompile does not perform them, its constants read from the committed constants library and its test vectors from the committed vector files, each file at the path and decoded by the function the committed vector-record library carries, all of which the harness binary wrote | `contracts/evm/IDeliveryVerifier`, `harness-crypto/generate/factory`; mirrors the pairing family's declared precompile encodings | CR-10 on chain |
| `contracts/evm/DeliveryVerifier` | Solidity mint and transfer verification over the statement fields, bit-for-bit with `proof/schnorr_fs`, implementing the interface `contracts/evm/IDeliveryVerifier` owns, and declaring the delivery-statement versions and verifier form under which the adapter registry binds it, its constants read from the committed constants library and its test vectors from the committed vector files through the committed vector-record library | `contracts/evm/PairingLib`, `contracts/evm/IDeliveryVerifier` | CD-03, CR-09 |
| `contracts/evm/deploy` | The deployment script for the verifier on each curve form to Anvil and Base Sepolia, emitting each deployed address as the deployment record, a runtime output the harness's later runs read as a configuration layer over the shipped defaults; exempt from the full support structure as a deployment script; carries the grouping's commit with the generated directory and the deployment record | `contracts/evm/DeliveryVerifier` | CD-07 |

#### chain/evm_forms ownership

- The EVM suite's forms as a private concrete of the chain family; the family's first concrete, private beneath the factory as every concrete is.
- The identity, entitlement, interval, and chain-identifier forms, over the twenty-byte, 256-bit unsigned, 64-bit unsigned, and 256-bit unsigned kinds the suite's schemas fix, each implementing the canonical-field contract.
- The identity form's address derivation, over a public key's canonical bytes and its scheme identifier, the type `domain/signature_scheme_identifier` holds, which depends on no signature crate.
- The signed-intent form, EIP-712 typed data over the call's fields, chain, contract, nonce, and expiry, shared by every EVM chain concrete.
- The type implementing the form interface that names these forms, setting its associated declaration constant.
- Consumes the canonical-field contract `ICanonicalField`: a form's declared field kind, its conversion to one canonical field value, and its fallible reconstruction from one with a typed refusal being that contract's.
- Authors, as its producers, in `adapters/chain`'s factory module: the forms identifier and `ChainFormsDeclaration`.
- Authors, after them, the chain family's form interface `IChainForms`, the trait carrying the identity, entitlement, interval, chain-identifier, and signed-intent forms as associated types each bounded by that contract, and the identity form declaring address derivation.
- Authors the family's mock, with the mock forms it names, address derivation, and the intent form.
- `ISettlementAdapter`, `IEntitlementStateAdapter`, and the remainder of the chain declaration wait for their first consumer in the chain and submission sprint.
- Creates the `adapters/chain` crate with dependencies on `adapters/encoding` and `domain`.

#### harness-crypto/generate/solidity ownership

- The Solidity concrete of the generate family, named and declared by the language it emits, one adapter generic over the form interface.
- The family's only module that touches the filesystem, an app module in the adapter role as the benchmark is over the clock.
- Constructed over the borrowed pairing, hash-to-scalar adapter, encoder, randomness source, credential KEM, key agreement, and delivery proof its caller obtained through their factories over the forms `create_chain_forms` handed it, with their declarations.
- Constructed with the delivery-statement version, the per-kind settings, the library names, the Foundry project root and the generated directory relative to it, and the Solidity compiler version and license identifier its libraries open with.
- Receives as payload the typed mint and transfer contexts, the mapping identities, and the proof identity as `AdmittedKemIdentity` values, admitted already by the run and the configuration, so it admits no identity and rebuilds no form.
- Calls each function it owns with each drawn kind's count.
- Renders the constants library through `render`.
- Encodes each vector record through the encoder under its canonical description into one line, `0x` and the encoding's lowercase hex digits and a line feed.
- Writes one vector file per kind its declaration names under the file name its per-kind settings carry.
- Renders through `render` the vector-record library declaring each kind's struct, decode function, and path constant under the names its per-kind settings carry with each file's path relative to the Foundry project root.
- Writes both libraries, all under the generated directory in the directory named from the curve the pairing's declaration carries.
- Authors, as its producers, the generate family's interface that emits a target suite's constants, vector-record types, and vector files from the Rust reference under a caller-supplied destination.
- The interface is generic over the forms it names as its associated `Forms` type, and its payload carries contexts in those forms.
- Authors the vector-kind identifier and the per-kind settings type.
- Authors the declaration naming the vector kinds it emits with each kind's sizing, drawn, enumerated, or over configured inputs, and gating each kind that needs second-group arithmetic on the verifier form the pairing declares.
- Authors the family's mock.
- Calls no factory and names no chain's forms, no path, no file, library, or symbol name, no count, no curve, no profile, no compiler version, and no license.

#### harness-crypto/config/admit_statements ownership

- The function the configuration owns that admits the document's network profile and statements.
- Admits the chain identifier, the entitlement contract, and the deployment record the verifier reaches as canonical field values, each a kind and a value the document carries, a fixed-width value as `0x`-prefixed hex of its exact width and an unsigned value within its kind's range.
- Admits the mint, transfer, and next-settlement contexts as `CanonicalDeliveryContext`s.
- Their suite identifier, asset root, and parameter-set identifier come through the domain constructors and the hashing family's root type's fallible constructor.
- Their entitlements, intervals, seller, and buyer are canonical field values, with their expiry; their chain identifier and entitlement contract are the network profile's.
- Admits the mint and transfer purposes through their declared codes.
- Admits the proof identity, each identity the identity mapping covers, and the other identity, each `0x`-prefixed hex, under the identity scope `harness-crypto/config/admit_selections` admitted.
- An identity is admitted as the root type under the asset scope, a length other than 32 bytes refused with that length and an all-zero root with the root type's refusal carried unchanged, and as canonical bytes under the entitlement scope, so no later module admits an identity.
- Each refusal is carried unchanged with its document path.
- Owns the admitted network-profile, statement, and identity types.
- Moves `CanonicalDeliveryContext`, with its builders, out of `harness-crypto/generate/solidity/challenge_vectors`' interface and mock into its own module, the type edit of the first file that consumes it.

### Protocol core and contract suite

The signature and swarm milestones are at ticket resolution; they depend on nothing the harness measures and run beside it from `encoding/factory` and `hashing/factory` onward, since their first tickets consume the encoding and hashing families, and each begins with the first concrete of its family, which carries the family's interface, and closes each family at its factory. The remaining milestones are sprints, re-mapped to tickets when the harness closes.

The hashing family is a harness ticket pair, `hashing/blake3_bao` and `hashing/factory`, at the head of the harness grouping beside the domain's identifier modules, because its root type is the asset's identity and the first type the encoding family describes.

#### Signature services, at ticket resolution

| Ticket | Owns | Depends on | Proves |
| --- | --- | --- | --- |
| `signature/ed25519` | The Ed25519 concrete with domain separation and complete-message binding; authors, as its producers, `ISignatureAdapter` with sign and isValidSignature over the public-key and signature types the concrete supplies as associated types, with their fallible constructors, no key recovery and no address derivation in the trait, the per-layer declaration of scheme identifier, the type `domain/signature_scheme_identifier` holds, key and signature lengths, recovery support, and layers served, and the family's mock; creates the `adapters/signature` crate; pins `ed25519-dalek` | `encoding/factory`, `random/factory`, `domain/signature_scheme_identifier` | CR-03 contract and CR-03 |
| `signature/secp256k1` | The secp256k1 concrete through `alloy`, declaring recovery support, signing the bytes the chain adapter's intent form hands it and owning no message format; pins `alloy` for the signature crate | `signature/ed25519` | CR-03, IW-07, LC-13 signature verification |
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
| `storage/redb` | The `redb` concrete; authors, as its producers, the key-value store interface with tables, transactions, and checkpoints, its types with their fallible constructors, the declaration, and the family's mock; creates the `adapters/storage` crate; pins `redb` | `encoding/factory` | XA-03 and SW-05 store contract; transactions and checkpoints under interruption |
| `storage/factory` | The storage-engine family's factory function with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete | `storage/redb` | The surface every index, the job engine, and the configuration registry consume |
| `seed-host/owned` | The owned concrete: persistent seeding independent of sessions, the root-keyed ciphertext store with holding reason, quota, and eviction, its index through the storage family, settings passthrough and enforcement, archive enumeration; authors, as its producers, `ISeedHostAdapter` with held-and-served, take custody, release, capacity and usage, and enumerate, the holding types with the obligated-versus-voluntary distinction and their fallible constructors, the capacity and holding-reason declaration, and the family's mock; creates the `adapters/seed-host` crate | `hashing/factory`, `encoding/factory`, `transport/factory`, `storage/factory` | SW-05 contract; SW-03, SW-05, SW-07, ST-06; PR-05 separation |
| `seed-host/rqbit` | The delegated concrete: the rqbit application driven through its HTTP API, root-to-identifier translation, and Bao custody challenges through the hashing family | `seed-host/owned`, `hashing/factory` | SW-04 |
| `seed-host/factory` | The seed host family's factory function with a branch per concrete, and the crate's public surface; carries the milestone's integration test across owned and delegated retrieval and its commit | `seed-host/owned`, `seed-host/rqbit` | The surface resolution and the First Finder consume |

#### Remaining milestones, as sprints

| Sprint by role | Contents | Depends on |
| --- | --- | --- |
| Domain model | The protocol's remaining types, each authored in the node of the file that first consumes it and housed in the module that implements it, fixed when that module's ticket is written, the identifier modules being harness tickets and the derivation context a sidecar-layer ticket; the `domain` crate holds only the types its own modules implement and no asset identity and no derivation context; for each encoded type the `domain` crate holds, its canonical description as the encoding family's module `encoding/<type>`, implementing the factory's contract for that type, one module per type, authored after the type and before the first ticket that encodes it; a type housed in a crate that depends on the encoding family carries its canonical description as a module of that crate, as `envelope/possession_statement` and `proof/mint_statement` do | the types the harness families own; `encoding/factory` |
| Payload cipher and sidecar layer | `cipher/aes_ctr` with the counter layout and extent rules, authoring `IPayloadCipherAdapter`, the counter-layout and addressable-extent declarations, and the family's mock as its producers and creating the `adapters/cipher` crate, then `cipher/factory`; `workflows/sidecar/build`, per live set a capsule and wrapped piece-group key per group committed by that set's Bao root, each sidecar its own object, and `workflows/sidecar/validate`, the manifest, hash-card, and sidecar validation ordering; the `sample_deployment` generator for the site demonstration, which needs the cipher and so lives here rather than in the harness and carries the configured piece-group size default | domain model; `hashing/factory`; `workflows/sidecar/wrap`; `kem/factory` |
| Registry and entitlement contracts, the EVM suite | See [Registry and entitlement contracts, the EVM suite ownership](#registry-and-entitlement-contracts-the-evm-suite-ownership) below this table | harness contracts, `contracts/evm/IDeliveryVerifier` among them; domain model |
| Chain and submission families | `chain/base` with the contract bindings, Base's tier mapping, `evaluateAuthorization` and batch with per-context bindings, and the event and calldata reader, authoring the chain interface, `ISettlementAdapter`, `IEntitlementStateAdapter`, the remainder of the declaration, and their mock as its producers in the factory module `chain/evm_forms` created, binding as its associated forms the forms `create_chain_forms` resolves, the signed-intent form among them, and authoring no form, its declaration naming the forms identifier it binds; `chain/quorum_view`, the family-owned aggregation of two of three configured nodes at a common reference, stale ignored, divergence and views older than `τ_soft` failing closed; `chain/factory`, the chain factory function constructing the chain concrete the configuration names, admitting it only when its declaration lists the forms identifier the resolved forms carry, with its `base` branch, authored once after the concrete; `submission/self_funded`, the self-funded call path, authoring the interface for submitting a signed intent and its declaration and creating the `adapters/submission` crate; `discovery/seeder_map`, the on-chain seeder map as a discovery source through the chain factory, then `discovery/factory`, the discovery family's factory function resolving every source the composition admits behind the aggregation, with a branch per concrete, carrying the family's integration test across factory and concretes | registry and entitlement contracts; `signature/factory`; `discovery/local`, `discovery/dht`, `discovery/pex`, `discovery/tracker`, `discovery/aggregate` |
| Adapter registry and factory, on chain | The `AdapterRegistry` governed by a single project-held key and the factory that constructor-injects the identity adapters, the signature adapter per scheme, the attestation verifier per source, the entitlement token form, the delivery verifier per delivery-statement version and verifier form, and the payment-asset adapter per payment asset, each immutable once bound, the registry contracts reaching the delivery verifier and the payment-asset adapter only through those bindings | registry and entitlement contracts |

#### Registry and entitlement contracts, the EVM suite ownership

- Under `contracts/evm`, the suite in its declared source language.
- Asset records keyed by the asset's root, in place of any per-name version index.
- Append-only alias records under a declared namespace and an opaque key that no Registry field interprets, each entry binding the key to one root and carrying the source's signed statement or its recorded absence, never rebound or removed.
- Payment locks carrying the payment-asset identifier beside the price, moving, releasing, and refunding payment only through `IPaymentAsset` as the adapter registry binds it per identifier, with the native-asset concrete as the MVP's.
- The claim verifier's key record as a scheme identifier and a commitment to the key, with the key in calldata and vouchers verified through the signature adapter the registry binds for the scheme.
- Deployments and hash-cards carrying the commitment-scheme identifier and length-carrying or committed width fields.
- Parameter sets, envelope keys, identity elements, and envelopes held as suite-tagged bytes with a digest and interpreted only by the delivery verifier the adapter registry binds.
- Attestation verification through the `IAttestationVerifier` adapter bound per source, with the npm P-256 verifier as the first concrete and the source-key table it reads.
- Later escrow deployments.
- Parameter-set liveness with the sidecar-coverage rule and sidecar addition.
- The envelope-key registry.
- `IEntitlement` with the ERC-721 concrete carrying interval state and the ownership override that disables standard transfers.
- Issuance and transfer calling the delivery verifier through its registry binding.
- Batched grant requests readable by holders and closed by grant or withdrawal.
- Escrow records per deployment.
- Identity binding storing the scheme identifier and a commitment to the handshake key.
- The identity account, a contract account under ERC-1271 on the EVM suite, whose signer set is its device registry keyed by scheme-tagged key identifiers verified through the signature adapter the registry binds per scheme, with the `isDeviceAdmitted` view.
- Claim-set state layout, batch and paginated views.
- Every identity-bound mutation taking the acting identity and a signed intent verified through the account against a signer whose permissions cover it.
- The contracts enforcing lock expiry and no volume limit.

### Local daemon and package serving, as epics

| Epic by role | Contents | Depends on |
| --- | --- | --- |
| Durable jobs and configuration | `telemetry/local_metrics` over the storage family, authoring the exporter interface, the declaration, and the family's mock as its producers and creating the `adapters/telemetry` crate, then `telemetry/tracing`, the family-owned tracing setup carrying per-request correlation identifiers, then `telemetry/opentelemetry`, the OpenTelemetry exporter concrete over the same interface, then `telemetry/factory`, with a branch per concrete; `workflows/jobs`, the job engine over the storage family with checkpointing and idempotent restart; `workflows/config`, the versioned configuration registry over the storage and encoding families; `platform-paths/linux`, authoring the application-directory interface and creating the crate, `platform-paths/macos`, `platform-paths/windows`, and `platform-paths/factory` for the store-root defaults; the `ipc` family's `ipc/framing` and `ipc/principals`, family-owned and creating the crate, `ipc/unix_socket` authoring the IPC transport interface, `ipc/named_pipe`, `ipc/server` with the control-principal and local-presence rules, and `ipc/factory`; `lifecycle/lock`, the single-instance lock as a family-owned function creating the `adapters/lifecycle` crate, the family's concretes and factory following in the installation coordinator | domain model; `storage/factory` |
| Identity and custody | `custody/holder_seed` with the control and envelope branches, family-owned and creating the `adapters/custody` crate; `custody/device_key`; `custody/local_keystore` over `keyring` with version-headed blobs, authoring `IKeyCustodyAdapter`, the versioned capability set including device roles, and the family's mock as its producers; `custody/upgrade`, the in-place migration between concretes verified against the chain, through the interface; `custody/factory`; `wallet/eip1193`, authoring the signing-only wallet interface and creating the crate, `wallet/walletconnect`, and `wallet/factory`; `identity/publisher_authority`, authoring `IIdentityAdapter` with the binding schema and creating the crate, `identity/escrow`, and `identity/factory`; `workflows/identity` with identity creation, relayer-paid binding and contract account creation, envelope-key registration, device pairing, signer admission, and revocation | the signature tickets; the chain and submission families; the relayer or paymaster milestone, for `submission/factory`; `envelope/factory`; `kdf/factory`; durable jobs and configuration |
| Plaintext CAS and package host | `cas/filesystem`, verified atomic CAS of tarballs extracted per project by the package manager with nothing linking into the store, quota, pinning, eviction, its index through the storage family, authoring the store interface and creating the crate, then `cas/factory`; `package-host/npm`, the npm registry protocol served locally with canonical tarball URLs and no metadata store, authoring the npm name type, `IPackageHostAdapter` with the namespace it declares, and the name-to-root alias lookup, which selects an alias entry only on the client's evidence, a signed statement checked against the lockfile `integrity` or the source, or an unsigned entry decrypted and compared, and fails closed on an alias without evidence naming the competing roots, the boundary PR-02 proves, and assembling packuments from the alias records, the decrypted tarballs' own `package.json`, and the entries' signed statements, the boundary PR-09 proves, and creating the crate, then `package-host/factory` | durable jobs and configuration; `hashing/factory`; `encoding/factory` |
| First Finder ingest | `ingest/npm` with attestation validation or recorded absence, writing the alias entry beside the root with the source's signed statement or its recorded absence and nothing the bytes contain, and availability as eligibility, authoring `IIngestSourceAdapter` with the same namespace the package host declares and creating the crate, then `ingest/factory`; `workflows/first_finder` with the foreground serve, the background bootstrap job, the state-locked registration race, escrow custody of the master scalar, seeding, and the finder's grant of the asset's first entitlement to itself | plaintext CAS and package host; the swarm tickets; the registry contracts; the chain and submission families; the relayer or paymaster milestone, for `submission/factory`; identity and custody; `kem/factory`, `envelope/factory`, and `proof/factory`; `workflows/sidecar/build` |
| Credential delivery and per-attempt authorization | `payment/native`, authoring `IPaymentAssetAdapter`, the payment-asset identifier, the declaration of the assets a concrete settles, and the family's mock and creating the `adapters/payment` crate, then `payment/factory`; envelope-key registration on first run; `workflows/request`, the batched grant request job and its pickup from chain events; `workflows/prefetch` with pinning, seeding, and set matching; `workflows/acquire` with mint and grant delivery, holders fulfilling requests for absent requesters, and sale delivery, reaching payment only through `payment/factory`; credential load and recovery; `workflows/grant`, the grant service under the asset scope, fulfilling open requests from a held credential after the credential engine and the request job; `workflows/attempt` with `τ_soft` and `τ_wallet`; `workflows/decrypt` with decapsulation through the KEM factory, unwrap through `workflows/sidecar/unwrap`, and decryption through the cipher factory; `workflows/interval_end`; `workflows/gate` | the chain and submission families; the relayer or paymaster milestone, for `submission/factory`; identity and custody; `kem/factory`, `envelope/factory`, and `proof/factory`; payload cipher and sidecar layer; First Finder ingest |
| Resolution orchestrator | `workflows/resolve` with the hedged source order under the source deadline and grant wait its params carry: the CAS, ciphertext under a held credential through the attempt engine and the decryption pipeline, the seed host and the swarm, upstream through the ingest factory for any identity without a credential, and a bounded wait on a requested grant when upstream is down; per-asset independence status in `workflows/health` | plaintext CAS and package host; the swarm tickets; First Finder ingest; credential delivery and per-attempt authorization |
| Daemon composition | `workflows/compose`, capability resolution across every factory the daemon and the installer compose, failing closed; `workflows/settings`, the settings service and the catalogue of every module's settings; the daemon binary, the daemon's composition root; the installer binary | resolution orchestrator; identity and custody; installation coordinator; explicit publisher path; claim verifier and escrow claim |
| Demonstrable milestone | No new subsystem: a second identity on a second machine profile installs a pinned closure the First Finder ingested, at the registry's speed with requests registered and ciphertext prefetched, then with the registry unavailable, through the packaged daemon and package host over the swarm, on a grant fulfilled while it was offline; the north star, first-run independence, and install wall-clock measured on the dogfood population | daemon composition; the swarm tickets |

### Onboarding shells and services, as milestones

| Milestone by role | Contents | Depends on |
| --- | --- | --- |
| Installation coordinator | `workflows/install`, the durable install plan with platform detection, repair, update, uninstall, the health probe, the consent items, and the first-run cost disclosure; `lifecycle/systemd`, authoring the service-lifecycle interface, `lifecycle/launchd`, `lifecycle/windows_service`, `lifecycle/scheduled_task`, and `lifecycle/factory`; `artifact-verifier/sigstore`, authoring the signed-artifact verification interface and creating the crate, `artifact-verifier/authenticode`, `artifact-verifier/apple_notarization`, and `artifact-verifier/factory`; `redirect/npm`, authoring the detect, redirect, back-up, and restore interface and creating the crate, then `redirect/factory` | durable jobs and configuration; identity and custody |
| Visual Studio Code extension and npm bootstrap | Thin shells over the coordinator | installation coordinator |
| Desktop application and CLI | Tauri and Rust control surfaces | installation coordinator; daemon composition, for the settings catalogue |
| Relayer or paymaster | The relayer binary with the chosen provider's bundler concrete, authoring the bundler-provider interface, and `relayer/bundler/factory`; free-path sponsorship under a global per-window budget and maximum liability with per-identity limits as one layer; the sponsorship mechanism for the identity contract account chosen here under LC-13; `submission/paymaster` and `submission/relayer`, authored as the chosen mechanism requires and depending on no module of `apps/relayer`, since the relayer service depends on the submission family, the relayer service's request and response types authored in `submission/relayer`'s node as their first consumer; then `submission/factory`, with a branch per submission concrete; then the relayer service; sponsored device admission and revocation; refusal to sponsor a paid acquisition, stated against the payment asset the lock names; cost and budget reporting | the chain and submission families sprint; registry and entitlement contracts |
| Explicit publisher path | `workflows/publish` with the publisher-derived lineage, swarm-native publication of the dependency closure, and the issuance policy service; `publisher-proof/provenance`, authoring `IPublisherProofAdapter` and creating the crate, `publisher-proof/maintainer_oauth`, and `publisher-proof/factory` | First Finder ingest; identity and custody |
| Claim verifier and escrow claim | The claim verifier binary signing under a custody-held key of a declared scheme, revocable and registered on chain as a scheme identifier and a key commitment; `claim-verifier/attestor`, authoring `IClaimVerifierAdapter`, the voucher types, and a declaration naming the scheme its vouchers carry, and creating the crate, then `claim-verifier/factory`; `workflows/claim`; the claim set established from upstream metadata at verification; claim-set vouchers; the claimant parameter set; optional handover; sidecar addition; voluntary migration | registry contracts; explicit publisher path; credential delivery |
| Project seed host and site | Persistent first seeder of the core closure holding an entitlement and credential for every asset it seeds; static site; WebAssembly demonstration | `seed-host/factory`; explicit publisher path; payload cipher and sidecar layer, for the sample deployment |
| Observability | RO-03 metrics, RO-04 correlation, RO-05 health, RO-06 attempt latency and install wall-clock reported against the values in force | every component that emits |
| Demonstration harness | Controlled participants, wallets, chain state, failures, and the deliberately incompatible adapter declarations the incompatibility scenario rejects, refused by the factories and by `workflows/compose` | every component under test |

### Acceptance and release, as objectives

| Objective by role | Contents | Depends on |
| --- | --- | --- |
| Transaction flow proof | Priced primary issuance and secondary transfer through the packaged applications on the Base mainnet pilot, settling in Base's native asset through the bound payment-asset adapter | explicit publisher path; credential delivery; relayer; daemon composition; external review dispositioned |
| Acceptance scenarios | Every acceptance scenario on clean machines through packaged applications | every milestone |
| External cryptographic review closed | Findings dispositioned before any priced deployment | harness report |
| Legal prerequisites | License text, regulatory review, export confirmation | none technical |
| Dogfood baseline and release | ChainTorrent published swarm-natively; baseline metrics recorded; release evidence complete | acceptance scenarios; review; legal |

## Integration Points

Where one component's output becomes another's input across a boundary that an integration test must cross, and the milestone whose integration test crosses it.

| Integration point | Producer | Consumer | Boundary crossed | Crossed in |
| --- | --- | --- | --- | --- |
| Factory admission | each family's concretes, through their declarations | the factory of every downstream family, and `workflows/compose` | A concrete admitted only when its declaration satisfies every downstream requirement; refused at construction with no side effect | The harness, for the cryptographic families; the daemon composition, for the whole composition |
| Verifier parity | `proof/schnorr_fs` | `contracts/evm/DeliveryVerifier` through `harness-crypto/verifier/evm`, bound to the suite's ABI description | Rust to chain; bit-for-bit acceptance on every vector | Delivery proof and on-chain verifier |
| Precompile encoding | the pairing factory's declared encodings, produced by every pairing concrete | `contracts/evm/PairingLib` | Rust encoding to precompile input | Delivery proof and on-chain verifier |
| Piece-group key | the KEM factory's decapsulate and `workflows/sidecar/unwrap` | the cipher factory | The decapsulation-to-cipher seam preserved for a future multi-key suite | Payload cipher and sidecar layer |
| Sidecar root | the KEM factory's encapsulate and `workflows/sidecar/build` | commitments and hash-card, through the hashing factory | Per live set, capsule and wrapped key per group committed by that set's Bao root; each sidecar its own object | Payload cipher and sidecar layer |
| Discovery engine | the transport factory's engine capability | `discovery/dht`, `discovery/pex`, `discovery/tracker` | Library-backed sources reach the engine through the generic surface, never through the transport concrete | Swarm transport and seed host |
| Store | the storage factory | `workflows/jobs`, `workflows/config`, the CAS index, the ciphertext index, `telemetry/local_metrics` | Tables, transactions, and checkpoints with no store library named by a consumer | Swarm transport and seed host, for the ciphertext index; durable jobs and configuration, for the job engine, the configuration registry, and the local metrics; plaintext CAS and package host, for the CAS index |
| Envelope in settlement | the envelope factory's wrap and the proof factory's provers | registry and entitlement contracts, through the submission factory | Calldata and event; digest stored | Relayer or paymaster |
| Signed intent | the chain family's signed-intent form, signed through the signature factory | registry and entitlement contracts, through the submission factory | A signed intent to the account's verification of the signer's scheme, EIP-712 typed data and ERC-1271 on the EVM suite | Relayer or paymaster |
| Attestation verification on chain | the source's attestation, carried by `ingest/npm` | `IAttestationVerifier` bound per source by the adapter registry | The registration refused unless the source's verifier accepts, absence admitted and recorded | First Finder ingest |
| Attempt context | domain model | `chain/quorum_view` over `chain/base`, and credential delivery | View call at the declared tier from a quorum of configured nodes | Chain and submission families, for the quorum view; credential delivery and per-attempt authorization, for the attempt |
| Resolution order | resolution orchestrator | the CAS, seed-host, transport, and ingest factories | Local, encrypted, swarm, upstream, hedged | Resolution orchestrator |
| Package-manager protocol | `package-host/npm` | npm | HTTP registry protocol; resolution stays in npm; canonical tarball URLs so lockfiles stay portable | Plaintext CAS and package host |
| Persistent credential | the envelope and custody factories | credential delivery | Stored only under `IKeyCustodyAdapter` | Credential delivery and per-attempt authorization |
| Device admission | identity manager on a root | identity contract account; the attempt engine's wallet-control assertion | Signer admitted and revoked on chain; admission read in the same state view as authorization | Identity and custody, for the identity contract account; credential delivery and per-attempt authorization, for the wallet-control assertion |
| Grant request to holder | request job, through the submission factory | registry request record; any holder's grant service | Batched sponsored request; holder reads open requests and fulfils them for absent requesters | Credential delivery and per-attempt authorization |
| Grant pickup from events | registry mint and grant events, through `chain/base` | request job and credential engine | Envelope recovered from chain history on the next run; set matched against held deployments | Credential delivery and per-attempt authorization |
| Installer to daemon | installation coordinator | daemon, package host, seed host, through the lifecycle and IPC factories | Service lifecycle and IPC | Installation coordinator |
| Artifact authentication | the release's signatures | the artifact-verifier factory | Every artifact and update verified before execution, under each scheme's concrete | Installation coordinator |
| Shells to coordinator | extension, npm bootstrap, desktop, CLI | installation coordinator | One coordinator, identical postcondition | Extension and npm bootstrap; desktop and CLI |
| Relayer to contracts | relayer, through its bundler factory | identity binding, requests, mint | Sponsored transactions under policy | Relayer or paymaster |
| Verifier voucher to contract | `claim-verifier/attestor` | escrow claim contract surface, through the signature adapter the adapter registry binds for the key's scheme | Voucher bound to claimant, set, contract, chain, nonce, expiry, verified against the registered scheme identifier and key commitment | Claim verifier and escrow claim |
| Payment settlement | `payment/factory`, reached by `workflows/acquire` | the lock and settlement contracts, through the payment-asset adapter the adapter registry binds for the lock's payment asset | Lock, release, and refund in the named payment asset; a lock naming an unbound asset refused | Registry and entitlement contracts, for the lock and the bound adapter; credential delivery and per-attempt authorization, for acquisition through the factory |
| Alias lookup | the registry's append-only alias records, written by `ingest/npm` with the source's signed statement or its recorded absence | `package-host/npm` | A request's name to the asset's root, the entry selected only on the client's evidence, a signed statement checked against the lockfile `integrity` or the source or an unsigned entry decrypted and compared, the decrypted bytes verified against the selected digest before commit, and an alias without evidence failing closed | Plaintext CAS and package host, for the lookup and packument assembly; registry and entitlement contracts, for the alias records and the root-keyed asset record |
| Site demonstration | WebAssembly build of the client crates | browser | Same crates as the client; no service on the read path | Project seed host and site |

## Conflict Flags

- **Grouping membership versus dependency order.** The signature and swarm tickets sit in the protocol core grouping and depend on no contract sprint, so they run in parallel with the harness and before the contracts.
- **Every factory follows its concretes.** A family's generic interface, declaration, and mock are authored in its first ticket, so the declarations the resolver reads exist from the family's first ticket; the factory function consumes the concretes it constructs and is authored once after the family's last concrete; and no consumer ticket depends on a concrete. The chain family's forms are constructed by a family-owned function, `chain/create_chain_forms`, not by the factory, because the factory module holds one function and that function constructs the chain concrete; the function is authored in the harness after the forms concrete, and the factory is authored once in the chain and submission sprint after the Base concrete.
- **Each family's factory follows its last concrete.** `discovery/factory` follows `discovery/seeder_map` in the chain and submission families sprint, since nothing consumes it before the daemon grouping; `telemetry/factory` follows `telemetry/local_metrics` and `telemetry/opentelemetry` in the durable jobs epic, the OpenTelemetry concrete needing only the exporter interface; `submission/factory` follows every submission concrete in the relayer or paymaster milestone, which precedes every daemon epic that submits a signed intent; the lifecycle family's lock precedes its concretes and factory, which arrive together at the installation coordinator.
- **The harness needs a verifier client before the chain family exists.** The daemon never submits a proof to the verifier, since the contracts call it inside mint, grant, and delivery, so the verifier client is the harness's own `verifier` family with an `evm` concrete, and no chain concrete carries one.
- **The harness's code order and its runtime sequence differ.** The delivery verifier reads the generated directory a run of `harness-crypto/main` writes, and the run's verifier stage reaches the deployed delivery verifier; neither is a code dependency. `harness-crypto/verifier/evm` binds the delivery verifier's ABI description, which the contract suite's own node, `contracts/evm/IDeliveryVerifier`, owns beside the Solidity interface, the Foundry configuration, and the Solidity continuous-integration definition; the Rust verifier client authors no contract source and no toolchain file, and speaks the EVM ABI, which is why it keeps its name while the generator is named for the language it emits; the contracts need the generated directory to be built and proven, and the verifier needs the deployed contract to run. The table and the graph hold the code order; the [Runtime sequence](#runtime-sequence) section holds the runtime sequence.
- **The chain family's forms concrete precedes its chain concrete by a sprint.** The proof family's transcript descriptions, challenge, concrete, and factory are generic over the chain family's identity, entitlement, interval, and chain-identifier forms, and the generator, verifier client, vectors, and measurement need forms the deployed verifier reproduces; none may name a concrete, so `chain/evm_forms`, the family's first concrete, supplies the EVM suite's forms, `chain/create_chain_forms`, a family-owned function module, follows it, admitting a forms concrete by the identifier the suite requires and handing it to a consumer generic over the form interface, since the factory module holds one function and that function constructs the chain concrete, and every one of those consumers follows the function. The forms are the suite's, not the network's: the EVM suite is one suite shared by every EVM chain concrete, Ethereum mainnet being a later adapter of the same suite. `chain/base`, the first consumer of the settlement and entitlement-state operations, authors those and the remainder of the declaration in the chain and submission sprint, binds the forms the function resolves, authors no form, address derivation and the signed-intent form being `chain/evm_forms`' and shared by every EVM chain concrete, and `chain/factory` is authored then, once, admitting a chain concrete only when its declaration lists the forms identifier the resolved forms carry.
- **The harness composes through configuration.** Nothing in the harness names a path, a curve, a concrete, a form, a version, or a network profile. `harness-crypto/config` carries every such value as a shipped default, overridable from a configuration file and from command-line flags, and `harness-crypto/run` and its consumers pass each as params through the factory it belongs to; the generator writes under the destination its params carry, an app module in the adapter role reached only through its factory, and the statements' context values arrive as payload from the configured profile.
- **The generate concrete is an adapter over functions it owns.** Rendering, the constants, and each kind of vector are functions beneath `harness-crypto/generate/solidity`, each its own ticket ahead of the adapter, as `proof/schnorr_fs/challenge` is beneath `proof/schnorr_fs`, because one adapter file holding all of it exceeds the size at which a file is decomposed. `harness-crypto/generate/solidity/render` owns the entry type the constants function returns, the record-declaration type, and the validated name types, owns no table from canonical field kinds to type names, which the encoding family's ABI concrete declares, and takes each record member's type name as data resolved by the caller; each vector function returns typed records with their canonical descriptions and member names; the adapter alone encodes records into lines, renders the libraries, and writes files.
- **The vectors are a composing function over functions it owns.** Each vector set is a function beneath `harness-crypto/vectors`, its own ticket ahead of it, because one file holding all of them exceeds the size at which a file is decomposed; each function owns its record type, and the composing function owns the sets' return.
- **The configuration is a composing function over functions it owns.** Layering the sources, admitting the family selections, admitting the network profile and statements, admitting the generate settings, and admitting the output destinations are functions beneath `harness-crypto/config`, each its own ticket ahead of it, because one file holding all of it exceeds the size at which a file is decomposed. `harness-crypto/config/layers` alone parses the command line, reads files, and names `clap`, `toml`, and `serde`, and owns the document every admission function reads; each admission function owns the admitted types it returns and maps the document's text values to the types their owning families declare; `harness-crypto/config` composes them, owns the typed configuration, and carries the shipped defaults. The selection types the configuration holds and the run hands each curve and each benchmarked concrete a copy of carry `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq`, added in their families' factory interfaces by `harness-crypto/config/admit_selections`, the first file that requires them.
- **Vectors are committed files, one encoded record per line, ABI-encoded and language-neutral.** A further contract language is a further generator concrete over the same vector files. Solidity holds no constant array, and a counted kind must be iterable by a Foundry test whatever count the configuration carries, so each vector kind is a file the adapter writes, each line the encoding family's encoding of one record as `0x`-prefixed hex; a test reads it with `vm.readLine`, converts each line with `vm.parseBytes`, and decodes it with the kind's function from the generated vector-record library, which also carries the file's path. One record per line needs no list kind in the encoding contract. The decode function decodes over the members' types, never over the struct, since a struct with a dynamic member encodes behind an offset and each line holds the parameter encoding of the fields.
- **Vector kinds, counts, and names are declared and configured.** The generate concrete's declaration names the kinds it emits and each kind's sizing: a drawn kind holds the count the configuration carries for it, an enumerated kind holds exactly what it enumerates, and a kind over configured inputs holds one record per input; every file, library, struct, function, and constant name is the configuration's, with shipped defaults; the generate factory admits a configuration only when its per-kind settings name exactly the declared kinds, with a count for each drawn kind and none for any other. A further target's generate concrete declares its own kinds, and no consumer holds a count, a kind list, or a name.
- **The harness run alone calls the factories.** `harness-crypto/run` and the consumers it owns nest each factory's consumer inside the previous one and hand the adapters and their declarations to the generate and verifier factories, which construct their concretes over them, and to the vectors and measurement modules; no other harness module calls a factory, and each reads a tag, a form, or a version from what it is handed.
- **The run is a composing function over consumers it owns.** A family's factory hands the concrete it constructs to a consumer rather than returning it, because the concrete's type carries the borrowed adapters' types, so composing the families is a chain of consumers, each implementing one factory's consumer trait and calling the next factory with the next consumer: pairing, then hash-to-scalar and encoding, chain forms, KEM, key agreement, delivery proof, and generate. One type per file admits one consumer per file, so each consumer is its own ticket beneath `harness-crypto/run`, authored innermost first since each outer consumer constructs the one inside it, as `harness-crypto/benchmark` is the pairing family's consumer in its own file. `harness-crypto/run` owns the randomness source's construction, the benchmark's calls, each curve's call to the pairing factory, and the outcome every call returns; the verifier, vectors, measurement, and report tickets precede the configuration and every consumer, so each consumer is authored with every value it carries and `harness-crypto/run/delivery_proof_consumer` with every call it makes.
- **The binary holds only what a binary's `main` keeps.** A binary's `main` cannot be called by a test, so, after the Rust book's separation of concerns for binary projects, the logic lives in the library and `apps/harness-crypto/src/main.rs` calls `harness-crypto/config`, `harness-crypto/run`, and `harness-crypto/output` in turn and sets the exit status from their results; it declares no type and holds no test, so it carries no element files and is checked by reading.
- **The harness's output is configured, as its inputs are.** The run's outcome, a success with the benchmark's measurements and the files written or the first refusal, is written by `harness-crypto/output` to each destination the configuration names, standard output, a file at a configured path, or both, so a caller chooses where a run reports by the same shipped default, file, and flag that set every other value; the shipped default is standard output. A configuration refusal arrives before any destination is admitted, so the binary writes it to standard error, and every refusal ends the process with a failure status.
- **The benchmark's concretes are configured.** The benchmark measures every pairing concrete so the default per curve can be set from its measurements, and the run names no concrete, so the configuration carries the concretes the benchmark measures, each with the target-group encoding identifier its curve requires, every pairing concrete in the shipped defaults; the run hands each to the pairing factory with a `PairingBenchmark` as its consumer, and the outcome carries each concrete's measurements to the output.
- **The challenge vectors are standalone transcripts.** The proof family's challenge function is private to its crate, and a proof in the second-group form carries no first messages, so `harness-crypto/generate/solidity/challenge_vectors` builds its own transcripts, encodes each through its public description, and maps the encoding under the challenge tag the declaration carries; the proofs' own challenges travel in the proof vectors.
- **An identity encodes as `bytes20`.** The encoding concrete encodes the fixed twenty-byte kind as a right-padded `bytes20`, so the constants render it as `bytes20` and `contracts/evm/DeliveryVerifier` encodes every identity field of a transcript as `bytes20`; an `address` is left-padded and yields another challenge.
- **A generated library's compiler version and license identifier are configured.** A Solidity source file opens with a license identifier and a compiler version pragma, neither of which is a library name or an entry, so `harness-crypto/config` ships a default for each, `harness-crypto/run/delivery_proof_consumer` passes them through the generate factory's params, the generate concrete holds them from construction, and `harness-crypto/generate/solidity/render` takes them as params and holds neither as a literal.
- **The curve directory is read, not chosen.** The adapter names each generated directory from the curve the resolved pairing's declaration carries and selects no curve.
- **The harness's parameter-set digest and identity bytes are configured.** No harness ticket computes a parameter-set digest or describes an entitlement identifier, and the delivery verifier takes the identity element as an input, so both arrive as configured values; the registry and entitlement contracts sprint binds them to the record. The entitlement contract a harness transcript names is the network profile's configured address whether or not a suite is deployed there when the generator runs, since the harness exercises the delivery verifier apart from the registry; in operation the registry and entitlement contracts supply every statement field from their own records, the stored envelope digest, the registered keys, the identity element, and the interval counter among them, and refuse a field the record contradicts, which LC-05 and AS-13 prove at the registry and entitlement contracts milestone and again through the daemon at credential delivery.
- **The generated directory is committed.** The Solidity continuous-integration definition runs only `forge build`, `forge fmt --check`, and `forge test`, so the constants library, the vector-record library, and the vector files `contracts/evm/PairingLib` and `contracts/evm/DeliveryVerifier` read exist in the repository when it runs; `contracts/evm/foundry.toml` grants `fs_permissions` read access to the generated directory and the ABI description directory and to nothing else, and the configuration's shipped default for the generated directory is the directory it grants; `ffi` stays disabled, and `via_ir = true` compiles every source through the IR pipeline, since a proof record's decode function destructures more values than the legacy pipeline's sixteen reachable stack slots hold; the first run of the harness binary from its shipped defaults writes them through the generator, `harness-crypto/run/delivery_proof_consumer` calling the generate factory before the verifier factory, and returns the verifier's refusal, nothing being deployed at the configured address; `contracts/evm/deploy` carries them in its commit with the deployment record it emits.
- **The wrap precedes the cipher.** The harness's cross-set vector needs the wrap and unwrap, so `workflows/sidecar/wrap` and `workflows/sidecar/unwrap` are harness tickets and create the workflows crate; `workflows/sidecar/build` and `workflows/sidecar/validate` wait for the cipher in the protocol core.
- **The benchmark follows the pairing concretes.** It constructs each through the pairing factory and names no library; it records each concrete's measurements, and every concrete remains beneath the factory whichever one the configuration names.
- **The benchmark sets configuration, not code.** The harness configuration names the pairing concrete per curve, and the factory constructs the concrete the composition names; the benchmark's measurements set that configured concrete's shipped default.
- **The library-backed discovery sources share the transport's engine.** They reach it through the engine capability the transport factory's generic surface exposes, so no discovery concrete depends on `transport/rqbit`.
- **The demonstrable milestone needs a grant.** Resolving against the swarm with the registry down requires a second identity to hold a credential, so the milestone sits after credential delivery and per-attempt authorization, not at the package host.
- **The resolution orchestrator follows its sources.** `workflows/resolve` reaches the CAS, the seed host, and the swarm through their factories, upstream through the ingest factory, ciphertext under a held credential through the attempt engine and the decryption pipeline, and a requested grant through the request job, so it follows First Finder ingest and credential delivery; the CAS and the package host are adapters beneath their factories and name no workflow, so they precede it, and `workflows/health` reports per-asset independence beside it.
- **The grant service follows its producers.** `workflows/grant` fulfils open requests from a held credential, so it follows the credential engine and the request job in credential delivery; the finder's grant of an asset's first entitlement to itself is `workflows/first_finder`'s.
- **The daemon's composition root follows everything it composes.** `workflows/compose` resolves every factory the daemon and the installer compose, `workflows/settings` carries every module's settings, and the daemon and installer binaries compose every module, so each follows the last of them, the installation coordinator's families and workflow, the explicit publisher path's, and the claim verifier's client among them, as `harness-crypto/config`, `harness-crypto/run`, and `harness-crypto/main` follow everything they carry, compose, and call. Every module takes its settings as params from the composition root; the control surfaces that read the catalogue follow it; the demonstrable milestone, which runs the packaged daemon, follows the daemon binary.
- **Escrow claim is last by dependency.** The escrow record carries no maintainer commitment and the verifier establishes the claim set at verification time, so the milestone waits only on the publisher path, the registry contracts, and credential delivery.
- **Two verifier forms, one primary.** `contracts/evm/PairingLib` exists in both forms so the harness measures both; BLS12-381 ships as the primary form on Base and BN254 is retained.
- **The delivery verifier is built in the harness and consumed by the contract sprint.** It is the first shipped contract, written to production standard, with its constants and vectors generated from the Rust reference by `harness-crypto/generate/solidity`.
- **The EVM suite is one concrete.** `contracts/evm` is the on-chain half of `chain/base`; a further chain form is a further suite beside it, and within the suite the adapter registry binds the entitlement token form, the attestation verifier per source, the identity adapters, the signature adapter per scheme, the delivery verifier per delivery-statement version and verifier form, and the payment-asset adapter per payment asset.
- **The payment asset is a seam on both sides of the chain.** The client reaches payment only through the `payment` family's factory, authored in credential delivery after `payment/native`, and the contracts only through the payment-asset adapter the registry binds, so a further payment asset is a further concrete and a further bound adapter and changes no lock, settlement, or acquisition code; the chain's native asset is the MVP realization beneath both.
- **The root type has one owner.** The hashing family owns the root type, the asset's identity, and every hashing operation is over bytes; the family carries no digest operation and no identity-hash identifier and depends on no encoding. A name is a lookup, never an identity: the package host's alias lookup is the one place a request becomes a root, and `workflows/sidecar/derivation_context` houses the derivation context, the `domain` crate holding no asset identity.
- **Identity and custody sits in the daemon grouping but the installer needs it.** The dependency runs the right way, coordinator depends on custody, and the epic is placed early in the grouping.
- **The identity is a contract account; its sponsorship mechanism is chosen at the relayer milestone.** The contract sprint builds the account with its device registry and takes every identity-bound mutation as a signed intent under LC-13, so `submission/paymaster`, `submission/relayer`, and `submission/self_funded` all remain open behind one factory.
- **Mint and transfer each have one owner, and every admission happens once.** Mint and transfer are different actions over different inputs: a mint starts from the master scalar, a transfer from an existing holder's secrets. `harness-crypto/delivery/mint` owns the mint delivery and `harness-crypto/delivery/transfer` the transfer delivery, each returning its statement, proof, credential state, recipient holder, uniforms, and the witnesses its prover took, so every node that proves, mutates, encodes, replays, or times a delivery consumes them and composes no delivery step: `harness-crypto/generate/solidity/proof_vectors`, `harness-crypto/vectors/statement_mutations`, `harness-crypto/vectors/replay`, and `harness-crypto/measure`. A transfer's recipient holder is a valid holder for the next transfer, so `replay`'s second settlement is a second transfer and nothing else. Forms admission happens once, in `harness-crypto/run/admit_contexts`, where `harness-crypto/run/chain_forms_consumer` resolves the forms; identity admission happens once, in `harness-crypto/config/admit_statements`, under the scope `harness-crypto/config/admit_selections` admitted, and `harness-crypto/run/kem_consumer` pairs each admitted identity with its setup scope as an `AdmittedKemIdentity`; every consumer takes typed contexts and admitted identities, rebuilds no form from a canonical context, and admits nothing. The witnesses each delivery returns are copies, since the provers consume what they are given and a `Secret` is not cloned. Measure takes its timing inputs from the delivery functions and not from the vector sets, whose witnesses a prover has already consumed. `CanonicalDeliveryContext` and `DeliveryContextField` stay declared in `harness-crypto/generate/solidity/challenge_vectors` until `harness-crypto/config/admit_statements` and `harness-crypto/run/admit_contexts`, their first consumers, move them, so every step builds.

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
        dom_dep["domain/deployment_identity"]
        dom_suite["domain/suite_identifier"]
        dom_pset["domain/parameter_set_identifier"]
        dom_group["domain/group_index"]
        dom_geom["domain/piece_geometry"]
        hashing_b3["hashing/blake3_bao"]
        hashing["hashing/factory"]
        dom_scheme["domain/signature_scheme_identifier"]
        sc_ctx["workflows/sidecar/derivation_context"]
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
        h_del_mint["harness-crypto/delivery/mint"]
        h_del_transfer["harness-crypto/delivery/transfer"]
        sol_pair["contracts/evm/PairingLib"]
        sol_iface["contracts/evm/IDeliveryVerifier"]
        gen_render["harness-crypto/generate/solidity/render"]
        gen_const["harness-crypto/generate/solidity/constants"]
        gen_group["harness-crypto/generate/solidity/group_vectors"]
        gen_reject["harness-crypto/generate/solidity/rejection_vectors"]
        gen_map["harness-crypto/generate/solidity/mapping_vectors"]
        gen_poss["harness-crypto/generate/solidity/possession_vectors"]
        gen_chal["harness-crypto/generate/solidity/challenge_vectors"]
        gen_proof["harness-crypto/generate/solidity/proof_vectors"]
        gen_evm["harness-crypto/generate/solidity"]
        gen["harness-crypto/generate/factory"]
        sol_verify["contracts/evm/DeliveryVerifier"]
        sol_deploy["contracts/evm/deploy"]
        hv_evm["harness-crypto/verifier/evm"]
        hv["harness-crypto/verifier/factory"]
        vec_holder["harness-crypto/vectors/cross_holder"]
        vec_rerand["harness-crypto/vectors/rerandomization"]
        vec_nonconv["harness-crypto/vectors/non_convertibility"]
        vec_capsule["harness-crypto/vectors/malformed_capsules"]
        vec_set["harness-crypto/vectors/cross_set"]
        vec_mut["harness-crypto/vectors/statement_mutations"]
        vec_replay["harness-crypto/vectors/replay"]
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
        run_admit["harness-crypto/run/admit_contexts"]
        run_gen["harness-crypto/run/generate_consumer"]
        run_ver["harness-crypto/run/verifier_consumer"]
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
        hashing_b3 --> hashing
        f_abi --> f_enc
        hashing --> sc_ctx
        dom_dep --> sc_ctx
        dom_suite --> sc_ctx
        dom_pset --> sc_ctx
        dom_group --> sc_ctx
        dom_geom --> sc_ctx
        f_enc --> sc_ctx
        sc_ctx --> sc_wrap
        hashing --> kem_bb1
        kdf_b3 --> kdf
        pair --> h2s_k
        h2s_k --> h2s
        pair --> kem_bb1
        h2s --> kem_bb1
        kem_bb1 --> kem
        f_enc --> env_ps
        pair --> env_ps
        env_ps --> env_eg
        pair --> env_eg
        h2s --> env_eg
        kem --> env_eg
        env_eg --> env
        dom_scheme --> chain_forms
        chain_forms --> chain_forms_fn
        f_enc --> proof_ms
        pair --> proof_ms
        env --> proof_ms
        chain_forms_fn --> proof_ms
        hashing --> proof_ms
        dom_suite --> proof_ms
        dom_pset --> proof_ms
        proof_ms --> proof_ts
        pair --> proof_ts
        env --> proof_ts
        chain_forms_fn --> proof_ts
        proof_ms --> proof_ch
        proof_ts --> proof_ch
        h2s --> proof_ch
        pair --> proof_ch
        chain_forms_fn --> proof_ch
        proof_ch --> proof_sfs
        pair --> proof_sfs
        env --> proof_sfs
        kem --> proof_sfs
        chain_forms_fn --> proof_sfs
        proof_sfs --> proof
        chain_forms_fn --> proof
        kdf --> sc_wrap
        kem --> sc_wrap
        sc_wrap --> sc_unwrap
        pair --> h_del_mint
        kem --> h_del_mint
        env --> h_del_mint
        proof --> h_del_mint
        chain_forms_fn --> h_del_mint
        h_del_mint --> h_del_transfer
        pair --> h_del_transfer
        kem --> h_del_transfer
        env --> h_del_transfer
        proof --> h_del_transfer
        chain_forms_fn --> h_del_transfer
        h_del_mint --> gen_chal
        h_del_mint --> gen_map
        h_del_mint --> gen_proof
        h_del_mint --> gen_evm
        h_del_transfer --> gen_proof
        chain_forms_fn --> gen
        h_del_mint --> vec_holder
        h_del_mint --> vec_rerand
        h_del_mint --> vec_nonconv
        h_del_mint --> vec_capsule
        h_del_mint --> vec_set
        h_del_mint --> vec_mut
        h_del_transfer --> vec_mut
        h_del_mint --> vec_replay
        h_del_transfer --> vec_replay
        h_del_mint --> h_vectors
        h_del_mint --> h_measure
        h_del_transfer --> h_measure
        kem --> h_measure
        env --> h_measure
        proof --> h_measure
        pair --> h_measure
        chain_forms_fn --> h_measure
        h_del_mint --> run_gen
        h_del_mint --> run_ver
        h_del_mint --> run_proof
        h_del_mint --> run_ka
        h_del_mint --> run_kem
        cfg_stmt --> run_admit
        gen_chal --> run_admit
        proof --> run_admit
        chain_forms_fn --> run_admit
        run_admit --> run_forms
        kem --> cfg_stmt
        gen_chal --> cfg_stmt
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
        hashing --> cfg_stmt
        dom_pset --> cfg_stmt
        proof --> cfg_stmt
        gen --> cfg_stmt
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
        sol_iface --> sol_pair
        gen --> sol_pair
        sol_pair --> sol_verify
        sol_iface --> sol_verify
        sol_iface --> hv_evm
        sol_verify --> sol_deploy
        proof --> hv_evm
        chain_forms_fn --> hv_evm
        hv_evm --> hv
        kem --> vec_holder
        kem --> vec_rerand
        kem --> vec_nonconv
        kem --> vec_capsule
        kem --> vec_set
        sc_unwrap --> vec_set
        pair --> vec_mut
        env --> vec_mut
        proof --> vec_mut
        chain_forms_fn --> vec_mut
        proof --> vec_replay
        chain_forms_fn --> vec_replay
        vec_holder --> h_vectors
        vec_rerand --> h_vectors
        vec_nonconv --> h_vectors
        vec_capsule --> h_vectors
        vec_set --> h_vectors
        vec_mut --> h_vectors
        vec_replay --> h_vectors
        hv --> h_measure
        h_measure --> h_report
        hv --> cfg_sel
        h_measure --> h_cfg
        h_report --> cfg_out
        h_report --> h_run
        hv --> run_ver
        h_measure --> run_ver
        h_cfg --> run_ver
        run_ver --> run_proof
        hv --> run_proof
        h_vectors --> run_proof
    end

    subgraph core["Protocol core and contract suite"]
        direction TB
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
        disc_map --> disc
    end

    subgraph daemon["Local daemon and package serving"]
        direction TB
        e_jobs["Epic: durable jobs and configuration"]
        e_identity["Epic: identity and custody"]
        e_cas["Epic: plaintext CAS and package host"]
        e_ff["Epic: First Finder ingest"]
        e_deliver["Epic: credential delivery and per-attempt authorization"]
        e_resolve["Epic: resolution orchestrator"]
        e_compose["Epic: daemon composition"]
        e_demo["Epic: demonstrable milestone"]

        e_jobs --> e_cas
        e_jobs --> e_identity
        e_cas --> e_ff
        e_identity --> e_ff
        e_identity --> e_deliver
        e_ff --> e_deliver
        e_cas --> e_resolve
        e_ff --> e_resolve
        e_deliver --> e_resolve
        e_resolve --> e_compose
        e_identity --> e_compose
        e_compose --> e_demo
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
    f_ci --> sol_iface
    f_sec --> dom_dep
    f_sec --> dom_suite
    f_sec --> dom_pset
    f_sec --> dom_group
    f_sec --> dom_geom
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
    f_rand --> h_del_mint
    f_rand --> h_del_transfer
    f_rand --> h_measure
    f_sec --> h_del_mint
    f_sec --> h_del_transfer
    f_enc --> vec_mut
    f_enc --> run_admit
    f_enc --> hv_evm
    f_ws --> hashing_b3
    f_sec --> hashing_b3
    f_enc --> sig_ed
    dom_scheme --> sig_ed
    f_enc --> disc_local
    f_enc --> st_redb
    f_enc --> chain_forms
    f_ws --> cfg_layers
    f_rand --> cfg_sel
    f_enc --> cfg_sel
    f_enc --> cfg_stmt
    f_rand --> gen_evm
    f_rand --> h_run
    f_rand --> bench
    f_rand --> vec_holder
    f_rand --> vec_rerand
    f_rand --> vec_nonconv
    f_rand --> vec_capsule
    f_rand --> vec_set
    f_rand --> vec_mut
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
    sh --> e_resolve
    tr --> e_resolve

    e_jobs --> m_installer
    e_identity --> m_installer
    s_chain --> m_relayer
    m_relayer --> e_identity
    m_relayer --> e_ff
    m_relayer --> e_deliver
    e_ff --> m_publisher
    e_identity --> m_publisher
    s_registry --> m_claim
    e_deliver --> m_claim
    sh --> m_seedhost
    s_cipher --> m_seedhost
    m_installer --> e_compose
    m_publisher --> e_compose
    m_claim --> e_compose
    e_compose --> m_desktop
    e_compose --> m_obs
    e_compose --> m_demo

    m_publisher --> o_txflow
    e_deliver --> o_txflow
    e_compose --> o_txflow
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

**Reading the graph.** Everything in the foundation and harness subgraphs, and every ticket-named node in the protocol core, is one source file, except the configuration tickets `workspace/cargo` and `workspace/ci`, each of which carries its configuration files together, the contract suite's ticket `contracts/evm/IDeliveryVerifier`, which carries the ABI description, its Solidity realization, and the toolchain files together, and the deployment script.
A factory node is the family's `factory` module; a concrete node is one adapter beneath it, private to the crate; a family's first ticket, its first concrete or a family-owned function preceding it, carries the family's interface and creates the crate, and the factory follows every concrete.
Every edge out of a family runs from its factory, never from a concrete: the registry sprint consumes the harness's `contracts/evm/DeliveryVerifier`, and the First Finder, credential delivery, and identity epics consume the KEM, envelope, proof, and KDF factories, which construct the concretes the harness proved.
The chain family's forms function sits in the harness beside its one concrete; the proof family's transcript descriptions, challenge, concrete, and factory, the generator, verifier client, vectors, and measurement consume the function, so no harness ticket names a form set; the chain factory is authored in the chain and submission sprint after the Base concrete, and the sprint's edge from the forms function is the Base concrete binding the forms it resolves.
The harness run is the grouping's composition root, consuming the configuration ticket, the benchmark, and every factory through the consumers it owns, each consumer following the one it constructs; the output follows the run, whose outcome it writes, and the binary follows the configuration, the run, and the output, which it calls in turn; every edge into a ticket, sprint, epic, or milestone is a code dependency, every edge into an objective or the demonstrable milestone, neither of which authors a file, is a condition it closes on, and the graph has no cycle; the generated directory, the deployed verifier, and the measured defaults are runtime inputs, held in the [Runtime sequence](#runtime-sequence), and appear in no edge.
The generate concrete's functions each consume the factories whose adapters they are handed, and the vector functions consume the encoding family's contract for their records' descriptions; `harness-crypto/generate/solidity/render` precedes every one of them, and the adapter follows them all.
The configuration's layering function precedes its admission functions, each of which consumes the families whose types it admits, the generate settings' admission following `render` and the generate factory, whose validated names and per-kind settings it admits; the composing configuration ticket follows every admission function and the benchmark, whose iteration count it admits.
The signature and swarm tickets have no incoming edge from any contract sprint, which is the point that bytes can move before any chain exists.
The relayer milestone depends only on the chain and submission families and can be built as soon as they exist.
The resolution orchestrator follows credential delivery, First Finder ingest, the package host, and the swarm, whose sources it orders.
The daemon composition is the daemon grouping's composition root, as the harness run is the harness's, following every epic and milestone whose modules it composes, the installation coordinator, the explicit publisher path, and the claim verifier among them. The demonstrable milestone has incoming edges from the daemon composition and the seed-host family, and nothing depends on it.

## Sequencing

`workspace/cargo` precedes everything. Within the harness the starting points are `pairing/bn254_arkworks`, which carries the pairing interface with its sampling bound, and the domain's identifier modules on `domain/secret`; `hashing/blake3_bao` and `hashing/factory` follow `workspace/cargo` and precede the encoding family, `workflows/sidecar/derivation_context` and its description follow the hashing factory, the identifier modules, and the encoding factory and precede `workflows/sidecar/wrap`, and `kdf/blake3_keyed`, the first ticket that encodes a context, follows the encoding factory;
every other ticket has a producer, each family's first ticket precedes its further concretes and its factory follows them all, `chain/evm_forms` follows `encoding/factory` and `domain/signature_scheme_identifier`, `chain/create_chain_forms` follows it, and every proof ticket follows `chain/create_chain_forms`, `harness-crypto/delivery/mint` follows `proof/factory` and `harness-crypto/delivery/transfer` follows it, both preceding `harness-crypto/generate/solidity/render`, every function the generate concrete owns and every vector set following `harness-crypto/delivery/mint`, and `harness-crypto/generate/solidity/proof_vectors`, `harness-crypto/vectors/statement_mutations`, `harness-crypto/vectors/replay`, and `harness-crypto/measure` following `harness-crypto/delivery/transfer` as well, `harness-crypto/generate/solidity/render` precedes every other function the generate concrete owns,
the adapter `harness-crypto/generate/solidity` follows them all and `harness-crypto/generate/factory` follows it, `harness-crypto/config/layers` precedes the configuration's admission functions, `harness-crypto/config/admit_generate` follows `harness-crypto/generate/factory`, whose per-kind settings it admits, `harness-crypto/config/admit_output` follows `harness-crypto/config/layers`;
`contracts/evm/IDeliveryVerifier`, authoring the delivery verifier's ABI description, its Solidity interface, and the suite's Foundry configuration, follows `workspace/ci`; `harness-crypto/verifier/evm`, binding that ABI description, follows it, the proof factory, the encoding factory, and the forms function;
`harness-crypto/verifier/factory`, the functions the vectors own, `harness-crypto/vectors/cross_holder`, `harness-crypto/vectors/rerandomization`, `harness-crypto/vectors/non_convertibility`, `harness-crypto/vectors/malformed_capsules`, `harness-crypto/vectors/cross_set`, `harness-crypto/vectors/statement_mutations`, and `harness-crypto/vectors/replay`, then `harness-crypto/vectors`, `harness-crypto/measure`, and `harness-crypto/report` follow in turn; the admission functions that take their values and `harness-crypto/config` follow `harness-crypto/report`, and `harness-crypto/config` follows every admission function; `harness-crypto/run/admit_contexts` follows `harness-crypto/config`;
the run's consumers follow `harness-crypto/run/admit_contexts` innermost first, from `harness-crypto/run/generate_consumer` through `harness-crypto/run/verifier_consumer` to `harness-crypto/run/pairing_consumer`; `harness-crypto/run`, `harness-crypto/output`, and `harness-crypto/main` follow in turn;
`contracts/evm/PairingLib`, `contracts/evm/DeliveryVerifier`, and `contracts/evm/deploy` follow `harness-crypto/main`, and `contracts/evm/deploy` carries the grouping's commit.
The signature tickets start from `signature/ed25519` on `encoding/factory` and `domain/signature_scheme_identifier`, and the swarm tickets from `transport/rqbit`, `discovery/local`, and `storage/redb` on `hashing/factory` and `encoding/factory`; each of those milestones closes at the factory its table names.

Across groupings the sequence is the [milestones](milestones.md)' order, with the parallelism the graph makes visible: the signature and swarm tickets run beside the harness and the contract sprints from `encoding/factory` and `hashing/factory` onward; the identity and custody epic begins as soon as the signature tickets, the chain and submission families, and the relayer or paymaster milestone exist, before the CAS epic, because the installation coordinator needs it; the relayer or paymaster milestone follows the chain and submission families sprint and precedes every daemon epic that submits a signed intent; the resolution orchestrator follows credential delivery and First Finder ingest; the daemon composition follows the resolution orchestrator, the installation coordinator, the explicit publisher path, and the claim verifier and escrow claim, and the desktop application and CLI follow it; the demonstrable milestone follows the daemon composition. Within the credential delivery epic `payment/native` precedes `payment/factory`, which precedes `workflows/acquire`; the registry contracts sprint authors the lock and `IPaymentAsset` with its native concrete, and the on-chain adapter registry sprint binds them. The package host maps a request's name to the asset's root through its alias lookup, the root type being the hashing family's, and `chain/evm_forms` authors address derivation and the signed-intent form with no signature dependency. The chain and submission sprint authors `chain/factory` once, after the Base concrete, since the harness authored the forms constructor as the family-owned module `chain/create_chain_forms` rather than in the factory.

The decay rule governs re-mapping. When the `contracts/evm/deploy` node commits, the remaining protocol core milestones are re-mapped from sprints to tickets using what the harness taught, the daemon grouping from epics to sprints, and so on outward. Each re-mapping is a revision of this document in place.

## Runtime sequence

What running the built harness and suite produces, each step after the inputs it reads. These are operations, not tickets, and none is an edge.

- The harness binary's benchmark measures every configured pairing concrete; the measurements set the configuration's shipped pairing concrete per curve.
- The harness binary's first run from its shipped defaults writes the generated directory and returns the verifier's refusal.
- The contract suite builds and passes its tests against the generated directory.
- `contracts/evm/deploy` deploys the delivery verifier to Anvil and Base Sepolia and emits each address as the deployment record.
- The harness binary's next run, with the deployment record layered over its shipped defaults, verifies, builds the vector sets, measures, and writes the report; the tests that read the deployment record go GREEN here.

## Risk Mitigation

| Risk from the register | How the map addresses it |
| --- | --- |
| R-02, cost makes installs noticeably slower | The harness grouping is entirely at ticket resolution; `harness-crypto/report` records the measurements against the configured piece-group size and curve defaults and any adjustment the data supports, and every ticket that encrypts reads the piece-group size from the deployment record and waits on nothing |
| R-03, verifier flaw | `proof/schnorr_fs` and `contracts/evm/DeliveryVerifier` are separate tickets with `harness-crypto/verifier/evm` as the parity integration point and `harness-crypto/generate/solidity` as the source of the contract's constants and vectors; `harness-crypto/vectors` carries the mutation and replay sets |
| R-04, execution breadth | Decaying resolution means no distant detail is authored to be rewritten; a concrete adapter is one node, so a family's ticket count is its factory plus its concretes |
| R-09, open selections | The identity epic carries custody recovery UX; the license gates release and no ticket |
| R-12, chain properties | Every pairing concrete and both verifier forms are harness tickets beneath the pairing and proof factories, so a precompile change on Base selects the retained form at resolution rather than rebuilding; a further chain is a further `chain` concrete with its own suite under `contracts/` |
| R-13, escrow record | No commitment and no custodian; the claim milestone has no policy gate |
| R-14, adapter registry governance | The on-chain adapter registry sprint is separate from the registry contracts sprint, so its single project-held key can be replaced without touching entitlements; the same registry binds the attestation verifier per source, the entitlement token form, the signature adapter per scheme, the delivery verifier, and the payment-asset adapter per payment asset |
| A consumer bound to a concrete | Every family crate exposes its factory alone, every consumer ticket follows the factory it names, and the harness constructs through the factories, so no ticket can name a concrete it does not own |

## Decisions

**The adapter family form.** Every family is a factory crate owning the generic interface, the capability declaration, and the types the generic surface needs; each concrete is a private module beneath the factory with its own interface implementing the generic one and owning only the types it alone produces; consumers depend on the factory's surface alone; the layout is general responsibility, then functional need, then concrete implementation. A family exists even where the MVP ships one concrete.

**Crate and path layout.** One workspace with a domain crate, a workflows crate, a crate per adapter family holding its `factory` module and its private concretes, and a crate per deployable, each created by the ticket of the first module that lives in it, which for a family is its first ticket; the workspace manifest lists members by glob; tickets are named by crate, factory, concrete, and function as the technical requirements' file tree spells them.

**Canonical encoding.** The `encoding` factory owns `IEncoderAdapter` and `IDecoderAdapter` under one versioned encoding identifier, the repo-owned encoding contract, and the declaration; the family owns one description module per encoded type housed in a crate it depends on, `encoding/<type>`, implementing the contract for that type, the encoding crate depending on the hashing crate and on the domain crate, and a type housed in a crate that depends on the encoding family, the derivation context among them, carrying its description in that crate; the family follows the hashing family's root type, the first encodable type, at the head of the harness grouping; `encoding/abi` implements both through `alloy`'s sol types and is the only module naming `alloy` for encoding; one encoding for everything hashed, signed, stored, or framed over IPC, including the configuration registry's records.

**Tools and dependencies.** A configuration file is authored once, complete, by the ticket that creates it: the workspace manifest with the `librqbit` overlay and `deny.toml` with the overlay repository's source at `workspace/cargo`, the Rust continuous-integration definition at `workspace/ci`, `contracts/evm/foundry.toml`, granting `fs_permissions` read access to the generated directory and the ABI description directory alone with `ffi` disabled and `via_ir = true`, and the Solidity continuous-integration definition, with `forge build`, `forge fmt --check`, and `forge test`, at `contracts/evm/IDeliveryVerifier`, the contract suite's own ticket, and the shell, fuzz, and end-to-end continuous-integration definitions each at the ticket that first needs it, as separate files. A crate's manifest and its `lib.rs` barrel accumulate one entry per module, each added by that module's ticket as its deps and provides elements. A vendor library is named only by the concrete that wraps it; `harness-crypto/benchmark` names none and reaches each pairing library through the pairing factory; `harness-crypto/config/layers` pins `clap`, `toml`, and `serde`, the one place the harness names any of them. `pairing/bn254_arkworks` pins `num-bigint` as the pairing crate's dev-dependency, at the version `Cargo.lock` already resolves through `ark-ff`, for the integer exponent its definition test computes; no tool outside the workspace takes part in the target-group proof.

**Secrets.** The secret-typed value is a `domain` ticket pinning `zeroize` as a domain crate dependency; the telemetry family carries correlation and tracing setup and no redaction layer.

**Ticket granularity for a concrete adapter.** One node: the concrete's operations are the methods of one adapter in one file, and a function the concrete owns, such as the Schnorr challenge or the Solidity generate concrete's rendering, constants, and vector functions, is a node beneath it.

**Interfaces first, factories last.** A family's generic interface, declaration, and mock are authored in the ticket of its first concrete, the first source file that requires them; each further concrete and family-owned function follows; the factory function, which consumes the concretes, is authored once, after the family's last concrete, and precedes every consumer; every consumer follows the factory, and the harness constructs its combinations through the factories. A family whose forms are consumed before its adapter exists constructs them through a family-owned function, `chain/create_chain_forms`, authored after the forms concrete, because the factory module holds one function and that function constructs the family's adapter; the factory is still authored once, after every concrete.

**The harness configuration.** A typed configuration the harness owns, `harness-crypto/config`, with a shipped default for every value the harness passes as params and overrides from a configuration file and from command-line flags, composed over a layering function and admission functions it owns, each its own ticket ahead of it; the shipped defaults are `apps/harness-crypto/harness.toml`, carried by the composing ticket, and the overrides are `--config <PATH>` files and `--set <KEY=VALUE>` assignments applied in command-line order over them; the configuration follows every ticket whose values it carries; the binary reads it, every factory call the run makes carries its values, and the output writes the run's outcome to the destinations it names, so no harness module holds a literal for a concrete, a curve, a benchmarked concrete, an identifier, a form, a version, a profile, an identity scope, a statement's context value, an identity, an iteration count, a vector kind's count, a path, a file, library, or symbol name, a compiler version, a license identifier, or an output destination.

**The harness run, its output, and its binary.** The run is a composing function over one consumer per factory it calls, each its own ticket, and the harness's composition root; the output writes the run's outcome, a success or a refusal, to each configured destination, standard output, a file, or both; the binary holds only what a binary's `main` keeps, calling the configuration, the run, and the output in turn and setting the exit status, and carries no element files because it declares no type and holds no test.

**The canonical target-group value.** The encapsulated value the KDF consumes is a target-group element's encoding, and two libraries of one curve are not assumed to compute the same element or serialize it the same way. The pairing family's declaration carries a target-group encoding identifier naming the pairing map, the generators, and the coefficient serialization; the suite requires it as an explicit hash-card field beside the forms identifier; the factory admits a concrete only when it declares the required one; and every concrete is proven by executing the identifier's definition in its own tests, which fixes the map, the generators, the tower, the exact exponent `(p^12 - 1) / r`, and the serialization: the test computes the library's Miller loop over the generators, raises it to the integer exponent, serializes it, and requires the concrete's pairing product to encode to the same bytes, and on BLS12-381 asserts the CFRG draft's published value as well; a library's own reduced pairing is never the proof, since several libraries compute a fixed multiple of the exact exponent for speed. A concrete wrapping such a library multiplies the library's output by the multiple's inverse in the scalar field, computed at construction from the multiple the library's own final exponentiation states; the multiple is a fact about that library recorded in the concrete's node, and the definition test proves it; the family's integration test then proves the two concretes of a curve equal to each other as a consequence.

**The harness's own families.** The harness's `generate` and `verifier` families, the generate family with a concrete named for the language it emits, `solidity`, writing ABI-encoded language-neutral vector files, and the verifier family with an `evm` concrete named for the ABI it speaks, each carrying the family's interface and a factory following it, because emitting a suite's vectors and reaching a suite's verifier are suite-specific and the daemon needs neither.

**The storage engine and the engines above it.** The `storage` factory with `storage/redb`; the job engine and the configuration registry are workflows modules over it, since nothing external remains in them.

**BitTorrent library.** `librqbit` compiled in as a soft fork through the workspace manifest's `[patch.crates-io]` overlay onto the project's overlay branch `chaintorrent-overlay` in `https://github.com/tsylvester/rqbit`, which tracks tagged upstream releases and exists before the bootstrap, with the hooks authored inside `transport/rqbit`, submitted upstream, and the commit pinned; no owned transport in the MVP; the library's discovery sources are `discovery` concretes reached through the transport factory's engine capability; the hard-fork trigger is recorded in the product requirements.

**The contract suite.** `contracts/evm` is the on-chain concrete of `chain/base`, identified by its virtual machine and its source language, Solidity under Foundry in the MVP, with the toolchain inside it; its boundary with the client is its ABI; a further chain form or a further source language is a further suite.

**The asset's identity.** The asset's identity is the object's BLAKE3/Bao root, the plaintext root for a public asset and the ciphertext root for a private one, owned by the hashing family, whose operations are all over bytes and which carries no digest operation and no identity-hash identifier. A name is a lookup: the package host and the ingest adapter declare the namespace their names live in, the Registry holds append-only alias records under a declared namespace and an opaque key, each entry a root with the source's signed statement or its recorded absence, and the client selects an entry only on evidence it holds independently of every registrant.

**Address derivation and the signed-intent form.** Both belong to the chain family's form interface and are realized for the EVM suite by `chain/evm_forms`, over a public key's canonical bytes and scheme identifier and as EIP-712 typed data over the call's fields, chain, contract, nonce, and expiry; `chain/base` binds the resolved forms and authors none, and the signature family signs the bytes the intent form hands it. The scheme identifier is the type `domain/signature_scheme_identifier` holds, in the domain ring both families reach, so the signature family's declaration carries it and the identity form takes it without either family depending on the other.

**The claim verifier's key.** The on-chain record is a scheme identifier and a commitment to the key, the key travelling in the calldata of the registering or rotating transaction; the contract verifies vouchers through the signature adapter the registry binds for the scheme; the claim verifier signs under a custody-held key of a declared scheme, and a further verifier form is a further `claim-verifier` concrete that changes no contract storage.

**The payment asset.** A price is an amount of a named payment asset; a lock carries the payment-asset identifier; the contracts move payment only through the payment-asset adapter the registry binds per identifier behind `IPaymentAsset`, whose MVP concrete settles in the chain's native asset; the client reaches payment only through the `payment` family's factory, `payment/native` being its MVP concrete; the relayer's refusal to sponsor a paid acquisition is stated against the payment asset the lock names.

**The harness's delivery verifier.** The shipped contract, written to production standard from the start.

**Milestones at ticket resolution beside the harness.** Signatures and the swarm, because they depend on nothing the harness measures; they start once `encoding/factory` and `hashing/factory` close, since their first tickets encode and hash. The hashing family is a harness ticket pair at the head of the grouping.

**The demonstrable milestone.** After the daemon composition, whose packaged daemon it runs.

**The delegated seed-host concrete** is `seed-host/rqbit`, driving the rqbit application through its HTTP API; a further external client or the project's own client is a further concrete beneath the seed-host factory; the swarm milestone's commit sits on it.

# Additional Content

**What a ticket is and is not.** A ticket here is a candidate for a workplan node: it names the source file's role, what it owns, what it depends on, and what requirement it proves. It is not a node. A node is authored through the ordinary path in the node template, with every element in the fixed order, and this document does not emit node structure. When a ticket is promoted, its row here is unchanged; the node is the instruction and this map is the overview.

**Why the harness has the tickets it has.** One source file per node, so each cryptographic family's concretes, one per curve per library for pairing, and its factory, the benchmark, the generate and verifier families with their concretes, each function the generate concrete owns, the transcript descriptions, the contract suite's interface and toolchain ticket, the Solidity library and verifier, the wrap and unwrap, and the measurement driver each get their own file with full support. The `sample_deployment` generator needs the payload cipher and belongs to the protocol core grouping. That list is the honest size of the harness grouping.

**Requirement coverage of the foundation and harness groupings.** CR-02, CR-03 in part, CR-04, CR-05, CR-07 in part, CR-08, CR-09, CR-10, CR-11, CD-03, CD-07, CD-08, LC-08 in part, XA-07 in part, AS-17 for the cryptographic families, and AS-21. Everything else the requirements name is in a later grouping.
