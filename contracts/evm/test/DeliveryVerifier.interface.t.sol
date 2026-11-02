// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.38;

import {Test} from "forge-std/Test.sol";
import {IDeliveryVerifier} from "../src/IDeliveryVerifier.sol";

contract DeliveryVerifierInterfaceTest is Test {
    /// Contract: `DeliveryContext` has exactly the members `suiteIdentifier`, `suiteVersion`, `chain`, `entitlementContract`, `assetRoot`, `parameterSet`, `sourceEntitlement`, `targetEntitlement`, `oldInterval`, `newInterval`, `purpose`, `seller`, `buyer`, and `expiry`.
    function test_DeliveryContextMembers() public pure {
        IDeliveryVerifier.DeliveryContext memory context = IDeliveryVerifier.DeliveryContext({
            suiteIdentifier: bytes32(uint256(0x01)),
            suiteVersion: 1,
            chain: 84_532,
            entitlementContract: hex"0000000000000000000000000000000000000001",
            assetRoot: bytes32(uint256(0x02)),
            parameterSet: bytes32(uint256(0x03)),
            sourceEntitlement: 1,
            targetEntitlement: 1,
            oldInterval: 0,
            newInterval: 1,
            purpose: 1,
            seller: hex"0000000000000000000000000000000000000002",
            buyer: hex"0000000000000000000000000000000000000003",
            expiry: 4_102_444_800
        });
        assertEq(abi.encode(context).length, 14 * 32);
    }

    // test_DeliveryContextMembers

    /// Contract: `MintStatement` has exactly the members `context`, `hpub`, `identityElement`, `buyerPk1`, `buyerPk2`, `envelopeC1`, `envelopeC2`, `envelopeD1`, and `envelopeD2`.
    function test_MintStatementMembers() public pure {
        IDeliveryVerifier.MintStatement memory statement = IDeliveryVerifier.MintStatement({
            context: IDeliveryVerifier.DeliveryContext({
                suiteIdentifier: bytes32(uint256(0x01)),
                suiteVersion: 1,
                chain: 84_532,
                entitlementContract: hex"0000000000000000000000000000000000000001",
                assetRoot: bytes32(uint256(0x02)),
                parameterSet: bytes32(uint256(0x03)),
                sourceEntitlement: 1,
                targetEntitlement: 1,
                oldInterval: 0,
                newInterval: 1,
                purpose: 1,
                seller: hex"0000000000000000000000000000000000000002",
                buyer: hex"0000000000000000000000000000000000000003",
                expiry: 4_102_444_800
            }),
            hpub: new bytes(0),
            identityElement: new bytes(0),
            buyerPk1: new bytes(0),
            buyerPk2: new bytes(0),
            envelopeC1: new bytes(0),
            envelopeC2: new bytes(0),
            envelopeD1: new bytes(0),
            envelopeD2: new bytes(0)
        });
        assertEq(abi.encode(statement).length, 31 * 32);
    }

    // test_MintStatementMembers

    /// Contract: `MintProof` has exactly the members `challenge`, `alpha`, `r`, `rho`, `sigma`, and `secondGroupFirstMessages`.
    function test_MintProofMembers() public pure {
        IDeliveryVerifier.MintProof memory proof = IDeliveryVerifier.MintProof({
            challenge: 1, alpha: 2, r: 3, rho: 4, sigma: 5, secondGroupFirstMessages: new bytes(0)
        });
        assertEq(abi.encode(proof).length, 8 * 32);
    }

    // test_MintProofMembers

    /// Contract: `TransferStatement` has exactly the members `context`, `identityElement`, `sellerPk1`, `sellerPk2`, `buyerPk1`, `buyerPk2`, `oldEnvelopeC1`, `oldEnvelopeC2`, `oldEnvelopeD1`, `oldEnvelopeD2`, `newEnvelopeC1`, `newEnvelopeC2`, `newEnvelopeD1`, and `newEnvelopeD2`.
    function test_TransferStatementMembers() public pure {
        IDeliveryVerifier.TransferStatement memory statement = IDeliveryVerifier.TransferStatement({
            context: IDeliveryVerifier.DeliveryContext({
                suiteIdentifier: bytes32(uint256(0x01)),
                suiteVersion: 1,
                chain: 84_532,
                entitlementContract: hex"0000000000000000000000000000000000000001",
                assetRoot: bytes32(uint256(0x02)),
                parameterSet: bytes32(uint256(0x03)),
                sourceEntitlement: 1,
                targetEntitlement: 1,
                oldInterval: 0,
                newInterval: 1,
                purpose: 2,
                seller: hex"0000000000000000000000000000000000000002",
                buyer: hex"0000000000000000000000000000000000000003",
                expiry: 4_102_444_800
            }),
            identityElement: new bytes(0),
            sellerPk1: new bytes(0),
            sellerPk2: new bytes(0),
            buyerPk1: new bytes(0),
            buyerPk2: new bytes(0),
            oldEnvelopeC1: new bytes(0),
            oldEnvelopeC2: new bytes(0),
            oldEnvelopeD1: new bytes(0),
            oldEnvelopeD2: new bytes(0),
            newEnvelopeC1: new bytes(0),
            newEnvelopeC2: new bytes(0),
            newEnvelopeD1: new bytes(0),
            newEnvelopeD2: new bytes(0)
        });
        assertEq(abi.encode(statement).length, 41 * 32);
    }

    // test_TransferStatementMembers

    /// Contract: `TransferProof` has exactly the members `challenge`, `x`, `y`, `s`, `rho`, `sigma`, and `secondGroupFirstMessages`.
    function test_TransferProofMembers() public pure {
        IDeliveryVerifier.TransferProof memory proof = IDeliveryVerifier.TransferProof({
            challenge: 1, x: 2, y: 3, s: 4, rho: 5, sigma: 6, secondGroupFirstMessages: new bytes(0)
        });
        assertEq(abi.encode(proof).length, 9 * 32);
    }

    // test_TransferProofMembers

    /// Contract: `verifyMint` takes one `MintStatement` and one `MintProof`, in that order, whose member types in order are those the selector's text states.
    function test_VerifyMintSelector() public pure {
        assert(
            IDeliveryVerifier.verifyMint.selector
                == bytes4(
                    keccak256(
                        "verifyMint(((bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,uint64),bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes),(uint256,uint256,uint256,uint256,uint256,bytes))"
                    )
                )
        );
    }

    // test_VerifyMintSelector

    /// Contract: `verifyTransfer` takes one `TransferStatement` and one `TransferProof`.
    function test_VerifyTransferSelector() public pure {
        assert(
            IDeliveryVerifier.verifyTransfer.selector
                == bytes4(
                    keccak256(
                        "verifyTransfer(((bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,uint64),bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes),(uint256,uint256,uint256,uint256,uint256,uint256,bytes))"
                    )
                )
        );
    }

    // test_VerifyTransferSelector

    /// Contract: `verifyMint` is `external view` and returns one `bool`; the assignment is the act.
    function test_VerifyMintIsViewAndReturnsBool() public pure {
        function(IDeliveryVerifier.MintStatement memory, IDeliveryVerifier.MintProof memory)
            external
            view returns (bool) verifyMintFunction = IDeliveryVerifier(address(0)).verifyMint;
    }

    // test_VerifyMintIsViewAndReturnsBool

    /// Contract: `verifyTransfer` is `external view` and returns one `bool`; the assignment is the act.
    function test_VerifyTransferIsViewAndReturnsBool() public pure {
        function(IDeliveryVerifier.TransferStatement memory, IDeliveryVerifier.TransferProof memory)
            external
            view returns (bool) verifyTransferFunction = IDeliveryVerifier(address(0)).verifyTransfer;
    }

    // test_VerifyTransferIsViewAndReturnsBool

    /// Contract: the entry of `abi/IDeliveryVerifier.json` named `verifyMint` states the interface's `verifyMint`.
    function test_AbiDescriptionVerifyMint() public view {
        string memory json = vm.readFile("abi/IDeliveryVerifier.json");
        assertEq(vm.parseJsonString(json, "$[0].type"), "function");
        assertEq(vm.parseJsonString(json, "$[0].name"), "verifyMint");
        assertEq(vm.parseJsonString(json, "$[0].stateMutability"), "view");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].name"), "statement");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].type"), "tuple");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].internalType"), "struct IDeliveryVerifier.MintStatement");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].name"), "context");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].type"), "tuple");
        assertEq(
            vm.parseJsonString(json, "$[0].inputs[0].components[0].internalType"),
            "struct IDeliveryVerifier.DeliveryContext"
        );
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[0].name"), "suiteIdentifier");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[0].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[1].name"), "suiteVersion");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[1].type"), "uint16");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[2].name"), "chain");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[2].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[3].name"), "entitlementContract");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[3].type"), "bytes20");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[4].name"), "assetRoot");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[4].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[5].name"), "parameterSet");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[5].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[6].name"), "sourceEntitlement");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[6].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[7].name"), "targetEntitlement");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[7].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[8].name"), "oldInterval");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[8].type"), "uint64");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[9].name"), "newInterval");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[9].type"), "uint64");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[10].name"), "purpose");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[10].type"), "uint16");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[11].name"), "seller");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[11].type"), "bytes20");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[12].name"), "buyer");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[12].type"), "bytes20");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[13].name"), "expiry");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[0].components[13].type"), "uint64");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[1].name"), "hpub");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[1].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[2].name"), "identityElement");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[2].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[3].name"), "buyerPk1");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[3].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[4].name"), "buyerPk2");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[4].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[5].name"), "envelopeC1");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[5].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[6].name"), "envelopeC2");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[6].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[7].name"), "envelopeD1");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[7].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[8].name"), "envelopeD2");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].components[8].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].name"), "proof");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].type"), "tuple");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].internalType"), "struct IDeliveryVerifier.MintProof");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].components[0].name"), "challenge");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].components[0].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].components[1].name"), "alpha");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].components[1].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].components[2].name"), "r");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].components[2].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].components[3].name"), "rho");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].components[3].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].components[4].name"), "sigma");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].components[4].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].components[5].name"), "secondGroupFirstMessages");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].components[5].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[0].outputs[0].name"), "accepted");
        assertEq(vm.parseJsonString(json, "$[0].outputs[0].type"), "bool");
    }

    // test_AbiDescriptionVerifyMint

    /// Contract: the entry of `abi/IDeliveryVerifier.json` named `verifyTransfer` states the interface's `verifyTransfer`.
    function test_AbiDescriptionVerifyTransfer() public view {
        string memory json = vm.readFile("abi/IDeliveryVerifier.json");
        assertEq(vm.parseJsonString(json, "$[1].type"), "function");
        assertEq(vm.parseJsonString(json, "$[1].name"), "verifyTransfer");
        assertEq(vm.parseJsonString(json, "$[1].stateMutability"), "view");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].name"), "statement");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].type"), "tuple");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].internalType"), "struct IDeliveryVerifier.TransferStatement");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].name"), "context");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].type"), "tuple");
        assertEq(
            vm.parseJsonString(json, "$[1].inputs[0].components[0].internalType"),
            "struct IDeliveryVerifier.DeliveryContext"
        );
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[0].name"), "suiteIdentifier");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[0].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[1].name"), "suiteVersion");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[1].type"), "uint16");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[2].name"), "chain");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[2].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[3].name"), "entitlementContract");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[3].type"), "bytes20");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[4].name"), "assetRoot");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[4].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[5].name"), "parameterSet");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[5].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[6].name"), "sourceEntitlement");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[6].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[7].name"), "targetEntitlement");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[7].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[8].name"), "oldInterval");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[8].type"), "uint64");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[9].name"), "newInterval");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[9].type"), "uint64");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[10].name"), "purpose");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[10].type"), "uint16");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[11].name"), "seller");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[11].type"), "bytes20");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[12].name"), "buyer");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[12].type"), "bytes20");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[13].name"), "expiry");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[0].components[13].type"), "uint64");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[1].name"), "identityElement");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[1].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[2].name"), "sellerPk1");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[2].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[3].name"), "sellerPk2");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[3].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[4].name"), "buyerPk1");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[4].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[5].name"), "buyerPk2");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[5].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[6].name"), "oldEnvelopeC1");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[6].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[7].name"), "oldEnvelopeC2");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[7].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[8].name"), "oldEnvelopeD1");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[8].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[9].name"), "oldEnvelopeD2");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[9].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[10].name"), "newEnvelopeC1");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[10].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[11].name"), "newEnvelopeC2");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[11].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[12].name"), "newEnvelopeD1");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[12].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[13].name"), "newEnvelopeD2");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].components[13].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].name"), "proof");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].type"), "tuple");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].internalType"), "struct IDeliveryVerifier.TransferProof");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].components[0].name"), "challenge");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].components[0].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].components[1].name"), "x");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].components[1].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].components[2].name"), "y");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].components[2].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].components[3].name"), "s");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].components[3].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].components[4].name"), "rho");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].components[4].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].components[5].name"), "sigma");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].components[5].type"), "uint256");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].components[6].name"), "secondGroupFirstMessages");
        assertEq(vm.parseJsonString(json, "$[1].inputs[1].components[6].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[1].outputs[0].name"), "accepted");
        assertEq(vm.parseJsonString(json, "$[1].outputs[0].type"), "bool");
    }

    // test_AbiDescriptionVerifyTransfer
}
