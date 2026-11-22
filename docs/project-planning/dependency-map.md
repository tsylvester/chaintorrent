<!-- Template: antithesis_dependency_map.md -->
# Dependency Map

## Overview

How the parts of the ChainTorrent MVP fit together and in what order they are built, drawn from the [revised business case](business-case-revised.md), the [technical approach](technical-approach.md), the [workplan](../workplans/current/ChainTorrent%20MVP.md)'s build sequence, and the [MVP Application Requirements](../research/MVP%20Application%20Requirements.md). Groupings are addressed by delivery role: **foundation**, **cryptographic validation harness**, **protocol core and contract suite**, **local daemon and package serving**, **onboarding shells and services**, and **acceptance and release**.

Resolution decays with distance, deliberately. The foundation and harness groupings are mapped at **ticket** resolution: one ticket per source file, which is the node template's unit, so that each ticket is a candidate node the workplan author can promote. Inside the protocol core, the signature and swarm milestones are also at ticket resolution, because they depend on nothing the harness measures and run beside it, and so are the tickets on the path to `chain/factory`, because the harness's verifier client, configuration, and run consume the chain family through its factory and are placed after it; the remaining protocol core milestones are mapped as **sprints** named by dependency role. Beyond that the map holds **epics**, then **milestones**, then **objectives**. The reason is that implementation of the nearest work will produce discoveries that revise the anticipated order of everything after it, and detail authored now for distant groupings would be rewritten rather than used. When the harness closes, the remaining protocol core milestones are re-mapped to ticket resolution from what was learned, and the decay shifts outward.

Tickets are candidates, not nodes. No node is authored here; a ticket names the source file's role and its dependencies so a node can be written from it through the ordinary authoring path.
Every adapter family is a crate holding its `factory` module, which owns the generic interface, the capability declaration, and the factory function, and its concretes as private modules beneath it, and the map names tickets accordingly: a family's factory ticket is `crate/factory`, a concrete's `crate/concrete`, a function a concrete owns `crate/concrete/function`, a function the family owns `crate/function`, a module of the domain or workflows crate `crate/module` with its functions beneath it, and a contract `contracts/<suite>/Contract`; the [technical requirements](technical-requirements.md)' file tree spells every path, crate directories hyphenated and module directories underscored.
A concrete adapter is one node, since its operations are the methods of one adapter in one file.
Within a family the first ticket, its first concrete or a family-owned function that precedes it, authors the generic interface, the declaration, and the family's mock as its producers, since it is the first source file that requires them, and creates the crate; each further concrete and family-owned function follows; a concrete constructed from a value configuration sets has a settings admission ticket `crate/concrete/settings`, holding the concrete's settings type, the guard that narrows the untrusted settings document, and the admission the family's selection admission invokes, which precedes its concrete, since the concrete is constructed from the admitted settings, and which, where it precedes every other ticket of its family, is the family's first ticket; a concrete with no settable value has no settings ticket, and the family's selection admission refuses a settings document for it;
the family's declaration lookup, selection admission, and composition admission tickets follow the concretes and the settings admissions they compose and precede the factory ticket; the factory ticket, which constructs the concretes and so consumes them, calls the composition admission before it constructs anything, follows the family's last concrete and the family's admission functions, and precedes every consumer that constructs through it; and every consumer ticket depends on the admission surface its file uses — the selection admission for an admitted selection, the declaration lookup for a declaration, the composition admission for an admitted composition, or the factory for a constructed member — and never on a concrete.
The declarations the composition resolver reads therefore exist from the family's first ticket.
A family's factory ticket closes a producer, implementation, consumer chain, so it carries the family's integration test across the factory and its concretes; the commit sits on the last ticket of the milestone's chain, which for a milestone that closes at a family is that family's factory.

A ticket depends on another only when its file needs the other's file in order to be written: a type, a trait, a function, or an interface it imports, or a configuration file it is built under. What a file needs only in order to run — output a program writes, a deployed contract, a measured value — is a runtime input; the [Runtime sequence](#runtime-sequence) section holds every runtime input, and none orders a ticket. Each ticket's row is its file's single, complete authoring.

## Components

Grouped by role. Within a grouping at ticket resolution each row is a ticket; elsewhere each row is a sprint, epic, milestone, or objective as the decay dictates.

### Foundation, at ticket resolution

The workspace every node builds on, the secret type that cannot be formatted, the randomness family every key-generating ticket draws from, and the continuous integration that proves each ticket where the completion boundary requires. The encoding family is not a foundation member, since nothing encodable exists here; it follows the hashing family's root type, the first encodable type, at the head of the harness grouping. No crate is created ahead of the module that first lives in it; each ticket below creates its crate when that crate does not yet exist. A configuration file is authored once, complete, by the ticket that creates it, and no later ticket amends it; a crate's manifest and its `lib.rs` barrel are the exceptions by the topics' own design, since each module's ticket adds its dependencies to the manifest as its deps element and its re-export line to the barrel as its provides element.

| Ticket | Owns | Depends on | Proves |
| --- | --- | --- | --- |
| `workspace/cargo` | The virtual workspace manifest with the glob members `crates/*`, `adapters/*`, and `apps/*`, the lint table, and the `[patch.crates-io]` overlay pointing `librqbit` at the project's overlay branch `chaintorrent-overlay` in `https://github.com/tsylvester/rqbit`, which exists at a tagged upstream release before the bootstrap as external setup; `rust-toolchain.toml`; `deny.toml` with the license allowlist and that repository in `allow-git`; `.gitignore`, each authored once and complete | nothing | The root configuration every later ticket builds within; its build proof arrives with the first member |
| `domain/secret` | The secret-typed value: no formatting or serialization trait, an explicit accessor for the cryptographic operations and custody wrapping that consume it, zeroization on drop through `zeroize`, a crate dependency; creates the `domain` crate | `workspace/cargo` | CR-07 exclusion of secrets from logs, proven at compile time; the lint table's rejection of `unsafe_code`, `unwrap_used`, `expect_used`, `panic`, and `as_conversions` in the first production crate |
| `random/os` | The concrete over the operating system's generator; authors, as its producers, the randomness family's interface that fills bytes, the family's member identifier, the declaration, and the family's mock; creates the `adapters/random` crate; pins `rand_core` and `getrandom` | `workspace/cargo` | CR-05 contract; repeated draws of a fixed width pairwise distinct across a fixed count, and a filled buffer of a fixed length holding more than one distinct byte value; CR-05's per-bootstrap independence is proven at the First Finder engine and its reproducible publisher derivation at the explicit publisher path |
| `random/lookup_random_declaration` | The family's declaration lookup, returning a member's declaration without constructing the member | `random/os` | a declaration read with no concrete constructed |
| `random/admit_random_selection` | The family's selection admission: receives the admitted member identifier and the untrusted settings document, refuses a document that supplies any setting, with the family's own error naming the member, since no member has a settable value, and returns the selection naming the member; an absent member list is the declared set, and every callee error is carried unchanged | `random/os`, `random/lookup_random_declaration` | configured spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `random/admit_random_composition` | The family's composition admission, comparing the selected member's declared requirements with the declarations of the members it is composed with and returning the family's `Incompatible` flavor where they disagree | `random/lookup_random_declaration`, `random/admit_random_selection` | an incompatible composition refused before construction |
| `random/factory` | The randomness family's factory function, which calls the composition admission before it constructs anything and then constructs the member the admitted selection names, and the crate's public surface, the family's interface declaring the declared set `RANDOM_SOURCE_KINDS` over real members with one spelling per member and no configuration text; carries the family's integration test across factory and concrete and the grouping's commit | `random/os`, `random/admit_random_composition` | CR-05: the one surface every production draw passes through |
| `workspace/ci` | The Rust continuous-integration definition `.github/workflows/rust.yml`, authored once and complete: `cargo fmt --check`, `cargo check`, `cargo clippy`, the unit and integration tests, `cargo-deny`, and `cargo-audit` on Windows, macOS, and Linux; the Solidity, shell, fuzz, and end-to-end checks are separate workflow files, each created once by the ticket that first needs it | `workspace/cargo` | XA-07 facilities run in CI; NF-M07; `deny.toml` read and enforced on every supported platform |

### Cryptographic validation harness, at ticket resolution

The harness begins with the domain's identifier modules and the hashing family, whose root type is the first encodable type and depends on no encoding, followed by the encoding family, whose first description is the root type's, and the derivation context in the sidecar layer, which composes the root type with the identifier modules and is described beside itself in the workflows crate,
and builds the pairing, key-derivation, hash-to-scalar, credential KEM, envelope, and delivery proof families, each from its first ticket, which carries the family's interface, to its factory, which follows every concrete, together with the chain forms family, from its EVM forms concrete to its factory,
and, ahead of the verifier client, the protocol core's tickets on the path to `chain/factory` — the domain type its attempt context carries, `chain/base/settings`, the Base concrete's settings admission, `chain/base`, which authors the ABI descriptions of the contracts it binds, the chain family's declaration lookup, selection admission, and composition admission, and `chain/factory` — placed at their dependency position because the verifier client, the configuration, and the run consume the chain family through its factory,
the mint and transfer delivery functions every ticket that proves, encodes, or times a delivery consumes, the harness's typed configuration with its shipped defaults, the harness run that composes every family through its factory over the consumers it owns,
the output that writes the run's outcome, a success or a refusal, to the destinations the configuration names, and the harness binary that reads the configuration, calls the run, and hands its outcome to the output, exercises the deployed verifier on Base Sepolia, and records the measurements against the configured piece-group size and curve defaults with any adjustment the data supports (CD-07, AS-21).
The run composes every cell of the selected pairing concretes, target-group encoding identifiers, and encodings, each list the family's declared set where the configuration carries none.
It constructs every cell through the factories, so an invalid combination is refused in the harness exactly as the daemon's resolver will refuse it.
Every ticket is one Rust source file with its full support system; a family's first ticket owns the family's generic interface, declaration, and mock beside its own implementation, a further concrete ticket is one adapter whose operations are its methods, and the factory ticket owns the factory function and the crate's public surface.

