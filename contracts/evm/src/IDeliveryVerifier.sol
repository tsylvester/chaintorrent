// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.37;

interface IDeliveryVerifier {
    struct DeliveryContext {
        bytes32 suiteIdentifier;
        uint16 suiteVersion;
        uint256 chain;
        bytes20 entitlementContract;
        bytes32 assetIdentityHash;
        bytes32 parameterSet;
        uint256 sourceEntitlement;
        uint256 targetEntitlement;
        uint64 oldInterval;
        uint64 newInterval;
        uint16 purpose;
        bytes20 seller;
        bytes20 buyer;
        uint64 expiry;
    }

    struct MintStatement {
        DeliveryContext context;
        bytes hpub;
        bytes identityElement;
        bytes buyerPk1;
        bytes buyerPk2;
        bytes envelopeC1;
        bytes envelopeC2;
        bytes envelopeD1;
        bytes envelopeD2;
    }

    struct MintProof {
        uint256 challenge;
        uint256 alpha;
        uint256 r;
        uint256 rho;
        uint256 sigma;
        bytes secondGroupFirstMessages;
    }

    struct TransferStatement {
        DeliveryContext context;
        bytes identityElement;
        bytes sellerPk1;
        bytes sellerPk2;
        bytes buyerPk1;
        bytes buyerPk2;
        bytes oldEnvelopeC1;
        bytes oldEnvelopeC2;
        bytes oldEnvelopeD1;
        bytes oldEnvelopeD2;
        bytes newEnvelopeC1;
        bytes newEnvelopeC2;
        bytes newEnvelopeD1;
        bytes newEnvelopeD2;
    }

    struct TransferProof {
        uint256 challenge;
        uint256 x;
        uint256 y;
        uint256 s;
        uint256 rho;
        uint256 sigma;
        bytes secondGroupFirstMessages;
    }

    function verifyMint(
        MintStatement calldata statement,
        MintProof calldata proof
    ) external view returns (bool accepted);

    function verifyTransfer(
        TransferStatement calldata statement,
        TransferProof calldata proof
    ) external view returns (bool accepted);
}
