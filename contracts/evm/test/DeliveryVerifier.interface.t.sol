// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.37;

import {Test} from "forge-std/Test.sol";
import {IDeliveryVerifier} from "../src/IDeliveryVerifier.sol";

contract DeliveryVerifierInterfaceTest is Test {
    /// Contract: `DeliveryContext` has exactly the members `suiteIdentifier`, `suiteVersion`, `chain`, `entitlementContract`, `assetIdentityHash`, `parameterSet`, `sourceEntitlement`, `targetEntitlement`, `oldInterval`, `newInterval`, `purpose`, `seller`, `buyer`, and `expiry`.
    function test_DeliveryContextMembers() public pure {
        IDeliveryVerifier.DeliveryContext memory context = IDeliveryVerifier.DeliveryContext({
            suiteIdentifier: bytes32(uint256(0x01)),
            suiteVersion: 1,
            chain: 84_532,
            entitlementContract: hex"0000000000000000000000000000000000000001",
            assetIdentityHash: bytes32(uint256(0x02)),
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

    /// Contract: `MintStatement` has exactly the members `context`, `hpub`, `identityElement`, `buyerPk1`, `buyerPk2`, `envelopeC1`, `envelopeC2`, `envelopeD1`, and `envelopeD2`.
    function test_MintStatementMembers() public pure {
        IDeliveryVerifier.MintStatement memory statement = IDeliveryVerifier.MintStatement({
            context: IDeliveryVerifier.DeliveryContext({
                suiteIdentifier: bytes32(uint256(0x01)),
                suiteVersion: 1,
                chain: 84_532,
                entitlementContract: hex"0000000000000000000000000000000000000001",
                assetIdentityHash: bytes32(uint256(0x02)),
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

    /// Contract: `MintProof` has exactly the members `challenge`, `alpha`, `r`, `rho`, `sigma`, and `secondGroupFirstMessages`.
    function test_MintProofMembers() public pure {
        IDeliveryVerifier.MintProof memory proof = IDeliveryVerifier.MintProof({
            challenge: 1,
            alpha: 2,
            r: 3,
            rho: 4,
            sigma: 5,
            secondGroupFirstMessages: new bytes(0)
        });
        assertEq(abi.encode(proof).length, 8 * 32);
    }

    /// Contract: `TransferStatement` has exactly the members `context`, `identityElement`, `sellerPk1`, `sellerPk2`, `buyerPk1`, `buyerPk2`, `oldEnvelopeC1`, `oldEnvelopeC2`, `oldEnvelopeD1`, `oldEnvelopeD2`, `newEnvelopeC1`, `newEnvelopeC2`, `newEnvelopeD1`, and `newEnvelopeD2`.
    function test_TransferStatementMembers() public pure {
        IDeliveryVerifier.TransferStatement memory statement = IDeliveryVerifier.TransferStatement({
            context: IDeliveryVerifier.DeliveryContext({
                suiteIdentifier: bytes32(uint256(0x01)),
                suiteVersion: 1,
                chain: 84_532,
                entitlementContract: hex"0000000000000000000000000000000000000001",
                assetIdentityHash: bytes32(uint256(0x02)),
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

    /// Contract: `TransferProof` has exactly the members `challenge`, `x`, `y`, `s`, `rho`, `sigma`, and `secondGroupFirstMessages`.
    function test_TransferProofMembers() public pure {
        IDeliveryVerifier.TransferProof memory proof = IDeliveryVerifier.TransferProof({
            challenge: 1,
            x: 2,
            y: 3,
            s: 4,
            rho: 5,
            sigma: 6,
            secondGroupFirstMessages: new bytes(0)
        });
        assertEq(abi.encode(proof).length, 9 * 32);
    }

    /// Contract: `verifyMint` takes one `MintStatement` and one `MintProof`, in that order, whose member types in order are those the selector's text states.
    function test_VerifyMintSelector() public pure {
        assert(
            IDeliveryVerifier.verifyMint.selector
                == bytes4(
                    keccak256(
                        "verifyMint((bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,uint64),bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes),(uint256,uint256,uint256,uint256,uint256,bytes))"
                    )
                )
        );
    }

    /// Contract: `verifyTransfer` takes one `TransferStatement` and one `TransferProof`.
    function test_VerifyTransferSelector() public pure {
        assert(
            IDeliveryVerifier.verifyTransfer.selector
                == bytes4(
                    keccak256(
                        "verifyTransfer((bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,uint64),bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes),(uint256,uint256,uint256,uint256,uint256,uint256,bytes))"
                    )
                )
        );
    }

    /// Contract: `verifyMint` is `external view` and returns one `bool`; the assignment is the act.
    function test_VerifyMintIsViewAndReturnsBool() public pure {
        function (IDeliveryVerifier.MintStatement calldata, IDeliveryVerifier.MintProof calldata)
            external
            view
            returns (bool) verifyMintFunction = IDeliveryVerifier(address(0)).verifyMint;
    }

    /// Contract: `verifyTransfer` is `external view` and returns one `bool`; the assignment is the act.
    function test_VerifyTransferIsViewAndReturnsBool() public pure {
        function (IDeliveryVerifier.TransferStatement calldata, IDeliveryVerifier.TransferProof calldata)
            external
            view
            returns (bool) verifyTransferFunction = IDeliveryVerifier(address(0)).verifyTransfer;
    }
}
