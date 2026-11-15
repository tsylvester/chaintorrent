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

* `[✅]`   `contracts/evm/IDeliveryVerifier` **The contract suite's delivery verifier interface, stated once as the language-neutral ABI description and realized in Solidity, with its interface test, the suite's Foundry configuration, and the suite's continuous-integration definition; creates the `contracts/evm` suite**

  * `[✅]`   `objective`
    * `[✅]`   Problem: the delivery verifier's ABI is the boundary between the Rust verifier client and the deployed contract, so it is stated once in a description the client binds and the contract implements, and the toolchain every contract ticket builds under is authored once and complete (CD-03 interface; the dependency map's `contracts/evm/IDeliveryVerifier` row; the technical requirements' `contracts/evm` file tree; the Delivery proof and on-chain verifier milestone)
    * `[✅]`   Functional: `contracts/evm/abi/IDeliveryVerifier.json` declares `verifyMint` and `verifyTransfer`, each a `view` function over one statement struct and one proof struct returning `bool accepted`; every struct member carries the name and order of the proof vector records' members, a group element and the second-group first messages being `bytes` so one interface serves every curve and both verifier forms
    * `[✅]`   Functional: `contracts/evm/src/IDeliveryVerifier.sol` realizes that interface in Solidity with the same functions, structs, members, and types
    * `[✅]`   Functional: the interface test proves the Solidity realization against independently written selector texts and proves the ABI description against the same texts, member by member, by reading the description
    * `[✅]`   Functional: `contracts/evm/foundry.toml` and `.github/workflows/contracts.yml` are created here, each authored once and complete, the suite compiling through the IR pipeline with `ffi` disabled, read access to the generated directory and the ABI description directory, and write access to the deployments directory, and to nothing else
    * `[✅]`   Non-functional: the interface is stated in one description and one realization, neither copied into another file; the interface owns the builders and mock that stand in for it

  * `[✅]`   `role`
    * `[✅]`   Contract suite: the suite's first node, which creates `contracts/evm` and its toolchain and owns the interface that `contracts/evm/DeliveryVerifier` implements and `harness-crypto/verifier/evm` binds
    * `[✅]`   Does not implement the interface, bind it from Rust, generate constants or vectors, or deploy anything
    * `[✅]`   Does not carry a commit

  * `[✅]`   `module`
    * `[✅]`   Bounded context: `contracts/evm`, holding `abi/IDeliveryVerifier.json`, `src/IDeliveryVerifier.sol`, `test/DeliveryVerifier.interface.t.sol`, `test/IDeliveryVerifierMock.sol`, and `foundry.toml`, and the workflow `.github/workflows/contracts.yml`
    * `[✅]`   Files: `contracts/evm/test/DeliveryVerifier.interface.t.sol`, `contracts/evm/abi/IDeliveryVerifier.json`, `contracts/evm/src/IDeliveryVerifier.sol`, `contracts/evm/test/IDeliveryVerifierMock.sol`, `contracts/evm/foundry.toml`, and `.github/workflows/contracts.yml`
    * `[✅]`   Outside: the contract's implementation, the pairing library, the generated constants and vectors, the deployment and its record, and the Rust binding of the description

  * `[✅]`   `deps`
    * `[✅]`   `workspace/ci`, `.github/workflows/rust.yml`, whose triggers, permissions, and concurrency `contracts.yml` repeats; direction inward, the workflow reads nothing in the suite
    * `[✅]`   `forge-std`, a git submodule at `contracts/evm/lib/forge-std`, which `contracts/evm/test/DeliveryVerifier.interface.t.sol` imports `Test` from; it is a prerequisite the user installs, nothing in this node creates it
    * `[✅]`   `actions/checkout@v7` and `foundry-rs/foundry-toolchain@v1`, external actions, the workflow's checkout and Foundry installation
    * `[✅]`   Foundry, the toolchain the suite builds and tests under: `forge build`, `forge fmt --check`, and `forge test`

  * `[✅]`   `context_slice`
    * `[✅]`   From `forge-std`: the `Test` base contract
    * `[✅]`   From Solidity: `abi.encode`, `keccak256`, `bytes4`, and a function's `.selector`

  * `[✅]`   `contracts/evm/test/DeliveryVerifier.interface.t.sol`
    * `[✅]`   Opens with `// SPDX-License-Identifier: UNLICENSED` and `pragma solidity ^0.8.38;`; imports `Test` from `forge-std/Test.sol` and `IDeliveryVerifier` from `../src/IDeliveryVerifier.sol`, nothing else; declares `contract DeliveryVerifierInterfaceTest is Test` holding only the blocks below
    * `[✅]`   Every block is a `test_` function over typed literals that name every member of the type they prove, never a builder, a mock, or a helper, and carries the one-line `/// Contract:` header the interface scope collapses to; the blocks are the enumeration of the interface's export surface, `IDeliveryVerifier` through its functions and structs, each proven by one block, and the ABI description proven by the last two
    * `[✅]`   `test_DeliveryContextMembers`: contract `DeliveryContext` has exactly the members `suiteIdentifier`, `suiteVersion`, `chain`, `entitlementContract`, `assetRoot`, `parameterSet`, `sourceEntitlement`, `targetEntitlement`, `oldInterval`, `newInterval`, `purpose`, `seller`, `buyer`, and `expiry`; the block assigns an `IDeliveryVerifier.DeliveryContext memory` literal naming all fourteen and asserts `abi.encode(context).length` equals `14 * 32`, the literal a static struct of fourteen words encodes to, so a member added or dropped changes the length
    * `[✅]`   `test_MintStatementMembers`: contract `MintStatement` has exactly the members `context`, `hpub`, `identityElement`, `buyerPk1`, `buyerPk2`, `envelopeC1`, `envelopeC2`, `envelopeD1`, and `envelopeD2`; the literal names all nine, the eight `bytes` members empty, and the block asserts `abi.encode(statement).length` equals `31 * 32`, the offset word, the fourteen-word context, eight offsets, and eight empty-length words
    * `[✅]`   `test_MintProofMembers`: contract `MintProof` has exactly the members `challenge`, `alpha`, `r`, `rho`, `sigma`, and `secondGroupFirstMessages`; the literal names all six, the `bytes` member empty, and the block asserts `abi.encode(proof).length` equals `8 * 32`
    * `[✅]`   `test_TransferStatementMembers`: contract `TransferStatement` has exactly the members `context`, `identityElement`, `sellerPk1`, `sellerPk2`, `buyerPk1`, `buyerPk2`, `oldEnvelopeC1`, `oldEnvelopeC2`, `oldEnvelopeD1`, `oldEnvelopeD2`, `newEnvelopeC1`, `newEnvelopeC2`, `newEnvelopeD1`, and `newEnvelopeD2`; the literal names all fourteen, the thirteen `bytes` members empty, and the block asserts `abi.encode(statement).length` equals `41 * 32`
    * `[✅]`   `test_TransferProofMembers`: contract `TransferProof` has exactly the members `challenge`, `x`, `y`, `s`, `rho`, `sigma`, and `secondGroupFirstMessages`; the literal names all seven, the `bytes` member empty, and the block asserts `abi.encode(proof).length` equals `9 * 32`
    * `[✅]`   `test_VerifyMintSelector`: contract `verifyMint` takes one `MintStatement` and one `MintProof`, in that order, whose member types in order are those the selector's text states; the block asserts `IDeliveryVerifier.verifyMint.selector` equals `bytes4(keccak256("verifyMint((bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,uint64),bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes),(uint256,uint256,uint256,uint256,uint256,bytes))"))`, a text no mention of the interface produced, so a member of another type or another order changes the selector
    * `[✅]`   `test_VerifyTransferSelector`: contract `verifyTransfer` takes one `TransferStatement` and one `TransferProof`; the block asserts `IDeliveryVerifier.verifyTransfer.selector` equals `bytes4(keccak256("verifyTransfer((bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,uint64),bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes),(uint256,uint256,uint256,uint256,uint256,uint256,bytes))"))`
    * `[✅]`   `test_VerifyMintIsViewAndReturnsBool`: contract `verifyMint` is `external view` and returns one `bool`; the block assigns `IDeliveryVerifier(address(0)).verifyMint` to a variable of the type `function (IDeliveryVerifier.MintStatement calldata, IDeliveryVerifier.MintProof calldata) external view returns (bool)`, an assignment that compiles only if the declaration holds, and asserts nothing further, the assignment being the act
    * `[✅]`   `test_VerifyTransferIsViewAndReturnsBool`: contract `verifyTransfer` is `external view` and returns one `bool`; the same assignment over `IDeliveryVerifier(address(0)).verifyTransfer` and the transfer struct types
    * `[✅]`   `test_AbiDescriptionVerifyMint`: contract the entry of `abi/IDeliveryVerifier.json` named `verifyMint` states the interface's `verifyMint`; the block reads the file with `vm.readFile("abi/IDeliveryVerifier.json")` and reads each value with `vm.parseJsonString` under a JSONPath key, and asserts against string literals written in the block: the entry's `type` is `function`, its `name` is `verifyMint`, its `stateMutability` is `view`; its two inputs are named `statement` and `proof`, each of `type` `tuple`, with `internalType` `struct IDeliveryVerifier.MintStatement` and `struct IDeliveryVerifier.MintProof`; each tuple's `components` carry, in order, the member names and `type` texts that the selector text of `test_VerifyMintSelector` states, `context` a `tuple` whose components are the `DeliveryContext` members in order; its one output is named `accepted` and is of `type` `bool`
    * `[✅]`   `test_AbiDescriptionVerifyTransfer`: contract the entry named `verifyTransfer` states the interface's `verifyTransfer`; the same reads and literal assertions over the transfer entry, its inputs `statement` and `proof` with `internalType` `struct IDeliveryVerifier.TransferStatement` and `struct IDeliveryVerifier.TransferProof`, and the member names and `type` texts of `test_VerifyTransferSelector`'s selector text
    * `[✅]`   Until `contracts/evm/src/IDeliveryVerifier.sol` exists, `forge build` reports the unresolved import, which is the RED state for every element below that precedes it

  * `[✅]`   `contracts/evm/abi/IDeliveryVerifier.json`
    * `[✅]`   A JSON array of the function entries `verifyMint` and `verifyTransfer`, each with `stateMutability` `view`, a `statement` input and a `proof` input, each a `tuple`, and one output of type `bool` named `accepted`
    * `[✅]`   Each tuple's `components` carry the member names, order, and Solidity types the structs of `contracts/evm/src/IDeliveryVerifier.sol` declare, `context` a nested `tuple` of the `DeliveryContext` members, and each tuple's `internalType` names its struct as `struct IDeliveryVerifier.<Struct>`
    * `[✅]`   The signatures the entries state are the selector texts `test_VerifyMintSelector` and `test_VerifyTransferSelector` assert, proven by `test_AbiDescriptionVerifyMint` and `test_AbiDescriptionVerifyTransfer`

  * `[✅]`   `contracts/evm/src/IDeliveryVerifier.sol`
    * `[✅]`   Opens with `// SPDX-License-Identifier: UNLICENSED` and `pragma solidity ^0.8.38;`; declares `interface IDeliveryVerifier` and nothing else, no import, no comment beyond the license line
    * `[✅]`   `struct DeliveryContext` with the members, in order, `bytes32 suiteIdentifier`, `uint16 suiteVersion`, `uint256 chain`, `bytes20 entitlementContract`, `bytes32 assetRoot`, `bytes32 parameterSet`, `uint256 sourceEntitlement`, `uint256 targetEntitlement`, `uint64 oldInterval`, `uint64 newInterval`, `uint16 purpose`, `bytes20 seller`, `bytes20 buyer`, and `uint64 expiry`, the kinds the encoding family's canonical fields name, an identity as `bytes20` and never `address`, the identity the encoding concrete encodes
    * `[✅]`   `struct MintStatement` with `DeliveryContext context` and the `bytes` members `hpub`, `identityElement`, `buyerPk1`, `buyerPk2`, `envelopeC1`, `envelopeC2`, `envelopeD1`, and `envelopeD2`, each a group element in the precompile encoding the resolved pairing declares, the member names and order those of the mint proof vector record; `struct MintProof` with `uint256 challenge`, `uint256 alpha`, `uint256 r`, `uint256 rho`, `uint256 sigma`, and `bytes secondGroupFirstMessages`, the three second-group first messages concatenated in the order `hpub`, `d1`, `d2` and empty under the both-groups form
    * `[✅]`   `struct TransferStatement` with `DeliveryContext context` and the `bytes` members `identityElement`, `sellerPk1`, `sellerPk2`, `buyerPk1`, `buyerPk2`, `oldEnvelopeC1`, `oldEnvelopeC2`, `oldEnvelopeD1`, `oldEnvelopeD2`, `newEnvelopeC1`, `newEnvelopeC2`, `newEnvelopeD1`, and `newEnvelopeD2`; `struct TransferProof` with `uint256 challenge`, `uint256 x`, `uint256 y`, `uint256 s`, `uint256 rho`, `uint256 sigma`, and `bytes secondGroupFirstMessages`, the three in the order `pk2`, `d1`, `d2`
    * `[✅]`   `function verifyMint(MintStatement calldata statement, MintProof calldata proof) external view returns (bool accepted);` and `function verifyTransfer(TransferStatement calldata statement, TransferProof calldata proof) external view returns (bool accepted);`

  * `[✅]`   `contracts/evm/test/IDeliveryVerifierMock.sol`
    * `[✅]`   Opens with `// SPDX-License-Identifier: UNLICENSED` and `pragma solidity ^0.8.38;`; imports `IDeliveryVerifier` from `../src/IDeliveryVerifier.sol`, nothing else
    * `[✅]`   `library IDeliveryVerifierBuilders` holding one `internal pure` builder per struct the interface owns, `buildDeliveryContext`, `buildMintStatement`, `buildMintProof`, `buildTransferStatement`, and `buildTransferProof`, each returning its struct in `memory` with every member set to a distinct nonzero value and every `bytes` member to a nonempty byte string; the statement builders build their `context` through `buildDeliveryContext`
    * `[✅]`   `contract IDeliveryVerifierMock is IDeliveryVerifier` with a constructor taking one `bool` and `verifyMint` and `verifyTransfer` each returning that `bool` whatever their inputs, the mock of the interface's two functions
    * `[✅]`   Imported by the tests of the contracts that consume the interface; it is not imported by the interface test

  * `[✅]`   `contracts/evm/foundry.toml`
    * `[✅]`   One `[profile.default]` table, authored once and complete, holding `src = "src"`, `test = "test"`, `out = "out"`, `libs = ["lib"]`, `remappings = ["forge-std/=lib/forge-std/src/"]`, `solc = "0.8.38"`, `evm_version = "prague"`, the EVM revision whose Pectra upgrade includes EIP-2537, the BLS12-381 precompiles, `via_ir = true`, since a proof record's decode function destructures more values than the legacy pipeline's sixteen reachable stack slots hold, `ffi = false`, and `fs_permissions = [{ access = "read", path = "generated" }, { access = "read", path = "abi" }, { access = "write", path = "deployments" }]`, read access to the generated directory the harness binary writes and to the ABI description directory, and write access to the deployments directory the deployment script writes the layer file of deployment records into, and to nothing else
    * `[✅]`   A configuration file with no types and no tests; no later ticket amends it

  * `[✅]`   `.github/workflows/contracts.yml`
    * `[✅]`   A workflow named `Contracts` with the triggers, `permissions`, and `concurrency` of `.github/workflows/rust.yml`, one job on `ubuntu-latest` with `defaults.run.working-directory` `contracts/evm`
    * `[✅]`   Steps in order: `actions/checkout@v7` with `submodules: recursive`, so `forge-std` is present; `foundry-rs/foundry-toolchain@v1`; `forge build`; `forge fmt --check`; `forge test`; each authored once and complete, the workflow running only Foundry, so the generated directory the contracts read is committed
    * `[✅]`   A configuration file with no types and no tests; no later ticket amends it

  * `[✅]`   `directionality`
    * `[✅]`   The suite depends inward on `forge-std` and, for the workflow's shape, on `workspace/ci`; nothing in `contracts/evm` imports a file outside the suite; no cycle
    * `[✅]`   `contracts/evm/DeliveryVerifier` implements the interface and `contracts/evm/PairingLib` builds under the toolchain; `harness-crypto/verifier/evm` binds the description and names no Solidity file

  * `[✅]`   `requirements`
    * `[✅]`   `forge build`, `forge fmt --check`, and `forge test` complete without error in `contracts/evm` (CD-03 interface)
    * `[✅]`   `test_DeliveryContextMembers`, `test_MintStatementMembers`, `test_MintProofMembers`, `test_TransferStatementMembers`, and `test_TransferProofMembers` pass
    * `[✅]`   `test_VerifyMintSelector`, `test_VerifyTransferSelector`, `test_VerifyMintIsViewAndReturnsBool`, and `test_VerifyTransferIsViewAndReturnsBool` pass
    * `[✅]`   `test_AbiDescriptionVerifyMint` and `test_AbiDescriptionVerifyTransfer` pass (the ABI description states the Solidity interface's functions, structs, members, and types)
    * `[✅]`   `contracts/evm/test/IDeliveryVerifierMock.sol` carries the builders and the mock stated above
    * `[✅]`   `contracts/evm/foundry.toml` and `.github/workflows/contracts.yml` carry exactly the entries stated above

* `[ ]`   `contracts/evm/ISignatureVerifier`

* `[ ]`   `contracts/evm/IdentityAccount`

* `[ ]`   `contracts/evm/Binding`

* `[ ]`   `contracts/evm/IEntitlement`

* `[ ]`   `contracts/evm/IIdentityAdapter`

* `[ ]`   `contracts/evm/IAttestationVerifier`

* `[ ]`   `contracts/evm/ParameterSets`

* `[ ]`   `contracts/evm/Registry`

* `[ ]`   `domain/handshake_key_commitment`

* `[ ]`   `chain/base`

* `[ ]`   `chain/admit_chain_selection`

* `[ ]`   `chain/lookup_chain_declaration`

* `[ ]`   `chain/admit_chain_composition`

* `[ ]`   `chain/factory`
