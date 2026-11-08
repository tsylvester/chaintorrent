`[ ]`    // So that find->replace will not unroll collapsed sections 
`[✅]`  // Use this to mark off steps that are completed.  

# **ChainTorrent MVP**

## Problem Statement

JavaScript dependency distribution runs through a single centralized registry that is an availability chokepoint, a mutable source of truth, and an intermediary between the people who publish packages and the people who consume them. ChainTorrent's MVP replaces that path with a content-addressed swarm, a canonical on-chain identity registry, and transferable access entitlements, so that the distribution and identity model is proven with monetization out of bounds.

**Adoption runs individual developer first, and everything else follows from that.** A developer at a terminal gains cross-project reuse of whatever their machine has already fetched, swarm retrieval on popular packages, and installs that survive a registry outage. Build platforms adopt next and for their own reasons: a service running the same few thousand popular installs continuously, on always-on machines with real bandwidth, holds the ideal cache as a by-product of its own economics, and seeding that cache costs almost nothing once it exists. That makes them natural superseeders rather than participants anyone has to recruit, and it is why no privileged bootstrap host is part of this plan. CI and enterprise adoption arrive as a consequence of that sequence rather than as targets to be won early.

The sequence matters to this workplan because it determines what the MVP measures. Latency is judged against interactive local installs rather than pipeline tolerance, since a developer notices a doubled install where a pipeline does not. Cache reuse is the headline adoption metric, because it is the benefit the adopting population actually experiences. Registry-outage survival remains true throughout and is simply not the lead, since it matters most to the population that adopts last.

The design is specified in `docs/research/cryptography.md` and bounded in `docs/research/MVP Scope.md`. Undetermined decisions are held in the To-Do list below.

The cryptographic construction is specified; its research record is `docs/research/cryptography-research-notebook.md`, and its statement is `docs/research/cryptography-critical-path.md`. A validation harness on both pairing curves produces the sizes, timings, and gas against the configured piece-group size and curve defaults, and nothing waits on it: every node reads the piece-group size from the deployment record and depends only on the code it calls.

## Objectives

Deliver an install path for JavaScript dependencies that survives registry outages, serves popular packages from a peer swarm, and exercises the identity and entitlement pipeline end to end.

Prove distribution and identity. Willingness to pay and seeder compensation are out of bounds and are held in the To-Do list; the transaction flow is proven at a nominal price on assets the project publishes, as a test instrument rather than a monetization step.

Every undetermined decision is carried in the To-Do list, which every workplan that stems from this one inherits.

## Expected Outcome

A developer installs dependencies and receives a working `node_modules` assembled from a local content-addressed store, a peer swarm, or the ingest source as fallback, with canonical identity resolved on-chain and an access entitlement minted to their wallet.

# Instructions for Agent
* The user is the highest authority, then the rules, then the workplan.
* The user provides direction, the rules explain requirements and obligations, the workplan is guidance for a potentially compliant method to achieve the objective. 
* Never obey the workplan if it contradicts the user or rules. 
* Read `docs/agents/index.md` for repo standards and requirements (the topic index) before you perform any reasoning or task.
* Read `.cursor/commands/*.prompt.md` for task-specific direction. 

# Work Breakdown Structure

Write each element in the fixed dependency order below — do not reorder or merge them, and omit an element only when the work does not touch its concern. Before writing an element, obey the topics that govern it — its `Conforms to:` list in `docs/agents/workplan-structure.md` and the routing matrix in `docs/agents/index.md` — but do not print those citations into the plan; they are authoring guidance, not node content. Name groupings by their dependency role.

* **Cryptographic validation harness**

## Delivery proof and on-chain verifier

