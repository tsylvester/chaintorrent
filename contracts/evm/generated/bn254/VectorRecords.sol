// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.38;

library Bn254VectorRecords {
    struct G1AdditionVector {
        bytes draws;
        bytes input;
        bytes output;
    }

    string internal constant G1_ADDITION_PATH = "generated/bn254/g1_addition.txt";

    function decodeG1Addition(bytes memory line) internal pure returns (G1AdditionVector memory) {
        (bytes memory draws, bytes memory input, bytes memory output) = abi.decode(line, (bytes, bytes, bytes));
        return G1AdditionVector({draws: draws, input: input, output: output});
    }

    struct G1MultiplicationVector {
        bytes draws;
        bytes input;
        bytes output;
    }

    string internal constant G1_MULTIPLICATION_PATH = "generated/bn254/g1_multiplication.txt";

    function decodeG1Multiplication(bytes memory line) internal pure returns (G1MultiplicationVector memory) {
        (bytes memory draws, bytes memory input, bytes memory output) = abi.decode(line, (bytes, bytes, bytes));
        return G1MultiplicationVector({draws: draws, input: input, output: output});
    }

    struct PairingCheckVector {
        bytes draws;
        bytes input;
        bytes32 output;
    }

    string internal constant PAIRING_CHECK_PATH = "generated/bn254/pairing_check.txt";

    function decodePairingCheck(bytes memory line) internal pure returns (PairingCheckVector memory) {
        (bytes memory draws, bytes memory input, bytes32 output) = abi.decode(line, (bytes, bytes, bytes32));
        return PairingCheckVector({draws: draws, input: input, output: output});
    }

    struct DecodeRejectionVector {
        string target;
        bytes input;
    }

    string internal constant DECODE_REJECTION_PATH = "generated/bn254/decode_rejection.txt";

    function decodeDecodeRejection(bytes memory line) internal pure returns (DecodeRejectionVector memory) {
        (string memory target, bytes memory input) = abi.decode(line, (string, bytes));
        return DecodeRejectionVector({target: target, input: input});
    }

    struct HashToScalarVector {
        bytes tag;
        bytes message;
        uint256 scalar;
    }

    string internal constant HASH_TO_SCALAR_PATH = "generated/bn254/hash_to_scalar.txt";

    function decodeHashToScalar(bytes memory line) internal pure returns (HashToScalarVector memory) {
        (bytes memory tag, bytes memory message, uint256 scalar) = abi.decode(line, (bytes, bytes, uint256));
        return HashToScalarVector({tag: tag, message: message, scalar: scalar});
    }

    struct IdentityMappingVector {
        bytes identity;
        uint256 scalar;
    }

    string internal constant IDENTITY_MAPPING_PATH = "generated/bn254/identity_mapping.txt";

    function decodeIdentityMapping(bytes memory line) internal pure returns (IdentityMappingVector memory) {
        (bytes memory identity, uint256 scalar) = abi.decode(line, (bytes, uint256));
        return IdentityMappingVector({identity: identity, scalar: scalar});
    }

    struct PossessionVector {
        uint256 x;
        uint256 y;
        bytes pk1;
        bytes pk2;
        bytes r1;
        uint256 z1;
        bytes r2;
        uint256 z2;
    }

    string internal constant POSSESSION_PATH = "generated/bn254/possession.txt";

    function decodePossession(bytes memory line) internal pure returns (PossessionVector memory) {
        (uint256 x, uint256 y, bytes memory pk1, bytes memory pk2, bytes memory r1, uint256 z1, bytes memory r2, uint256 z2) = abi.decode(line, (uint256, uint256, bytes, bytes, bytes, uint256, bytes, uint256));
        return PossessionVector({x: x, y: y, pk1: pk1, pk2: pk2, r1: r1, z1: z1, r2: r2, z2: z2});
    }

    struct ChallengeVector {
        uint16 purpose;
        bytes draws;
        bytes encoding;
        uint256 challenge;
    }

    string internal constant CHALLENGE_PATH = "generated/bn254/challenge.txt";

    function decodeChallenge(bytes memory line) internal pure returns (ChallengeVector memory) {
        (uint16 purpose, bytes memory draws, bytes memory encoding, uint256 challenge) = abi.decode(line, (uint16, bytes, bytes, uint256));
        return ChallengeVector({purpose: purpose, draws: draws, encoding: encoding, challenge: challenge});
    }

    struct MintProofVector {
        bytes draws;
        bytes32 suiteIdentifier;
        uint16 suiteVersion;
        uint256 chain;
        bytes20 entitlementContract;
        bytes32 assetRoot;
        bytes32 parameterSet;
        uint256 sourceEntitlement;
        uint256 targetEntitlement;
        uint64 oldInterval;
        uint64 newInterval;
        uint16 purpose;
        bytes20 seller;
        bytes20 buyer;
        uint64 expiry;
        bytes hpub;
        bytes identityElement;
        bytes buyerPk1;
        bytes buyerPk2;
        bytes envelopeC1;
        bytes envelopeC2;
        bytes envelopeD1;
        bytes envelopeD2;
        uint256 challenge;
        uint256 alpha;
        uint256 r;
        uint256 rho;
        uint256 sigma;
        bytes secondGroupFirstMessages;
    }

    string internal constant MINT_PROOF_PATH = "generated/bn254/mint_proof.txt";

    function decodeMintProof(bytes memory line) internal pure returns (MintProofVector memory) {
        (bytes memory draws, bytes32 suiteIdentifier, uint16 suiteVersion, uint256 chain, bytes20 entitlementContract, bytes32 assetRoot, bytes32 parameterSet, uint256 sourceEntitlement, uint256 targetEntitlement, uint64 oldInterval, uint64 newInterval, uint16 purpose, bytes20 seller, bytes20 buyer, uint64 expiry, bytes memory hpub, bytes memory identityElement, bytes memory buyerPk1, bytes memory buyerPk2, bytes memory envelopeC1, bytes memory envelopeC2, bytes memory envelopeD1, bytes memory envelopeD2, uint256 challenge, uint256 alpha, uint256 r, uint256 rho, uint256 sigma, bytes memory secondGroupFirstMessages) = abi.decode(line, (bytes, bytes32, uint16, uint256, bytes20, bytes32, bytes32, uint256, uint256, uint64, uint64, uint16, bytes20, bytes20, uint64, bytes, bytes, bytes, bytes, bytes, bytes, bytes, bytes, uint256, uint256, uint256, uint256, uint256, bytes));
        return MintProofVector({draws: draws, suiteIdentifier: suiteIdentifier, suiteVersion: suiteVersion, chain: chain, entitlementContract: entitlementContract, assetRoot: assetRoot, parameterSet: parameterSet, sourceEntitlement: sourceEntitlement, targetEntitlement: targetEntitlement, oldInterval: oldInterval, newInterval: newInterval, purpose: purpose, seller: seller, buyer: buyer, expiry: expiry, hpub: hpub, identityElement: identityElement, buyerPk1: buyerPk1, buyerPk2: buyerPk2, envelopeC1: envelopeC1, envelopeC2: envelopeC2, envelopeD1: envelopeD1, envelopeD2: envelopeD2, challenge: challenge, alpha: alpha, r: r, rho: rho, sigma: sigma, secondGroupFirstMessages: secondGroupFirstMessages});
    }

    struct TransferProofVector {
        bytes draws;
        bytes32 suiteIdentifier;
        uint16 suiteVersion;
        uint256 chain;
        bytes20 entitlementContract;
        bytes32 assetRoot;
        bytes32 parameterSet;
        uint256 sourceEntitlement;
        uint256 targetEntitlement;
        uint64 oldInterval;
        uint64 newInterval;
        uint16 purpose;
        bytes20 seller;
        bytes20 buyer;
        uint64 expiry;
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
        uint256 challenge;
        uint256 x;
        uint256 y;
        uint256 s;
        uint256 rho;
        uint256 sigma;
        bytes secondGroupFirstMessages;
    }

    string internal constant TRANSFER_PROOF_PATH = "generated/bn254/transfer_proof.txt";

    function decodeTransferProof(bytes memory line) internal pure returns (TransferProofVector memory) {
        (bytes memory draws, bytes32 suiteIdentifier, uint16 suiteVersion, uint256 chain, bytes20 entitlementContract, bytes32 assetRoot, bytes32 parameterSet, uint256 sourceEntitlement, uint256 targetEntitlement, uint64 oldInterval, uint64 newInterval, uint16 purpose, bytes20 seller, bytes20 buyer, uint64 expiry, bytes memory identityElement, bytes memory sellerPk1, bytes memory sellerPk2, bytes memory buyerPk1, bytes memory buyerPk2, bytes memory oldEnvelopeC1, bytes memory oldEnvelopeC2, bytes memory oldEnvelopeD1, bytes memory oldEnvelopeD2, bytes memory newEnvelopeC1, bytes memory newEnvelopeC2, bytes memory newEnvelopeD1, bytes memory newEnvelopeD2, uint256 challenge, uint256 x, uint256 y, uint256 s, uint256 rho, uint256 sigma, bytes memory secondGroupFirstMessages) = abi.decode(line, (bytes, bytes32, uint16, uint256, bytes20, bytes32, bytes32, uint256, uint256, uint64, uint64, uint16, bytes20, bytes20, uint64, bytes, bytes, bytes, bytes, bytes, bytes, bytes, bytes, bytes, bytes, bytes, bytes, bytes, uint256, uint256, uint256, uint256, uint256, uint256, bytes));
        return TransferProofVector({draws: draws, suiteIdentifier: suiteIdentifier, suiteVersion: suiteVersion, chain: chain, entitlementContract: entitlementContract, assetRoot: assetRoot, parameterSet: parameterSet, sourceEntitlement: sourceEntitlement, targetEntitlement: targetEntitlement, oldInterval: oldInterval, newInterval: newInterval, purpose: purpose, seller: seller, buyer: buyer, expiry: expiry, identityElement: identityElement, sellerPk1: sellerPk1, sellerPk2: sellerPk2, buyerPk1: buyerPk1, buyerPk2: buyerPk2, oldEnvelopeC1: oldEnvelopeC1, oldEnvelopeC2: oldEnvelopeC2, oldEnvelopeD1: oldEnvelopeD1, oldEnvelopeD2: oldEnvelopeD2, newEnvelopeC1: newEnvelopeC1, newEnvelopeC2: newEnvelopeC2, newEnvelopeD1: newEnvelopeD1, newEnvelopeD2: newEnvelopeD2, challenge: challenge, x: x, y: y, s: s, rho: rho, sigma: sigma, secondGroupFirstMessages: secondGroupFirstMessages});
    }
}