| Ticket | Owns | Depends on | Proves |
| --- | --- | --- | --- |
| `pairing/bn254_arkworks` | The BN254 concrete on arkworks, precompile-matching encodings, declaring, in the family's value identifiers, the curve, first-group-only arithmetic at the verifier, the verifier form, the precompile encodings, and the target-group encoding, proven by executing the identifier's definition in its tests; authors, as its producers, `IPairingAdapter` with generators, add, mul, MSM, subgroup check, and the pairing-product check, the reference trait `IPairingReference`, beside the generic interface, with the scalar field's order and the precompile encoding of the defined point outside each source group's prime-order subgroup, the group-element and scalar associated types, the sampling bound on the scalar associated type, sample from uniform bytes, the family's member identifier and its curve, verifier-arithmetic, verifier-form, precompile-encoding, and target-group-encoding value identifier types with total spellings and declared wire codes, the declaration type carrying its member's identifier and providing those values without a concrete-name text, and the family's mock; owns its scalar and group-element types, holding the library's elements as private fields behind fallible constructors that perform the subgroup and encoding checks, names them as the associated types, implements the sampling bound on its scalar type, and proves the in-range draw; creates the `adapters/pairing` crate; pins `ark-ec`, `ark-ff`, and `ark-bn254` | `workspace/cargo`, `random/factory` | CR-10 contract and CR-10 on BN254; the declaration the proof and chain compositions read |
| `pairing/bn254_halo2curves` | The BN254 concrete on `halo2curves`, precompile-matching encodings, declaring, in the family's value identifiers, the curve, first-group-only arithmetic at the verifier, the verifier form, the precompile encodings, and the target-group encoding, proven by executing the identifier's definition in its tests; owns its scalar and group-element types over its library's elements behind fallible constructors, implements the sampling bound on its scalar type, and proves the in-range draw; pins `halo2curves` | `pairing/bn254_arkworks` | CR-10 on BN254 |
| `pairing/bls12_381_arkworks` | The BLS12-381 concrete on arkworks, precompile-matching encodings, subgroup checks on every input, declaring, in the family's value identifiers, the curve, second-group arithmetic at the verifier, the verifier form, the precompile encodings, and the target-group encoding, proven by executing the identifier's definition in its tests and against the CFRG draft's published value; owns its scalar and group-element types over its library's elements behind fallible constructors, implements the sampling bound on its scalar type, and proves the in-range draw; pins `ark-bls12-381` | `pairing/bn254_arkworks` | CR-10 on BLS12-381 |
| `pairing/bls12_381_halo2curves` | The BLS12-381 concrete on `halo2curves`, precompile-matching encodings, subgroup checks on every input, declaring, in the family's value identifiers, the curve, second-group arithmetic at the verifier, the verifier form, the precompile encodings, and the target-group encoding, proven by executing the identifier's definition in its tests and against the CFRG draft's published value; owns its scalar and group-element types over its library's elements behind fallible constructors, implements the sampling bound on its scalar type, and proves the in-range draw | `pairing/bn254_arkworks` | CR-10 on BLS12-381 |
| `pairing/lookup_pairing_declaration` | The family's declaration lookup, returning a member's declaration without constructing the member | `pairing/bn254_arkworks`, `pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, `pairing/bls12_381_halo2curves` | a declaration read with no concrete constructed |
| `pairing/admit_pairing_selection` | The family's selection admission: receives the admitted member identifier and the untrusted settings document, refuses a document that supplies any setting, with the family's own error naming the member, since no member has a settable value, and returns the selection naming the member; an absent member list is the declared set, and every callee error is carried unchanged | `pairing/bn254_arkworks`, `pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, `pairing/bls12_381_halo2curves`, `pairing/lookup_pairing_declaration` | configured spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `pairing/admit_pairing_composition` | The family's composition admission, comparing the selected member's declared curve, verifier-form, arithmetic, precompile-encoding, and target-group-encoding provisions with the requirements the composed declarations state and returning the family's `Incompatible` flavor where they disagree | `pairing/lookup_pairing_declaration`, `pairing/admit_pairing_selection` | the suite's required target-group encoding and verifier capabilities compared before construction, disagreement refused as `Incompatible` |
| `pairing/factory` | The pairing family's factory function, which calls the composition admission before it constructs anything and then constructs the member the admitted selection names, and the crate's public surface, the family's interface declaring the declared sets `PRECOMPILE_ENCODINGS` and `TARGET_GROUP_ENCODING_IDENTIFIERS` over real members with total spellings and declared wire codes; carries the family's integration test across factory and concretes, which proves the concretes of a curve return the same scalar field order and the same outside-the-subgroup encodings | `pairing/bn254_arkworks`, `pairing/bn254_halo2curves`, `pairing/bls12_381_arkworks`, `pairing/bls12_381_halo2curves`, `pairing/admit_pairing_composition` | The surface the hash-to-scalar, KEM, envelope, proof, and chain tickets consume |
| `harness-crypto/benchmark` | The measurement of scalar multiplication, multi-scalar multiplication, and pairing over every pairing concrete, each constructed through the pairing factory, recording each concrete's measurements; names no library; creates the `apps/harness-crypto` crate | `pairing/factory`, `random/factory` | Each selected concrete's measurements, recorded |
| `domain/deployment_identity` | The registry-assigned deployment identity with its fallible `try_new` constructor enforcing its invariants | `domain/secret` | The deployment every derivation and object is bound to |
| `domain/suite_identifier` | The cryptographic suite identifier and version with its fallible `try_new` constructor enforcing its invariants | `domain/secret` | The suite every derivation is bound to |
| `domain/parameter_set_identifier` | The parameter-set identifier with its fallible `try_new` constructor enforcing its invariants | `domain/secret` | The set every capsule and wrapping key is bound to |
| `domain/group_index` | The piece-group index with its fallible `try_new` constructor enforcing its bounds | `domain/secret` | The group every piece-group key is bound to |
| `domain/piece_geometry` | The piece size, piece-group size, and addressable extent with their fallible `try_new` constructor enforcing alignment | `domain/secret` | The geometry every derivation and the cipher are bound to |
| `hashing/blake3_bao` | The BLAKE3/Bao concrete as one adapter: root and outboard construction, verification, challenge and response, and keyed mode for the keyed plaintext-root disclosure mode, which takes the plaintext-root key as a borrowed `Secret` the key-derivation family derives and computes keyed roots and keyed authentication paths through `blake3`'s keyed mode, `bao` serving the unkeyed roots and outboards; authors, as its producers, the commitment interface with root and outboard construction over a byte stream, streaming and random-access verification against a root with an authentication path, and random chunk challenge and response, the root, outboard, path, and chunk types with their fallible constructors, the root type being the object's BLAKE3/Bao root, the asset's identity, the plaintext root for a public asset and the ciphertext root for a private one, refusing the all-zero value, the family's member identifier and its commitment-scheme and chunk-granularity value identifier types with total spellings and declared wire codes, the commitment-scheme identifier the hash-card carries, the declaration carrying its member's identifier and providing the commitment scheme and the chunk granularity without a concrete-name text, and the family's mock; every hashing operation is over bytes and the family carries no digest operation and no identity-hash identifier, so it depends on no encoding; creates the `adapters/hashing` crate; pins `blake3` and `bao` | `workspace/cargo`, `domain/secret` | CR-02 contract, construction, verification, and challenges; EC-02; SW-04 |
| `hashing/lookup_hashing_declaration` | The family's declaration lookup, returning a member's declaration without constructing the member | `hashing/blake3_bao` | a declaration read with no concrete constructed |
| `hashing/admit_hashing_selection` | The family's selection admission: receives the admitted member identifier and the untrusted settings document, refuses a document that supplies any setting, with the family's own error naming the member, since no member has a settable value, and returns the selection naming the member; an absent member list is the declared set, and every callee error is carried unchanged | `hashing/blake3_bao`, `hashing/lookup_hashing_declaration` | configured spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `hashing/admit_hashing_composition` | The family's composition admission, comparing the selected member's declared commitment-scheme and chunk-granularity provisions with the requirements the composed declarations state and returning the family's `Incompatible` flavor where they disagree | `hashing/lookup_hashing_declaration`, `hashing/admit_hashing_selection` | the hash-card's named commitment scheme compared before construction, disagreement refused as `Incompatible` |
| `hashing/factory` | The hashing family's factory function, which calls the composition admission before it constructs anything and then constructs the member the admitted selection names, and the crate's public surface; carries the family's integration test | `hashing/blake3_bao`, `hashing/admit_hashing_composition` | The surface every content root passes through |
| `encoding/canonical_field_reader` | The encoding family's family-owned canonical field reader, the one check that a field list matches a description's `FIELDS` in count and in kind: `try_new` over the field list and the field kinds refusing a wrong count, `read_value` refusing a value of the wrong kind at its index, a read per kind, and `read_form` carrying a form's refusal with its index, the reader's refusal one enum every caller carries unchanged; authors, in the family's `factory` module, the family's member identifier, the versioned encoding value identifier, and the canonical field-kind and type-name value identifier types with total spellings and declared wire codes, the eight canonical field kinds and their values, the byte-string, fixed twenty-byte, and 256-bit unsigned kinds among them, the field list, the kind of a value, and the canonical-field contract with its refusal, and their builders; creates the `adapters/encoding` crate | `workspace/cargo` | A field list refused for a wrong count before any kind is compared, a value of the wrong kind refused at its index, the lowest wrong value reported, each kind read returning its contents, a form's refusal carried with its index, and a read past the end refused, the only tests of count, kind, index, and ordering any description or encoder relies on |
| `encoding/abi` | The Ethereum ABI concrete through `alloy`'s sol types, implementing both interfaces as one adapter; untrusted bytes to the owned type or its error; authors, as its producers, `IEncoderAdapter` and `IDecoderAdapter` with the versioned encoding identifier, the encoding types, the declaration carrying its member's identifier without a concrete-name text and providing the versioned encoding and the total mapping from each canonical field-kind identifier to the `EncodingTypeName` identifier it gives that kind, and the family's mock; its known-answer vectors are the root type's through its description, and the encodings of the identifiers it names authored from the ABI specification, which the generator later mirrors, and its round trip and its rejections of truncated input, trailing bytes, non-canonical padding, and a wrong element count run over the same values through the description; encodes the byte-string kind, which carries a pairing group element's precompile encoding, as ABI `bytes` against its own known-answer vector, and, for the proof family's transcript descriptions, the fixed twenty-byte and 256-bit unsigned kinds, each named as a width, against theirs, the kinds being `encoding/canonical_field_reader`'s; maps each canonical kind to the one ABI type name it declares: `bytes32`, `bytes20`, `uint16`, `uint32`, `uint64`, `uint256`, `string`, and `bytes` for the eight canonical kinds respectively; `FixedBytes20` is always `bytes20`, never `address`; checks the field list it encodes through `encoding/canonical_field_reader`; pins `alloy` | `encoding/canonical_field_reader`, `hashing/factory` | ABI encoding of the root type through its description against its vectors; round trip; malformed input rejected; the hash-card's encoding identifier; every canonical field kind's ABI type name, including `FixedBytes20` as `bytes20`; the byte string a statement over group elements is encoded as; the widths a transcript's parties and entitlements are encoded as |
| `encoding/lookup_encoding_declaration` | The family's declaration lookup, returning a member's declaration without constructing the member | `encoding/abi` | a declaration read with no concrete constructed |
| `encoding/admit_encoding_selection` | The family's selection admission: receives the admitted member identifier the configuration or hash-card names and the untrusted settings document, refuses a document that supplies any setting, with the family's own error naming the member, since no member has a settable value, and returns the selection naming the member; an absent member list is the declared set, and every callee error is carried unchanged | `encoding/abi`, `encoding/lookup_encoding_declaration` | configuration and hash-card spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `encoding/admit_encoding_composition` | The family's composition admission, comparing the selected member's declared versioned-encoding and kind-to-type-name provisions with the requirements the composed declarations state and returning the family's `Incompatible` flavor where they disagree | `encoding/lookup_encoding_declaration`, `encoding/admit_encoding_selection` | the configuration's or hash-card's named encoding compared before construction, disagreement refused as `Incompatible` |
| `encoding/factory` | The encoding family's factory function, which calls the composition admission before it constructs anything and then constructs the member the admitted selection names, and the crate's public surface; carries the family's integration test across factory, concrete, and description | `encoding/abi`, `encoding/admit_encoding_composition` | The generic surface every hashed, signed, stored, or framed value passes through |
| `kdf/blake3_keyed` | The BLAKE3 keyed-derivation concrete; context strings, serialization, and output lengths frozen with known-answer vectors from an independent implementation, over a context its caller encodes through the encoding family; authors, as its producers, the interface that derives a key of a stated length from key material under an encoded context, the family's member identifier and its KDF value identifier type with total spellings and declared wire codes, the declaration carrying its member's identifier and providing the KDF value without a concrete-name text, and the family's mock; creates the `adapters/kdf` crate; pins `blake3` | `encoding/factory` | CR-05, CR-11 derivation contract and vectors |
| `kdf/lookup_kdf_declaration` | The family's declaration lookup, returning a member's declaration without constructing the member | `kdf/blake3_keyed` | a declaration read with no concrete constructed |
| `kdf/admit_kdf_selection` | The family's selection admission: receives the admitted member identifier and the untrusted settings document, refuses a document that supplies any setting, with the family's own error naming the member, since no member has a settable value, and returns the selection naming the member; an absent member list is the declared set, and every callee error is carried unchanged | `kdf/blake3_keyed`, `kdf/lookup_kdf_declaration` | configured spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `kdf/admit_kdf_composition` | The family's composition admission, comparing the selected member's declared KDF provision with the requirements the composed declarations state and returning the family's `Incompatible` flavor where they disagree | `kdf/lookup_kdf_declaration`, `kdf/admit_kdf_selection` | an incompatible composition refused before construction |
| `kdf/factory` | The key-derivation family's factory function, which calls the composition admission before it constructs anything and then constructs the member the admitted selection names, and the crate's public surface; carries the family's integration test across factory and concrete | `kdf/blake3_keyed`, `kdf/admit_kdf_composition` | The surface every off-chain derivation passes through |
| `hash-to-scalar/keccak256` | The keccak256 concrete under domain tags, reduced modulo the group order, with vectors the generator mirrors; authors, as its producers, the interface that maps domain-tagged bytes to a scalar of the resolved pairing's field, the domain-tag types with their fallible constructors, the family's member identifier and its hash-to-scalar and domain-tag value identifier types with total spellings and declared wire codes, the declaration carrying its member's identifier and providing the hash-to-scalar mapping and its domain tags without a concrete-name text, and the family's mock; creates the `adapters/hash-to-scalar` crate | `encoding/factory`, `pairing/factory` | CR-08, CR-09 identity-mapping and challenge contract and vectors |
| `hash-to-scalar/lookup_hash_to_scalar_declaration` | The family's declaration lookup, returning a member's declaration without constructing the member | `hash-to-scalar/keccak256` | a declaration read with no concrete constructed |
| `hash-to-scalar/admit_hash_to_scalar_selection` | The family's selection admission: receives the admitted member identifier and the untrusted settings document, refuses a document that supplies any setting, with the family's own error naming the member, since no member has a settable value, and returns the selection naming the member; an absent member list is the declared set, and every callee error is carried unchanged | `hash-to-scalar/keccak256`, `hash-to-scalar/lookup_hash_to_scalar_declaration` | configured spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `hash-to-scalar/admit_hash_to_scalar_composition` | The family's composition admission, comparing the selected member's declared hash-to-scalar and domain-tag provisions with the requirements the composed declarations state and returning the family's `Incompatible` flavor where they disagree | `hash-to-scalar/lookup_hash_to_scalar_declaration`, `hash-to-scalar/admit_hash_to_scalar_selection` | an incompatible composition refused before construction |
| `hash-to-scalar/factory` | The hash-to-scalar family's factory function, which calls the composition admission before it constructs anything and then constructs the member the admitted selection names, and the crate's public surface; carries the family's integration test across factory and concrete | `hash-to-scalar/keccak256`, `hash-to-scalar/admit_hash_to_scalar_composition` | The surface every value a contract recomputes passes through |
| `kem/bb1_depth_one` | The depth-one Boneh–Boyen concrete as one adapter: random master scalar and parameter set for escrow lineage, issuance with trivial identity-element refusal, seller-side rerandomization without the master scalar, the public validity check with encoding and subgroup validation, encapsulation under both identity scopes, well-formedness, and decapsulation to the encapsulated value; owns its parameter-set, credential, capsule, and identity-element types; authors, as its producers, `ICredentialKemAdapter` with setup, deriveIdentity, issue, rerandomize, isValid, encapsulate, isWellFormed, and decapsulate, the parameter-set, master-scalar, credential, capsule, identity-element, and encapsulated-value types as associated types, the identity mapping over caller-supplied canonical identity bytes, which under the asset scope are the asset's root, the root type's canonical field through its description, the family's member identifier and its entitlement and asset identity-scope value identifier types with canonical spellings and declared wire codes, the declaration carrying its member's identifier and providing the identity scopes it encapsulates under and the identity mapping's domain tag without a concrete-name text, and the family's mock; creates the `adapters/kem` crate | `pairing/factory`, `hash-to-scalar/factory`, `random/factory`, `domain/secret`, `hashing/factory`, `encoding/factory` | CR-08 contract and algebraic properties, LC-08, EC-06, CD-08 declaration |
| `kem/lookup_kem_declaration` | The family's declaration lookup, returning a member's declaration without constructing the member | `kem/bb1_depth_one` | a declaration read with no concrete constructed |
| `kem/admit_kem_selection` | The family's selection admission: receives the admitted member identifier and the untrusted settings document, refuses a document that supplies any setting, with the family's own error naming the member, since no member has a settable value, and returns the selection naming the member; an absent member list is the declared set, and every callee error is carried unchanged | `kem/bb1_depth_one`, `kem/lookup_kem_declaration` | configured spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `kem/admit_kem_composition` | The family's composition admission, comparing the selected member's declared identity scopes with the scope and pairing requirements the composed declarations state and returning the family's `Incompatible` flavor where they disagree | `kem/lookup_kem_declaration`, `kem/admit_kem_selection`, `pairing/lookup_pairing_declaration` | the suite's declared scope and the resolved pairing compared before construction, disagreement refused as `Incompatible` |
| `kem/factory` | The credential KEM family's factory function, which calls the composition admission before it constructs anything and then constructs the member the admitted selection names, and the crate's public surface; carries the family's integration test across factory and concrete | `kem/bb1_depth_one`, `kem/admit_kem_composition` | CD-08 admission; the surface the sidecar, generate, and delivery tickets consume |
| `envelope/possession_statement` | The family-owned description of a proof of possession's statement, the key and commitment in the selected pairing's typed groups, flattened to precompile encodings only at the encoding boundary and validated with that pairing on reconstruction, implementing the encoding contract as its canonical field sequence; creates the `adapters/envelope` crate | `encoding/factory`, `pairing/factory` | LC-08 and CR-09, the proof of possession's statement frozen as one field sequence |
| `envelope/pairing_elgamal` | The pairing ElGamal concrete as one adapter: two independently keyed envelope keys generated from caller-supplied secret draws, each with a Schnorr proof of possession whose challenge is the hash-to-scalar mapping of the encoded possession statement, identity-element and shared-secret rejection, encryption of a credential's `CredentialComponents` with independent coins drawn through the randomness family, decryption under the recipient's key pair; owns its key-pair, public-key, possession, and envelope types; authors, as its producers, `IKeyAgreementAdapter` with key generation under proofs of possession, wrap, and unwrap, the key-pair, public-key, possession, and envelope types as associated types with their fallible constructors, a recipient's public keys admitted only with their proof of possession, the family's member identifier and its envelope-algebra value identifier type with total spellings and declared wire codes, the declaration carrying its member's identifier and providing the envelope algebra and the proofs of possession's domain tags without a concrete-name text, and the family's mock | `envelope/possession_statement`, `pairing/factory`, `hash-to-scalar/factory`, `random/factory`, `domain/secret`, `encoding/factory`, `kem/factory` | CR-04 contract; LC-08 at the algebra level |
| `envelope/lookup_envelope_declaration` | The family's declaration lookup, returning a member's declaration without constructing the member | `envelope/pairing_elgamal` | a declaration read with no concrete constructed |
| `envelope/admit_envelope_selection` | The family's selection admission: receives the admitted member identifier and the untrusted settings document, refuses a document that supplies any setting, with the family's own error naming the member, since no member has a settable value, and returns the selection naming the member; an absent member list is the declared set, and every callee error is carried unchanged | `envelope/pairing_elgamal`, `envelope/lookup_envelope_declaration` | configured spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `envelope/admit_envelope_composition` | The family's composition admission, comparing the selected member's declared envelope algebra and domain tags with the requirements the composed KEM, pairing, and encoding declarations state and returning the family's `Incompatible` flavor where they disagree | `envelope/lookup_envelope_declaration`, `envelope/admit_envelope_selection`, `kem/lookup_kem_declaration`, `pairing/lookup_pairing_declaration`, `encoding/lookup_encoding_declaration` | the KEM, pairing, encoding, and algebra declarations compared before any key or envelope operation, disagreement refused as `Incompatible` |
| `envelope/factory` | The envelope family's factory function, which calls the composition admission before it constructs anything and then constructs the member the admitted selection names, and the crate's public surface; carries the family's integration test across factory and concrete | `envelope/pairing_elgamal`, `envelope/admit_envelope_composition` | The surface every envelope passes through |
| `chain-forms/evm` | See [chain-forms/evm ownership](#chain-formsevm-ownership) below this table | `encoding/factory`, `signature/factory` | CR-09: the forms a transcript's chain, entitlement, interval, and identity fields take, declared and admitted rather than assumed; the chain forms family's declaration the resolver reads |
| `chain-forms/lookup_chain_forms_declaration` | The chain forms family's declaration lookup, returning a member's declaration without constructing the member | `chain-forms/evm` | a declaration read with no concrete constructed |
| `chain-forms/admit_chain_forms_selection` | The family's selection admission: receives the admitted forms identifier, which the hash-card carries as an explicit field, supplied by the configuration in the harness and read from the deployment record in the daemon, and the untrusted settings document, refuses a document that supplies any setting, with the family's own error naming the member, since no member has a settable value, and returns the selection naming the member; an absent member list is the declared set, and every callee error is carried unchanged | `chain-forms/evm`, `chain-forms/lookup_chain_forms_declaration` | configured spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `chain-forms/admit_chain_forms_composition` | The family's composition admission: reads the selected member's declaration through the declaration lookup and compares, as typed identifiers, the forms identifier the suite requires with the selected member's, and the versioned encoding identifier the member requires with the one the handed encoder's declaration provides, returning the family's `Incompatible` flavor naming the identifiers that disagree before forms are constructed or any side effect | `chain-forms/lookup_chain_forms_declaration`, `chain-forms/admit_chain_forms_selection`, `encoding/lookup_encoding_declaration` | an incompatible encoder refused before construction or side effect |
| `chain-forms/factory` | The chain forms family's factory function, which calls the composition admission before it constructs anything, then constructs the forms concrete the admitted selection names over the borrowed encoder it is handed and hands the result to the family's consumer generic over the form interface, and the crate's public surface; carries the family's integration test across factory and concrete | `chain-forms/evm`, `chain-forms/admit_chain_forms_composition` | The surface the proof family's transcript descriptions, challenge, concrete, and factory, the chain family, the generator, verifier client, and measurement obtain their forms through, and later every consumer of the forms; incompatible encoder refusal |
| `proof/mint_statement` | The family-owned description of the mint transcript under delivery-statement version one, implementing the encoding contract as its canonical field sequence: the suite identifier, the chain, the entitlement contract, the asset's root taken through the root type's description, the parameter-set digest, the source and target entitlements, the old and new interval counters, the purpose, the issuer's identity in the seller field, the buyer's identity, the buyer's typed envelope keys, the typed new envelope, the expiry, and the mint relation's typed first messages, flattening group elements to precompile encodings only at the encoding boundary and validating them on reconstruction; relation-specific purpose identifiers with their declared sixteen-bit wire codes and relations; the delivery-statement version identifier; generic over the selected pairing and the chain forms family's identity, entitlement, interval, and chain-identifier forms, the entitlement contract taking the identity form; creates the `adapters/proof` crate | `encoding/factory`, `pairing/factory`, `envelope/factory`, `chain-forms/factory`, `hashing/factory`, `domain/suite_identifier`, `domain/parameter_set_identifier` | CR-09 statement binding under the version the hash-card names; the mint transcript frozen as one field sequence |
| `proof/transfer_statement` | The family-owned description of the transfer transcript under delivery-statement version one, serving sale and grant: the mint transcript's context, its asset root field included, with the seller's identity and typed envelope keys, the typed old envelope, and the transfer relation's typed first messages, the source entitlement equal to the target for a sale and the author's entitlement for a grant with the old counter the author's current interval and the new counter zero ; reconstruction validates group encodings with the selected pairing | `proof/mint_statement`, `pairing/factory`, `envelope/factory`, `chain-forms/factory` | CR-09; CD-05's grant statement frozen as one field sequence |
| `proof/schnorr_fs/challenge` | The function the concrete owns: the Fiat–Shamir challenge as the hash-to-scalar mapping, under the proof's domain tag, of a transcript's encoding through the encoding and hash-to-scalar concretes the hash-card's identifiers name; creates the `schnorr_fs` module | `proof/mint_statement`, `proof/transfer_statement`, `hash-to-scalar/factory`, `encoding/factory`, `pairing/factory`, `chain-forms/factory` | CR-09 statement binding |
| `proof/schnorr_fs` | The generalized Schnorr concrete as one adapter: the mint prover, the transfer prover from the seller's fresh decryption and total offset, and the reference verifier in both forms, second-group arithmetic and the hash-weighted pairing product with first-group arithmetic only; declares both forms; authors, as its producers, `IDeliveryProofAdapter` with proveMint, proveTransfer, and verify, the algebraic mint and transfer statement types over the pairing's group elements, the proof type as an associated type, the family's member identifier and its purpose and delivery-statement-version value identifier types with declared sixteen-bit and other wire codes, the declaration carrying its member's identifier and providing the supported envelope algebras and the challenge and weight domain tags, stating the verifier form it requires in the pairing family's identifier and the delivery-statement version it requires in the family's own, without a concrete-name text, and the family's mock | `proof/schnorr_fs/challenge`, `pairing/factory`, `envelope/factory`, `kem/factory`, `random/factory`, `chain-forms/factory` | CR-09, CD-01, CD-02 |
| `proof/lookup_proof_declaration` | The family's declaration lookup, returning a member's declaration without constructing the member | `proof/schnorr_fs` | a declaration read with no concrete constructed |
| `proof/admit_proof_selection` | The family's selection admission: receives the admitted member identifier and the untrusted settings document, refuses a document that supplies any setting, with the family's own error naming the member, since no member has a settable value, and returns the selection naming the member; an absent member list is the declared set, and every callee error is carried unchanged | `proof/schnorr_fs`, `proof/lookup_proof_declaration` | configured spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `proof/admit_proof_composition` | The family's composition admission, comparing the selected member's declared envelope-algebra and domain-tag provisions with the requirements the resolved envelope, pairing, and hash-card declarations state and returning the family's `Incompatible` flavor where they disagree | `proof/lookup_proof_declaration`, `proof/admit_proof_selection`, `envelope/lookup_envelope_declaration`, `pairing/lookup_pairing_declaration` | the resolved envelope's algebra, the verifier form the resolved pairing declares, and the delivery-statement version the hash-card names compared before construction, disagreement refused as `Incompatible` |
| `proof/factory` | The delivery proof family's factory function, which calls the composition admission before it constructs anything and then constructs the member the admitted selection names, and the crate's public surface; carries the family's integration test across factory and concrete | `proof/schnorr_fs`, `chain-forms/factory`, `proof/admit_proof_composition` | CR-09 contract; the admission rule the harness exercises |
| `workflows/sidecar/derivation_context` | The derivation context composed of the hashing family's root type as its asset component, the deployment identity, suite identifier, parameter-set identifier, group index, and geometry, with its fallible `try_new` constructor enforcing its invariants; housed in the sidecar layer, which composes it for derivation, and not in the `domain` crate; creates the `workflows` crate, which depends on the encoding family | `hashing/factory`, `domain/deployment_identity`, `domain/suite_identifier`, `domain/parameter_set_identifier`, `domain/group_index`, `domain/piece_geometry`, `encoding/factory` | The context every wrapping key and lineage derivation is domain-separated by |
| `workflows/sidecar/wrap` | The wrapping key derived through the KDF family from an encapsulated value and the context, and the XOR wrap of a piece-group key under it; creates no crate | `workflows/sidecar/derivation_context`, `kdf/factory`, `kem/factory`, `encoding/factory` | CR-11 wrap |
| `workflows/sidecar/unwrap` | The unwrap of a piece-group key from its wrapped value under the derived wrapping key | `workflows/sidecar/wrap` | CR-11 cross-set agreement |
| `harness-crypto/delivery/mint` | The mint delivery, generic over the form interface, over the borrowed credential KEM, key agreement, delivery proof, and randomness source its caller hands it, from a `&DeliveryContext<F, MintPurpose>` and an `AdmittedKemIdentity` it is handed: sets up a parameter set, derives the identity element, issues a credential, generates and admits the recipient's keys, wraps the credential to them, and proves the mint statement, drawing master, `u0`, `u1`, issue, recipient `x`, and recipient `y` in that order; returns the statement, cloned from the borrowed context, and the proof, the parameter set, identity element, and credential, the mint prover's witnesses, `MasterScalarComponents`, the issue randomness, and the wrap's `EnvelopeCoins`, each copied through `Secret::try_new` over its exposed value since the prover consumes what it is given, the recipient as a `DeliveryHolder` of its key pair, public keys, and envelope, and the uniforms in draw order; each callee error carried unchanged; owns `AdmittedKemIdentity`, the KEM's `SetupScope` and `KemIdentity` paired and deriving `Clone` and `Copy`, `DeliveryHolder`, `MintDelivery`, and `MintWitnesses`; adds a `Clone` impl for `DeliveryContext` to the proof family's factory interface, manual since a derive would bound the forms, and `Clone` and `Copy` to `SetupScope` and `KemIdentity` in the KEM family's factory interface; creates the `delivery` module | `kem/factory`, `envelope/factory`, `proof/factory`, `pairing/factory`, `chain-forms/factory`, `random/factory`, `domain/secret` | CD-01 delivery at mint, with its witnesses held for every consumer that proves, encodes, or times it |
| `harness-crypto/delivery/transfer` | The transfer delivery, generic over the form interface, over the borrowed credential KEM, key agreement, delivery proof, and randomness source its caller hands it, from a `&DeliveryContext<F, TransferPurpose>`, the parameter set, the identity element, and the `DeliveryHolder` it transfers from: unwraps the holder's envelope under the holder's key pair, rerandomizes the credential, generates and admits the buyer's keys, wraps the credential to them, and proves the transfer statement, drawing buyer `x`, buyer `y`, and the rerandomization in that order; returns the statement and the proof, the rerandomized credential, the transfer prover's witnesses, the seller's `KeyPairComponents`, the offset, and the wrap's `EnvelopeCoins`, each copied as the mint copies its witnesses, the buyer as a `DeliveryHolder`, a valid holder for the next transfer, and the uniforms in draw order; each callee error carried unchanged; owns `TransferDelivery` and `TransferWitnesses` | `harness-crypto/delivery/mint`, `kem/factory`, `envelope/factory`, `proof/factory`, `pairing/factory`, `chain-forms/factory`, `random/factory`, `domain/secret` | CD-02 delivery at sale, chainable from any holder, with its witnesses held for every consumer that proves, encodes, or times it |
| `harness-crypto/generate/solidity/render` | The function the Solidity generate concrete owns that renders Solidity library source, opening with the license identifier and the compiler version pragma its params carry: a constants library from a library name and an ordered list of named entries, each a byte string, a 256-bit, sixty-four-bit, or sixteen-bit unsigned value, a boolean, or a string; and a vector-record library from a library name and, per vector kind, a struct name, the struct's members as names with their canonical field kinds and already-resolved `SolidityAbiType` names in canonical order, a decode function name, a path constant name, and the kind's configured file name, which the path constant holds alone, rendering each member as the type name it carries and using the kind only to choose Solidity data location (`Text` and `Bytes` in `memory`), each decode function as `abi.decode` of one line's bytes over the members' types in canonical order returning the struct, and each path constant as a string; owns the entry type, the record-declaration type, and the validated name types; pure, reading no adapter and touching no operating-system facility; creates the `generate` area and the `solidity` module; the vector files the concrete writes stay ABI-encoded and language-neutral, and only the libraries it renders are Solidity | `harness-crypto/benchmark` | CR-09 parity inputs: one rendering for every constant and every vector record type |
| `harness-crypto/generate/solidity/constants` | See [harness-crypto/generate/solidity/constants ownership](#harness-cryptogeneratesolidityconstants-ownership) below this table | `harness-crypto/generate/solidity/render`, `encoding/factory`, `pairing/factory`, `kem/factory`, `envelope/factory`, `proof/factory`, `chain-forms/factory` | CR-09 parity inputs; CR-11 the tags and the modulus a contract hashes under; LC-13 the key identifier and the intent's typed data a contract derives |
| `harness-crypto/generate/solidity/group_vectors` | The function the concrete owns that returns the pairing library's group-operation and pairing-product vector records, each kind drawn and holding the count its params carry for that kind, over scalars it draws through the randomness family, each record carrying its draws, its points in the precompile encodings the pairing declares, and the pairing-product verdict read from the reference: first-group addition, first-group multiplication, and the pairing-product check for every pairing, and second-group addition and both groups' multi-scalar multiplication where the pairing declares second-group arithmetic at the verifier; each record type with its canonical description implementing the encoding contract and the member names the record declaration `render` owns takes; declares `VectorCount` and `MsmTermCount` and the generate family's vector-kind identifier type, one spelling per vector kind over the declared set and no text keys, in the generate family's factory interface, the first source file that requires them, which the other vector functions, the generate family's per-kind settings, and the configuration consume | `harness-crypto/generate/solidity/render`, `encoding/factory`, `pairing/factory`, `random/factory` | CR-10 on-chain vectors |
| `harness-crypto/generate/solidity/rejection_vectors` | The function the concrete owns that returns the decode-rejection vector records, an enumerated kind holding exactly the candidates it enumerates: one record per candidate encoding, for each source group and for a scalar, each naming what it decodes into and holding the candidate's bytes, every record an input the suite must refuse, the outside-the-subgroup candidates being the pairing family's reference encodings, absent where a group is the whole curve, and the non-canonical scalar candidate being the scalar field's order; the record type with its canonical description and member names | `harness-crypto/generate/solidity/render`, `encoding/factory`, `pairing/factory` | CR-10 subgroup rejection and encoding edge cases on chain |
| `harness-crypto/generate/solidity/mapping_vectors` | The function the concrete owns that returns the hash-to-scalar vector records, a drawn kind holding the count its params carry for it, each a message it draws and the scalar that message maps to under each family's domain tag read from that family's declaration, the identity mapping's, the two proofs of possession's, and the delivery proof's challenge and weight, the tags the constants library carries, so one message under every tag proves the tag is bound; and the identity-mapping vector records, a kind over configured inputs holding one record per `AdmittedKemIdentity` its payload carries, each mapped through the credential KEM under that identity's setup scope, the scalar read from the identity element's components; admits no identity, every identity it is handed being admitted already; each record type with its canonical description and member names | `harness-crypto/generate/solidity/render`, `harness-crypto/delivery/mint`, `encoding/factory`, `hash-to-scalar/factory`, `kem/factory`, `envelope/factory`, `proof/factory`, `pairing/factory`, `random/factory` | CR-08, CR-11 identity-mapping and hash-to-scalar vectors |
| `harness-crypto/generate/solidity/possession_vectors` | The function the concrete owns that returns the possession vector records, a drawn kind holding the count its params carry for it: envelope keys with their proofs of possession's components, generated through the key-agreement family from secret draws the function makes and records; the proofs' nonces are drawn inside the adapter and are not recorded; the record type with its canonical description and member names | `harness-crypto/generate/solidity/render`, `encoding/factory`, `envelope/factory`, `pairing/factory`, `random/factory` | LC-08 at the algebra level, on chain |
| `harness-crypto/generate/solidity/challenge_vectors` | The function the concrete owns that returns the challenge vector records, generic over the form interface, a drawn kind holding the count its params carry for it, each record carrying the values the function drew through the randomness family: mint and transfer transcripts over the `&DeliveryContext<F, MintPurpose>` and `&DeliveryContext<F, TransferPurpose>` its payload carries, each context cloned per record and its purpose read from it, each transcript with its encoding through its description and the encoding family and its challenge, the hash-to-scalar mapping of that encoding under the challenge tag the proof family's declaration carries; rebuilds no form; the transcripts stand alone, since the proof family's challenge function is private to its crate and a proof in the second-group form carries no first messages; the record type with its canonical description and member names; declares `CanonicalDeliveryContext`, which carries no chain identifier and no entitlement contract, and `DeliveryContextField`, which it does not consume, until `harness-crypto/config/admit_statements` and `harness-crypto/run/admit_contexts` move them | `harness-crypto/generate/solidity/render`, `harness-crypto/delivery/mint`, `proof/factory`, `encoding/factory`, `hash-to-scalar/factory`, `chain-forms/factory`, `pairing/factory`, `random/factory` | CR-09, CR-11 challenge vectors |
| `harness-crypto/generate/solidity/proof_vectors` | The function the concrete owns that returns the mint-proof and transfer-proof vector records, generic over the form interface, each a drawn kind holding the count its params carry for it, each record a statement with its proof's components: a mint record is one mint delivery from `harness-crypto/delivery/mint` over the typed mint context and the `AdmittedKemIdentity` its payload carries, its draws that delivery's six uniforms; a transfer record is a mint delivery followed by a transfer delivery from `harness-crypto/delivery/transfer` over the typed transfer context, its draws those nine uniforms in draw order; each statement as its context values and its elements' precompile encodings, the first-group-only form carrying its second-group first messages; composes no delivery step, draws nothing, rebuilds no form, and admits no identity; each record type with its canonical description and member names | `harness-crypto/generate/solidity/render`, `harness-crypto/delivery/mint`, `harness-crypto/delivery/transfer`, `encoding/factory`, `kem/factory`, `envelope/factory`, `proof/factory`, `pairing/factory`, `chain-forms/factory` | CR-09 parity inputs |
| `harness-crypto/generate/solidity/settings` | See [harness-crypto/generate/solidity/settings ownership](#harness-cryptogeneratesoliditysettings-ownership) below this table | `harness-crypto/generate/solidity/render`, `harness-crypto/generate/solidity/group_vectors`, `encoding/factory`, `chain-forms/factory` | A generate member's settings admitted only within what its declaration states, a key naming no setting and a value outside its declared bounds each refused naming the setting, and the declared default taken where the document supplies none; the declaration every generate consumer reads |
| `harness-crypto/generate/solidity` | See [harness-crypto/generate/solidity ownership](#harness-cryptogeneratesolidity-ownership) below this table | `harness-crypto/generate/solidity/settings`, `harness-crypto/generate/solidity/constants`, `harness-crypto/generate/solidity/group_vectors`, `harness-crypto/generate/solidity/rejection_vectors`, `harness-crypto/generate/solidity/mapping_vectors`, `harness-crypto/generate/solidity/possession_vectors`, `harness-crypto/generate/solidity/challenge_vectors`, `harness-crypto/generate/solidity/proof_vectors`, `harness-crypto/delivery/mint`, `pairing/factory`, `random/factory`, `kem/factory`, `envelope/factory`, `proof/factory`, `hash-to-scalar/factory`, `encoding/factory`, `chain-forms/factory`, `domain/suite_identifier` | CR-09 parity inputs; CR-10 and CR-11 on-chain vectors |
| `harness-crypto/generate/lookup_generate_declaration` | The generate family's declaration lookup, from a member identifier to that member's declaration | `harness-crypto/generate/solidity` | The generate member's requirements and emitted suite identity read before construction |
| `harness-crypto/generate/admit_generate_selection` | The generate family's selection admission, in the validating form: an exhaustive branch per member narrows the untrusted settings document, the member's own table, through that member's guard, reads that member's declaration through the declaration lookup its deps carry, and calls that member's settings admission through the handle its deps carry over that declaration, each refusal carried intact in a variant named by the member, returning the settings-carrying selection | `harness-crypto/generate/solidity/settings`, `harness-crypto/generate/solidity`, `harness-crypto/generate/lookup_generate_declaration` | A generate selection built only inside the generate family, so a further generate concrete adds a table and no key to any shared table |
| `harness-crypto/generate/admit_generate_composition` | The generate family's composition admission: its params carry the selection's identifier and its payload the declarations of the resolved chain forms and the borrowed encoder; it reads the member's declaration through the declaration lookup and the resolved chain forms' and encoder's declarations through their families' lookups and compares the chain forms identifier and the encoding identifier the generate member requires with what they declare, returning the family's `Incompatible` flavor naming the identifiers and declared values that disagree and no concrete; the encoder admission `harness-crypto/generate/factory` performs moves here | `harness-crypto/generate/solidity`, `harness-crypto/generate/lookup_generate_declaration`, `harness-crypto/generate/admit_generate_selection`, `chain-forms/lookup_chain_forms_declaration`, `encoding/lookup_encoding_declaration` | An incompatible chain forms concrete or encoder refused before construction and before any file is written (CR-09 parity-input contract) |
| `harness-crypto/generate/factory` | The generate family's factory function, constructing the concrete the settings-carrying selection its params carry names over the borrowed adapters and declarations its deps carry and the family's per-kind quantities its params also carry, the member's own settings arriving inside the selection, the emitted suite identity carried as the typed `domain/suite_identifier` value, admitting the configuration only when its per-kind settings name exactly the vector kinds the concrete declares, with a count for each drawn kind and none for any other kind, and calling `harness-crypto/generate/admit_generate_composition` before constructing anything, with a branch per concrete, and its public surface; its consumer trait generic over the forms, handing a consumer only a concrete whose `Forms` are the consumer's; takes no scope; carries the family's integration test across factory and concrete | `harness-crypto/generate/solidity`, `chain-forms/factory`, `harness-crypto/generate/admit_generate_selection`, `harness-crypto/generate/lookup_generate_declaration`, `harness-crypto/generate/admit_generate_composition` | CR-09 parity-input contract |
| `contracts/evm/ParameterSets` | The parameter-set records the registry consumes: each set's digest, asset, suite identifier, suite-tagged material, identity scope, and liveness, registered and retired through the registry; the suite's first source file, so it creates `contracts/evm/foundry.toml`, granting `fs_permissions` read access to the generated directory and the ABI description directory and write access to the deployments directory alone with `ffi` disabled and `via_ir = true`, and `.github/workflows/contracts.yml` with `forge build`, `forge fmt --check`, and `forge test`, each authored once and complete | `workspace/ci` | The liveness the sidecar-per-live-set rule reads; the toolchain every contract ticket builds under |
| `domain/handshake_key_commitment` | The commitment to an identity's bound handshake key with the scheme identifier the signature family's `provides` exports, under which the key is bound, with its fallible `try_new` constructor refusing the all-zero commitment; its first consumer is the attempt context `chain/base` authors | `signature/factory` | The bound key every attempt context and binding names |
| `chain/base/settings` | See [chain/base/settings ownership](#chainbasesettings-ownership) below this table | `chain-forms/factory`, `pairing/factory` | A chain member's settings admitted only within what its declaration states, a key naming no setting and a value outside its declared bounds each refused naming the setting, and the declared default taken where the document supplies none; the declaration every chain consumer reads |
| `chain/base` | See [chain/base ownership](#chainbase-ownership) below this table | `chain/base/settings`, `chain-forms/factory`, `domain/handshake_key_commitment`, `domain/deployment_identity`, `hashing/factory`, `envelope/factory`, `proof/factory`, `pairing/factory`, `encoding/factory` | The registry's and the entitlement interface's ABI descriptions, the suite's boundary with the client; LC-04 tier mapping with reference age; CD-04 envelope recovery from events and calldata; LC-06 per-context bindings; a transaction's fee estimated and described with nothing signed or sent |
| `chain/lookup_chain_declaration` | The chain family's declaration lookup, from a member identifier to that member's declaration, returning it without constructing the member, so a level holding the identifier reads the forms identifier the member binds and the values it fixes | `chain/base` | No caller restates a chain member's declared value; a declaration read with no concrete constructed |
| `chain/admit_chain_selection` | The chain family's selection admission, in the validating form: receives the admitted member identifier and the untrusted settings document; an exhaustive branch per member narrows the document, the member's own table, through that member's guard, reads the member's declaration through the declaration lookup its deps carry, and calls the member's settings admission through the handle its deps carry over that declaration, each refusal carried intact in a variant named by the member, returning the settings-carrying selection; an absent member list is the declared set; the production counterpart of the family's declared-selection builders | `chain/base/settings`, `chain/base`, `chain/lookup_chain_declaration` | Configured spellings admitted through the owner and unknown text refused by a typed error carrying that text; a chain selection built only inside the chain family |
| `chain/admit_chain_composition` | The chain family's composition admission: reads the selected member's declaration through the declaration lookup and compares, as typed identifiers, the forms identifier the member binds, stated in the chain forms family's identifier, with the one the resolved forms' declaration provides, returning the family's `Incompatible` flavor naming the identifiers that disagree and no concrete | `chain/lookup_chain_declaration`, `chain/admit_chain_selection`, `chain-forms/lookup_chain_forms_declaration` | An incompatible forms concrete refused before construction or side effect |
| `chain/factory` | The chain family's factory function: calls the composition admission before it constructs anything, then constructs the member the settings-carrying selection names, `base`, and hands it to the family's consumer generic over the chain interface; the crate's public surface; carries the family's integration test across factory, concrete, and family-owned functions | `chain/base`, `chain/admit_chain_composition` | The one surface every chain consumer receives a chain through, the harness's verifier client, configuration, and run first |
| `harness-crypto/verifier/evm` | See [harness-crypto/verifier/evm ownership](#harness-cryptoverifierevm-ownership) below this table | `proof/factory`, `encoding/factory`, `chain-forms/factory`, `chain/factory` | CD-03 cross-verification |
| `harness-crypto/verifier/lookup_verifier_declaration` | The verifier family's declaration lookup, from a member identifier to that member's declaration | `harness-crypto/verifier/evm` | The verifier member's requirements read before construction |
| `harness-crypto/verifier/admit_verifier_selection` | The verifier family's selection admission, in the validating form: an exhaustive branch per member, the EVM member's branch refusing a settings document that supplies any setting, with the family's own error naming the member, since the member has no settable value, and returning the selection naming the member | `harness-crypto/verifier/evm`, `harness-crypto/verifier/lookup_verifier_declaration` | A verifier selection built only inside the verifier family, so no configuration function names a verifier concrete's type, constant, or constructor |
| `harness-crypto/verifier/admit_verifier_composition` | The verifier family's composition admission: its params carry the selection's identifier and its payload the declarations of the resolved chain forms, the selected chain member, the borrowed encoder, the selected pairing, and the selected generate member; it reads the member's declaration through the declaration lookup and compares the chain, forms, encoding, curve, verifier-form, and suite identifiers the verifier member requires — each stated in its providing family's value identifier, never naming another family's member — with what the resolved chain, forms, encoder, pairing, and generate declarations provide, the contract suite identity among them, returning the family's `Incompatible` flavor naming the identifiers and declared values that disagree and no concrete | `harness-crypto/verifier/evm`, `harness-crypto/verifier/lookup_verifier_declaration`, `harness-crypto/verifier/admit_verifier_selection`, `chain/lookup_chain_declaration`, `chain-forms/lookup_chain_forms_declaration`, `encoding/lookup_encoding_declaration`, `pairing/lookup_pairing_declaration`, `harness-crypto/generate/lookup_generate_declaration` | An incompatible chain, encoding, curve, or contract suite refused before construction and before effort |
| `harness-crypto/verifier/factory` | The verifier family's factory function, constructing the concrete the admitted selection its params carry names over the borrowed adapters and declarations its deps carry, the chain adapter among them, and the deployed location its params carry beside the selection, calling `harness-crypto/verifier/admit_verifier_composition` before constructing anything, with a branch per concrete, and its public surface; carries the family's integration test across factory and concrete | `harness-crypto/verifier/evm`, `harness-crypto/verifier/admit_verifier_selection`, `harness-crypto/verifier/lookup_verifier_declaration`, `harness-crypto/verifier/admit_verifier_composition`, `chain/factory` | CD-03 cross-verification contract |
| `harness-crypto/measure` | Size, timing, and gas capture per cell, over the borrowed pairing, credential KEM, key agreement, delivery proof, randomness source, and verifier concrete its caller hands it, the typed mint and transfer contexts, the `AdmittedKemIdentity`, and the configured iteration count, an adapter over the operating system's monotonic clock as `harness-crypto/benchmark` is: for each iteration, fresh inputs from `harness-crypto/delivery/mint` and `harness-crypto/delivery/transfer`, untimed, then only the measured call timed, `prove_mint` over a fresh mint's statement and witnesses, `prove_transfer` over a fresh mint and transfer's statement and witnesses, `verify` over each relation's statement and proof, and `decapsulate` over the mint delivery's credential and identity element and a capsule encapsulated, untimed, from a drawn uniform, each timing the mean over the iteration count, one per operation per cell; the capsule, envelope, and each relation's proof bytes from the pairing's encoders over the objects it produced; its own mint and transfer submitted through the verifier concrete, an accepted delivery's cost carried as the verifier family's cost description and a rejected verdict returned with its relation | `harness-crypto/delivery/mint`, `harness-crypto/delivery/transfer`, `kem/factory`, `envelope/factory`, `proof/factory`, `pairing/factory`, `chain-forms/factory`, `random/factory`, `harness-crypto/verifier/factory` | CD-07, RO-03 |
| `harness-crypto/report` | The release-evidence report recording the measurements, the piece-group size and curve defaults in force, and any adjustment the data supports, selecting nothing against a threshold, written under the paths the configuration names | `harness-crypto/measure` | AS-21 |
| `harness-crypto/config/layers` | The function the configuration owns that produces the configuration document from layered sources: parses the command line through `clap` into its repeatable `--config <PATH>` and `--set <KEY=VALUE>` arguments, reads each named file through `std::fs`, parses the defaults text it is handed, each file's text, and each assignment as TOML, merges them over the defaults in command-line order, a table merging key by key and any other value replacing the earlier one whole, and deserializes the merged table into the configuration document; owns the argument and document types, each document table refusing a key it does not declare; returns the argument error, a file's read error with its path, a text's parse error, and an assignment that is not a key and a value, each carried unchanged; an app module in the adapter role, as the benchmark is over the clock; creates the `config` module; pins `clap` with its derive feature, `toml`, and `serde` with its derive feature, the one place the harness names any of them | `workspace/cargo` | Every configured value overridable from a configuration file and from a command-line flag, applied in order over the shipped defaults |
| `harness-crypto/config/admit_selections` | See [harness-crypto/config/admit_selections ownership](#harness-cryptoconfigadmit_selections-ownership) below this table | `harness-crypto/config/layers`, `random/admit_random_selection`, `pairing/admit_pairing_selection`, `encoding/admit_encoding_selection`, `hash-to-scalar/admit_hash_to_scalar_selection`, `chain-forms/admit_chain_forms_selection`, `chain/admit_chain_selection`, `kem/admit_kem_selection`, `envelope/admit_envelope_selection`, `proof/admit_proof_selection`, `chain/lookup_chain_declaration`, `harness-crypto/verifier/admit_verifier_selection`, `harness-crypto/verifier/lookup_verifier_declaration` | The concrete and identifier per family and the pairing per cell every factory call takes, configured and never literal |
| `harness-crypto/config/admit_statements` | See [harness-crypto/config/admit_statements ownership](#harness-cryptoconfigadmit_statements-ownership) below this table | `harness-crypto/config/layers`, `harness-crypto/generate/solidity/challenge_vectors`, `domain/suite_identifier`, `hashing/lookup_hashing_declaration`, `domain/parameter_set_identifier`, `encoding/lookup_encoding_declaration`, `kem/lookup_kem_declaration`, `proof/lookup_proof_declaration`, `harness-crypto/generate/lookup_generate_declaration` | The context values, purposes, and identities every statement the harness proves carries, configured and never literal |
| `harness-crypto/config/admit_generate` | The function the configuration owns that admits the document's generate settings: hands the selected member's own table to `harness-crypto/generate/admit_generate_selection`, which narrows it through the member's guard, reads the member's declaration through the declaration lookup, calls the member's settings admission over it, and returns the settings-carrying selection, each refusal carried intact in a variant named by the member; and admits the family's per-kind quantities, each vector kind's text value mapped to its `VectorKind` and its count through `VectorCount` and `MsmTermCount` with its message length, the member's per-kind names, library names, Foundry project root, generated directory, compiler version, and license identifier arriving inside its own table; each refusal carried unchanged with its document path; admits each kind it is given and leaves the admission of the kind set against the member's declaration to `harness-crypto/generate/admit_generate_composition` | `harness-crypto/config/layers`, `harness-crypto/generate/solidity/group_vectors`, `harness-crypto/generate/admit_generate_selection`, `harness-crypto/generate/lookup_generate_declaration`, `harness-crypto/generate/admit_generate_composition`, `pairing/lookup_pairing_declaration` | The per-kind quantities and the selected member's own settings the generator takes, configured and never literal |
| `harness-crypto/config/admit_output` | The function the configuration owns that admits the document's output destinations, the places the run's outcome is written, and the report's destinations: standard output, a file at a path the document carries, or both, each destination's text mapped to the variant it names and a file's path carried; an unknown destination and an empty destination list each refused with its document path; owns the admitted output types | `harness-crypto/config/layers`, `harness-crypto/report` | The run's outcome written where the caller configures, from a shipped default, a file, or a flag, never literal |
| `harness-crypto/config` | The configuration's composing function: hands the shipped defaults and the command line to `harness-crypto/config/layers`, then the document's parts to `harness-crypto/config/admit_selections`, then to `harness-crypto/config/admit_statements` with the identity scope `admit_selections` admitted, then to `harness-crypto/config/admit_generate` and `harness-crypto/config/admit_output`, and admits the benchmark's iteration count and the measurement's settings, returning the typed configuration every factory call and every write in the harness takes or the first refusal unchanged; owns the typed configuration and its error; carries `apps/harness-crypto/harness.toml`, the shipped defaults embedded in the binary, a default for every value the document declares but the matrix's lists — the pairing concretes, the target-group encoding identifiers, and the encodings — and the deployment records, the chain selection, its member named `base` under its own settings table in place of a network profile, the generate selection, its member named `solidity` under its own settings table, the generated directory `contracts/evm/foundry.toml` grants, the verifier's selection, the measurement's settings, the report's destinations, and the output destinations among them; each list the configuration does not carry is the owning family's declared set, `PAIRING_CONCRETES`, `TARGET_GROUP_ENCODING_IDENTIFIERS`, and the encoding family's declared selections | `harness-crypto/config/admit_selections`, `harness-crypto/config/admit_statements`, `harness-crypto/config/admit_generate`, `harness-crypto/config/admit_output`, `harness-crypto/benchmark`, `harness-crypto/verifier/admit_verifier_selection`, `harness-crypto/measure`, `harness-crypto/report` | The values every factory call and every write in the harness takes, configured and never literal |
| `harness-crypto/run/admit_contexts` | The function the run owns that admits a `CanonicalDeliveryContext`, the chain identifier and the entitlement contract `harness-crypto/run/chain_consumer` hands it in its params, and its purpose into a `DeliveryContext<F, Purpose>`, generic over the form interface: the chain identifier, entitlement contract, source and target entitlements, old and new intervals, seller, and buyer rebuilt in transcript order through each form's `from_field`, the domain values and expiry taken from the canonical context; a form's refusal returned with its `DeliveryContextField`, or as the chain identifier's refusal, and the shared `FromFieldErrorReturn`, carried unchanged; the one place a canonical context becomes a typed one, so no later module rebuilds a form from a canonical context; moves `DeliveryContextField` out of `harness-crypto/generate/solidity/challenge_vectors`' interface into its own module, the type edit of the first file that consumes it | `harness-crypto/config/admit_statements`, `harness-crypto/generate/solidity/challenge_vectors`, `proof/lookup_proof_declaration`, `encoding/lookup_encoding_declaration`, `chain-forms/factory` | Every context a statement carries admitted into the forms once, where the forms are resolved |
| `harness-crypto/run/generate_consumer` | The run's consumer of the generate family's factory, one type implementing `IGenerateConsumer` generic over the forms: handed the generate concrete the factory admitted, calls `emit` with the typed mint and transfer contexts, the mapping identities, and the proof identity as `AdmittedKemIdentity` values its constructor carries, and returns the files written or the concrete's refusal unchanged; names no concrete, calls no factory, and admits nothing | `harness-crypto/generate/factory`, `harness-crypto/delivery/mint`, `harness-crypto/config` | The generated directory written from the configured statements through a concrete the run never names |
| `harness-crypto/run/verifier_consumer` | The run's consumer of the verifier family's factory, one type implementing `IVerifierConsumer` generic over the forms: handed the verifier concrete, calls `harness-crypto/measure` over it and the adapters and declarations it carries, with the typed mint and transfer contexts, the proof identity as an `AdmittedKemIdentity`, and the configured iteration count, and returns the measurements or the first refusal unchanged; names no concrete, calls no factory, and admits nothing | `harness-crypto/verifier/factory`, `harness-crypto/measure`, `harness-crypto/delivery/mint`, `harness-crypto/config` | The deployed verifier's acceptance and cost reached through a concrete the run never names |
| `harness-crypto/run/delivery_proof_consumer` | The run's consumer of the delivery proof family's factory, one type implementing `IDeliveryProofConsumer`: handed the delivery proof concrete, calls `harness-crypto/verifier/admit_verifier_composition` and `harness-crypto/generate/admit_generate_composition` over the declarations it looks up and carries, before the generate factory's emission writes any file, then calls `create_generate` over the adapters and declarations it carries with the configured generate concrete and the configured delivery-statement version, its consumer `harness-crypto/run/generate_consumer` constructed with the typed contexts and admitted identities it carries; then calls `create_verifier` with the one verifier selection over the borrowed chain it carries, the deployment record it was handed carried beside it in the factory's params as the deployed location, its consumer `harness-crypto/run/verifier_consumer` constructed with the typed contexts, the proof identity, and the configured iteration count; where it was handed no deployment record it returns the refusal naming the curve after the generate factory's call has written the files, and otherwise returns the files written and the measurements, or the first refusal unchanged | `harness-crypto/run/generate_consumer`, `harness-crypto/delivery/mint`, `harness-crypto/generate/factory`, `proof/factory`, `harness-crypto/verifier/factory`, `harness-crypto/run/verifier_consumer`, `harness-crypto/config`, `chain/factory`, `harness-crypto/verifier/lookup_verifier_declaration`, `harness-crypto/verifier/admit_verifier_composition`, `harness-crypto/generate/lookup_generate_declaration`, `harness-crypto/generate/admit_generate_composition` | The composed adapters handed to the generate factory with the configured values its params carry |
| `harness-crypto/run/key_agreement_consumer` | The run's consumer of the envelope family's factory, one type implementing `IKeyAgreementConsumer`: handed the key-agreement concrete, calls `create_delivery_proof` with the configured delivery-proof concrete and statement version, the envelope algebra the handed key-agreement concrete declares, and the verifier form the selected pairing declares, its consumer `harness-crypto/run/delivery_proof_consumer`; carrying the typed contexts, the admitted identities, the borrowed chain, and the verifier selection and the deployment-record lookup's result, and the measurement settings to the consumer it constructs | `harness-crypto/run/delivery_proof_consumer`, `harness-crypto/delivery/mint`, `proof/factory`, `envelope/factory` | The proof factory's admission of the configured selection exercised on every cell |
| `harness-crypto/run/kem_consumer` | The run's consumer of the credential KEM family's factory, one type implementing `IKemConsumer`: handed the KEM concrete and the scope it admitted, builds an `AdmittedKemIdentity` for the proof identity and each mapping identity by borrowing the configuration's admitted identities, each paired with its setup scope, then calls `create_key_agreement` with the configured key-agreement selection, its consumer `harness-crypto/run/key_agreement_consumer`; carrying the typed contexts, the admitted identities, the borrowed chain, and the verifier selection and the deployment-record lookup's result, and the measurement settings to the consumer it constructs | `harness-crypto/run/key_agreement_consumer`, `harness-crypto/delivery/mint`, `envelope/factory`, `kem/factory` | The admitted scope and the admitted identities carried to every later stage |
| `harness-crypto/run/chain_consumer` | The run's consumer of `chain/factory`, one type implementing the chain family's consumer trait generic over the forms: handed the chain concrete, reads the chain identifier and the entitlement contract from it through the family's consumer surface, admits the configured mint and transfer contexts with their purposes into typed contexts through `harness-crypto/run/admit_contexts` with those two values, a refusal returned unchanged, then calls `create_kem` with the configured KEM selection, its consumer `harness-crypto/run/kem_consumer`; carrying the typed contexts, the borrowed chain, the verifier selection and its deployment location, and the measurement settings to the consumer it constructs; names no concrete and admits no selection or identity | `harness-crypto/run/kem_consumer`, `harness-crypto/run/admit_contexts`, `kem/factory`, `chain/factory` | The statements' chain identifier and entitlement contract, and the chain the verifier reaches, come from the constructed chain and never from configuration |
| `harness-crypto/run/chain_forms_consumer` | The run's consumer of the chain forms family's factory, one type implementing `IChainFormsConsumer`: handed the forms concrete, calls `create_chain` with the configured chain selection over the forms it received, its consumer `harness-crypto/run/chain_consumer`; carrying the verifier selection and the deployment-record lookup's result, and the measurement settings to the consumer it constructs | `harness-crypto/run/chain_consumer`, `chain/factory`, `chain-forms/factory` | The forms every later stage is generic over, admitted by the configured identifier, and every context admitted into them once |
| `harness-crypto/run/encoding_consumer` | The run's consumer of the encoding family's factory, one type implementing `IEncodingConsumer`: handed the encoding concrete, calls `create_chain_forms` with the configured chain-forms selection, its consumer `harness-crypto/run/chain_forms_consumer`; carrying the verifier selection and the deployment-record lookup's result, and the measurement settings to the consumer it constructs | `harness-crypto/run/chain_forms_consumer`, `chain-forms/factory`, `encoding/factory` | The encoder every later stage borrows, admitted by the configured identifier |
| `harness-crypto/run/pairing_consumer` | The run's consumer of the pairing family's factory, one type implementing `IPairingConsumer`: handed a cell's pairing concrete, reads the curve's identifier from the received pairing's declaration and the contract suite identity the configured verifier member requires through `harness-crypto/verifier/lookup_verifier_declaration`, and looks up the deployment record that identity and the curve identifier key, obtains the hash-to-scalar adapter through `create_hash_to_scalar` with the configured selection and calls `create_encoding` with the cell's encoding selection, which it hands `create_encoding`, its consumer `harness-crypto/run/encoding_consumer`; carrying the verifier selection and the deployment-record lookup's result, and the measurement settings to the consumer it constructs | `harness-crypto/run/encoding_consumer`, `encoding/factory`, `hash-to-scalar/factory`, `pairing/factory`, `harness-crypto/verifier/lookup_verifier_declaration` | Each cell composed through every family's factory |
| `harness-crypto/run` | See [harness-crypto/run ownership](#harness-cryptorun-ownership) below this table | `harness-crypto/run/pairing_consumer`, `harness-crypto/report`, `harness-crypto/benchmark`, `harness-crypto/config`, `random/factory`, `pairing/factory`, `chain/lookup_chain_declaration` | Every family composed through its factory from the configuration's values; the benchmark's measurements per configured concrete |
| `harness-crypto/output` | The function the harness owns that writes the run's outcome, a success or a refusal, rendered as text, to each destination the configuration names, standard output, the file at the configured path, or both, in the configured order; an app module in the adapter role over the console and the filesystem, as the generate concrete is over the filesystem; returns a destination's write error with the destination it concerned | `harness-crypto/run`, `harness-crypto/config/admit_output` | The run's outcome, success or refusal, reaching every destination the caller configured |
| `harness-crypto/main` | The harness binary's entry point, `apps/harness-crypto/src/main.rs`: calls `harness-crypto/config` with the shipped defaults and the command line, `harness-crypto/run` with the typed configuration, and `harness-crypto/output` with the run's outcome and the configured destinations, and exits with a success status only when the run succeeded and every destination was written; a configuration refusal, which arrives before any destination is admitted, is written to standard error; holds no literal | `harness-crypto/config`, `harness-crypto/run`, `harness-crypto/output` | The harness invoked from its shipped defaults, a file, and a flag; its first run writes the generated directory the contract suite builds against |
| `contracts/evm/PairingLib` | The Solidity library over the precompiles in both forms, with contract-side subgroup checks where the precompile does not perform them, its constants, the values that are the same for every cell, read from the committed constants library and its test vectors from the committed vector files, each file the kind's configured file name the committed vector-record library carries joined to the cell directory being read and decoded by the function it carries, all of which the harness binary wrote; its tests run once per cell directory they enumerate under the generated directory, reading that directory's values and vector files | `contracts/evm/ParameterSets`, `harness-crypto/generate/factory`; mirrors the pairing family's declared precompile encodings | CR-10 on chain |
| `contracts/evm/DeliveryVerifier` | See [contracts/evm/DeliveryVerifier ownership](#contractsevmdeliveryverifier-ownership) below this table | `contracts/evm/PairingLib`, `harness-crypto/verifier/evm` | CD-03, CR-09 |
| `contracts/evm/IdentityAccount` | See [contracts/evm/IdentityAccount ownership](#contractsevmidentityaccount-ownership) below this table | `contracts/evm/ParameterSets`, `harness-crypto/generate/solidity/constants` | LC-13 through the account: foreign, expired, replayed, revoked-signer, and out-of-permission intents refused; the one seam every on-chain signature check passes through, per declared signature-scheme identifier |
| `contracts/evm/Binding` | Identity binding: `bindHandshakeKey`, storing the signature family's scheme identifier, the commitment to the handshake key, whose key travels in calldata, and the anchor hash, as a signed intent verified through the identity account, its call type assembled from the committed constants library's entries, and creating the identity's account at binding; authors its ABI description | `contracts/evm/IdentityAccount`, `harness-crypto/generate/solidity/constants` | The binding every attempt context's handshake key is checked against |
| `contracts/evm/Registry` | See [contracts/evm/Registry ownership](#contractsevmregistry-ownership) below this table | `contracts/evm/ParameterSets`, `contracts/evm/Binding`, `contracts/evm/IdentityAccount`, `chain/base`, `harness-crypto/generate/solidity/constants` | LC-06's three-state view and batch parity; LC-12's open requests authorizing nothing; alias records never rebound or removed; a recorded attestation absence admitted; the registry's and the entitlement interface's Solidity realizations matching the ABI descriptions `chain/base` binds; issuance lineage decided through the bound identity adapter; attestation through the adapter bound per declared attestation-source identifier |
| `contracts/evm/deploy` | See [contracts/evm/deploy ownership](#contractsevmdeploy-ownership) below this table | `contracts/evm/DeliveryVerifier` | CD-07 |

#### harness-crypto/generate/solidity/constants ownership

- The function the Solidity generate concrete owns that returns the constants library's entries, generic over the form interface.
- Each family's domain tags read from its declaration, the identity mapping's, the two proofs of possession's, and the delivery proof's challenge and weight, each a byte string.
- The delivery-statement version it is handed and the delivery purposes' codes from the proof family's declared purposes, each a sixteen-bit unsigned value.
- The mint and transfer transcripts' field sequences read from their descriptions over the forms in play, each rendered as one string of ABI type names read from the borrowed encoder for each field kind and wrapped as `SolidityAbiType`.
- The signed-intent form's entries read from the declaration of the forms it is handed: the key-identifier domain tag as a byte string, and the intent domain's name and version, the intent struct's name, and its member names, each a string.
- The type names of the nonce and expiry kinds and of each canonical field kind, each read from the borrowed encoder and wrapped as `SolidityAbiType`.
- The cell's values record — the canonical spellings of the curve identifier and of the precompile-encoding identifier the pairing declares, rendered at the file boundary, the scalar field's order, the two generators' precompile encodings, and whether the verifier has second-group arithmetic — its record type, canonical description, and member names owned.

#### harness-crypto/verifier/evm ownership

- Authors, as its producers, the language-neutral ABI description of the delivery verifier's interface, `contracts/evm/abi/IDeliveryVerifier.json`, complete, binds it through `alloy`'s `sol!`, and authors no contract source and no toolchain file.
- The EVM concrete, named for the EVM ABI it speaks, over the deployed verifier through `alloy`, generic over the form interface, constructed over the borrowed adapters and declarations its caller hands it and calling no factory.
- Its statements built over the forms `create_chain_forms` hands it, their context values the canonical field values its params carry.
- Reaches the deployed verifier at the endpoint of the chain adapter its factory hands it, the deployed location carried in the factory's params beside the selection, the concrete holding no endpoint, gas price oracle, or deployed verifier address of its own.
- Authors, as its producers, the verifier family's interface that reaches a suite's deployed delivery verifier, submits a statement and proof, and reads acceptance and cost, the cost carried as the family's cost description, which carries the chain family's estimated-fee description.
- Authors, as its producers, in the verifier family's `factory` module: the family's declaration type and the member identifier it names.
- The family's declaration states, for a member: its identifier; and the chain, forms, encoding, curve, verifier-form, and suite value identifiers it requires, each stated in its providing family's identifier.
- Authors, in its own interface, its declaration constant, stating each of those for the EVM concrete, implements the family's interface over that constant, and reads it; it has no settable value, so it has no settings admission.
- Authors the family's mock; pins `alloy` for the harness.

#### contracts/evm/DeliveryVerifier ownership

- Solidity mint and transfer verification over the statement fields, bit-for-bit with `proof/schnorr_fs`.
- Authors, as its producers, the delivery verifier's Solidity interface `contracts/evm/src/IDeliveryVerifier.sol`, realized against the ABI description `harness-crypto/verifier/evm` authors, with its builders, mock, and interface test proving the realization against that description's selector texts, and implements it.
- Declares the proof family's declared delivery-statement version and verifier-form identifiers under which the adapter registry binds it.
- Its constants, the values that are the same for every cell, read from the committed constants library, the scalar field's order and the generators' encodings reaching it as constructor arguments, and its test vectors from the committed vector files through the committed vector-record library.
- Its tests run once per cell directory they enumerate under the generated directory, reading that directory's values and vector files.

#### contracts/evm/IdentityAccount ownership

- The identity account under ERC-1271: its device registry keyed by key identifiers tagged with the signature family's scheme identifier, each signer verified through the signature verifier interface.
- `admitDevice`, `rescopeDevice`, and `revokeDevice`, each a signed intent under the control branch; the `isDeviceAdmitted` view; the intent nonce.
- The verification of a signed intent, EIP-712 typed data over the call's fields, chain, contract, nonce, and expiry, against a signer whose permissions cover the call.
- Its key-identifier tag, intent domain, intent struct and member names, and type names read from the committed constants library.
- Authors, as its producers, the signature verifier interface, which the adapter registry binds per declared scheme identifier, verifying a signature under the signature family's scheme identifier over a message for a scheme-tagged key, with its ABI description, builders, mock, and interface test.
- Authors the account's own interface with its ABI description, builders, mock, and interface test.

#### contracts/evm/deploy ownership

- The deployment script enumerating the cell directories under the generated directory, reading each one's values, and deploying the delivery verifier once per distinct curve identifier, in the verifier form that curve's values state and constructed with that curve's values, to Anvil and Base Sepolia.
- Emits each address as the deployment record keyed by the suite's own declared identity and that curve identifier, together as the layer file `contracts/evm/deployments/base-sepolia.toml`, written under the deployments directory `contracts/evm/foundry.toml` grants write access to.
- The layer is a TOML document whose only table is `[deployment_records]` holding one key per distinct curve identifier beneath the suite's declared identity, each a canonical value of kind `FixedBytes20` and the `0x` address of the verifier deployed in that curve's verifier form.
- The layer is a runtime output the harness's later runs read as a configuration layer over the shipped defaults through `--config`.
- Carries the grouping's commit with the generated directory and the deployment records.

#### chain-forms/evm ownership

- The EVM suite's forms as a private concrete of the chain forms family; the family's first concrete, private beneath the factory as every concrete is.
- The identity, entitlement, interval, and chain-identifier forms, over the twenty-byte, 256-bit unsigned, 64-bit unsigned, and 256-bit unsigned kinds the suite's schemas fix, each implementing the canonical-field contract.
- The identity form's address derivation, over a public key's canonical bytes and its scheme identifier, the signature family's scheme-identifier type its `provides` exports.
- The signed-intent form, EIP-712 typed data over the call's fields, chain, contract, nonce, and expiry, shared by every EVM chain concrete.
- The type implementing the form interface that names these forms, setting its associated declaration constant.
- Consumes the canonical-field contract `ICanonicalField`: a form's declared field kind, its conversion to one canonical field value, and its fallible reconstruction from one with a typed refusal being that contract's.
- Authors, as its producers, in `adapters/chain-forms`' factory module: the forms identifier, the family's member identifier, and `ChainFormsDeclaration`.
- Authors, after them, the family's form interface `IChainForms`, the trait carrying the identity, entitlement, interval, chain-identifier, and signed-intent forms as associated types each bounded by that contract, and the identity form declaring address derivation.
- Authors the family's mock, with the mock forms it names, address derivation, and the intent form.
- Constructed over a borrowed encoder its caller hands it, `chain-forms/factory` supplying it; it names no encoding concrete.
- Reads each EIP-712 struct member's type name from that encoder, holding no kind-to-type table of its own.
- Its declaration names the versioned encoding value identifier it requires, which `chain-forms/admit_chain_forms_composition` compares as an identifier with the encoding identifier the handed encoder's declaration provides.
- Creates the `adapters/chain-forms` crate with dependencies on `adapters/encoding` and `adapters/signature`.

#### contracts/evm/Registry ownership

- The registry of the EVM suite, in the suite's declared source language, constructed over the adapters the adapter registry's factory injects, each reached through its interface.
- Asset records keyed by the asset's root, and append-only alias records under a declared namespace and an opaque key that no Registry field interprets, each entry binding the key to one root with the source's signed statement or its recorded absence, never rebound or removed.
- Authors, as its producers, the entitlement token form's interface, `IEntitlement`, which the adapter registry binds, realized in Solidity against `contracts/evm/abi/IEntitlement.json`, which `chain/base` authors: `mint`, `deliver`, `grant`, and `replace` with their signed intents, the ownership and interval views, and the settlement events carrying the full envelope and proof at every mint, grant, and transfer; with its builders, mock, and interface test.
- Authors, as its producers, `IIdentityAdapter`, answering whether a proved identity may register, claim, mint, or administer an asset, and `IAttestationVerifier`, verifying a source's release attestation, each bound by the adapter registry, the attestation verifier per declared attestation-source identifier, each with its ABI description, builders, mock, and interface test.
- `registerAsset`; `registerDeployment` with the sidecar-per-live-set check through `contracts/evm/ParameterSets` and attestation verification through the attestation verifier interface, a declared absence admitted and recorded; `addSidecar`; `registerParameterSet` and `retireParameterSet` through `contracts/evm/ParameterSets`; each attestation source's key under the governance key; the `resolve` and `resolveBatch` views.
- The grant requests: `requestGrants` and `withdrawRequests`, each a signed intent verified through `contracts/evm/IdentityAccount`, its call type assembled from the committed constants library's entries, and the `openRequests` view for holders; an open request authorizes nothing.
- `evaluateAuthorization` and `evaluateAuthorizationBatch` over the attempt context: ownership of the exact token at the credential's interval read through the entitlement interface, issuance lineage through the identity adapter interface, and the identity's bound handshake key through `contracts/evm/Binding`, at the deployment's declared tier, returning `AUTHORIZED`, `PENDING_SETTLEMENT`, or `DENIED`, the batch a batching form of the identical operation.
- Realizes its interface in Solidity against `contracts/evm/abi/Registry.json`, which `chain/base` authors, its interface test proving the realization against that description's selector texts.

#### chain/base/settings ownership

- A function the Base concrete of the chain family owns, beneath the concrete and ahead of the adapter, as `harness-crypto/generate/solidity/settings` is beneath its concrete: the admission of the concrete's own settings, in the trusted form, over the narrowed settings document; the chain family's first ticket, it creates the `adapters/chain` crate, with dependencies on `adapters/chain-forms` and `adapters/pairing`, the `base` module, and the `settings` module beneath it.
- Reads the concrete's declaration: refuses a value outside its declared bounds and takes the declared default where the document supplies none; the guard refuses a key naming no field of the settings type; each refusal is the concrete's own typed error naming the setting.
- Owns, as the first source file that requires them, the concrete's settings type and the types it carries.
- Owns the concrete's settings-document type, its settings type with every field optional, and the guard that narrows the untrusted document, the concrete's own table beneath the family's, to it; `chain/admit_chain_selection` calls the guard, reads the declaration through the declaration lookup, and hands it to this admission, each reached through the family's private path.
- Authors, as its producers, in the family's `factory` module: the family's member identifier, the family's declaration type, the setting-statement and fee-entry declaration types the declaration carries, the interface version, and the endpoint type the settings carry.
- The family's declaration states, for a member: its identifier; its adapter and interface versions; the forms identifier it binds, in the chain forms family's identifier; the precompile encodings, as the pairing family's declared values it works with; the names and units of its fee entries; and, for each field of its settings type, the bounds within which it may be set and its default, a value the member fixes being stated among what it provides and never as a setting.
- Authors, in the concrete's interface, the concrete's declaration constant, stating each of those for Base: the gas price oracle, the chain identifier, and the precompile encodings as values it provides, and its settings — the network profile, the RPC endpoints, the archive endpoint, preferring its own node, the contract addresses per network profile, the relayer endpoint, and the backoff ceiling — each within declared bounds with a default; `chain/base` implements the family's interface over that constant and reads it.
- The gas price oracle, which only this member's model has, stays in its declaration and reaches no family surface.

#### chain/base ownership

- The Base concrete of the chain family, one adapter through `alloy`, binding as its associated forms the forms `chain-forms/factory` resolves, the signed-intent form among them, and authoring no form, address derivation and the intent form being `chain-forms/evm`'s.
- Authors, as its producers, the language-neutral ABI descriptions `contracts/evm/abi/Registry.json` and `contracts/evm/abi/IEntitlement.json`, each complete from the technical requirements' contract-suite API surface, binds them through `alloy`'s `sol!`, and authors no contract source and no toolchain file.
- Holds the client, Base's tier mapping to INCLUDED, SOFT, HARD, and SETTLED with reference age, `evaluateAuthorization` and its batch with per-context bindings, and the event and calldata reader for envelope recovery.
- Authors, as its producers, in the factory module `chain/base/settings` created: the chain interface, `ISettlementAdapter`, `IEntitlementStateAdapter`, the settlement reference and tier types, and the three-state authorization result.
- Authors the attempt context, generic over the forms, carrying the asset's root, the deployment identity, the entitlement contract and token, the identity and its handshake key commitment, the interval its credential was issued for, and the settlement reference of the view.
- Authors the family's selection, declared set, and error unions, the consumer surface through which a caller receives the constructed chain, and the family's mock.
- Authors the estimated fee of a transaction as the family's description: a capability every chain member provides, estimating, from the request its caller hands it, the fee that transaction would be charged, without signing, sending, or settling it; the fee's shape is the member's own, Base's composed of its execution gas and its L1 data fee read through its gas price oracle, and its entries carry names and units from the producing member's declaration, so no consumer names a fee field.
- Owns its own error type.
- Consumes from `chain/base/settings` its settings type, its declaration constant, and the family's declaration type with the member identifier it names; implements the family's interface over that constant and reads it.
- The endpoint, the chain identifier, and the entitlement contract, which every chain member provides, are read from the constructed chain through the family's consumer surface; a value the chain fixes is read before construction through `chain/lookup_chain_declaration`; the gas price oracle, which only this member's model has, stays in its declaration and settings and reaches no family surface.
- Reached only through `chain/factory`; no module outside the family names it.

#### harness-crypto/generate/solidity/settings ownership

- A function the Solidity concrete of the generate family owns, beneath the concrete as its other functions are and ahead of the adapter: the admission of the concrete's own settings, in the trusted form, over the narrowed settings document; creates the `settings` module beneath the concrete.
- Reads the concrete's declaration: refuses a value outside its declared bounds and takes the declared default where the document supplies none; the guard refuses a key naming no field of the settings type; each refusal is the concrete's own typed error naming the setting.
- Owns, as the first source file that requires them, the concrete's settings type and the types it carries: each vector kind's configured names, the library names, the file-name type, and the generated-directory type.
- Owns the concrete's settings-document type, its settings type with every field optional, and the guard that narrows the untrusted document, the concrete's own table beneath the family's, to it; `harness-crypto/generate/admit_generate_selection` calls the guard, reads the declaration through the declaration lookup, and hands it to this admission.
- Authors, as its producers, in the generate family's `factory` module: the family's declaration type, the member identifier it names, the vector-kind identifier, and each kind's declared sizing and second-group gate.
- The family's declaration states, for a member: its identifier; its adapter and interface versions; the vector kinds it emits, each with its sizing and whether it needs second-group arithmetic at the verifier; the encoding identifier and the chain forms identifier it requires; the contract suite identity it emits for; and, for each field of its settings type, the bounds within which it may be set and its default, a value the member fixes being stated among what it provides and never as a setting.
- Authors, in the concrete's interface, the concrete's declaration constant, stating each of those for the Solidity concrete; `harness-crypto/generate/solidity` implements the family's interface over that constant and reads it.

#### harness-crypto/generate/solidity ownership

- The Solidity concrete of the generate family, named and declared by the language it emits, one adapter generic over the form interface.
- The family's only module that touches the filesystem, an app module in the adapter role as the benchmark is over the clock.
- Constructed over the borrowed pairing, hash-to-scalar adapter, encoder, randomness source, credential KEM, key agreement, and delivery proof its caller obtained through their factories over the forms `create_chain_forms` handed it, with their declarations.
- Constructed with the delivery-statement version, the per-kind settings, the library names, the Foundry project root and the generated directory relative to it, and the Solidity compiler version and license identifier its libraries open with.
- Receives as payload the typed mint and transfer contexts, the mapping identities, and the proof identity as `AdmittedKemIdentity` values, admitted already by the run and the configuration, so it admits no identity and rebuilds no form.
- Resolves every record member's type name from its borrowed encoder, for the member's canonical field kind, before calling `render`, and hands `constants` the borrowed encoder.
- Consumes from `harness-crypto/generate/solidity/settings` its settings type, its declaration constant, and the generate family's declaration type with the member identifier, the vector-kind identifier, and each kind's sizing it holds.
- Calls each function it owns with each drawn kind's count.
- Renders the constants library through `render`.
- Encodes each vector record through the encoder under its canonical description into one line, `0x` and the encoding's lowercase hex digits and a line feed.
- Writes one vector file per kind its declaration names under the file name its per-kind settings carry.
- Renders through `render` the vector-record library declaring each kind's struct, decode function, and path constant under the names its per-kind settings carry, each path constant holding the kind's configured file name alone.
- Writes each kind's file in the cell directory, the generated directory joined with the name the received pairing's declaration supplies for its concrete and the name the borrowed encoder's declaration supplies, each admitted through `GeneratedFileName`, and both libraries in the generated directory outside every cell directory.
- Authors, as its producers, the generate family's interface that emits a target suite's constants, vector-record types, and vector files from the Rust reference under a caller-supplied destination, naming `VectorCount` and `MsmTermCount` as `harness-crypto/generate/solidity/group_vectors` declared them there.
- The interface is generic over the forms it names as its associated `Forms` type, and its payload carries contexts in those forms.
- Authors the per-kind settings type.
- Emits the vector kinds its declaration names with each kind's sizing, drawn, enumerated, or over configured inputs, each kind that needs second-group arithmetic gated on the verifier form the pairing declares, and one further kind, enumerated and not gated on second-group arithmetic, holding the cell's values record.
- Authors the family's mock.
- Calls no factory and names no chain's forms, no path, no file, library, or symbol name, no count, no curve, no profile, no compiler version, and no license.

#### harness-crypto/config/admit_selections ownership

- The function the configuration owns that admits the document's selections into the adapter families' own types.
- Admits the randomness source, the hash-to-scalar, chain-forms, KEM, key-agreement, delivery-proof, and verifier concretes with their identifiers, the identity scope, the envelope algebra, and the delivery-statement version, each a single selection, the chain-forms selection admitted through `chain-forms/admit_chain_forms_selection`, the chain selection, carrying Base's own settings from Base's own table, admitted through `chain/admit_chain_selection`, and the verifier selection admitted through `harness-crypto/verifier/admit_verifier_selection`; admits no precompile encodings.
- Admits the pairing concretes, the target-group encoding identifiers, and the encodings, each as a list, an absent list admitted as the family's declared set and a present list member by member against that set.
- Owns the admitted selection types.
- Adds `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq` where they are absent to `RandomSourceKind`, `PairingConcrete`, `PrecompileEncoding`, `HashToScalarConcrete`, `HashToScalarIdentifier`, `KemConcrete`, `KemIdentifier`, `KeyAgreementConcrete`, `KeyAgreementIdentifier`, and `DeliveryProofConcrete` in their families' factory interfaces, so the configuration holds them and the run hands each cell and each benchmarked concrete its own copy.

#### harness-crypto/config/admit_statements ownership

- The function the configuration owns that admits the document's statements and deployment records.
- Admits the deployment records, held in the document under `deployment_records` as a map from each contract suite's declared identity and curve name to address, each admitted value by value, a malformed address refused with its key, each a kind and a value the document carries; a fixed-width value as `0x`-prefixed hex of its exact width and an unsigned value within its kind's range.
- Admits the mint and transfer contexts as `CanonicalDeliveryContext`s.
- Their suite identifier, asset root, and parameter-set identifier come through the domain constructors and the hashing family's root type's fallible constructor.
- Their entitlements, intervals, seller, and buyer are canonical field values, with their expiry; a `CanonicalDeliveryContext` carries no chain identifier and no entitlement contract, which `harness-crypto/run/chain_consumer` reads from the constructed chain and hands to `harness-crypto/run/admit_contexts`.
- Admits the mint and transfer purposes through their declared codes.
- Admits the proof identity and each identity the identity mapping covers, each `0x`-prefixed hex, under the identity scope `harness-crypto/config/admit_selections` admitted.
- An identity is admitted as the root type under the asset scope, a length other than 32 bytes refused with that length and an all-zero root with the root type's refusal carried unchanged, and as canonical bytes under the entitlement scope, so no later module admits an identity.
- Each refusal is carried unchanged with its document path.
- Owns the admitted statement, deployment-record, and identity types.
- Moves `CanonicalDeliveryContext`, with its builders, out of `harness-crypto/generate/solidity/challenge_vectors`' interface and mock into its own module, the type edit of the first file that consumes it.

#### harness-crypto/run ownership

- The harness run, the composing function over the consumers it owns and the harness's composition root.
- Obtains the randomness source for the cell consumers through `create_random_source`, borrowed by each of them.
- Iterates the selected pairing concretes in list order and, within each, the selected target-group encoding identifiers in list order; for each such pair it obtains a randomness source of its own through `create_random_source`, since the `PairingBenchmark` owns the `Box<dyn IRandomSourceAdapter>` it is constructed with, and calls `create_pairing` with the pair's concrete, the pair's identifier, and the precompile encodings the configured chain selection's declaration states, read through `chain/lookup_chain_declaration`, its consumer a `PairingBenchmark` over that source and the configured iteration count.
- Then, for each selected encoding in list order, calls `create_pairing` with the pair's concrete and the pair's identifier likewise, its consumer `harness-crypto/run/pairing_consumer`, the cell's encoding carried to that consumer as its encoding selection.
- For each cell it constructs the consumer chain with the verifier selection, so every cell's verifier is reached at the deployment record looked up by the contract suite identity the selected verifier member's declaration requires and the curve name the cell's pairing declares.
- No refusal ends the run: a refusal from any call is that pair's or that cell's outcome and the next call is made, so the run reaches every compatible combination however many incompatible ones precede it; a pair or cell is completed when it ran to its end, incompatible when a family's factory refused the combination at its admission before constructing anything, or refused otherwise, a cell with no deployment record among them.
- Returns the run's outcome, the benchmark's measurements per pair and every cell's outcome once, in iteration order, each naming its members — the files written, the measurements, and the report `harness-crypto/report` records over them where the cell ran to completion, its refusal unchanged where it refused — the error arm carrying every pair's and every cell's outcome when any is refused, and the first refusal unchanged of a stage before the pairs.
- Owns the outcome and its error; with its consumers, the harness's only caller of a family's factory.
- Its public integration test runs the whole chain over the shipped defaults, which hold no deployment record: each admitted cell writes its files and returns the verifier-stage refusal, each cell whose identifier its concrete does not declare is reported incompatible, and the error arm is returned for the verifier-stage refusals alone.
- The run over the layer `contracts/evm/deploy` emits, passed as `--config ../../contracts/evm/deployments/base-sepolia.toml` to `config`, each cell's run accepted with a cost present through the verifier family's cost description, which carries the chain family's estimated-fee description, no cost value asserted since cost is a measured value with no target, is an end-to-end test run after `contracts/evm/deploy` executes, outside the ordinary test cycle.

### Protocol core and contract suite

The signature and swarm milestones are at ticket resolution; they depend on nothing the harness measures and run beside it from `encoding/factory` and `hashing/factory` onward, since their first tickets consume the encoding and hashing families, and each begins with the first concrete of its family, which carries the family's interface, and closes each family at its factory. The tickets on the path to `chain/factory` — `domain/handshake_key_commitment`, `chain/base/settings`, `chain/base`, the chain family's declaration lookup, selection admission, and composition admission, and `chain/factory` — are at ticket resolution in the harness table, ahead of `harness-crypto/verifier/evm`, because the harness's verifier client, configuration, and run consume the chain family through its factory. The remaining milestones are sprints, re-mapped to tickets when the harness closes, each sprint row below holding what the path leaves in it.

The hashing family is a complete harness ticket sequence at the head of the harness grouping beside the domain's identifier modules: `hashing/blake3_bao`, `hashing/lookup_hashing_declaration`, `hashing/admit_hashing_selection`, `hashing/admit_hashing_composition`, and `hashing/factory`, in that dependency order, because its root type is the asset's identity and the first type the encoding family describes.

#### Signature services, at ticket resolution

| Ticket | Owns | Depends on | Proves |
| --- | --- | --- | --- |
| `signature/ed25519` | The Ed25519 concrete with domain separation and complete-message binding; authors, as its producers, `ISignatureAdapter` with sign and isValidSignature over the public-key and signature types the concrete supplies as associated types, with their fallible constructors, no key recovery and no address derivation in the trait, and, in the family's factory module, the family's member identifier, the family's declaration type, and the family's scheme-identifier type with its `uint8` wire mapping, exported through `provides`, the per-layer declaration of scheme identifier, key and signature lengths, recovery support, and layers served, and the family's mock; creates the `adapters/signature` crate; pins `ed25519-dalek` | `encoding/factory`, `random/factory` | CR-03 contract and CR-03 |
| `signature/secp256k1` | The secp256k1 concrete through `alloy`, declaring recovery support, signing the bytes the resolved chain forms' signed-intent form hands it and owning no message format; pins `alloy` for the signature crate | `signature/ed25519` | CR-03, IW-07, LC-13 signature verification |
| `signature/ml_dsa_65` | The ML-DSA-65 concrete, the lattice scheme FIPS 204 standardizes at NIST security category 3, as one adapter implementing `ISignatureAdapter`: key generation from a seed, signing with the randomness it draws through the randomness family, verification, domain separation carried in the scheme's context string, and complete-message binding; owns its public-key and signature types with their fallible constructors, each refusing a wrong length; declares its member identifier in the family's declared real set and its scheme identifier in the family's scheme-identifier type, its key and signature lengths, no recovery support, and the handshake and content-signing layers; pins `fips204`, through its seed-based key generation and signing | `signature/ed25519`, `random/factory` | CR-03 for a scheme without recovery, against the FIPS 204 known-answer vectors |
| `signature/lookup_signature_declaration` | The family's declaration lookup, returning a member's declaration without constructing the member | `signature/ed25519`, `signature/secp256k1`, `signature/ml_dsa_65` | a declaration read with no concrete constructed |
| `signature/admit_signature_selection` | The family's selection admission: receives the admitted member identifier and the untrusted settings document, refuses a document that supplies any setting, with the family's own error naming the member, since no member has a settable value, and returns the selection naming the member; an absent member list is the declared set, and every callee error is carried unchanged | `signature/ed25519`, `signature/secp256k1`, `signature/ml_dsa_65`, `signature/lookup_signature_declaration` | configured layer and scheme spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `signature/admit_signature_composition` | The family's composition admission, comparing the selected member's declared scheme identifier, layers served, key and signature lengths, and recovery support with the requirements its params state and returning the family's `Incompatible` flavor where they disagree; the chain forms family consumes the signature family's scheme identifier through `provides` and hands the signed-intent bytes to the signature adapter, so no signature ticket depends on chain forms | `signature/lookup_signature_declaration`, `signature/admit_signature_selection` | the signature requirements compared as typed identifiers before construction or signing, disagreement refused as `Incompatible` |
| `signature/factory` | The signature family's factory function, which calls the composition admission before it constructs anything and then constructs the member the admitted selection names, resolving a layer's required scheme as a typed identifier, with a branch per concrete, `ed25519`, `secp256k1`, and `ml_dsa_65`, and the crate's public surface; carries the milestone's integration test across every concrete, a wrong-scheme substitution refused across all of them, and the milestone's commit | `signature/ed25519`, `signature/secp256k1`, `signature/ml_dsa_65`, `signature/admit_signature_composition` | The surface every layer signs through |

#### Swarm transport, discovery, storage, and seed host, at ticket resolution

| Ticket | Owns | Depends on | Proves |
| --- | --- | --- | --- |
| `transport/rqbit/settings` | The `librqbit` concrete's settings admission, ahead of the concrete and the family's first ticket: the concrete's settings type — the listening port and interface, port mapping, peer connection limits, the encryption requirement for compatibility peers, and the backoff ceiling — its settings-document type with every field optional and the guard that narrows the untrusted document to it, refusing a key that names no setting, and the admission that refuses a value outside its declared bounds and takes the declared default where the document supplies none, each refusal the concrete's own typed error naming the setting; authors, as its producers, in the family's factory module, the family's member identifier, the family's declaration type, and the locator-form, integrity-structure, and engine-capability identifiers the declaration carries, each with one spelling over the family's declared set, and, in the concrete's interface, the concrete's declaration constant stating each setting's bounds and default; creates the `adapters/transport` crate | `hashing/factory`, `encoding/factory` | the transport concrete's settings admitted within its declaration |
| `transport/rqbit` | The `librqbit` concrete over the overlay branch the workspace manifest names, constructed from its admitted settings: the piece-completion hook, the peer-injection call, and the storage-backend trait on that branch with each hook's upstream issue; root-to-infohash translation; torrent creation per object with the ciphertext and each sidecar as its own object and locator; Bao verification of every completed piece after the library's check, through the hashing family; seeder-map peer injection; the engine capability; authors, as its producers, `ISwarmTransportAdapter`, the locator types with their fallible constructors, and the family's mock; pins `librqbit` | `transport/rqbit/settings`, `hashing/factory`, `encoding/factory` | SW-01 contract, SW-01, SW-06, EC-02 |
| `transport/lookup_transport_declaration` | The family's declaration lookup, returning a member's declaration without constructing the member | `transport/rqbit` | a declaration read with no concrete constructed |
| `transport/admit_transport_selection` | The family's selection admission: receives the admitted member identifier and the untrusted settings document, narrows that document through the settings module's guard, invokes its settings admission, and returns the selection carrying the admitted settings; an absent member list is the declared set, and every callee error is carried unchanged | `transport/rqbit/settings`, `transport/lookup_transport_declaration` | configured spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `transport/admit_transport_composition` | The family's composition admission, comparing the selected member's declared locator-form, integrity-structure, and engine-capability provisions with the requirements the composed hashing, encoding, and chain declarations state and returning the family's `Incompatible` flavor where they disagree | `transport/lookup_transport_declaration`, `transport/admit_transport_selection`, `hashing/lookup_hashing_declaration`, `encoding/lookup_encoding_declaration`, `chain/lookup_chain_declaration` | the hashing, encoding, and chain requirements compared before construction and before any peer or transfer effect |
| `transport/factory` | The transport family's factory function, which calls the composition admission before it constructs anything and then constructs the member the admitted selection names, with a branch per concrete, and the crate's public surface, including the engine capability; carries the family's integration test across factory and concrete | `transport/rqbit`, `transport/admit_transport_composition` | The surface the seed host, resolution, and library-backed discovery consume |
| `discovery/admit_peers` | The family-owned function admitting the addresses a source yielded into discovered peers: each address admitted through the peer type's fallible constructor, a refused address dropped, a repeat kept once, in first-arrival order, each peer carrying the discovery-source identifier its params carry; authors, as its producers, `IPeerDiscoveryAdapter`, the key and peer types with their fallible constructors, and, in the family's factory module, the family's member identifier, the family's discovery-source identifier with one spelling per declared source, the family's declaration type, and the family's mock; creates the `adapters/discovery` crate | `workspace/cargo` | SW-02 contract; SW-02, unavailable and malicious results handled safely |
| `discovery/local/settings` | The local source's settings admission, ahead of the concrete: its settings type — the backoff ceiling — its settings-document type with every field optional and the guard that narrows the untrusted document to it, refusing a key that names no setting, and the admission that refuses a value outside its declared bounds and takes the declared default where the document supplies none, each refusal the concrete's own typed error naming the setting; authors, in the concrete's interface, the concrete's declaration constant stating the setting's bounds and default | `discovery/admit_peers` | the local source's settings admitted within its declaration |
| `discovery/local` | Local-network discovery as a source, constructed from its admitted settings, its heard addresses admitted through `discovery/admit_peers` | `discovery/local/settings`, `discovery/admit_peers` | SW-02 |
| `discovery/locator_of` | The family-owned function converting a discovery key into the object locator that carries the key's info-hash, the locator the transport's engine is asked for a peer feed of | `discovery/admit_peers`, `transport/factory` | SW-02 |
| `discovery/dht/settings` | The DHT source's settings admission, ahead of the concrete: its settings type — the backoff ceiling — its settings-document type with every field optional and the guard that narrows the untrusted document to it, refusing a key that names no setting, and the admission that refuses a value outside its declared bounds and takes the declared default where the document supplies none, each refusal the concrete's own typed error naming the setting; authors, in the concrete's interface, the concrete's declaration constant stating the setting's bounds and default | `discovery/admit_peers` | the DHT source's settings admitted within its declaration |
| `discovery/dht` | The library's DHT as a source, constructed from its admitted settings, through the engine capability the transport factory's surface exposes | `discovery/dht/settings`, `discovery/admit_peers`, `discovery/locator_of`, `transport/factory` | SW-02 |
| `discovery/pex/settings` | The peer-exchange source's settings admission, ahead of the concrete: its settings type — the backoff ceiling — its settings-document type with every field optional and the guard that narrows the untrusted document to it, refusing a key that names no setting, and the admission that refuses a value outside its declared bounds and takes the declared default where the document supplies none, each refusal the concrete's own typed error naming the setting; authors, in the concrete's interface, the concrete's declaration constant stating the setting's bounds and default | `discovery/admit_peers` | the peer-exchange source's settings admitted within its declaration |
| `discovery/pex` | Peer exchange as a source, constructed from its admitted settings, through the same engine capability | `discovery/pex/settings`, `discovery/admit_peers`, `discovery/locator_of`, `transport/factory` | SW-02 |
| `discovery/tracker/settings` | The tracker source's settings admission, ahead of the concrete: its settings type — the tracker URLs and the backoff ceiling — its settings-document type with every field optional and the guard that narrows the untrusted document to it, refusing a key that names no setting, and the admission that refuses a value outside its declared bounds and takes the declared default where the document supplies none, each refusal the concrete's own typed error naming the setting; authors, in the concrete's interface, the concrete's declaration constant stating each setting's bounds and default | `discovery/admit_peers` | the tracker source's settings admitted within its declaration |
| `discovery/tracker` | Trackers as a source, constructed from its admitted settings, through the same engine capability | `discovery/tracker/settings`, `discovery/admit_peers`, `discovery/locator_of`, `transport/factory` | SW-02 |
| `discovery/aggregate` | The family-owned aggregation: concurrent sources unioned and deduplicated, none authoritative | `discovery/admit_peers` | SW-02 aggregation under overlapping, conflicting, unavailable, and malicious results |
| `discovery/lookup_discovery_declaration` | The family's declaration lookup, returning a member's declaration without constructing the member | `discovery/local`, `discovery/dht`, `discovery/pex`, `discovery/tracker` | a declaration read with no concrete constructed |
| `discovery/admit_discovery_selection` | The family's selection admission: receives the admitted member identifier and the untrusted settings document, narrows that document through the selected member's settings module's guard, invokes its settings admission, and returns the selection carrying the admitted settings; an absent source list is the family's declared real set, and every callee error is carried unchanged | `discovery/local/settings`, `discovery/dht/settings`, `discovery/pex/settings`, `discovery/tracker/settings`, `discovery/lookup_discovery_declaration` | configured source spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `discovery/admit_discovery_composition` | The family's composition admission, comparing the selected member's declared source, locator, and engine-capability requirements with what the composed transport, hashing, encoding, and chain declarations provide and returning the family's `Incompatible` flavor where they disagree | `discovery/lookup_discovery_declaration`, `discovery/admit_discovery_selection`, `discovery/aggregate`, `transport/lookup_transport_declaration`, `hashing/lookup_hashing_declaration`, `encoding/lookup_encoding_declaration`, `chain/lookup_chain_declaration` | the transport, hashing, encoding, and chain requirements compared before construction and before any peer feed or aggregation effect |
| `discovery/factory` | The discovery family's factory function, which calls the composition admission before it constructs anything, constructing the concrete the admitted selection names and resolving the admitted sources behind the aggregation, with a branch per concrete the family holds at this ticket, `local`, `dht`, `pex`, and `tracker`, and the crate's public surface; carries the family's integration test across factory, concretes, and aggregation; each concrete authored after it, `discovery/seeder_map` among them, adds its branch in its own node | `discovery/local`, `discovery/dht`, `discovery/pex`, `discovery/tracker`, `discovery/aggregate`, `discovery/admit_discovery_composition` | SW-02: the surface the seed host, resolution, and the chain and submission families' seeder-map concrete consume |
| `storage/redb` | The `redb` concrete; authors, as its producers, the key-value store interface with tables, transactions, and checkpoints, its types with their fallible constructors, and, in the family's factory module, the family's member identifier over the declared storage engines and the family's declaration type, and the family's mock; creates the `adapters/storage` crate; pins `redb` | `encoding/factory` | XA-03 and SW-05 store contract; transactions and checkpoints under interruption |
| `storage/lookup_storage_declaration` | The family's declaration lookup, returning a member's declaration without constructing the member | `storage/redb` | a declaration read with no concrete constructed |
| `storage/admit_storage_selection` | The family's selection admission: receives the admitted member identifier and the untrusted settings document, refuses a document that supplies any setting, with the family's own error naming the member, since no member has a settable value, and returns the selection naming the member; an absent member list is the declared set, and every callee error is carried unchanged | `storage/redb`, `storage/lookup_storage_declaration` | configured spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `storage/admit_storage_composition` | The family's composition admission, comparing the selected member's declared capabilities with the requirements the composed encoding and seed-host declarations state and returning the family's `Incompatible` flavor where they disagree | `storage/lookup_storage_declaration`, `storage/admit_storage_selection`, `encoding/lookup_encoding_declaration` | the index's encoding requirements compared before construction and before any store effect |
| `storage/factory` | The storage-engine family's factory function, which calls the composition admission before it constructs anything and then constructs the member the admitted selection names, with a branch per concrete, and the crate's public surface; carries the family's integration test across factory and concrete | `storage/redb`, `storage/admit_storage_composition` | The surface every index, the job engine, and the configuration registry consume |
| `seed-host/owned/settings` | The owned concrete's settings admission, ahead of the concrete and the family's first ticket: the concrete's settings type — the ciphertext store root and capacity, the upload and download bandwidth limits, the idle CPU limit, the schedule windows, and pausing on battery and on a metered connection — its settings-document type with every field optional and the guard that narrows the untrusted document to it, refusing a key that names no setting, and the admission that refuses a value outside its declared bounds and takes the declared default where the document supplies none, each refusal the concrete's own typed error naming the setting; authors, as its producers, in the family's factory module, the family's member identifier, the family's declaration type, the holding-reason and eviction identifiers with one spelling each over their declared sets, and the capacity and holding-reason declaration, and, in the concrete's interface, the concrete's declaration constant stating each setting's bounds and default; creates the `adapters/seed-host` crate | `hashing/factory`, `encoding/factory`, `transport/factory`, `storage/factory` | the owned concrete's settings admitted within its declaration |
| `seed-host/owned` | The owned concrete, constructed from its admitted settings: persistent seeding independent of sessions, the root-keyed ciphertext store with holding reason, quota, and eviction, its index through the storage family, settings enforcement, archive enumeration; authors, as its producers, `ISeedHostAdapter` with held-and-served, take custody, release, capacity and usage, and enumerate, the holding types carrying the obligated-versus-voluntary distinction as the family's holding-kind identifier and their fallible constructors, and the family's mock | `seed-host/owned/settings`, `hashing/factory`, `encoding/factory`, `transport/factory`, `storage/factory` | SW-05 contract; SW-03, SW-05, SW-07, ST-06; PR-05 separation |
| `seed-host/rqbit/settings` | The delegated concrete's settings admission, ahead of the concrete: its settings type — the rqbit application's endpoint and the backoff ceiling for its HTTP API — its settings-document type with every field optional and the guard that narrows the untrusted document to it, refusing a key that names no setting, and the admission that refuses a value outside its declared bounds and takes the declared default where the document supplies none, each refusal the concrete's own typed error naming the setting; authors, in the concrete's interface, the concrete's declaration constant stating each setting's bounds and default | `seed-host/owned/settings` | the delegated concrete's settings admitted within its declaration |
| `seed-host/rqbit` | The delegated concrete, constructed from its admitted settings: the rqbit application driven through its HTTP API, root-to-identifier translation, and Bao custody challenges through the hashing family | `seed-host/rqbit/settings`, `seed-host/owned`, `hashing/factory` | SW-04 |
| `seed-host/lookup_seed_host_declaration` | The family's declaration lookup, returning a member's declaration without constructing the member | `seed-host/owned`, `seed-host/rqbit` | a declaration read with no concrete constructed |
| `seed-host/admit_seed_host_selection` | The family's selection admission: receives the admitted member identifier and the untrusted settings document, narrows that document through the selected member's settings module's guard, invokes its settings admission, and returns the selection carrying the admitted settings; an absent member list is the declared set, and every callee error is carried unchanged | `seed-host/owned/settings`, `seed-host/rqbit/settings`, `seed-host/lookup_seed_host_declaration` | configured spellings admitted through the owner and unknown text refused by a typed error carrying that text |
| `seed-host/admit_seed_host_composition` | The family's composition admission, comparing the selected member's declared holding-kind, holding-reason, quota, and eviction provisions with the requirements the composed hashing, encoding, transport, and storage declarations state and returning the family's `Incompatible` flavor where they disagree | `seed-host/lookup_seed_host_declaration`, `seed-host/admit_seed_host_selection`, `hashing/lookup_hashing_declaration`, `encoding/lookup_encoding_declaration`, `transport/lookup_transport_declaration`, `storage/lookup_storage_declaration` | the hashing, encoding, transport, and storage requirements compared before construction and before any custody or serving effect |
| `seed-host/factory` | The seed host family's factory function, which calls the composition admission before it constructs anything and then constructs the member the admitted selection names, with a branch per concrete, and the crate's public surface; carries the milestone's integration test across owned and delegated retrieval and its commit | `seed-host/owned`, `seed-host/rqbit`, `seed-host/admit_seed_host_composition` | The surface resolution and the First Finder consume |

#### Remaining milestones, as sprints

| Sprint by role | Contents | Depends on |
| --- | --- | --- |
| Domain model | The protocol's remaining types, each authored in the node of the file that first consumes it and housed in the module that implements it, fixed when that module's ticket is written, the identifier modules being harness tickets and the derivation context a sidecar-layer ticket; the `domain` crate holds only the types its own modules implement and no asset identity and no derivation context; for each encoded type the `domain` crate holds, its canonical description as the encoding family's module `encoding/<type>`, implementing the factory's contract for that type, one module per type, authored after the type and before the first ticket that encodes it; a type housed in a crate that depends on the encoding family carries its canonical description as a module of that crate, as `envelope/possession_statement` and `proof/mint_statement` do; `domain/handshake_key_commitment`, which the attempt context `chain/base` authors carries, is mapped in the harness table | the types the harness families own; `encoding/factory` |
| Payload cipher and sidecar layer | `cipher/aes_ctr` with the counter layout and extent rules, authoring `IPayloadCipherAdapter`, the counter-layout and addressable-extent declarations, and the family's mock as its producers and creating the `adapters/cipher` crate, the concrete preceded by its settings admission where it has a settable value, then the family's declaration lookup, selection admission, and composition admission, then `cipher/factory`; `workflows/sidecar/build`, per live set a capsule and wrapped piece-group key per group committed by that set's Bao root, each sidecar its own object, and `workflows/sidecar/validate`, the manifest, hash-card, and sidecar validation ordering; the `sample_deployment` generator for the site demonstration, which needs the cipher and so lives here rather than in the harness and carries the configured piece-group size default | domain model; `hashing/factory`; `workflows/sidecar/wrap`; `kem/factory` |
| Registry and entitlement contracts, the EVM suite | See [Registry and entitlement contracts, the EVM suite ownership](#registry-and-entitlement-contracts-the-evm-suite-ownership) below this table | the contract tickets mapped in the harness table, `contracts/evm/Registry` among them; `contracts/evm/DeliveryVerifier`; domain model |
| Chain and submission families | The chain family's remaining tickets, `chain/base` and `chain/factory` being mapped in the harness table ahead of `harness-crypto/verifier/evm`: `chain/quorum_view`, the family-owned aggregation of two of three configured nodes at a common reference, stale ignored, divergence and views older than `τ_soft` failing closed; `submission/self_funded`, the self-funded call path, authoring the interface for submitting a signed intent and its declaration and creating the `adapters/submission` crate, the member preceded by its settings admission where it has a settable value, and the family's declaration lookup, selection admission, and composition admission preceding `submission/factory`; `discovery/seeder_map`, the on-chain seeder map as a discovery source through the chain factory, a further concrete beneath the discovery factory, preceded by its settings admission where it has a settable value, then its increments to the family's declaration lookup, selection admission, composition admission, and factory branch | `chain/factory`; registry and entitlement contracts; `signature/factory`; `discovery/factory` |
| Adapter registry and factory, on chain | The `AdapterRegistry` governed by a single project-held key and the factory that constructor-injects the identity adapters, the signature adapter per declared signature-scheme identifier, the attestation verifier per declared attestation-source identifier, the entitlement token form per declared entitlement-form identifier, the delivery verifier per declared delivery-statement-version and verifier-form identifier, and the payment-asset adapter per declared payment-asset identifier, each immutable once bound, the registry contracts reaching the delivery verifier and the payment-asset adapter only through those bindings | registry and entitlement contracts |

#### Registry and entitlement contracts, the EVM suite ownership

- `contracts/evm/IdentityAccount`, which authors the signature verifier interface, `contracts/evm/Binding`, `contracts/evm/ParameterSets`, and `contracts/evm/Registry`, which authors the entitlement, identity adapter, and attestation verifier interfaces, are mapped in the harness table; the items below they author are theirs, and the rest, `Erc721Entitlement`, `EnvelopeKeys`, `Locks`, `IPaymentAsset` and its native concrete, `Escrow`, `Claims`, `VerifierKeys`, the npm P-256 verifier, and the publisher-authority and escrow-identity adapters, stays in this sprint.
- Under `contracts/evm`, the suite in its declared source language.
- Asset records keyed by the asset's root, in place of any per-name version index.
- Append-only alias records under a declared namespace and an opaque key that no Registry field interprets, each entry binding the key to one root and carrying the source's signed statement or its recorded absence, never rebound or removed.
- Payment locks carrying the payment-asset identifier beside the price, moving, releasing, and refunding payment only through `IPaymentAsset` as the adapter registry binds it per identifier, with the native-asset concrete as the MVP's.
- The claim verifier's key record as the signature family's scheme identifier and a commitment to the key, with the key in calldata and vouchers verified through the signature adapter the registry binds for the scheme.
- Deployments and hash-cards carrying the hashing family's commitment-scheme identifier and length-carrying or committed width fields.
- Parameter sets, envelope keys, identity elements, and envelopes held as suite-tagged bytes with a digest and interpreted only by the delivery verifier the adapter registry binds.
- Attestation verification through the `IAttestationVerifier` adapter bound per declared attestation-source identifier, with the npm P-256 verifier as the first concrete and the source-key table it reads.
- Later escrow deployments.
- Parameter-set liveness with the sidecar-coverage rule and sidecar addition.
- The envelope-key registry.
- `IEntitlement` with the ERC-721 concrete carrying interval state and the ownership override that disables standard transfers.
- Issuance and transfer calling the delivery verifier through its registry binding.
- Batched grant requests readable by holders and closed by grant or withdrawal.
- Escrow records per deployment.
- Identity binding storing the signature family's scheme identifier and a commitment to the handshake key.
- The identity account, a contract account under ERC-1271 on the EVM suite, whose signer set is its device registry keyed by key identifiers tagged with the signature family's scheme identifier, verified through the signature adapter the registry binds per declared scheme identifier, with the `isDeviceAdmitted` view.
- Claim-set state layout, batch and paginated views.
- Every identity-bound mutation taking the acting identity and a signed intent verified through the account against a signer whose permissions cover it.
- The contracts enforcing lock expiry and no volume limit.

### Local daemon and package serving, as epics

| Epic by role | Contents | Depends on |
| --- | --- | --- |
| Durable jobs and configuration | `telemetry/local_metrics` over the storage family, authoring the exporter interface, the declaration, and the family's mock as its producers and creating the `adapters/telemetry` crate, then `telemetry/tracing`, the family-owned tracing setup carrying per-request correlation identifiers, then `telemetry/opentelemetry`, the OpenTelemetry exporter concrete over the same interface, each concrete with a settable value preceded by its settings admission, then the family's declaration lookup, selection admission, and composition admission, then `telemetry/factory`, with a branch per concrete; `workflows/jobs`, the job engine over the storage family with checkpointing and idempotent restart; `workflows/config`, the versioned configuration registry over the storage and encoding families; `platform-paths/linux`, authoring the application-directory interface and creating the crate, `platform-paths/macos`, `platform-paths/windows`, each concrete with a settable value preceded by its settings admission, the family's lookup, selection, and composition admission, and `platform-paths/factory` for the store-root defaults; the `ipc` family's `ipc/framing` and `ipc/principals`, family-owned and creating the crate, `ipc/unix_socket` authoring the IPC transport interface, `ipc/named_pipe`, `ipc/server` with the control-principal and local-presence rules, each concrete with a settable value preceded by its settings admission, the family's lookup, selection, and composition admission, and `ipc/factory`; `lifecycle/lock`, the single-instance lock as a family-owned function creating the `adapters/lifecycle` crate, the family's concretes, each with a settable value preceded by its settings admission, and the family's lookup, selection, composition admission, and factory following in the installation coordinator | domain model; `storage/factory` |
| Identity and custody | `custody/holder_seed` with the control and envelope branches, family-owned and creating the `adapters/custody` crate; `custody/device_key`; `custody/local_keystore` over `keyring` with version-headed blobs, authoring `IKeyCustodyAdapter`, the versioned capability set including the family's declared custody-role identifiers, and the family's mock as its producers; `custody/upgrade`, the in-place migration between concretes verified against the chain, through the interface; each concrete with a settable value preceded by its settings admission, and the family's declaration lookup, selection admission, and composition admission preceding `custody/factory`; `wallet/eip1193`, authoring the signing-only wallet interface and creating the crate, `wallet/walletconnect`, each concrete with a settable value preceded by its settings admission, the family's lookup, selection, and composition admission, and `wallet/factory`; `identity/publisher_authority`, authoring `IIdentityAdapter` with the binding schema and creating the crate, `identity/escrow`, each concrete with a settable value preceded by its settings admission, the family's lookup, selection, and composition admission, and `identity/factory`; `workflows/identity` with identity creation, relayer-paid binding and contract account creation, envelope-key registration, device pairing, signer admission, and revocation | the signature tickets; the chain and submission families; the relayer or paymaster milestone, for `submission/factory`; `envelope/factory`; `kdf/factory`; durable jobs and configuration |
| Plaintext CAS and package host | `cas/filesystem`, verified atomic CAS of tarballs extracted per project by the package manager with nothing linking into the store, quota, pinning, eviction, its index through the storage family, authoring the store interface and creating the crate, the concrete preceded by its settings admission where it has a settable value, then the family's lookup, selection, and composition admission, then `cas/factory`; `package-host/npm`, the npm registry protocol served locally with canonical tarball URLs and no metadata store, authoring the npm name type, `IPackageHostAdapter` with the namespace identifier it declares, and the name-to-root alias lookup, which selects an alias entry only on the client's evidence, a signed statement checked against the lockfile `integrity` or the source, or an unsigned entry decrypted and compared, and fails closed on an alias without evidence naming the competing roots, the boundary PR-02 proves, and assembling packuments from the alias records, the decrypted tarballs' own `package.json`, and the entries' signed statements, the boundary PR-09 proves, and creating the crate, the concrete preceded by its settings admission where it has a settable value, then the family's lookup, selection, and composition admission, then `package-host/factory` | durable jobs and configuration; `storage/factory`; `hashing/factory`; `encoding/factory` |
| First Finder ingest | `ingest/npm` with attestation validation or recorded absence, writing the alias entry beside the root with the source's signed statement or its recorded absence and nothing the bytes contain, and availability as eligibility, authoring `IIngestSourceAdapter` with the same namespace identifier the package-host family declares and creating the crate, the concrete preceded by its settings admission where it has a settable value, then the family's lookup, selection, and composition admission, then `ingest/factory`; `workflows/first_finder` with the foreground serve, the background bootstrap job, the state-locked registration race, escrow custody of the master scalar, seeding, and the finder's grant of the asset's first entitlement to itself | plaintext CAS and package host; the swarm tickets; the registry contracts; the chain and submission families; the relayer or paymaster milestone, for `submission/factory`; identity and custody; `kem/factory`, `envelope/factory`, and `proof/factory`; `workflows/sidecar/build` |
| Credential delivery and per-attempt authorization | `payment/native`, authoring `IPaymentAssetAdapter`, the payment-asset identifier, the declaration of the assets a concrete settles, and the family's mock and creating the `adapters/payment` crate, the concrete preceded by its settings admission where it has a settable value, then the family's lookup, selection, and composition admission, then `payment/factory`; envelope-key registration on first run; `workflows/request`, the batched grant request job and its pickup from chain events; `workflows/prefetch` with pinning, seeding, and set matching; `workflows/acquire` with mint and grant delivery, holders fulfilling requests for absent requesters, and sale delivery, reaching payment only through the admitted member the family's surfaces and `payment/factory` supply; credential load and recovery; `workflows/grant`, the grant service under the asset scope, fulfilling open requests from a held credential after the credential engine and the request job; `workflows/attempt` with `τ_soft` and `τ_wallet`; `workflows/decrypt` with decapsulation through the admitted member `kem/factory` constructs, unwrap through `workflows/sidecar/unwrap`, and decryption through the admitted member `cipher/factory` constructs; `workflows/interval_end`; `workflows/gate` | the chain and submission families; the relayer or paymaster milestone, for `submission/factory`; identity and custody; `kem/factory`, `envelope/factory`, and `proof/factory`; payload cipher and sidecar layer; First Finder ingest |
| Resolution orchestrator | `workflows/resolve` with the hedged source order under the source deadline and grant wait its params carry: the CAS, ciphertext under a held credential through the attempt engine and the decryption pipeline, the seed host and the swarm, upstream through the ingest factory for any identity without a credential, and a bounded wait on a requested grant when upstream is down; per-asset independence status in `workflows/health`, each status a declared health-state identifier and each attempt's resolution outcome a declared resolution-reason identifier | plaintext CAS and package host; the swarm tickets; First Finder ingest; credential delivery and per-attempt authorization |
| Daemon composition | `workflows/compose`, capability resolution across every family's declaration lookup, selection admission, composition admission, and factory the daemon and the installer compose, each family's composition admission completing before any construction or side effect, failing closed; `workflows/settings`, the settings service and the catalogue of every module's settings; the daemon binary, the daemon's composition root; the installer binary | resolution orchestrator; identity and custody; installation coordinator; explicit publisher path; claim verifier and escrow claim |
| Demonstrable milestone | No new subsystem: a second identity on a second machine profile installs a pinned closure the First Finder ingested, at the registry's speed with requests registered and ciphertext prefetched, then with the registry unavailable, through the packaged daemon and package host over the swarm, on a grant fulfilled while it was offline; the north star, first-run independence, and install wall-clock measured on the dogfood population | daemon composition; the swarm tickets |

### Onboarding shells and services, as milestones

| Milestone by role | Contents | Depends on |
| --- | --- | --- |
| Installation coordinator | `workflows/install`, the durable install plan with platform detection, repair, update, uninstall, the health probe, the consent items, and the first-run cost disclosure; `lifecycle/systemd`, authoring the service-lifecycle interface, `lifecycle/launchd`, `lifecycle/windows_service`, `lifecycle/scheduled_task`, each concrete with a settable value preceded by its settings admission, the family's lookup, selection, and composition admission, each platform carried as a declared platform identifier, and `lifecycle/factory`; `artifact-verifier/sigstore`, authoring the signed-artifact verification interface and creating the crate, `artifact-verifier/authenticode`, `artifact-verifier/apple_notarization`, each concrete with a settable value preceded by its settings admission, the family's lookup, selection, and composition admission, and `artifact-verifier/factory`; `redirect/npm`, authoring the detect, redirect, back-up, and restore interface and creating the crate, the concrete preceded by its settings admission where it has a settable value, then the family's lookup, selection, and composition admission, then `redirect/factory` | durable jobs and configuration; identity and custody |
| Visual Studio Code extension and npm bootstrap | Thin shells over the coordinator | installation coordinator |
| Desktop application and CLI | Tauri and Rust control surfaces | installation coordinator; daemon composition, for the settings catalogue |
| Relayer or paymaster | The relayer binary with the chosen provider's bundler concrete, authoring the bundler-provider interface, preceded by its settings admission where it has a settable value, then the bundler family's declaration lookup, selection admission, and composition admission, then `relayer/bundler/factory`; free-path sponsorship under a global per-window budget and maximum liability with per-identity limits as one layer, each sponsorship outcome a declared sponsorship identifier; the sponsorship mechanism for the identity contract account chosen here under LC-13; `submission/paymaster` and `submission/relayer`, authored as the chosen mechanism requires and depending on no module of `apps/relayer`, since the relayer service depends on the submission family, the relayer service's request and response types authored in `submission/relayer`'s node as their first consumer; then each concrete with a settable value preceded by its settings admission, the family's lookup, selection, and composition admission, then `submission/factory`, with a branch per submission concrete; then the relayer service; sponsored device admission and revocation; refusal to sponsor a paid acquisition, stated against the payment-asset identifier the lock names; cost and budget reporting | the chain and submission families sprint; registry and entitlement contracts |
| Explicit publisher path | `workflows/publish` with the publisher-derived lineage, swarm-native publication of the dependency closure, and the issuance policy service; `publisher-proof/provenance`, authoring `IPublisherProofAdapter` and creating the crate, `publisher-proof/maintainer_oauth`, each concrete with a settable value preceded by its settings admission, the family's lookup, selection, and composition admission, each proof class a declared proof-class identifier, and `publisher-proof/factory` | First Finder ingest; identity and custody |
| Claim verifier and escrow claim | The claim verifier binary signing under a custody-held key of a declared signature-scheme identifier, revocable and registered on chain as the signature family's scheme identifier and a key commitment; `claim-verifier/attestor`, authoring `IClaimVerifierAdapter`, the voucher types, and a declaration naming the signature-scheme identifier its vouchers carry, and creating the crate, the concrete preceded by its settings admission where it has a settable value, then the family's lookup, selection, and composition admission, then `claim-verifier/factory`; `workflows/claim`; the claim set established from upstream metadata at verification; claim-set vouchers; the claimant parameter set; optional handover; sidecar addition; voluntary migration | registry contracts; explicit publisher path; credential delivery |
| Project seed host and site | Persistent first seeder of the core closure holding an entitlement and credential for every asset it seeds; static site; WebAssembly demonstration | `seed-host/factory`; explicit publisher path; payload cipher and sidecar layer, for the sample deployment |
| Observability | RO-03 metrics, RO-04 correlation, RO-05 health, RO-06 attempt latency and install wall-clock reported against the values in force | every component that emits |
| Demonstration harness | Controlled participants, wallets, chain state, failures, and the deliberately incompatible adapter declarations the incompatibility scenario rejects, refused as each family's `Incompatible` flavor through its composition admission before construction and by `workflows/compose` | every component under test |

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
| Factory admission | each family's concretes, through their declarations | the declaration lookup, selection admission, and composition admission of every downstream family, and `workflows/compose` | A concrete admitted only when its declaration satisfies every downstream requirement; refused before constructing anything and before any side effect | The harness, for the cryptographic families; the daemon composition, for the whole composition |
| Verifier parity | `proof/schnorr_fs` | `contracts/evm/DeliveryVerifier` through `harness-crypto/verifier/evm`, the member the verifier family's surfaces admit and its factory constructs, bound to the suite's ABI description | Rust to chain; bit-for-bit acceptance on every vector | Delivery proof and on-chain verifier |
| Precompile encoding | the pairing family's declared precompile-encoding identifiers, read through `pairing/lookup_pairing_declaration` and produced by every pairing concrete | `contracts/evm/PairingLib` | Rust encoding to precompile input | Delivery proof and on-chain verifier |
| Piece-group key | the decapsulate of the KEM member `kem/factory` constructs after its admissions, and `workflows/sidecar/unwrap` | the cipher family's surfaces and `cipher/factory` | The decapsulation-to-cipher seam preserved for a future multi-key suite | Payload cipher and sidecar layer |
| Sidecar root | the encapsulate of the KEM member `kem/factory` constructs after its admissions, and `workflows/sidecar/build` | commitments and hash-card, through the hashing family's surfaces and `hashing/factory` | Per live set, capsule and wrapped key per group committed by that set's Bao root; each sidecar its own object | Payload cipher and sidecar layer |
| Discovery engine | the transport factory's engine capability | `discovery/dht`, `discovery/pex`, `discovery/tracker` | Library-backed sources reach the engine through the generic surface, never through the transport concrete | Swarm transport and seed host |
| Store | the storage member the family's surfaces admit and `storage/factory` constructs | `workflows/jobs`, `workflows/config`, the CAS index, the ciphertext index, `telemetry/local_metrics` | Tables, transactions, and checkpoints with no store library named by a consumer | Swarm transport and seed host, for the ciphertext index; durable jobs and configuration, for the job engine, the configuration registry, and the local metrics; plaintext CAS and package host, for the CAS index |
| Envelope in settlement | the wrap of the admitted envelope member and the provers of the admitted proof member | registry and entitlement contracts, through the submission family's surfaces and `submission/factory` | Calldata and event; digest stored | Relayer or paymaster |
| Signed intent | the chain forms family's signed-intent form, signed through the signature member `signature/factory` constructs after its admissions | registry and entitlement contracts, through the submission family's surfaces and `submission/factory` | A signed intent to the account's verification of the signer's signature-scheme identifier, EIP-712 typed data and ERC-1271 on the EVM suite | Relayer or paymaster |
| Attestation verification on chain | the source's attestation, carried by `ingest/npm` | `IAttestationVerifier` bound per declared attestation-source identifier by the adapter registry | The registration refused unless the source's verifier accepts, absence admitted and recorded | First Finder ingest |
| Attempt context | the attempt context `chain/base` authors in the chain family's factory module | `chain/quorum_view` over `chain/base`, and credential delivery | View call at the declared settlement-tier identifier from a quorum of configured nodes | Chain and submission families, for the quorum view; credential delivery and per-attempt authorization, for the attempt |
| Resolution order | resolution orchestrator | the CAS, seed-host, transport, and ingest families' surfaces and factories | Local, encrypted, swarm, upstream, hedged | Resolution orchestrator |
| Package-manager protocol | `package-host/npm` | npm | HTTP registry protocol; resolution stays in npm; canonical tarball URLs so lockfiles stay portable | Plaintext CAS and package host |
| Persistent credential | the envelope and custody families' surfaces and factories | credential delivery | Stored only under `IKeyCustodyAdapter` | Credential delivery and per-attempt authorization |
| Device admission | identity manager on a root | identity contract account; the attempt engine's wallet-control assertion | Signer admitted and revoked on chain; admission read in the same state view as authorization | Identity and custody, for the identity contract account; credential delivery and per-attempt authorization, for the wallet-control assertion |
| Grant request to holder | request job, through the submission family's surfaces and `submission/factory` | registry request record; any holder's grant service | Batched sponsored request; holder reads open requests and fulfils them for absent requesters | Credential delivery and per-attempt authorization |
| Grant pickup from events | registry mint and grant events, through the chain member the family's surfaces admit and `chain/factory` constructs | request job and credential engine | Envelope recovered from chain history on the next run; set matched against held deployments | Credential delivery and per-attempt authorization |
| Installer to daemon | installation coordinator | daemon, package host, seed host, through the lifecycle and IPC families' surfaces and factories | Service lifecycle and IPC | Installation coordinator |
| Artifact authentication | the release's signatures | the artifact-verifier family's surfaces and `artifact-verifier/factory` | Every artifact and update verified before execution, under each declared artifact-scheme identifier's concrete | Installation coordinator |
| Shells to coordinator | extension, npm bootstrap, desktop, CLI | installation coordinator | One coordinator, identical postcondition | Extension and npm bootstrap; desktop and CLI |
| Relayer to contracts | relayer, through the bundler family's surfaces and `relayer/bundler/factory` | identity binding, requests, mint | Sponsored transactions under policy | Relayer or paymaster |
| Verifier voucher to contract | `claim-verifier/attestor` | escrow claim contract surface, through the signature adapter the adapter registry binds for the key's declared signature-scheme identifier | Voucher bound to claimant, set, contract, chain, nonce, expiry, verified against the registered scheme identifier and key commitment | Claim verifier and escrow claim |
| Payment settlement | the payment member the family's surfaces admit and `payment/factory` constructs, reached by `workflows/acquire` | the lock and settlement contracts, through the payment-asset adapter the adapter registry binds for the lock's declared payment-asset identifier | Lock, release, and refund in the named payment asset; a lock naming an unbound payment-asset identifier refused | Registry and entitlement contracts, for the lock and the bound adapter; credential delivery and per-attempt authorization, for acquisition through the factory |
| Alias lookup | the registry's append-only alias records, written by `ingest/npm` with the source's signed statement or its recorded absence | `package-host/npm` | A request's name to the asset's root, the entry selected only on the client's evidence, a signed statement checked against the lockfile `integrity` or the source or an unsigned entry decrypted and compared, the decrypted bytes verified against the selected digest before commit, and an alias without evidence failing closed | Plaintext CAS and package host, for the lookup and packument assembly; registry and entitlement contracts, for the alias records and the root-keyed asset record |
| Site demonstration | WebAssembly build of the client crates | browser | Same crates as the client; no service on the read path | Project seed host and site |

## Conflict Flags

- **Grouping membership versus dependency order.** The signature and swarm tickets sit in the protocol core grouping and depend on no contract sprint, so they run in parallel with the harness and with the contract tickets.
- **Every factory follows its concretes.** A family's generic interface, declaration, and mock are authored in its first ticket, so the declarations the composition admission reads exist from the family's first ticket; each settings admission precedes the concrete constructed from it, and the family's declaration lookup, selection admission, and composition admission follow the family's last concrete; the factory function consumes the concretes it constructs and is authored once after them; and no consumer ticket depends on a concrete. The chain forms and the chain adapters are two families, each with its own interface, selection, admission functions, and factory: `chain-forms/factory` is authored in the harness after the forms concrete and its family's lookup, selection admission, and composition admission, and `chain/factory` is authored once after the Base concrete and its family's lookup, selection admission, and composition admission, both ahead of `harness-crypto/verifier/evm`, the chain factory's first consumer.
- **Each family's factory follows its last concrete and its admission functions.** `discovery/factory` follows `discovery/aggregate`, the swarm tickets' discovery concretes, and the family's lookup, selection, and composition admission, so those concretes have a consumer when they are authored, and `discovery/seeder_map` is a further concrete beneath it, preceded by its settings admission where it has a settable value, with the family's declaration lookup, selection admission, and composition admission increments preceding its factory branch; `telemetry/factory` follows `telemetry/local_metrics`, `telemetry/opentelemetry`, and the family's admission functions in the durable jobs epic, the OpenTelemetry concrete needing only the exporter interface; `submission/factory` follows every submission concrete and the family's admission functions in the relayer or paymaster milestone, which precedes every daemon epic that submits a signed intent; the lifecycle family's lock precedes its concretes, each with a settable value preceded by its settings admission, the family's functions, and its factory, which arrive together at the installation coordinator.
- **The verifier client is the harness's own family.** The daemon never submits a proof to the verifier, since the contracts call it inside mint, grant, and delivery, so the verifier client is the harness's own `verifier` family with an `evm` concrete, and no chain concrete carries one.
- **The harness's code order and its runtime sequence differ.** The delivery verifier reads the generated directory a run of `harness-crypto/main` writes, and the run's verifier stage reaches the deployed delivery verifier; neither is a code dependency.
  `harness-crypto/verifier/evm` authors and binds the delivery verifier's language-neutral ABI description; `contracts/evm/DeliveryVerifier` realizes that interface in Solidity, and `contracts/evm/ParameterSets`, the suite's first source file, creates the Foundry configuration and the Solidity continuous-integration definition; the Rust verifier client authors no contract source and no toolchain file, and speaks the EVM ABI, which is why it keeps its name while the generator is named for the language it emits.
  The contracts need the generated directory to be built and proven, and the verifier needs the deployed contract to run. The table and the graph hold the code order; the [Runtime sequence](#runtime-sequence) section holds the runtime sequence.
- **The chain forms family precedes the chain family.** The proof family's transcript descriptions, challenge, concrete, and factory are generic over the identity, entitlement, interval, and chain-identifier forms, and the generator, verifier client, and measurement need forms the deployed verifier reproduces; none may name a concrete, so the forms are their own family: `chain-forms/evm`, the family's first concrete, supplies the EVM suite's forms, the family's declaration lookup, selection admission, and composition admission follow it, the selection admission admitting the configured forms identifier the suite requires, and `chain-forms/factory` follows them, handing the constructed concrete to a consumer generic over the form interface, and every one of those consumers follows the factory. The forms are the suite's, not the network's: the EVM suite is one suite shared by every EVM chain concrete, Ethereum mainnet being a later adapter of the same suite. The chain family consumes the forms family one way: `chain/base/settings`, the chain family's first ticket, creates the chain crate and authors the chain declaration, which states the forms identifier a member binds in the forms family's identifier; `chain/base`, the first consumer of the settlement and entitlement-state operations and of the registry's and the entitlement interface's ABI descriptions, authors those ahead of `harness-crypto/verifier/evm`, binds the forms the forms factory resolves, authors no form, address derivation and the signed-intent form being `chain-forms/evm`'s and shared by every EVM chain concrete; the chain family's declaration lookup, selection admission, and composition admission follow it, each authored once, and `chain/factory` follows them, admitting a chain concrete through the composition admission only when its declaration lists the forms identifier the resolved forms carry.
- **The harness consumes the chain family.** The harness's verifier client, configuration, and run take the chain's network values, the endpoint, the chain identifier, the entitlement contract, and the precompile encodings, from the chain adapter, reached through the chain family's admission surfaces and `chain/factory`, the member's declaration read through `chain/lookup_chain_declaration`, and the verifier's cost carries the chain family's description of a transaction's estimated fee, no transaction being settled, so `chain/factory` and everything it needs in order to be written precede `harness-crypto/verifier/evm`: `domain/handshake_key_commitment`, `chain/base/settings`, `chain/base`, and the family's declaration lookup, selection admission, and composition admission.
  Nothing on that path needs a harness ticket at or after `harness-crypto/verifier/evm`, and no contract ticket and none of the signature, transport, and discovery families is on it. A value only one chain member's model has, the gas price oracle among them, stays in that member's declaration and settings and reaches no family surface.
- **The contracts that verify signed intents consume generated constants at runtime.** `contracts/evm/IdentityAccount`, `contracts/evm/Binding`, and `contracts/evm/Registry` verify or assemble signed intents whose key-identifier tag, intent domain, intent struct and member names, and type names are the chain forms family's declared values and the encoding's type names; their ticket rows hold the authoring dependencies, including `harness-crypto/generate/solidity/constants`. The suite reads the committed constants library after `harness-crypto/main` writes it, a runtime input that orders no ticket; the same boundary applies to `contracts/evm/PairingLib` and `contracts/evm/DeliveryVerifier`.
  `chain/base` binds the registry's and the entitlement interface's ABI descriptions, the suite's boundary with the client, and authors them, so no Solidity source is on the path to `chain/factory`; the Solidity realizations of those interfaces are `contracts/evm/Registry`'s, proven against the descriptions.
- **Incompatible combinations are refused before construction.** Each member's declaration states what it requires of the adapters it is composed with, so the compatibility of every pairing of chain, verifier, generator, contract suite, and curve is determinate from declarations alone: the chain member against the resolved forms' identifier, admitted by `chain/admit_chain_composition`; the verifier member against the resolved chain forms' identifier, the selected chain member, the borrowed encoder's encoding identifier, the selected pairing's declared verifier form, and the contract suite identity the selected generate member declares it emits for, admitted by `harness-crypto/verifier/admit_verifier_composition`; and the generate member against the resolved chain forms' identifier and the borrowed encoder's encoding identifier, admitted by `harness-crypto/generate/admit_generate_composition`. Each factory calls its family's composition admission before constructing anything and before any side effect, so a mismatched chain, curve, encoder, verifier, or suite is refused before effort.
- **The harness composes through configuration.** Nothing in the harness names a path, a curve, a concrete, a form, or a version. `harness-crypto/config` carries every such value as a shipped default, overridable from a configuration file and from command-line flags, and `harness-crypto/run` and its consumers pass each as a typed identifier through the family's selection admission and the factory it belongs to; the generator writes under the destination its params carry, an app module in the adapter role reached only through its family's surfaces and its factory, and the statements' context values arrive as payload, the configured ones from the document and the chain identifier and the entitlement contract read from the constructed chain by `harness-crypto/run/chain_consumer` and handed to `harness-crypto/run/admit_contexts`.
- **The generate concrete is an adapter over functions it owns.** Rendering, the constants, each kind of vector, and the admission of the concrete's own settings are functions beneath `harness-crypto/generate/solidity`, each its own ticket ahead of the adapter, as `proof/schnorr_fs/challenge` is beneath `proof/schnorr_fs`, because one adapter file holding all of it exceeds the size at which a file is decomposed. `harness-crypto/generate/solidity/render` owns the entry type the constants function returns, the record-declaration type, and the validated name types, owns no table from canonical field kinds to type names, which the encoding family's ABI concrete declares, and takes each record member's type name as data resolved by the caller; each vector function returns typed records with their canonical descriptions and member names; `harness-crypto/generate/solidity/settings` owns the concrete's settings, their document and its guard, and the settings admission, and authors the generate family's declaration type and the concrete's declaration constant, which the adapter consumes; the adapter alone encodes records into lines, renders the libraries, and writes files.
- **The configuration is a composing function over functions it owns.** Layering the sources, admitting the family selections, admitting the statements and the deployment records, admitting the generate settings, and admitting the output destinations are functions beneath `harness-crypto/config`, each its own ticket ahead of it, because one file holding all of it exceeds the size at which a file is decomposed. `harness-crypto/config/layers` alone parses the command line, reads files, and names `clap`, `toml`, and `serde`, and owns the document every admission function reads; each admission function owns the admitted types it returns and maps the document's text values to the types their owning families declare; `harness-crypto/config` composes them, owns the typed configuration, and carries the shipped defaults. The selection types the configuration holds and the run hands each cell and each benchmarked concrete a copy of carry `Clone`, `Copy`, `Debug`, `PartialEq`, and `Eq`, added in their families' factory interfaces by `harness-crypto/config/admit_selections`, each the first file that requires them.
- **Vectors are committed files, one encoded record per line, ABI-encoded and language-neutral.** A further contract language is a further generator concrete over the same vector files. Solidity holds no constant array, and a counted kind must be iterable by a Foundry test whatever count the configuration carries, so each vector kind is a file the adapter writes, each line the encoding family's encoding of one record as `0x`-prefixed hex; a test reads it with `vm.readLine`, converts each line with `vm.parseBytes`, and decodes it with the kind's function from the generated vector-record library, which also carries the file's name. One record per line needs no list kind in the encoding contract. The decode function decodes over the members' types, never over the struct, since a struct with a dynamic member encodes behind an offset and each line holds the parameter encoding of the fields.
- **Vector kinds, counts, and names are declared and configured.** The generate concrete's declaration names the kinds it emits and each kind's sizing: a drawn kind holds the count the configuration carries for it, an enumerated kind holds exactly what it enumerates, and a kind over configured inputs holds one record per input; every file, library, struct, function, and constant name is the configuration's, with shipped defaults; the generate family's composition admission admits a configuration only when its per-kind settings name exactly the declared kinds, with a count for each drawn kind and none for any other. A further target's generate concrete declares its own kinds, and no consumer holds a count, a kind list, or a name.
- **The harness run alone calls the factories.** `harness-crypto/run` and the consumers it owns nest each factory's consumer inside the previous one and hand the adapters and their declarations to the generate and verifier factories, which construct their concretes over them, and to the measurement module; no other harness module calls a factory, and each reads a declared identifier — a domain tag, a form, or a version — from what it is handed.
- **The run is a composing function over consumers it owns.** A family's factory hands the concrete it constructs to a consumer rather than returning it, because the concrete's type carries the borrowed adapters' types, so composing the families is a chain of consumers, each implementing one factory's consumer trait and calling the next factory with the admitted selection and the next consumer: pairing, then hash-to-scalar and encoding, chain forms, chain, KEM, key agreement, delivery proof, and generate. One type per file admits one consumer per file, so each consumer is its own ticket beneath `harness-crypto/run`, authored innermost first since each outer consumer constructs the one inside it, as `harness-crypto/benchmark` is the pairing family's consumer in its own file. `harness-crypto/run` owns the randomness source's construction, the benchmark's calls, each cell's call to the pairing factory, and the outcome every call returns; the verifier, measurement, and report tickets precede the configuration and every consumer, so each consumer is authored with every value it carries and `harness-crypto/run/delivery_proof_consumer` with every call it makes.
- **The harness's output is configured, as its inputs are.** The run's outcome, a success with the benchmark's measurements and every cell's outcome — the files written where the cell ran to completion, its refusal where it refused — or, when any cell is refused, the error arm carrying every pair's and every cell's outcome once in iteration order, is written by `harness-crypto/output` to each destination the configuration names, standard output, a file at a configured path, or both, so a caller chooses where a run reports by the same shipped default, file, and flag that set every other value; the shipped default is standard output. A configuration refusal arrives before any destination is admitted, so the binary writes it to standard error, and every refusal ends the process with a failure status.
- **The benchmark's concretes are configured.** The benchmark measures the selected pairing concretes, the family's declared set by default, and sets no shipped default; the run hands each pair — a pairing member identifier and the target-group encoding identifier — admitted through `pairing/admit_pairing_selection` to the pairing factory with a `PairingBenchmark` as its consumer, and the outcome carries each concrete's measurements to the output.
- **The challenge vectors are standalone transcripts.** The proof family's challenge function is private to its crate, and a proof in the second-group form carries no first messages, so `harness-crypto/generate/solidity/challenge_vectors` builds its own transcripts, encodes each through its public description, and maps the encoding under the challenge tag the declaration carries; the proofs' own challenges travel in the proof vectors.
- **An identity encodes as `bytes20`.** The encoding concrete encodes the fixed twenty-byte kind as a right-padded `bytes20`, so the constants render it as `bytes20` and `contracts/evm/DeliveryVerifier` encodes every identity field of a transcript as `bytes20`; an `address` is left-padded and yields another challenge.
- **A generated library's compiler version and license identifier are configured.** A Solidity source file opens with a license identifier and a compiler version pragma, neither of which is a library name or an entry, so the shipped defaults carry each in the selected generate member's own table, admitted into the member's settings inside the settings-carrying selection `harness-crypto/run/delivery_proof_consumer` hands the generate factory, the generate concrete holds them from construction, and `harness-crypto/generate/solidity/render` takes them as params and holds neither as a literal.
- **The cell directory is read, not chosen.** The adapter names each cell directory from the canonical spellings the pairing's and the encoder's declared identifiers render and selects no curve.
- **The harness's parameter-set digest and identity bytes are configured.** No harness ticket computes a parameter-set digest or describes an entitlement identifier, and the delivery verifier takes the identity element as an input, so both arrive as configured values; the registry and entitlement contracts sprint binds them to the record. The entitlement contract a harness transcript names is the address the constructed chain adapter reports through the chain family's consumer surface, whether or not a suite is deployed there when the generator runs, since the harness exercises the delivery verifier apart from the registry; in operation the registry and entitlement contracts supply every statement field from their own records, the stored envelope digest, the registered keys, the identity element, and the interval counter among them, and refuse a field the record contradicts, which LC-05 and AS-13 prove at the registry and entitlement contracts milestone and again through the daemon at credential delivery.
- **The generated directory is committed.** The Solidity continuous-integration definition runs only `forge build`, `forge fmt --check`, and `forge test`, so the vector files in each cell directory, and the constants library and the vector-record library in the generated directory outside every cell directory, `contracts/evm/PairingLib` and `contracts/evm/DeliveryVerifier` read exist in the repository when it runs; `contracts/evm/foundry.toml` grants `fs_permissions` read access to the generated directory and the ABI description directory and write access to the deployments directory, and to nothing else, and the configuration's shipped default for the generated directory is the directory it grants; `ffi` stays disabled, and `via_ir = true` compiles every source through the IR pipeline, since a proof record's decode function destructures more values than the legacy pipeline's sixteen reachable stack slots hold; the first run of the harness binary from its shipped defaults writes them through the generator, `harness-crypto/run/delivery_proof_consumer` completing the generate and verifier composition admissions and calling the generate factory before the verifier factory, and returns the verifier-stage refusals, no deployment record being configured; `contracts/evm/deploy` carries them in its commit with the deployment records it emits.
- **The wrap precedes the cipher.** The credential KEM milestone's cross-set agreement (CR-08, CR-11) is proven by `workflows/sidecar/unwrap`'s integration test, so `workflows/sidecar/wrap` and `workflows/sidecar/unwrap` are harness tickets and create the workflows crate; `workflows/sidecar/build` and `workflows/sidecar/validate` wait for the cipher in the protocol core.
- **The benchmark follows the pairing concretes.** It constructs each through the pairing family's surfaces and `pairing/factory` and names no library; it records each concrete's measurements, and every concrete remains beneath the factory whichever one the configuration names.
- **The benchmark sets configuration, not code.** The factory constructs the concrete the admitted selection names once the composition admission has passed; the benchmark measures the selected pairing concretes, the declared set by default, and sets no shipped default.
- **The library-backed discovery sources share the transport's engine.** They reach it through the engine capability the transport factory's generic surface exposes, so no discovery concrete depends on `transport/rqbit`.
- **The demonstrable milestone needs a grant.** Resolving against the swarm with the registry down requires a second identity to hold a credential, so the milestone sits after credential delivery and per-attempt authorization, not at the package host.
- **The resolution orchestrator follows its sources.** `workflows/resolve` reaches the CAS, the seed host, and the swarm through their families' surfaces and factories, upstream through the ingest family's surfaces and `ingest/factory`, ciphertext under a held credential through the attempt engine and the decryption pipeline, and a requested grant through the request job, so it follows First Finder ingest and credential delivery; the CAS and the package host are adapters beneath their factories and name no workflow, so they precede it, and `workflows/health` reports per-asset independence beside it.
- **The grant service follows its producers.** `workflows/grant` fulfils open requests from a held credential, so it follows the credential engine and the request job in credential delivery; the finder's grant of an asset's first entitlement to itself is `workflows/first_finder`'s.
- **The daemon's composition root follows everything it composes.** `workflows/compose` resolves every family's declaration lookup, selection admission, composition admission, and factory the daemon and the installer compose, `workflows/settings` carries every module's settings, and the daemon and installer binaries compose every module, so each follows the last of them, the installation coordinator's families and workflow, the explicit publisher path's, and the claim verifier's client among them, as `harness-crypto/config`, `harness-crypto/run`, and `harness-crypto/main` follow everything they carry, compose, and call. Every module takes its settings as params from the composition root; the control surfaces that read the catalogue follow it; the demonstrable milestone, which runs the packaged daemon, follows the daemon binary.
- **Escrow claim is last by dependency.** The escrow record carries no maintainer commitment and the verifier establishes the claim set at verification time, so the milestone waits only on the publisher path, the registry contracts, and credential delivery.
- **Two verifier forms, one primary.** `contracts/evm/PairingLib` exists in both forms so the harness measures both; BLS12-381 ships as the primary form on Base and BN254 is retained.
- **The delivery verifier is built in the harness and consumed by the contract sprint.** It is the first shipped contract, written to production standard, with its constants and vectors generated from the Rust reference by `harness-crypto/generate/solidity`.
- **The EVM suite is one concrete.** `contracts/evm` is the on-chain half of `chain/base`; a further declared chain-forms member provides a further suite beside it, and each chain member's declaration binds the forms identifier it requires; within the suite the adapter registry binds the entitlement token form per declared entitlement-form identifier, the attestation verifier per declared attestation-source identifier, the identity adapters, the signature adapter per declared signature-scheme identifier, the delivery verifier per declared delivery-statement-version and verifier-form identifier, and the payment-asset adapter per declared payment-asset identifier.
- **The payment asset is a seam on both sides of the chain.** The client reaches payment only through the `payment` family's surfaces and `payment/factory`, authored in credential delivery after `payment/native` and its admission functions, and the contracts only through the payment-asset adapter the registry binds, so a further payment asset is a further concrete and a further bound adapter and changes no lock, settlement, or acquisition code; the chain's native asset is the MVP realization beneath both.
- **The root type has one owner.** The hashing family owns the root type, the asset's identity, and every hashing operation is over bytes; the family carries no digest operation and no identity-hash identifier and depends on no encoding. A name is a lookup, never an identity: the package host's alias lookup is the one place a request becomes a root, and `workflows/sidecar/derivation_context` houses the derivation context, the `domain` crate holding no asset identity.
- **Identity and custody sits in the daemon grouping but the installer needs it.** The dependency runs the right way, coordinator depends on custody, and the epic is placed early in the grouping.
- **The identity is a contract account; its sponsorship mechanism is chosen at the relayer milestone.** The contract sprint builds the account with its device registry and takes every identity-bound mutation as a signed intent under LC-13, so `submission/paymaster`, `submission/relayer`, and `submission/self_funded` all remain open behind one factory.
- **Mint and transfer each have one owner, and every admission happens once.** Mint and transfer are different actions over different inputs: a mint starts from the master scalar, a transfer from an existing holder's secrets. `harness-crypto/delivery/mint` owns the mint delivery and `harness-crypto/delivery/transfer` the transfer delivery, each returning its statement, proof, credential state, recipient holder, uniforms, and the witnesses its prover took, so every node that proves, encodes, or times a delivery consumes them and composes no delivery step: `harness-crypto/generate/solidity/proof_vectors`, `harness-crypto/generate/solidity/challenge_vectors`, `harness-crypto/generate/solidity/mapping_vectors`, and `harness-crypto/measure`. A transfer's recipient holder is a valid holder for the next transfer. Forms admission happens once, in `harness-crypto/run/admit_contexts`, where `harness-crypto/run/chain_forms_consumer` resolves the forms; identity admission happens once, in `harness-crypto/config/admit_statements`, under the scope `harness-crypto/config/admit_selections` admitted, and `harness-crypto/run/kem_consumer` pairs each admitted identity with its setup scope as an `AdmittedKemIdentity`; every consumer takes typed contexts and admitted identities, rebuilds no form from a canonical context, and admits nothing. The witnesses each delivery returns are copies, since the provers consume what they are given and a `Secret` is not cloned. Measure takes its timing inputs from the delivery functions. `CanonicalDeliveryContext` and `DeliveryContextField` stay declared in `harness-crypto/generate/solidity/challenge_vectors` until `harness-crypto/config/admit_statements` and `harness-crypto/run/admit_contexts`, their first consumers, move them, so every step builds.

## Dependencies

The dependency graph across the groupings. Edges point from producer to consumer. Identifiers are descriptive; nothing is numbered.

```mermaid
flowchart TB
    subgraph foundation["Foundation"]
        direction TB
        f_ws["workspace/cargo"]
        f_sec["domain/secret"]
        f_rand_os["random/os"]
        f_rand_look["random/lookup_random_declaration"]
        f_rand_sel["random/admit_random_selection"]
        f_rand_comp["random/admit_random_composition"]
        f_rand["random/factory"]
        f_ci["workspace/ci"]
        f_ws --> f_sec
        f_ws --> f_rand_os
        f_rand_os --> f_rand_look
        f_rand_os --> f_rand_sel
        f_rand_look --> f_rand_sel
        f_rand_look --> f_rand_comp
        f_rand_sel --> f_rand_comp
        f_rand_os --> f_rand
        f_rand_comp --> f_rand
        f_ws --> f_ci
    end

    subgraph harness["Cryptographic validation harness"]
        direction TB
        pair_bn_ark["pairing/bn254_arkworks"]
        pair_bn_h2c["pairing/bn254_halo2curves"]
        pair_bls_ark["pairing/bls12_381_arkworks"]
        pair_bls_h2c["pairing/bls12_381_halo2curves"]
        pair_look["pairing/lookup_pairing_declaration"]
        pair_sel["pairing/admit_pairing_selection"]
        pair_comp["pairing/admit_pairing_composition"]
        pair["pairing/factory"]
        bench["harness-crypto/benchmark"]
        dom_dep["domain/deployment_identity"]
        dom_suite["domain/suite_identifier"]
        dom_pset["domain/parameter_set_identifier"]
        dom_group["domain/group_index"]
        dom_geom["domain/piece_geometry"]
        hashing_b3["hashing/blake3_bao"]
        hash_look["hashing/lookup_hashing_declaration"]
        hash_sel["hashing/admit_hashing_selection"]
        hash_comp["hashing/admit_hashing_composition"]
        hashing["hashing/factory"]
        sc_ctx["workflows/sidecar/derivation_context"]
        f_reader["encoding/canonical_field_reader"]
        f_abi["encoding/abi"]
        enc_look["encoding/lookup_encoding_declaration"]
        enc_sel["encoding/admit_encoding_selection"]
        enc_comp["encoding/admit_encoding_composition"]
        f_enc["encoding/factory"]
        kdf_b3["kdf/blake3_keyed"]
        kdf_look["kdf/lookup_kdf_declaration"]
        kdf_sel["kdf/admit_kdf_selection"]
        kdf_comp["kdf/admit_kdf_composition"]
        kdf["kdf/factory"]
        h2s_k["hash-to-scalar/keccak256"]
        h2s_look["hash-to-scalar/lookup_hash_to_scalar_declaration"]
        h2s_sel["hash-to-scalar/admit_hash_to_scalar_selection"]
        h2s_comp["hash-to-scalar/admit_hash_to_scalar_composition"]
        h2s["hash-to-scalar/factory"]
        kem_bb1["kem/bb1_depth_one"]
        kem_look["kem/lookup_kem_declaration"]
        kem_sel["kem/admit_kem_selection"]
        kem_comp["kem/admit_kem_composition"]
        kem["kem/factory"]
        env_ps["envelope/possession_statement"]
        env_eg["envelope/pairing_elgamal"]
        env_look["envelope/lookup_envelope_declaration"]
        env_sel["envelope/admit_envelope_selection"]
        env_comp["envelope/admit_envelope_composition"]
        env["envelope/factory"]
        proof_ms["proof/mint_statement"]
        proof_ts["proof/transfer_statement"]
        proof_ch["proof/schnorr_fs/challenge"]
        proof_sfs["proof/schnorr_fs"]
        proof_look["proof/lookup_proof_declaration"]
        proof_sel["proof/admit_proof_selection"]
        proof_comp["proof/admit_proof_composition"]
        proof["proof/factory"]
        sc_wrap["workflows/sidecar/wrap"]
        sc_unwrap["workflows/sidecar/unwrap"]
        h_del_mint["harness-crypto/delivery/mint"]
        h_del_transfer["harness-crypto/delivery/transfer"]
        sol_pair["contracts/evm/PairingLib"]
        gen_render["harness-crypto/generate/solidity/render"]
        gen_const["harness-crypto/generate/solidity/constants"]
        gen_group["harness-crypto/generate/solidity/group_vectors"]
        gen_reject["harness-crypto/generate/solidity/rejection_vectors"]
        gen_map["harness-crypto/generate/solidity/mapping_vectors"]
        gen_poss["harness-crypto/generate/solidity/possession_vectors"]
        gen_chal["harness-crypto/generate/solidity/challenge_vectors"]
        gen_proof["harness-crypto/generate/solidity/proof_vectors"]
        gen_settings["harness-crypto/generate/solidity/settings"]
        gen_evm["harness-crypto/generate/solidity"]
        gen_look["harness-crypto/generate/lookup_generate_declaration"]
        gen_sel["harness-crypto/generate/admit_generate_selection"]
        gen_comp["harness-crypto/generate/admit_generate_composition"]
        gen["harness-crypto/generate/factory"]
        sol_verify["contracts/evm/DeliveryVerifier"]
        sol_deploy["contracts/evm/deploy"]
        hv_evm["harness-crypto/verifier/evm"]
        hv_look["harness-crypto/verifier/lookup_verifier_declaration"]
        hv_sel["harness-crypto/verifier/admit_verifier_selection"]
        hv_comp["harness-crypto/verifier/admit_verifier_composition"]
        hv["harness-crypto/verifier/factory"]
        h_measure["harness-crypto/measure"]
        h_report["harness-crypto/report"]
        chain_forms["chain-forms/evm"]
        cf_look["chain-forms/lookup_chain_forms_declaration"]
        cf_sel["chain-forms/admit_chain_forms_selection"]
        cf_comp["chain-forms/admit_chain_forms_composition"]
        chain_forms_fn["chain-forms/factory"]
        sol_acct["contracts/evm/IdentityAccount"]
        sol_bind["contracts/evm/Binding"]
        sol_pset["contracts/evm/ParameterSets"]
        sol_reg["contracts/evm/Registry"]
        dom_hkc["domain/handshake_key_commitment"]
        chain_base_set["chain/base/settings"]
        chain_base["chain/base"]
        chain_look["chain/lookup_chain_declaration"]
        chain_sel["chain/admit_chain_selection"]
        chain_comp["chain/admit_chain_composition"]
        chain_fac["chain/factory"]
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
        run_chain["harness-crypto/run/chain_consumer"]
        run_forms["harness-crypto/run/chain_forms_consumer"]
        run_enc["harness-crypto/run/encoding_consumer"]
        run_pair["harness-crypto/run/pairing_consumer"]
        h_run["harness-crypto/run"]
        h_out["harness-crypto/output"]
        h_main["harness-crypto/main"]

        pair_bn_ark --> pair_bn_h2c
        pair_bn_ark --> pair_bls_ark
        pair_bn_ark --> pair_bls_h2c
        pair_bn_ark --> pair_look
        pair_bn_h2c --> pair_look
        pair_bls_ark --> pair_look
        pair_bls_h2c --> pair_look
        pair_bn_ark --> pair_sel
        pair_bn_h2c --> pair_sel
        pair_bls_ark --> pair_sel
        pair_bls_h2c --> pair_sel
        pair_look --> pair_sel
        pair_look --> pair_comp
        pair_sel --> pair_comp
        pair_bn_ark --> pair
        pair_bn_h2c --> pair
        pair_bls_ark --> pair
        pair_bls_h2c --> pair
        pair_comp --> pair
        pair --> bench
        hashing_b3 --> hash_look
        hashing_b3 --> hash_sel
        hash_look --> hash_sel
        hash_look --> hash_comp
        hash_sel --> hash_comp
        hashing_b3 --> hashing
        hash_comp --> hashing
        f_reader --> f_abi
        hashing --> f_abi
        f_abi --> enc_look
        f_abi --> enc_sel
        enc_look --> enc_sel
        enc_look --> enc_comp
        enc_sel --> enc_comp
        f_abi --> f_enc
        enc_comp --> f_enc
        hashing --> sc_ctx
        dom_dep --> sc_ctx
        dom_suite --> sc_ctx
        dom_pset --> sc_ctx
        dom_group --> sc_ctx
        dom_geom --> sc_ctx
        f_enc --> sc_ctx
        sc_ctx --> sc_wrap
        f_enc --> sc_wrap
        hashing --> kem_bb1
        kdf_b3 --> kdf_look
        kdf_b3 --> kdf_sel
        kdf_look --> kdf_sel
        kdf_look --> kdf_comp
        kdf_sel --> kdf_comp
        kdf_b3 --> kdf
        kdf_comp --> kdf
        pair --> h2s_k
        h2s_k --> h2s_look
        h2s_k --> h2s_sel
        h2s_look --> h2s_sel
        h2s_look --> h2s_comp
        h2s_sel --> h2s_comp
        h2s_k --> h2s
        h2s_comp --> h2s
        pair --> kem_bb1
        h2s --> kem_bb1
        kem_bb1 --> kem_look
        kem_bb1 --> kem_sel
        kem_look --> kem_sel
        kem_look --> kem_comp
        kem_sel --> kem_comp
        pair_look --> kem_comp
        kem_bb1 --> kem
        kem_comp --> kem
        f_enc --> env_ps
        pair --> env_ps
        env_ps --> env_eg
        pair --> env_eg
        h2s --> env_eg
        kem --> env_eg
        env_eg --> env_look
        env_eg --> env_sel
        env_look --> env_sel
        env_look --> env_comp
        env_sel --> env_comp
        kem_look --> env_comp
        pair_look --> env_comp
        enc_look --> env_comp
        env_eg --> env
        env_comp --> env
        chain_forms --> cf_look
        chain_forms --> cf_sel
        cf_look --> cf_sel
        cf_look --> cf_comp
        cf_sel --> cf_comp
        enc_look --> cf_comp
        chain_forms --> chain_forms_fn
        cf_comp --> chain_forms_fn
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
        proof_sfs --> proof_look
        proof_sfs --> proof_sel
        proof_look --> proof_sel
        proof_look --> proof_comp
        proof_sel --> proof_comp
        env_look --> proof_comp
        pair_look --> proof_comp
        proof_sfs --> proof
        proof_comp --> proof
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
        proof_look --> run_admit
        enc_look --> run_admit
        chain_forms_fn --> run_admit
        run_admit --> run_chain
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
        gen_render --> gen_settings
        gen_group --> gen_settings
        chain_forms_fn --> gen_settings
        gen_settings --> gen_evm
        pair --> gen_evm
        kem --> gen_evm
        env --> gen_evm
        proof --> gen_evm
        h2s --> gen_evm
        chain_forms_fn --> gen_evm
        dom_suite --> gen_evm
        gen_evm --> gen_look
        gen_settings --> gen_sel
        gen_evm --> gen_sel
        gen_look --> gen_sel
        gen_evm --> gen_comp
        gen_look --> gen_comp
        gen_sel --> gen_comp
        cf_look --> gen_comp
        enc_look --> gen_comp
        gen_evm --> gen
        chain_forms_fn --> gen
        gen_sel --> gen
        gen_look --> gen
        gen_comp --> gen
        cfg_layers --> cfg_sel
        rand_sel --> cfg_sel
        pair_sel --> cfg_sel
        enc_sel --> cfg_sel
        h2s_sel --> cfg_sel
        cf_sel --> cfg_sel
        chain_sel --> cfg_sel
        kem_sel --> cfg_sel
        env_sel --> cfg_sel
        proof_sel --> cfg_sel
        chain_look --> cfg_sel
        cfg_layers --> cfg_stmt
        gen_chal --> cfg_stmt
        dom_suite --> cfg_stmt
        hash_look --> cfg_stmt
        dom_pset --> cfg_stmt
        enc_look --> cfg_stmt
        kem_look --> cfg_stmt
        proof_look --> cfg_stmt
        gen_look --> cfg_stmt
        cfg_layers --> cfg_gen
        gen_group --> cfg_gen
        gen_sel --> cfg_gen
        gen_look --> cfg_gen
        gen_comp --> cfg_gen
        pair_look --> cfg_gen
        cfg_layers --> cfg_out
        h_report --> cfg_out
        cfg_sel --> h_cfg
        cfg_stmt --> h_cfg
        cfg_gen --> h_cfg
        cfg_out --> h_cfg
        bench --> h_cfg
        hv_sel --> h_cfg
        h_measure --> h_cfg
        h_report --> h_cfg
        gen --> run_gen
        h_cfg --> run_gen
        hv --> run_ver
        h_measure --> run_ver
        h_cfg --> run_ver
        run_gen --> run_proof
        gen --> run_proof
        proof --> run_proof
        hv --> run_proof
        run_ver --> run_proof
        h_cfg --> run_proof
        chain_fac --> run_proof
        hv_look --> run_proof
        hv_comp --> run_proof
        gen_look --> run_proof
        gen_comp --> run_proof
        run_proof --> run_ka
        proof --> run_ka
        env --> run_ka
        run_ka --> run_kem
        env --> run_kem
        kem --> run_kem
        run_kem --> run_chain
        kem --> run_chain
        chain_fac --> run_chain
        run_chain --> run_forms
        chain_fac --> run_forms
        chain_forms_fn --> run_forms
        run_forms --> run_enc
        chain_forms_fn --> run_enc
        f_enc --> run_enc
        run_enc --> run_pair
        f_enc --> run_pair
        h2s --> run_pair
        pair --> run_pair
        hv_look --> run_pair
        run_pair --> h_run
        h_report --> h_run
        bench --> h_run
        h_cfg --> h_run
        pair --> h_run
        chain_look --> h_run
        h_run --> h_out
        cfg_out --> h_out
        h_cfg --> h_main
        h_run --> h_main
        h_out --> h_main
        sol_pset --> sol_pair
        gen --> sol_pair
        sol_pair --> sol_verify
        hv_evm --> sol_verify
        sol_verify --> sol_deploy
        proof --> hv_evm
        chain_forms_fn --> hv_evm
        f_enc --> hv_evm
        chain_fac --> hv_evm
        hv_evm --> hv_look
        hv_evm --> hv_sel
        hv_look --> hv_sel
        hv_evm --> hv_comp
        hv_look --> hv_comp
        hv_sel --> hv_comp
        chain_look --> hv_comp
        cf_look --> hv_comp
        enc_look --> hv_comp
        pair_look --> hv_comp
        gen_look --> hv_comp
        hv_sel --> hv
        hv_look --> hv
        hv_comp --> hv
        hv_evm --> hv
        chain_fac --> hv
        hv --> h_measure
        h_measure --> h_report
        hv_sel --> cfg_sel
        hv_look --> cfg_sel
        sol_pset --> sol_acct
        gen_const --> sol_acct
        sol_acct --> sol_bind
        gen_const --> sol_bind
        sol_pset --> sol_reg
        sol_bind --> sol_reg
        sol_acct --> sol_reg
        gen_const --> sol_reg
        chain_base --> sol_reg
        sig --> dom_hkc
        chain_forms_fn --> chain_base_set
        pair --> chain_base_set
        chain_base_set --> chain_base
        chain_forms_fn --> chain_base
        dom_hkc --> chain_base
        dom_dep --> chain_base
        hashing --> chain_base
        env --> chain_base
        proof --> chain_base
        pair --> chain_base
        f_enc --> chain_base
        chain_base --> chain_look
        chain_base_set --> chain_sel
        chain_base --> chain_sel
        chain_look --> chain_sel
        chain_look --> chain_comp
        chain_sel --> chain_comp
        cf_look --> chain_comp
        chain_base --> chain_fac
        chain_comp --> chain_fac
    end

    subgraph core["Protocol core and contract suite"]
        direction TB
        sig_ed["signature/ed25519"]
        sig_k1["signature/secp256k1"]
        sig_mldsa["signature/ml_dsa_65"]
        sig_look["signature/lookup_signature_declaration"]
        sig_sel["signature/admit_signature_selection"]
        sig_comp["signature/admit_signature_composition"]
        sig["signature/factory"]
        tr_rq_set["transport/rqbit/settings"]
        tr_rq["transport/rqbit"]
        tr_look["transport/lookup_transport_declaration"]
        tr_sel["transport/admit_transport_selection"]
        tr_comp["transport/admit_transport_composition"]
        tr["transport/factory"]
        disc_admit["discovery/admit_peers"]
        disc_local_set["discovery/local/settings"]
        disc_local["discovery/local"]
        disc_loc["discovery/locator_of"]
        disc_dht_set["discovery/dht/settings"]
        disc_dht["discovery/dht"]
        disc_pex_set["discovery/pex/settings"]
        disc_pex["discovery/pex"]
        disc_trk_set["discovery/tracker/settings"]
        disc_trk["discovery/tracker"]
        disc_agg["discovery/aggregate"]
        disc_look["discovery/lookup_discovery_declaration"]
        disc_sel["discovery/admit_discovery_selection"]
        disc_comp["discovery/admit_discovery_composition"]
        disc["discovery/factory"]
        st_redb["storage/redb"]
        st_look["storage/lookup_storage_declaration"]
        st_sel["storage/admit_storage_selection"]
        st_comp["storage/admit_storage_composition"]
        st["storage/factory"]
        sh_owned_set["seed-host/owned/settings"]
        sh_owned["seed-host/owned"]
        sh_rqbit_set["seed-host/rqbit/settings"]
        sh_rqbit["seed-host/rqbit"]
        sh_look["seed-host/lookup_seed_host_declaration"]
        sh_sel["seed-host/admit_seed_host_selection"]
        sh_comp["seed-host/admit_seed_host_composition"]
        sh["seed-host/factory"]
        s_domain["Sprint: domain model"]
        s_cipher["Sprint: payload cipher and sidecar layer"]
        s_registry["Sprint: registry and entitlement contracts, the EVM suite"]
        s_chain["Sprint: chain and submission families"]
        s_adreg["Sprint: adapter registry and factory, on chain"]

        sig_ed --> sig_k1
        sig_ed --> sig_mldsa
        sig_ed --> sig_look
        sig_k1 --> sig_look
        sig_mldsa --> sig_look
        sig_ed --> sig_sel
        sig_k1 --> sig_sel
        sig_mldsa --> sig_sel
        sig_look --> sig_sel
        sig_look --> sig_comp
        sig_sel --> sig_comp
        sig_ed --> sig
        sig_k1 --> sig
        sig_mldsa --> sig
        sig_comp --> sig
        hashing --> tr_rq_set
        f_enc --> tr_rq_set
        tr_rq_set --> tr_rq
        hashing --> tr_rq
        f_enc --> tr_rq
        tr_rq --> tr_look
        tr_rq_set --> tr_sel
        tr_look --> tr_sel
        tr_look --> tr_comp
        tr_sel --> tr_comp
        hash_look --> tr_comp
        enc_look --> tr_comp
        chain_look --> tr_comp
        tr_rq --> tr
        tr_comp --> tr
        disc_admit --> disc_local_set
        disc_local_set --> disc_local
        disc_admit --> disc_local
        disc_admit --> disc_loc
        tr --> disc_loc
        disc_admit --> disc_dht_set
        disc_admit --> disc_pex_set
        disc_admit --> disc_trk_set
        disc_dht_set --> disc_dht
        disc_pex_set --> disc_pex
        disc_trk_set --> disc_trk
        disc_admit --> disc_dht
        disc_admit --> disc_pex
        disc_admit --> disc_trk
        disc_admit --> disc_agg
        disc_loc --> disc_dht
        disc_loc --> disc_pex
        disc_loc --> disc_trk
        tr --> disc_dht
        tr --> disc_pex
        tr --> disc_trk
        disc_local --> disc_look
        disc_dht --> disc_look
        disc_pex --> disc_look
        disc_trk --> disc_look
        disc_local_set --> disc_sel
        disc_dht_set --> disc_sel
        disc_pex_set --> disc_sel
        disc_trk_set --> disc_sel
        disc_look --> disc_sel
        disc_look --> disc_comp
        disc_sel --> disc_comp
        disc_agg --> disc_comp
        tr_look --> disc_comp
        hash_look --> disc_comp
        enc_look --> disc_comp
        chain_look --> disc_comp
        disc_local --> disc
        disc_dht --> disc
        disc_pex --> disc
        disc_trk --> disc
        disc_agg --> disc
        disc_comp --> disc
        st_redb --> st_look
        st_redb --> st_sel
        st_look --> st_sel
        st_look --> st_comp
        st_sel --> st_comp
        enc_look --> st_comp
        st_redb --> st
        st_comp --> st
        hashing --> sh_owned_set
        f_enc --> sh_owned_set
        tr --> sh_owned_set
        st --> sh_owned_set
        sh_owned_set --> sh_owned
        hashing --> sh_owned
        f_enc --> sh_owned
        tr --> sh_owned
        st --> sh_owned
        sh_owned_set --> sh_rqbit_set
        sh_rqbit_set --> sh_rqbit
        sh_owned --> sh_rqbit
        hashing --> sh_rqbit
        sh_owned --> sh_look
        sh_rqbit --> sh_look
        sh_owned_set --> sh_sel
        sh_rqbit_set --> sh_sel
        sh_look --> sh_sel
        sh_look --> sh_comp
        sh_sel --> sh_comp
        hash_look --> sh_comp
        enc_look --> sh_comp
        tr_look --> sh_comp
        st_look --> sh_comp
        sh_owned --> sh
        sh_rqbit --> sh
        sh_comp --> sh
        s_domain --> s_cipher
        hashing --> s_cipher
        s_domain --> s_registry
        s_registry --> s_chain
        sig --> s_chain
        disc --> s_chain
        s_registry --> s_adreg
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
    f_ws --> f_reader
    f_rand --> pair_bn_ark
    f_ci --> sol_pset
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
    f_enc --> gen_const
    f_enc --> gen_chal
    f_enc --> gen_group
    f_enc --> gen_reject
    f_enc --> gen_map
    f_enc --> gen_poss
    f_enc --> gen_proof
    f_enc --> gen_settings
    f_rand --> gen_group
    f_rand --> gen_map
    f_rand --> gen_poss
    f_rand --> gen_chal
    f_rand --> h_del_mint
    f_rand --> h_del_transfer
    f_rand --> h_measure
    f_sec --> h_del_mint
    f_sec --> h_del_transfer
    f_ws --> hashing_b3
    f_sec --> hashing_b3
    f_enc --> sig_ed
    f_ws --> disc_admit
    f_enc --> st_redb
    f_enc --> chain_forms
    sig --> chain_forms
    f_ws --> cfg_layers
    f_rand --> gen_evm
    f_rand --> h_run
    f_rand --> bench
    chain_fac --> s_chain
    f_rand --> kem_bb1
    f_rand --> env_eg
    f_rand --> proof_sfs
    f_rand --> sig_ed
    f_rand --> sig_mldsa
    f_sec --> s_domain
    f_enc --> s_domain
    f_sec --> kem_bb1
    f_sec --> env_eg
    st --> e_jobs
    f_enc --> e_cas

    kem --> s_domain
    sol_verify --> s_registry
    sol_reg --> s_registry
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
    s_registry --> m_relayer
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

**Reading the graph.** Everything in the foundation and harness subgraphs, and every ticket-named node in the protocol core, is one source file.
A factory node is the family's `factory` module; a concrete node is one adapter beneath it, private to the crate; a family's first ticket, its first concrete, a family-owned function, or a settings admission preceding it, carries the family's interface or the producers the admission reads and creates the crate; a concrete with a settable value follows its settings admission, a concrete with none has no settings admission, the family's declaration lookup, selection admission, and composition admission follow in turn, and the factory follows every concrete and every admission surface.
Every edge out of a family runs from its factory or from the family-owned admission surface the consumer uses, never from a concrete: a consumer that reads a declared value follows the family's declaration lookup, one that receives a configured member follows the selection admission, one that composes members follows the composition admission, and one that needs the constructed member follows the factory; the registry sprint consumes the harness's `contracts/evm/DeliveryVerifier`, and the First Finder, credential delivery, and identity epics consume the KEM, envelope, proof, and KDF factories, which construct the concretes the harness proved.
The chain forms family sits in the harness, its factory following its one forms concrete and the family's declaration lookup, selection admission, and composition admission, and the proof family's transcript descriptions, challenge, concrete, and factory, the chain family, the generator, verifier client, and measurement consume the forms factory, so no harness ticket names a form set; the chain family's tickets sit in the harness ahead of the verifier client, `chain/base/settings` first, then `chain/base`, authoring the ABI descriptions it binds, then the family's declaration lookup, selection admission, and composition admission, then `chain/factory`, and the chain and submission sprint's edge from the factory is its remaining tickets' consumption of it.
The harness run is the grouping's composition root, consuming the configuration ticket, the benchmark, and every factory and admission surface its consumers name through the consumers it owns, each consumer following the one it constructs; the output follows the run, whose outcome it writes, and the binary follows the configuration, the run, and the output, which it calls in turn; every edge into a ticket, sprint, epic, or milestone is a code dependency, every edge into an objective or the demonstrable milestone, neither of which authors a file, is a condition it closes on, and the graph has no cycle; the generated directory, the deployed verifier, and the measured defaults are runtime inputs, held in the [Runtime sequence](#runtime-sequence), and appear in no edge.
The generate concrete's functions each consume the factories whose adapters they are handed, and the vector functions consume the encoding family's contract for their records' descriptions; `harness-crypto/generate/solidity/render` precedes every one of them, the adapter follows them all, and the family's declaration lookup, selection admission, and composition admission follow the adapter and precede the generate factory.
The configuration's layering function precedes its admission functions, each of which depends on the admission surfaces whose values it admits: `admit_selections` consumes every family's selection admission plus the chain and verifier declaration lookups whose declared values the selections need, `admit_statements` consumes the hashing, encoding, KEM, proof, and generate declaration lookups, `admit_generate` consumes the generate family's selection, lookup, and composition admissions and the pairing declaration lookup, and `admit_output` consumes the report whose destinations it names; the composing configuration ticket follows every admission function, the benchmark, the verifier's selection admission, and the measurement and report surfaces it carries.
The signature and swarm tickets have no incoming edge from any contract sprint, which is the point that bytes can move with no chain.
The relayer milestone depends on the chain and submission families and the registry contracts it submits signed intents against and can be built as soon as they exist.
The resolution orchestrator follows credential delivery, First Finder ingest, the package host, and the swarm, whose sources it orders.
The daemon composition is the daemon grouping's composition root, as the harness run is the harness's, following every epic and milestone whose modules it composes, the installation coordinator, the explicit publisher path, and the claim verifier among them. The demonstrable milestone has incoming edges from the daemon composition and the seed-host family, and nothing depends on it.

## Sequencing

`workspace/cargo` precedes everything. Within the harness the starting points are `pairing/bn254_arkworks`, which carries the pairing interface with its sampling bound, and the domain's identifier modules on `domain/secret`; `hashing/blake3_bao` follows `workspace/cargo`, the family's declaration lookup, selection admission, and composition admission follow it, and `hashing/factory` follows them and precedes the encoding family; `encoding/canonical_field_reader` is the encoding family's first ticket and precedes `encoding/abi`, and the family's lookup, selection admission, and composition admission follow `encoding/abi` and precede `encoding/factory`; `workflows/sidecar/derivation_context` and its description follow the hashing factory, the identifier modules, and the encoding factory and precede `workflows/sidecar/wrap`, and `kdf/blake3_keyed`, the first ticket that encodes a context, follows the encoding factory;
every other ticket has a producer, each family's first ticket precedes its further concretes, a concrete with a settable value follows its settings admission, the family's lookup, selection admission, and composition admission follow the concretes and their settings admissions, and the factory follows them all; `chain-forms/evm` follows `encoding/factory` and `signature/factory`, the family's declaration lookup, selection admission, and composition admission follow it, `chain-forms/factory` follows them, and every proof ticket follows `chain-forms/factory`, `harness-crypto/delivery/mint` follows `proof/factory` and `harness-crypto/delivery/transfer` follows it, both preceding `harness-crypto/generate/solidity/render`, every function the generate concrete owns following `harness-crypto/delivery/mint`, and `harness-crypto/generate/solidity/proof_vectors` and `harness-crypto/measure` following `harness-crypto/delivery/transfer` as well, `harness-crypto/generate/solidity/render` precedes every other function the generate concrete owns,
the adapter `harness-crypto/generate/solidity` follows them all, the generate family's declaration lookup follows it, the selection admission and composition admission follow the lookup, and `harness-crypto/generate/factory` follows them, `harness-crypto/config/layers` precedes the configuration's admission functions, `harness-crypto/config/admit_generate` follows the generate family's selection, declaration lookup, and composition admissions, whose per-kind quantities it admits, `harness-crypto/config/admit_output` follows `harness-crypto/config/layers`;
`contracts/evm/ParameterSets`, creating the suite's Foundry configuration and the Solidity continuous-integration definition, follows `workspace/ci`; `domain/handshake_key_commitment` follows `signature/factory`;
`chain/base/settings`, the chain family's first ticket, follows `chain-forms/factory` and `pairing/factory`; `chain/base`, authoring the registry's and the entitlement interface's ABI descriptions it binds, follows `chain/base/settings`, `domain/handshake_key_commitment`, and the forms factory, the chain family's declaration lookup, selection admission, and composition admission follow it, and `chain/factory` follows them; `harness-crypto/verifier/evm`, authoring the verifier family's declaration and authoring and binding the delivery verifier's ABI description, follows `chain/factory`, the proof factory, the encoding factory, and the forms factory;
the verifier family's declaration lookup follows `harness-crypto/verifier/evm`, the selection admission and composition admission follow the lookup, and `harness-crypto/verifier/factory`, `harness-crypto/measure`, and `harness-crypto/report` follow in turn; the admission functions that take their values and `harness-crypto/config` follow `harness-crypto/report`, and `harness-crypto/config` follows every admission function; `harness-crypto/run/admit_contexts` follows `harness-crypto/config`;
the run's consumers follow `harness-crypto/run/admit_contexts` innermost first, from `harness-crypto/run/generate_consumer` through `harness-crypto/run/verifier_consumer` and `harness-crypto/run/chain_consumer` to `harness-crypto/run/pairing_consumer`; `harness-crypto/run`, `harness-crypto/output`, and `harness-crypto/main` follow in turn;
`contracts/evm/PairingLib` follows `contracts/evm/ParameterSets` and `harness-crypto/generate/factory`; `contracts/evm/DeliveryVerifier` follows `contracts/evm/PairingLib` and `harness-crypto/verifier/evm`; `contracts/evm/IdentityAccount` follows `contracts/evm/ParameterSets` and `harness-crypto/generate/solidity/constants`; `contracts/evm/Binding` follows `contracts/evm/IdentityAccount` and the constants function; `contracts/evm/Registry` follows `contracts/evm/ParameterSets`, `contracts/evm/Binding`, `contracts/evm/IdentityAccount`, `chain/base`, and the constants function; and `contracts/evm/deploy` follows `contracts/evm/DeliveryVerifier` and carries the grouping's commit. `harness-crypto/main` orders no ticket: the generated directory and deployed verifier are runtime inputs held in the Runtime sequence.
The signature tickets start from `signature/ed25519` on `encoding/factory` and `random/factory`, `signature/secp256k1` and `signature/ml_dsa_65` following it, and the family's declaration lookup, selection admission, and composition admission follow the concretes and precede `signature/factory`; the swarm tickets start from `transport/rqbit/settings` and `storage/redb` on `hashing/factory` and `encoding/factory` and from `discovery/admit_peers` on `workspace/cargo`, each settings admission preceding its concrete and each family's three admission functions preceding the factory its table names.

Across groupings the sequence is the [milestones](milestones.md)' order, with the parallelism the graph makes visible: the signature and swarm tickets run beside the harness and the contract sprints from `encoding/factory` and `hashing/factory` onward; the identity and custody epic begins as soon as the signature tickets, the chain and submission families, and the relayer or paymaster milestone exist, before the CAS epic, because the installation coordinator needs it; the relayer or paymaster milestone follows the chain and submission families sprint and precedes every daemon epic that submits a signed intent; the resolution orchestrator follows credential delivery and First Finder ingest; the daemon composition follows the resolution orchestrator, the installation coordinator, the explicit publisher path, and the claim verifier and escrow claim, and the desktop application and CLI follow it; the demonstrable milestone follows the daemon composition. Within the credential delivery epic `payment/native` precedes `payment/factory`, which precedes `workflows/acquire`; the registry contracts sprint authors the lock and `IPaymentAsset` with its native concrete, and the on-chain adapter registry sprint binds them. The package host maps a request's name to the asset's root through its alias lookup, the root type being the hashing family's, and `chain-forms/evm` authors address derivation and the signed-intent form against the signature family's scheme identifier, which is why the forms concrete follows `signature/factory`. `chain/factory` is authored once, after the Base concrete and the chain family's admission functions and ahead of `harness-crypto/verifier/evm`; the chain and submission sprint holds the family's remaining tickets.

The decay rule governs re-mapping. When the `contracts/evm/deploy` node commits, the remaining protocol core milestones are re-mapped from sprints to tickets using what the harness taught, the daemon grouping from epics to sprints, and so on outward. Each re-mapping is a revision of this document in place.

## Runtime sequence

What running the built harness and suite produces, each step after the inputs it reads. These are operations, not tickets, and none is an edge.

- The harness binary's benchmark measures every selected pairing concrete, recording each concrete's measurements.
- The harness binary's first run from its shipped defaults writes every admitted cell's cell directory and returns each cell's verifier-stage refusal.
- The contract suite builds and passes its tests against the generated directory.
- `contracts/evm/deploy` reads each cell directory's values and deploys the delivery verifier once per distinct curve identifier, in the verifier form that curve's values state, to Anvil and Base Sepolia, and emits each address as the deployment record keyed by the suite's declared identity and that curve identifier, rendered at the file boundary as each value's declared wire code, together in the layer `contracts/evm/deployments/base-sepolia.toml`.
- The harness binary's next run, with that layer passed through `--config` over its shipped defaults and admitted through the configuration's layering and the owning families' identifier admissions, looks each cell's deployment record up by the selected verifier member's declared contract suite identity and its curve identifier, reaches the verifier at that address, verifies, measures, and writes its report over every pair's and every cell's outcome; the end-to-end tests that read the deployment records run here, outside `cargo test`, while over the shipped defaults, which hold no deployment record, the integration tests observe each cell's verifier-stage refusal as a typed refusal value carried to the output boundary.

## Risk Mitigation

| Risk from the register | How the map addresses it |
| --- | --- |
| R-02, cost makes installs noticeably slower | The harness grouping is entirely at ticket resolution; `harness-crypto/report` records the measurements against the configured piece-group size and curve defaults and any adjustment the data supports, and every ticket that encrypts reads the piece-group size from the deployment record and waits on nothing |
| R-03, verifier flaw | `proof/schnorr_fs` and `contracts/evm/DeliveryVerifier` are separate tickets with `harness-crypto/verifier/evm` as the parity integration point and `harness-crypto/generate/solidity` as the source of the contract's constants and vectors; the mutation and replay proof sits with `proof/schnorr_fs`'s tests and the contract suite's Foundry tests |
| R-04, execution breadth | Decaying resolution means no distant detail is authored to be rewritten; a family's complete shape is its concretes, the settings admission of each concrete with a settable value, the family's declaration lookup, selection admission, and composition admission, and its factory, so a family's ticket count follows that shape rather than its concretes alone |
| R-09, open selections | The identity epic carries custody recovery UX; the license gates release and no ticket |
| R-12, chain properties | Every pairing concrete and both verifier forms are declared members beneath the pairing and proof families' complete admission increments, so a precompile change on Base selects the retained form through declaration lookup and composition admission at resolution rather than rebuilding; a further chain is a further declared `chain` member with its own settings admission, declaration, and suite under `contracts/` |
| R-13, escrow record | No commitment and no custodian; the claim milestone has no policy gate |
| R-14, adapter registry governance | The on-chain adapter registry sprint is separate from the registry contracts sprint, so its single project-held key can be replaced without touching entitlements; the same registry binds the attestation verifier per source identifier, the entitlement token form, the signature adapter per scheme identifier, the delivery verifier, and the payment-asset adapter per payment-asset identifier, each addition selected through its family's declarations and composition admission |
| A consumer bound to a concrete | Every family crate exposes its factory and its family-owned declaration lookup, selection admission, and composition admission, every consumer ticket follows the admission surface that supplies the value it uses and reaches constructed members only through the factory, and the harness constructs through the factories after composition admission, so no ticket can name a concrete it does not own |

## Decisions

**The adapter family form.** Every family is a factory crate owning the generic interface, the capability declaration, the family's member identifier over its declared real members, the value identifiers for the shared values its members provide — each declared member carrying exactly one canonical spelling and, where a wire encoding exists, exactly one declared wire code — and the types the generic surface needs; each concrete is a private module beneath the factory with its own interface implementing the generic one, owning only the types it alone produces, and, where configuration sets a value it is constructed from, preceded by its own settings admission; the family's declaration lookup, selection admission, and composition admission precede the factory, the selection admission returning a selection carrying a member's admitted settings where the member has any and the composition admission comparing requirements with declarations before anything is constructed; consumers depend on the admission surface that supplies the value they use and on the factory for construction; the layout is general responsibility, then functional need, then concrete implementation. A family exists even where the MVP ships one concrete.

**Crate and path layout.** One workspace with a domain crate, a workflows crate, a crate per adapter family holding its `factory` module and its private concretes, and a crate per deployable, each created by the ticket of the first module that lives in it, which for a family is its first ticket; the workspace manifest lists members by glob; tickets are named by crate, factory, concrete, and function as the technical requirements' file tree spells them.

**Canonical encoding.** The `encoding` factory owns `IEncoderAdapter` and `IDecoderAdapter` under one versioned encoding identifier, the repo-owned encoding contract, and the declaration; the family owns one description module per encoded type housed in a crate it depends on, `encoding/<type>`, implementing the contract for that type, the encoding crate depending on the hashing crate and on the domain crate, and a type housed in a crate that depends on the encoding family, the derivation context among them, carrying its description in that crate; the family follows the hashing family's root type, the first encodable type, at the head of the harness grouping; `encoding/abi` implements both through `alloy`'s sol types and is the only module naming `alloy` for encoding; one encoding for everything hashed, signed, stored, or framed over IPC, including the configuration registry's records.

**Tools and dependencies.**
A configuration file is authored once, complete, by the ticket that creates it: the workspace manifest with the `librqbit` overlay and `deny.toml` with the overlay repository's source at `workspace/cargo`, the Rust continuous-integration definition at `workspace/ci`, `contracts/evm/foundry.toml`, granting `fs_permissions` read access to the generated directory and the ABI description directory and write access to the deployments directory alone with `ffi` disabled and `via_ir = true`, and the Solidity continuous-integration definition, with `forge build`, `forge fmt --check`, and `forge test`, at `contracts/evm/ParameterSets`, the suite's first source file, and the shell, fuzz, and end-to-end continuous-integration definitions each at the ticket that first needs it, as separate files.
A crate's manifest and its `lib.rs` barrel accumulate one entry per module, each added by that module's ticket as its deps and provides elements.
A vendor library is named only by the concrete that wraps it; `harness-crypto/benchmark` names none and reaches each pairing library through the pairing factory; `harness-crypto/config/layers` pins `clap`, `toml`, and `serde`, the one place the harness names any of them. `pairing/bn254_arkworks` pins `num-bigint` as the pairing crate's dev-dependency, at the version `Cargo.lock` already resolves through `ark-ff`, for the integer exponent its definition test computes; no tool outside the workspace takes part in the target-group proof.

**Secrets.** The secret-typed value is a `domain` ticket pinning `zeroize` as a domain crate dependency; the telemetry family carries correlation and tracing setup and no redaction layer.

**Ticket granularity for a concrete adapter.** One node: the concrete's operations are the methods of one adapter in one file, and a function the concrete owns, such as the Schnorr challenge or the Solidity generate concrete's rendering, constants, and vector functions, is a node beneath it.

**Interfaces first, factories last.** A family's generic interface, member and value identifiers, declaration, and mock are authored in the ticket of its first concrete, the first source file that requires them; each further concrete follows, preceded by its settings admission where it has a settable value; the family's declaration lookup, selection admission, and composition admission follow the concretes they cover; the factory function, which calls composition admission before it constructs anything, is authored once, after the family's admission surfaces, and precedes every consumer that needs a constructed member; every consumer follows the admission surface that supplies the value it uses, and the harness constructs its combinations through the factories after the required admissions. Values consumed before the adapter that binds them exists are a family of their own, as the chain forms are beside the chain adapters, so each family's factory is authored once, after every concrete and every admission surface of that family.

**The harness configuration.** A typed configuration the harness owns, `harness-crypto/config`, with a shipped default for every value the harness passes as params and overrides from a configuration file and from command-line flags, composed over a layering function and admission functions it owns, each its own ticket ahead of it; the shipped defaults are `apps/harness-crypto/harness.toml`, carried by the composing ticket, and the overrides are `--config <PATH>` files and `--set <KEY=VALUE>` assignments applied in command-line order over them; the configuration follows every admission surface whose values it carries, admitting configured spellings through each owning family's selection admission and reading declared values through its lookup; the binary reads it, every factory call the run makes carries its admitted selections, and the output writes the run's outcome to the destinations it names, so no harness module holds a literal for a concrete, a curve, a benchmarked concrete, an identifier, a form, a version, a profile, an identity scope, a statement's context value, an identity, an iteration count, a vector kind's count, a path, a file, library, or symbol name, a compiler version, a license identifier, or an output destination; the matrix's lists — the pairing concretes, the target-group encoding identifiers, and the encodings — default to the families' declared real sets, and every configuration, vector, deployment, and benchmark name is a canonical spelling or wire code rendered at a boundary, never an internal text key.

**The harness run, its output, and its binary.** The run is a composing function over one consumer per factory it calls, each its own ticket, and the harness's composition root; the output writes the run's outcome, a success or a refusal, to each configured destination, standard output, a file, or both; the binary calls the configuration, the run, and the output in turn and sets the exit status.

**The canonical target-group value.** The encapsulated value the KDF consumes is a target-group element's encoding, and two libraries of one curve are not assumed to compute the same element or serialize it the same way. The pairing family's declaration carries a target-group encoding identifier naming the pairing map, the generators, and the coefficient serialization; the suite requires it as an explicit hash-card field beside the forms identifier; the family's composition admission compares the declared identifier with the required one and refuses `Incompatible` before construction, so the factory admits a concrete only when it declares the required one; and every concrete is proven by executing the identifier's definition in its own tests, which fixes the map, the generators, the tower, the exact exponent `(p^12 - 1) / r`, and the serialization: the test computes the library's Miller loop over the generators, raises it to the integer exponent, serializes it, and requires the concrete's pairing product to encode to the same bytes, and on BLS12-381 asserts the CFRG draft's published value as well; a library's own reduced pairing is never the proof, since several libraries compute a fixed multiple of the exact exponent for speed. A concrete wrapping such a library multiplies the library's output by the multiple's inverse in the scalar field, computed at construction from the multiple the library's own final exponentiation states; the multiple is a fact about that library recorded in the concrete's node, and the definition test proves it; the family's integration test then proves the two concretes of a curve equal to each other as a consequence.

**The harness's own families.** The harness's `generate` and `verifier` families, the generate family with a concrete named for the language it emits, `solidity`, writing ABI-encoded language-neutral vector files, and the verifier family with an `evm` concrete named for the ABI it speaks, each carrying the family's interface and the family's declaration lookup, selection admission, and composition admission ahead of a factory following them, the generate concrete preceded by its settings admission and the verifier concrete, which has no settable value, by none, because emitting a suite's vectors and reaching a suite's verifier are suite-specific and the daemon needs neither.

**The storage engine and the engines above it.** The `storage` factory with `storage/redb`; the job engine and the configuration registry are workflows modules over it, since nothing external remains in them.

**BitTorrent library.** `librqbit` compiled in as a soft fork through the workspace manifest's `[patch.crates-io]` overlay onto the project's overlay branch `chaintorrent-overlay` in `https://github.com/tsylvester/rqbit`, which tracks tagged upstream releases and exists before the bootstrap, with the hooks authored inside `transport/rqbit`, submitted upstream, and the commit pinned; no owned transport in the MVP; the library's discovery sources are `discovery` members reached through the transport family's engine-capability identifier; the hard-fork trigger is recorded in the product requirements.

**The contract suite.** `contracts/evm` is the on-chain concrete of `chain/base`, identified by its virtual machine and its source language, Solidity under Foundry in the MVP, with the toolchain inside it; its boundary with the client is its ABI; a further forms suite is a further declared `chain-forms` member carrying the forms family's admission increments, while a further chain adapter or source language is a further declared `chain` member carrying its own settings and admission increments, the chain member's declaration binding the forms identifier it requires, and no forms suite or source language enters either family's declared set until a real member exists.

**The asset's identity.** The asset's identity is the object's BLAKE3/Bao root, the plaintext root for a public asset and the ciphertext root for a private one, owned by the hashing family, whose operations are all over bytes and which carries no digest operation and no identity-hash identifier. A name is a lookup: the package host and the ingest adapter declare the namespace their names live in, the Registry holds append-only alias records under a declared namespace and an opaque key, each entry a root with the source's signed statement or its recorded absence, and the client selects an entry only on evidence it holds independently of every registrant.

**Address derivation and the signed-intent form.** Both belong to the chain forms family's form interface and are realized for the EVM suite by `chain-forms/evm`, over a public key's canonical bytes and scheme identifier and as EIP-712 typed data over the call's fields, chain, contract, nonce, and expiry; `chain/base` binds the resolved forms and authors none, and the signature family signs the bytes the intent form hands it. The scheme identifier is the signature family's owned value identifier, declared with its members' spellings and `uint8` wire codes and exported through the family's `provides`, so the signature family's declarations carry it and the intent form's declarations name it as a typed identifier, the forms concrete depending on the signature factory that provides it.

**The post-quantum signature concrete.** `signature/ml_dsa_65`, FIPS 204 ML-DSA-65, is a concrete beneath the signature factory beside `signature/ed25519` and `signature/secp256k1`, declaring no recovery support, and the family's declaration lookup, selection admission, composition admission, and factory cover it when they are authored; which scheme identifier a layer signs under is configuration admitted through the family's selection admission, and the binding record's scheme identifier and key commitment carry a key of any width, so the concrete changes no record, form, or consumer.

**The claim verifier's key.** The on-chain record is a scheme identifier and a commitment to the key, the key travelling in the calldata of the registering or rotating transaction; the contract verifies vouchers through the signature adapter the registry binds for the scheme; the claim verifier signs under a custody-held key of a declared scheme identifier, and a further verifier form is a further declared `claim-verifier` member with the family's complete settings and admission increment that changes no contract storage.

**The payment asset.** A price is an amount of the asset a payment-asset identifier names; a lock carries the payment-asset identifier, a value the `payment` family owns and declares with its members' spellings; the contracts move payment only through the payment-asset adapter the registry binds per identifier behind `IPaymentAsset`, whose MVP concrete settles in the chain's native asset; the client reaches payment only through the `payment` family's factory, `payment/native` being its MVP concrete and a declared member carrying the family's complete settings and admission increment; the relayer's refusal to sponsor a paid acquisition is stated against the payment-asset identifier the lock names.

**The harness's delivery verifier.** The shipped contract, written to production standard from the start.

**Milestones at ticket resolution beside the harness.** Signatures and the swarm, because they depend on nothing the harness measures; they start once `encoding/factory` and `hashing/factory` close, since their first tickets encode and hash. The hashing family's concrete, declaration lookup, selection admission, composition admission, and factory form a complete harness ticket sequence at the head of the grouping.

**The demonstrable milestone.** After the daemon composition, whose packaged daemon it runs.

**The delegated seed-host concrete** is `seed-host/rqbit`, driving the rqbit application through its HTTP API; a further external client or the project's own client is a further concrete beneath the seed-host factory; the swarm milestone's commit sits on it.

# Additional Content

**What a ticket is and is not.** A ticket here is a candidate for a workplan node: it names the source file's role, what it owns, what it depends on, and what requirement it proves. It is not a node. A node is authored through the ordinary path in the node template, with every element in the fixed order, and this document does not emit node structure. When a ticket is promoted, its row here is unchanged; the node is the instruction and this map is the overview. In a ticket-level family the ticket list includes the settings admission of each concrete with a settable value, ahead of that concrete, and the family's declaration lookup, selection admission, and composition admission ahead of the factory, and a row's dependency cell names the specific admission surface the ticket consumes; coarser sprints, epics, milestones, and objectives name families without these rows, and decomposing them into tickets — and remapping the requirements they prove — belongs to the re-mapping pass the decay rule describes, not to this document's current rows.

**Why the harness has the tickets it has.** One source file per node, so each cryptographic family's concretes, one per curve per library for pairing, the family's declaration lookup, selection admission, and composition admission, and its factory, the benchmark, the generate and verifier families with their concretes, each function the generate concrete owns, the generate concrete's settings admission, the chain family's Base concrete's settings admission, the transcript descriptions, the parameter-set records, which create the suite's toolchain, the Solidity library and verifier, the identity account, binding, and registry, the wrap and unwrap, and the measurement driver each get their own file with full support. The `sample_deployment` generator needs the payload cipher and belongs to the protocol core grouping. That list is the honest size of the harness grouping.

**Requirement coverage of the foundation and harness groupings.** CR-02, CR-03 in part, CR-04, CR-05, CR-07 in part, CR-08, CR-09, CR-10, CR-11, CD-03, CD-07, CD-08, LC-08 in part, XA-07 in part, AS-17 for the cryptographic families, and AS-21. Everything else the requirements name is in a later grouping.
