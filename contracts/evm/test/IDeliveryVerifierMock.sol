// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.38;

import {IDeliveryVerifier} from "../src/IDeliveryVerifier.sol";

library IDeliveryVerifierBuilders {
    function buildDeliveryContext() internal pure returns (IDeliveryVerifier.DeliveryContext memory) {
        return IDeliveryVerifier.DeliveryContext({
            suiteIdentifier: bytes32(uint256(0x01)),
            suiteVersion: 2,
            chain: 3,
            entitlementContract: bytes20(uint160(4)),
            assetRoot: bytes32(uint256(0x05)),
            parameterSet: bytes32(uint256(0x06)),
            sourceEntitlement: 7,
            targetEntitlement: 8,
            oldInterval: 9,
            newInterval: 10,
            purpose: 11,
            seller: bytes20(uint160(12)),
            buyer: bytes20(uint160(13)),
            expiry: 14
        });
    }

    function buildMintStatement() internal pure returns (IDeliveryVerifier.MintStatement memory) {
        return IDeliveryVerifier.MintStatement({
            context: buildDeliveryContext(),
            hpub: hex"01",
            identityElement: hex"02",
            buyerPk1: hex"03",
            buyerPk2: hex"04",
            envelopeC1: hex"05",
            envelopeC2: hex"06",
            envelopeD1: hex"07",
            envelopeD2: hex"08"
        });
    }

    function buildMintProof() internal pure returns (IDeliveryVerifier.MintProof memory) {
        return IDeliveryVerifier.MintProof({
            challenge: 1, alpha: 2, r: 3, rho: 4, sigma: 5, secondGroupFirstMessages: hex"06"
        });
    }

    function buildTransferStatement() internal pure returns (IDeliveryVerifier.TransferStatement memory) {
        return IDeliveryVerifier.TransferStatement({
            context: buildDeliveryContext(),
            identityElement: hex"01",
            sellerPk1: hex"02",
            sellerPk2: hex"03",
            buyerPk1: hex"04",
            buyerPk2: hex"05",
            oldEnvelopeC1: hex"06",
            oldEnvelopeC2: hex"07",
            oldEnvelopeD1: hex"08",
            oldEnvelopeD2: hex"09",
            newEnvelopeC1: hex"0a",
            newEnvelopeC2: hex"0b",
            newEnvelopeD1: hex"0c",
            newEnvelopeD2: hex"0d"
        });
    }

    function buildTransferProof() internal pure returns (IDeliveryVerifier.TransferProof memory) {
        return IDeliveryVerifier.TransferProof({
            challenge: 1, x: 2, y: 3, s: 4, rho: 5, sigma: 6, secondGroupFirstMessages: hex"07"
        });
    }
}

contract IDeliveryVerifierMock is IDeliveryVerifier {
    bool private immutable _verdict;

    constructor(bool verdict) {
        _verdict = verdict;
    }

    function verifyMint(MintStatement calldata, MintProof calldata) external view override returns (bool) {
        return _verdict;
    }

    function verifyTransfer(TransferStatement calldata, TransferProof calldata) external view override returns (bool) {
        return _verdict;
    }
}