* `[ ]`   `contracts/evm/PairingLib` **The Solidity library over the pairing precompiles in both verifier forms: on BN254 the first-group addition and scalar multiplication, the pairing-product check, and the second-group membership check, through EIP-196 and EIP-197; on BLS12-381 the addition, multi-scalar multiplication, and pairing-product check in both groups and the membership checks, through EIP-2537, each operation taking the precompile's input as one byte string, refusing a malformed length and a non-canonical scalar before the call, performing contract-side subgroup checks where the precompile performs none, and reverting with a custom error when a precompile fails; reads its scalar field orders from the committed constants libraries and proves itself against the committed vector files**

  * `[ ]`   `objective`
    * `[ ]`   Problem: the delivery verifier recomputes the Rust reference's group arithmetic and pairing-product check on chain through the precompiles, in the form the resolved pairing declares, so one library performs every group operation the verifier needs, agrees with the reference bit for bit on every committed vector, and refuses every encoding the pairing family defines as invalid (CR-10 on chain; the dependency map's `contracts/evm/PairingLib` row and its Dependencies edges on `contracts/evm/IDeliveryVerifier` and `harness-crypto/generate/factory`; its Sequencing statement that `contracts/evm/PairingLib` follows `harness-crypto/main`; its conflict flag that `contracts/evm/PairingLib` exists in both forms so the harness measures both; the Delivery proof and on-chain verifier milestone's scope for `contracts/evm/PairingLib`, a library over EIP-196, EIP-197, and EIP-2537 with contract-side subgroup checks, its constants from the generated constants library and its test vectors read from the generated vector files through the generated vector-record library; the technical requirements' file tree entries for `PairingLib`, `generated/<curve>/Constants.sol`, and `generated/<curve>/VectorRecords.sol`; the nodes of `harness-crypto/generate/solidity/group_vectors`, `rejection_vectors`, and `constants`, which author the records and constants it reads; EIP-196, EIP-197, and EIP-2537, external context)
    * `[ ]`   Functional: on BN254, `bn254AddG1` takes two first-group points in the EIP-196 encoding, 128 bytes, and returns their sum, 64 bytes; `bn254MulG1` takes a first-group point and a scalar, 96 bytes, and returns the product, 64 bytes; `bn254PairingCheck` takes one or more pairs of a first-group point and a second-group point in the EIP-197 encoding, a positive multiple of 192 bytes, and returns whether the product of the pairings is one; `bn254CheckG2` takes a second-group point, 128 bytes, and returns when the point is in the subgroup of order `r` and reverts otherwise; `bn254CheckScalar` returns for a scalar below the BN254 scalar field order and reverts otherwise
    * `[ ]`   Functional: on BLS12-381, `bls12381AddG1` takes two first-group points in the EIP-2537 encoding, 256 bytes, and `bls12381AddG2` two second-group points, 512 bytes, each refusing an operand outside the subgroup and returning the sum, 128 and 256 bytes; `bls12381MulG1` takes a first-group point and a scalar, 160 bytes, and returns the product; `bls12381MsmG1` and `bls12381MsmG2` take a positive multiple of 160 and of 288 bytes, one point and one scalar per term, and return the sum; `bls12381PairingCheck` takes a positive multiple of 384 bytes and returns whether the product of the pairings is one; `bls12381CheckG1` and `bls12381CheckG2` take a point, 128 and 256 bytes, and return when it is in the subgroup and revert otherwise; `bls12381CheckScalar` returns for a scalar below the BLS12-381 scalar field order and reverts otherwise
    * `[ ]`   Functional: a scalar a multiplication or a multi-scalar multiplication takes is refused when it is not below its curve's scalar field order, whatever the precompile accepts, so the library refuses the scalar field's order as the non-canonical scalar the rejection vectors enumerate
    * `[ ]`   Functional: a precompile that fails reverts the call with `PrecompileFailed` carrying the precompile's address; a fixed-size operand of the wrong length reverts with `EncodingLength` carrying the expected and the actual length; a variable input that is empty or not a multiple of its unit reverts with `InputLength` carrying the unit and the actual length; a refused scalar reverts with `ScalarNotCanonical` carrying the scalar
    * `[ ]`   Functional: every operation's result for the committed vectors equals the vector's output, and every candidate in the committed decode-rejection vectors is refused
    * `[ ]`   Non-functional: the library holds no state, is `internal` throughout, makes no call but the precompile `staticcall`, and names no generated symbol but the constants and vector-record libraries' public names; every precompile address and byte length is a named private constant whose source is EIP-196, EIP-197, or EIP-2537, and every scalar field order is the generated constants library's

  * `[ ]`   `role`
    * `[ ]`   Contract suite library: a Solidity library of `contracts/evm`, compiled into each contract that calls it, whose boundary is crossed by that contract's integration test; the first library of the suite and the arithmetic layer `contracts/evm/DeliveryVerifier` calls
    * `[ ]`   Owns its custom errors `EncodingLength`, `InputLength`, `ScalarNotCanonical`, and `PrecompileFailed`, its private constants, and its private helpers, all declared in the library body
    * `[ ]`   Has no interface test, interface, mock, guard test, or guard element, and no provides file: consumers import `PairingLib` by name from `PairingLib.sol` and reach `PairingLib.<member>`
    * `[ ]`   Does not verify a delivery proof, hash, recompute a transcript, map a scalar, hold a generator or a verifier form of its own, or deploy anything; the generators, the scalar field orders, and the verifier forms are the generated constants libraries', and the verifier form each curve declares is `Bn254Constants.SECOND_GROUP_ARITHMETIC` false and `Bls12381Constants.SECOND_GROUP_ARITHMETIC` true
    * `[ ]`   Does not carry a commit; the commit of the grouping sits on `contracts/evm/deploy`

  * `[ ]`   `module`
    * `[ ]`   Bounded context: `contracts/evm`, holding `src/PairingLib.sol`, `src/PairingLib.interaction.spec.md`, and `test/PairingLib.t.sol`, beside the interface, foundry configuration, and workflow `contracts/evm/IDeliveryVerifier` created, and reading the generated directory `contracts/evm/generated`
    * `[ ]`   Files: `contracts/evm/src/PairingLib.interaction.spec.md`, `contracts/evm/test/PairingLib.t.sol`, and `contracts/evm/src/PairingLib.sol`
    * `[ ]`   Outside: the delivery verifier and its constants, the proof vectors, the hash-to-scalar, mapping, possession, and challenge vectors, the generator, the deployment, and every Rust file

  * `[ ]`   `deps`
    * `[ ]`   `contracts/evm/IDeliveryVerifier`, the contract suite's first node, direction inward: `contracts/evm/foundry.toml`, whose `solc` is `0.8.38`, `evm_version` is `prague`, `via_ir` is true, `ffi` is false, and `fs_permissions` grants read access to `generated` and `abi` alone, and `.github/workflows/contracts.yml`, whose `forge build`, `forge fmt --check`, and `forge test` build and prove this node; the suite's `// SPDX-License-Identifier: UNLICENSED` and `pragma solidity ^0.8.38;` opening
    * `[ ]`   `forge-std`, a git submodule at `contracts/evm/lib/forge-std`, a prerequisite the user installs: `Test`, `vm.readLine`, `vm.closeFile`, `vm.parseBytes`, `vm.expectRevert`, `vm.assume`, `bound`, and the `allow_internal_expect_revert` inline configuration
    * `[ ]`   `harness-crypto/generate/factory`, through `harness-crypto/generate/solidity`, whose emitted directory `contracts/evm/generated` the harness binary writes: `generated/bn254/Constants.sol` declaring `library Bn254Constants` and `generated/bls12_381/Constants.sol` declaring `library Bls12381Constants`, each with the entries `SCALAR_FIELD_ORDER` as `uint256`, `G1_GENERATOR` and `G2_GENERATOR` as `bytes`, and `SECOND_GROUP_ARITHMETIC` as `bool`, as `harness-crypto/generate/solidity/constants` authors them and `harness-crypto/config`'s `[generate]` defaults name them; `generated/bn254/VectorRecords.sol` declaring `library Bn254VectorRecords` and `generated/bls12_381/VectorRecords.sol` declaring `library Bls12381VectorRecords`, each holding, per vector kind, the struct `<Kind>Vector`, the function `decode<Kind>(bytes memory line)`, and the string constant `<KIND>_PATH`, as `render` rules them and the `[generate.kinds.<Kind>]` defaults name them; and the vector files at those paths, one encoded record per line
    * `[ ]`   The vector kinds this node reads, with the records `harness-crypto/generate/solidity/group_vectors` and `rejection_vectors` author: `G1Addition`, `G1Multiplication`, and `PairingCheck` on both curves, with the members `draws`, `input`, and `output`, `output` a `bytes32` on `PairingCheck` and `bytes` otherwise; `G2Addition`, `G1MultiScalarMultiplication`, and `G2MultiScalarMultiplication` on BLS12-381 alone, where the pairing declares second-group arithmetic, with the same members; and `DecodeRejection` on both curves, with the members `target`, a `string` holding `G1`, `G2`, or `Scalar`, and `input`, `bytes`, one record for the first group's outside-the-subgroup encoding where that group is not the whole curve, one for the second group's, and one for the scalar field's order
    * `[ ]`   EIP-196, EIP-197, and EIP-2537, external standards: BN254 addition at address `0x06` taking 128 bytes and returning 64, multiplication at `0x07` taking 96 and returning 64, both verifying every coordinate below the field modulus and the point on the curve and neither bounding the scalar; the pairing check at `0x08` taking a multiple of 192 bytes, a first-group point of 64 bytes and a second-group point of 128 bytes per pair, the imaginary coefficient first, returning a 32-byte word of zero or one, failing on a point not on its curve and on a second-group point outside the subgroup, and returning one for an empty input; BLS12-381 first-group addition at `0x0b` taking 256 bytes and returning 128 and second-group addition at `0x0d` taking 512 and returning 256, neither checking the subgroup; first-group multi-scalar multiplication at `0x0c` taking 160 bytes per term and second-group at `0x0e` taking 288, each checking the subgroup and failing on empty input, a scalar being 32 bytes big-endian with no bound; the pairing check at `0x0f` taking 384 bytes per pair, checking the subgroup of every point, failing on empty input, returning a 32-byte word of zero or one; the point at infinity encoded as all zero bytes; every failing call burning the gas it was given
    * `[ ]`   Foundry, the Foundry Book's cheatcode reference and configuration reference: `readLine` returns the next line of the file with its line feed removed and an empty string at the end of the file, `closeFile` resets the offset so the next `readLine` starts at the beginning, `expectRevert(bytes)` requires the full revert data and applies to a call at a lower depth than the cheatcode unless the test carries `/// forge-config: default.allow_internal_expect_revert = true`
    * `[ ]`   No reverse dependency: nothing in the generated directory, the foundry configuration, or the harness imports this library

  * `[ ]`   `context_slice`
    * `[ ]`   From `Bn254Constants` and `Bls12381Constants`: `SCALAR_FIELD_ORDER`, `uint256 internal constant`; `G1_GENERATOR` and `G2_GENERATOR`, `bytes internal constant`, the precompile encodings of the generators, 64 and 128 bytes on BN254 and 128 and 256 on BLS12-381; `SECOND_GROUP_ARITHMETIC`, `bool internal constant`
    * `[ ]`   From `Bn254VectorRecords` and `Bls12381VectorRecords`: for each kind the test reads, `<Kind>Vector` with its members in the order above, `decode<Kind>(bytes memory line) internal pure returns (<Kind>Vector memory)`, and `<KIND>_PATH`, `string internal constant`, the path relative to `contracts/evm`; the kinds' names and their constants' names are `G1Addition` with `decodeG1Addition` and `G1_ADDITION_PATH`, `G1Multiplication` with `decodeG1Multiplication` and `G1_MULTIPLICATION_PATH`, `PairingCheck` with `decodePairingCheck` and `PAIRING_CHECK_PATH`, `G2Addition` with `decodeG2Addition` and `G2_ADDITION_PATH`, `G1MultiScalarMultiplication` with `decodeG1MultiScalarMultiplication` and `G1_MULTI_SCALAR_MULTIPLICATION_PATH`, `G2MultiScalarMultiplication` with `decodeG2MultiScalarMultiplication` and `G2_MULTI_SCALAR_MULTIPLICATION_PATH`, and `DecodeRejection` with `decodeDecodeRejection` and `DECODE_REJECTION_PATH`
    * `[ ]`   From Solidity: `staticcall`, `abi.encodePacked`, `abi.decode`, `abi.encodeWithSelector`, inline assembly `mload` and `mcopy`, `addmod`, and custom errors with arguments
    * `[ ]`   From `forge-std`: `Test`, `vm.readLine(string) returns (string)`, `vm.closeFile(string)`, `vm.parseBytes(string) returns (bytes)`, `vm.expectRevert(bytes)`, `bound(uint256, uint256, uint256) returns (uint256)`, and `vm.assume(bool)`

  * `[ ]`   `contracts/evm/src/PairingLib.interaction.spec.md`
    * `[ ]`   `PairingLib` is a library of `internal` functions; each takes the precompile's input as one `bytes memory` and the conditions below are selected by the input, never by state
    * `[ ]`   Fixed-size operand: condition the input's length differs from the operation's expected length, `bn254AddG1` 128, `bn254MulG1` 96, `bn254CheckG2` 128, `bls12381AddG1` 256, `bls12381AddG2` 512, `bls12381MulG1` 160, `bls12381CheckG1` 128, and `bls12381CheckG2` 256; decision length equality, before any other step; dependency call none; outcome revert `EncodingLength(expected, actual)`
    * `[ ]`   Variable input: condition the input of `bn254PairingCheck`, `bls12381MsmG1`, `bls12381MsmG2`, or `bls12381PairingCheck` is empty or its length is not a multiple of its unit, 192, 160, 288, and 384; decision a zero length or a nonzero remainder, before any other step; dependency call none; outcome revert `InputLength(unit, actual)`
    * `[ ]`   Scalar: condition a scalar of `bn254MulG1`, `bls12381MulG1`, `bls12381MsmG1`, `bls12381MsmG2`, `bn254CheckScalar`, or `bls12381CheckScalar` is not below its curve's `SCALAR_FIELD_ORDER`; decision comparison, after the length decision and before the precompile call, the scalars of a multi-scalar input read in term order and the first refused reverted; dependency call none; outcome revert `ScalarNotCanonical(scalar)`
    * `[ ]`   Subgroup, BLS12-381 addition: condition `bls12381AddG1` or `bls12381AddG2` is called with an operand outside the subgroup; decision the operand's membership, each operand checked through `bls12381CheckG1` or `bls12381CheckG2` in operand order after the length decision and before the addition; dependency call the multi-scalar multiplication precompile with the operand and the scalar one, which checks the subgroup; outcome revert `PrecompileFailed(0x0c)` for a first-group operand and `PrecompileFailed(0x0e)` for a second-group operand
    * `[ ]`   Membership checks: `bls12381CheckG1` calls `0x0c` and `bls12381CheckG2` calls `0x0e` over the point and the scalar one, returning on success; `bn254CheckG2` calls `0x08` over the first-group point at infinity, 64 zero bytes, followed by the point, returning on success
    * `[ ]`   Precompile call: condition the length and scalar decisions passed; dependency call `staticcall` of the operation's precompile over the input, `0x06`, `0x07`, `0x08` on BN254 and `0x0b`, `0x0c`, `0x0d`, `0x0e`, `0x0f` on BLS12-381; decision the call's success flag; outcome revert `PrecompileFailed(precompile)` on failure, otherwise the returned bytes of the add, mul, and multi-scalar operations and, for the pairing checks, the returned word equal to one
    * `[ ]`   Multiplication on BLS12-381 is the multi-scalar multiplication at `0x0c` over one term; the library holds no multiplication precompile for it
    * `[ ]`   Ordering: the length decision, then the scalar decision, then the subgroup decisions, then the call; the first refusal is returned; no state is read or written
    * `[ ]`   The boundary is crossed by the integration test of `contracts/evm/DeliveryVerifier`, the contract that calls the library; the Rust integration elements do not apply to a library

  * `[ ]`   `contracts/evm/test/PairingLib.t.sol`
    * `[ ]`   Opens with `// SPDX-License-Identifier: UNLICENSED` and `pragma solidity ^0.8.38;`; imports `Test` from `forge-std/Test.sol`, `PairingLib` from `../src/PairingLib.sol`, `Bn254Constants` from `../generated/bn254/Constants.sol`, `Bn254VectorRecords` from `../generated/bn254/VectorRecords.sol`, `Bls12381Constants` from `../generated/bls12_381/Constants.sol`, and `Bls12381VectorRecords` from `../generated/bls12_381/VectorRecords.sol`, nothing else; declares `contract PairingLibTest is Test` holding the blocks below and no helper function, no builder, and no state
    * `[ ]`   Every block is a `test_` or `testFuzz_` function with the full form of the unit scope, the four-field `Contract`, `Arrange`, `Act`, and `Assert` header above it and the inline `// Arrange`, `// Act`, and `// Assert` markers, and ends with a comment stating its name; expected values are independent literals, the vector files' outputs, or the arithmetic stated in the block, never read from the library
    * `[ ]`   A block that reads a vector kind inlines the read in its body: `vm.closeFile(path)`, then a loop of `vm.readLine(path)` until it returns the empty string, `vm.parseBytes(line)`, the curve's `decode<Kind>` over the parsed bytes, the assertion for the record, and `vm.closeFile(path)` after the loop; the path is the curve's `<KIND>_PATH` constant, the block holds no count, no file name, and no kind list, and fails when the file holds no record
    * `[ ]`   A block that asserts a revert from an `internal` call carries `/// forge-config: default.allow_internal_expect_revert = true` above it and calls `vm.expectRevert(abi.encodeWithSelector(...))` with the error's full data immediately before the single failing call; a block makes one failing precompile call, because a failing precompile burns the gas it was given and a second failing call in the same block would have too little left
    * `[ ]`   `test_Bn254AddG1MatchesEveryVector`
      * `[ ]`   Contract: `bn254AddG1` over each `G1Addition` record's `input` returns the record's `output`
      * `[ ]`   Arrange: every `G1Addition` record of `Bn254VectorRecords`, whose inputs differ from each other so a result relayed from a constant fails
      * `[ ]`   Act: `PairingLib.bn254AddG1(record.input)` per record
      * `[ ]`   Assert: the returned bytes equal `record.output`, and at least one record was read
    * `[ ]`   `test_Bn254MulG1MatchesEveryVector`
      * `[ ]`   Contract: `bn254MulG1` over each `G1Multiplication` record's `input` returns the record's `output`
      * `[ ]`   Arrange: every `G1Multiplication` record of `Bn254VectorRecords`
      * `[ ]`   Act: `PairingLib.bn254MulG1(record.input)` per record
      * `[ ]`   Assert: the returned bytes equal `record.output`, and at least one record was read
    * `[ ]`   `test_Bn254PairingCheckMatchesEveryVector`
      * `[ ]`   Contract: `bn254PairingCheck` over each `PairingCheck` record's `input` returns true where the record's `output` word is one and false where it is zero
      * Arrange: every `PairingCheck` record of `Bn254VectorRecords`, which holds a passing product at even indices and a failing product at odd indices, so a constant verdict fails
      * `[ ]`   Act: `PairingLib.bn254PairingCheck(record.input)` per record
      * `[ ]`   Assert: the returned bool equals `uint256(record.output) == 1`, and both verdicts were seen across the records
    * `[ ]`   `test_Bn254RefusesTheG2CandidateOutsideTheSubgroup`
      * `[ ]`   Contract: `bn254CheckG2` over the `DecodeRejection` record whose `target` is `G2` reverts with `PrecompileFailed(0x08)`
      * `[ ]`   Arrange: the `DecodeRejection` records of `Bn254VectorRecords`, the one whose `target` hashes equal to `G2`
      * `[ ]`   Act: `PairingLib.bn254CheckG2(record.input)` after `vm.expectRevert` of `PrecompileFailed` with `address(0x08)`
      * `[ ]`   Assert: the call reverts with exactly that error data, and a `G2` record was found
    * `[ ]`   `test_Bn254RefusesTheNonCanonicalScalarCandidate`
      * `[ ]`   Contract: `bn254CheckScalar` over the `DecodeRejection` record whose `target` is `Scalar` reverts with `ScalarNotCanonical` carrying the scalar field's order
      * `[ ]`   Arrange: the `Scalar` record of `Bn254VectorRecords`, whose `input` is the order as 32 bytes
      * `[ ]`   Act: `PairingLib.bn254CheckScalar(abi.decode(record.input, (uint256)))` after `vm.expectRevert`
      * `[ ]`   Assert: the revert data equals `abi.encodeWithSelector(PairingLib.ScalarNotCanonical.selector, Bn254Constants.SCALAR_FIELD_ORDER)`, and a `Scalar` record was found
    * `[ ]`   `test_Bn254HoldsNoG1Candidate`
      * `[ ]`   Contract: BN254's first group is the whole curve, so the `DecodeRejection` records of `Bn254VectorRecords` hold no record whose `target` is `G1`
      * `[ ]`   Arrange: every `DecodeRejection` record of `Bn254VectorRecords`
      * `[ ]`   Act: the read loop
      * `[ ]`   Assert: no record's `target` equals `G1`
    * `[ ]`   `test_Bn254FormHasNoSecondGroupArithmetic`
      * `[ ]`   Contract: the BN254 form declares no second-group arithmetic at the verifier, which EIP-196 and EIP-197 expose no precompile for
      * `[ ]`   Arrange: `Bn254Constants.SECOND_GROUP_ARITHMETIC` and `Bls12381Constants.SECOND_GROUP_ARITHMETIC`
      * `[ ]`   Act: the two reads
      * `[ ]`   Assert: the first is false and the second is true
    * `[ ]`   `testFuzz_Bn254ScalarMultiplesAddAsScalars`
      * `[ ]`   Contract: for scalars `s` and `t` below the order, the sum of `s` and `t` times the generator equals `bn254MulG1` of the generator and `addmod(s, t, order)`
      * `[ ]`   Arrange: `s` and `t` bounded below `Bn254Constants.SCALAR_FIELD_ORDER`, the generator `Bn254Constants.G1_GENERATOR`
      * `[ ]`   Act: `bn254MulG1` for `s` and for `t`, `bn254AddG1` over the two products, and `bn254MulG1` over the generator and `addmod(s, t, order)`
      * `[ ]`   Assert: the sum equals the single product
    * `[ ]`   `test_Bn254NegatedGeneratorSumsToInfinity`
      * `[ ]`   Contract: the generator plus the generator times the order minus one is the point at infinity
      * `[ ]`   Arrange: the generator and the scalar `order - 1`
      * `[ ]`   Act: `bn254MulG1` then `bn254AddG1`
      * `[ ]`   Assert: the sum equals 64 zero bytes
    * `[ ]`   `test_Bn254PairingCheckAcceptsAProductOfOne`
      * `[ ]`   Contract: the pairing of the generators times the pairing of the negated first-group generator and the second-group generator is one
      * `[ ]`   Arrange: the generator pair followed by the negated generator pair
      * `[ ]`   Act: `bn254PairingCheck`
      * `[ ]`   Assert: true
    * `[ ]`   `test_Bn254PairingCheckRefusesAGeneratorPair`
      * `[ ]`   Contract: the pairing of the generators alone is not one
      * `[ ]`   Arrange: the generator pair alone
      * `[ ]`   Act: `bn254PairingCheck`
      * `[ ]`   Assert: false
    * `[ ]`   `test_Bn254AddG1RefusesAWrongLength`, `test_Bn254MulG1RefusesAWrongLength`, and `test_Bn254CheckG2RefusesAWrongLength`
      * `[ ]`   Contract: each reverts with `EncodingLength` carrying 128 and 127, 96 and 95, and 128 and 129
      * `[ ]`   Arrange: `new bytes(127)`, `new bytes(95)`, and `new bytes(129)`
      * `[ ]`   Act: the function after `vm.expectRevert`
      * `[ ]`   Assert: the revert data equals the encoded error
    * `[ ]`   `test_Bn254PairingCheckRefusesAnInputThatIsNotAPositiveMultiple`
      * `[ ]`   Contract: an empty input and an input of 191 bytes each revert with `InputLength` carrying 192 and the length
      * `[ ]`   Arrange: `new bytes(0)` and `new bytes(191)`
      * `[ ]`   Act: `bn254PairingCheck` after `vm.expectRevert`, once per input, the failing calls being the library's length decision, which makes no precompile call
      * `[ ]`   Assert: each revert data equals `abi.encodeWithSelector(PairingLib.InputLength.selector, 192, length)`
    * `[ ]`   `test_Bn254MulG1RefusesANonCanonicalScalar`
      * `[ ]`   Contract: `bn254MulG1` over a 96-byte input whose last word is the order reverts with `ScalarNotCanonical` carrying the order
      * `[ ]`   Arrange: the generator followed by the order, and the order plus one beside it
      * `[ ]`   Act: `bn254MulG1` after `vm.expectRevert`, once per input
      * `[ ]`   Assert: each revert data equals the encoded error with that scalar
    * `[ ]`   `test_Bls12381AddG1MatchesEveryVector`, `test_Bls12381MulG1MatchesEveryVector`, `test_Bls12381AddG2MatchesEveryVector`, `test_Bls12381MsmG1MatchesEveryVector`, and `test_Bls12381MsmG2MatchesEveryVector`
      * `[ ]`   Contract: each library function over its kind's record `input` returns the record's `output`: `bls12381AddG1` over `G1Addition`, `bls12381MulG1` over `G1Multiplication`, `bls12381AddG2` over `G2Addition`, `bls12381MsmG1` over `G1MultiScalarMultiplication`, and `bls12381MsmG2` over `G2MultiScalarMultiplication`
      * `[ ]`   Arrange: every record of that kind of `Bls12381VectorRecords`, the multi-scalar records holding several terms each
      * `[ ]`   Act: the function over `record.input` per record
      * `[ ]`   Assert: the returned bytes equal `record.output`, and at least one record was read
    * `[ ]`   `test_Bls12381PairingCheckMatchesEveryVector`
      * `[ ]`   Contract: `bls12381PairingCheck` over each `PairingCheck` record's `input` returns true where the record's `output` word is one and false where it is zero
      * `[ ]`   Arrange: every `PairingCheck` record of `Bls12381VectorRecords`, holding both verdicts
      * `[ ]`   Act: `PairingLib.bls12381PairingCheck(record.input)` per record
      * `[ ]`   Assert: the returned bool equals `uint256(record.output) == 1`, and both verdicts were seen
    * `[ ]`   `test_Bls12381RefusesTheG1CandidateOutsideTheSubgroup`, `test_Bls12381RefusesTheG2CandidateOutsideTheSubgroup`, and `test_Bls12381RefusesTheNonCanonicalScalarCandidate`
      * `[ ]`   Contract: `bls12381CheckG1` over the `G1` record reverts with `PrecompileFailed(0x0c)`, `bls12381CheckG2` over the `G2` record with `PrecompileFailed(0x0e)`, and `bls12381CheckScalar` over the `Scalar` record with `ScalarNotCanonical` carrying `Bls12381Constants.SCALAR_FIELD_ORDER`
      * `[ ]`   Arrange: the `DecodeRejection` record of `Bls12381VectorRecords` whose `target` is that text, each block one record and one failing call
      * `[ ]`   Act: the function over `record.input` after `vm.expectRevert`
      * `[ ]`   Assert: the revert data equals the encoded error, and the record was found
    * `[ ]`   `test_Bls12381AddG1RefusesAnOperandOutsideTheSubgroup` and `test_Bls12381AddG2RefusesAnOperandOutsideTheSubgroup`
      * `[ ]`   Contract: the addition refuses a first operand outside the subgroup with the membership check's error, which the addition precompile alone would not
      * `[ ]`   Arrange: the `G1` or `G2` rejection record as the first operand followed by the generator
      * `[ ]`   Act: `bls12381AddG1` or `bls12381AddG2` after `vm.expectRevert`
      * `[ ]`   Assert: the revert data equals `PrecompileFailed` with `address(0x0c)` or `address(0x0e)`
    * `[ ]`   `testFuzz_Bls12381ScalarMultiplesAddAsScalars`
      * `[ ]`   Contract: for scalars `s` and `t` below the order, the sum of the products of the generator equals the product for `addmod(s, t, order)`
      * `[ ]`   Arrange: `s` and `t` bounded below `Bls12381Constants.SCALAR_FIELD_ORDER`, the generator `Bls12381Constants.G1_GENERATOR`
      * `[ ]`   Act: `bls12381MulG1` for `s`, `t`, and their sum, and `bls12381AddG1` over the first two
      * `[ ]`   Assert: the sum equals the third product
    * `[ ]`   `test_Bls12381G2AdditionOfTheGeneratorEqualsItsDouble`
      * `[ ]`   Contract: the second-group generator added to itself equals the multi-scalar product of the generator and the scalar two
      * `[ ]`   Arrange: `Bls12381Constants.G2_GENERATOR` twice, and the generator with the word two
      * `[ ]`   Act: `bls12381AddG2` and `bls12381MsmG2`
      * `[ ]`   Assert: the two results are equal
    * `[ ]`   `test_Bls12381NegatedGeneratorSumsToInfinity`
      * `[ ]`   Contract: the generator plus the generator times the order minus one is the point at infinity, 128 zero bytes
      * `[ ]`   Arrange: the generator and the scalar `order - 1`
      * `[ ]`   Act: `bls12381MulG1` then `bls12381AddG1`
      * `[ ]`   Assert: the sum equals 128 zero bytes
    * `[ ]`   `test_Bls12381PairingCheckAcceptsAProductOfOne` and `test_Bls12381PairingCheckRefusesAGeneratorPair`
      * `[ ]`   Contract: the generators' pairing times the pairing of the negated first-group generator and the second-group generator is one, and the generators' pairing alone is not
      * `[ ]`   Arrange: the generator pair followed by the negated generator pair, and the generator pair alone
      * `[ ]`   Act: `bls12381PairingCheck`
      * `[ ]`   Assert: true, then false
    * `[ ]`   `test_Bls12381AddG1RefusesAWrongLength`, `test_Bls12381AddG2RefusesAWrongLength`, `test_Bls12381MulG1RefusesAWrongLength`, `test_Bls12381CheckG1RefusesAWrongLength`, and `test_Bls12381CheckG2RefusesAWrongLength`
      * `[ ]`   Contract: each reverts with `EncodingLength` carrying its expected length, 256, 512, 160, 128, and 256, and the length given
      * `[ ]`   Arrange: an input one byte shorter than the expected length
      * `[ ]`   Act: the function after `vm.expectRevert`
      * `[ ]`   Assert: the revert data equals the encoded error
    * `[ ]`   `test_Bls12381MsmG1RefusesAnInputThatIsNotAPositiveMultiple`, `test_Bls12381MsmG2RefusesAnInputThatIsNotAPositiveMultiple`, and `test_Bls12381PairingCheckRefusesAnInputThatIsNotAPositiveMultiple`
      * `[ ]`   Contract: an empty input and an input one byte short of its unit each revert with `InputLength` carrying the unit, 160, 288, and 384, and the length
      * `[ ]`   Arrange: `new bytes(0)` and an input one byte short of the unit
      * `[ ]`   Act: the function after `vm.expectRevert`, once per input
      * `[ ]`   Assert: each revert data equals the encoded error
    * `[ ]`   `test_Bls12381MulG1RefusesANonCanonicalScalar`, `test_Bls12381MsmG1RefusesANonCanonicalScalarInTheSecondTerm`, and `test_Bls12381MsmG2RefusesANonCanonicalScalarInTheSecondTerm`
      * `[ ]`   Contract: a scalar equal to the order reverts with `ScalarNotCanonical` carrying it, the multi-scalar blocks placing it in the second term after a canonical first term
      * `[ ]`   Arrange: the generator and the scalar one, then the generator and the order
      * `[ ]`   Act: the function after `vm.expectRevert`
      * `[ ]`   Assert: the revert data equals the encoded error with the order
    * `[ ]`   Until `contracts/evm/src/PairingLib.sol` exists, `forge build` reports the unresolved import, which is the RED state for the implementation

  * `[ ]`   `contracts/evm/src/PairingLib.sol`
    * `[ ]`   Opens with `// SPDX-License-Identifier: UNLICENSED` and `pragma solidity ^0.8.38;`; imports `Bn254Constants` from `../generated/bn254/Constants.sol` and `Bls12381Constants` from `../generated/bls12_381/Constants.sol`; declares `library PairingLib` and nothing else
    * `[ ]`   The custom errors `EncodingLength(uint256 expected, uint256 actual)`, `InputLength(uint256 unit, uint256 actual)`, `ScalarNotCanonical(uint256 scalar)`, and `PrecompileFailed(address precompile)`
    * `[ ]`   Private constants, each named for the precompile or length it holds, with its source in the comment above it: the addresses `0x06`, `0x07`, and `0x08` of EIP-196 and EIP-197 and `0x0b`, `0x0c`, `0x0d`, `0x0e`, and `0x0f` of EIP-2537; the lengths 64 and 128 for a BN254 first-group and second-group point, 96 for a multiplication input, and 192 for a pair; 128 and 256 for a BLS12-381 first-group and second-group point, 160 and 288 for a multi-scalar term, 384 for a pair; and 32 for a scalar
    * `[ ]`   `internal view` functions `bn254AddG1`, `bn254MulG1`, `bn254PairingCheck`, `bn254CheckG2`, `bls12381AddG1`, `bls12381AddG2`, `bls12381MulG1`, `bls12381MsmG1`, `bls12381MsmG2`, `bls12381PairingCheck`, `bls12381CheckG1`, and `bls12381CheckG2`, each `bytes memory input` in and `bytes memory` out, or `bool` for the pairing checks and nothing for the checks; and `internal pure` functions `bn254CheckScalar(uint256)` and `bls12381CheckScalar(uint256)`
    * `[ ]`   Each function follows the interaction spec's branches in their order; the scalar of a multiplication or a term is read from its word of the input with inline assembly `mload`, the operands of an addition are copied with a private `pure` helper using `mcopy`, and the precompile call is one private `view` helper doing `staticcall` and reverting with `PrecompileFailed` when the call does not succeed
    * `[ ]`   The BLS12-381 multiplication calls the multi-scalar multiplication address over its one term, and each membership check on BLS12-381 calls the multi-scalar multiplication address over the point and the scalar one
    * `[ ]`   Holds no storage, no event, no external call but `staticcall`, and no value taken from a parameter that a generated constants library declares; introduces no undeclared dependency and bypasses no check

  * `[ ]`   `directionality`
    * `[ ]`   The library imports the two generated constants libraries and nothing else; `contracts/evm/DeliveryVerifier` imports the library by name from `PairingLib.sol`; no cycle
    * `[ ]`   The generated directory is a runtime input the harness binary writes, held in the dependency map's Runtime sequence, and orders no ticket; the library's tests read the committed vector files through `forge-std`'s file cheatcodes under the read grant `contracts/evm/foundry.toml` carries

  * `[ ]`   `requirements`
    * `[ ]`   `forge build`, `forge fmt --check`, and `forge test` complete without error in `contracts/evm` (CR-10 on chain)
    * `[ ]`   Every vector block passes: each operation's result equals its record's output on both curves, the multi-scalar and second-group blocks on BLS12-381 alone, and the pairing-check verdicts match both outcomes
    * `[ ]`   Every rejection block passes: each candidate of the decode-rejection vectors is refused with its error and the BN254 file holds no first-group candidate
    * `[ ]`   Every fuzz and identity block passes, the sums of scalar multiples, the negated generator summing to infinity, and the generators' pairing product
    * `[ ]`   Every length, input, and scalar refusal block passes with the error's full data

* `[ ]`   `contracts/evm/DeliveryVerifier`

* `[ ]`   `contracts/evm/deploy`
