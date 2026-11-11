// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.38;

library Bn254Constants {
    bytes internal constant IDENTITY_TAG = hex"436861696e546f7272656e742d76312d6b656d2d6964656e74697479";
    bytes internal constant POSSESSION_G1_TAG = hex"436861696e546f7272656e742d76312d656e76656c6f70652d706f7373657373696f6e2d6731";
    bytes internal constant POSSESSION_G2_TAG = hex"436861696e546f7272656e742d76312d656e76656c6f70652d706f7373657373696f6e2d6732";
    bytes internal constant CHALLENGE_TAG = hex"436861696e546f7272656e742d76312d70726f6f662d6368616c6c656e6765";
    bytes internal constant WEIGHT_TAG = hex"436861696e546f7272656e742d76312d70726f6f662d776569676874";
    uint16 internal constant DELIVERY_STATEMENT_VERSION = 1;
    uint16 internal constant PURPOSE_MINT = 1;
    uint16 internal constant PURPOSE_TRANSFER = 2;
    uint16 internal constant PURPOSE_GRANT = 3;
    uint16 internal constant PURPOSE_REPLACEMENT = 4;
    string internal constant MINT_FIELDS = "bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,bytes,bytes,bytes,bytes,bytes,bytes,uint64,bytes,bytes,bytes,bytes,bytes";
    string internal constant TRANSFER_FIELDS = "bytes32,uint16,uint256,bytes20,bytes32,bytes32,uint256,uint256,uint64,uint64,uint16,bytes20,bytes20,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,bytes,uint64,bytes,bytes,bytes,bytes,bytes,bytes";
    uint256 internal constant SCALAR_FIELD_ORDER = 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001;
    bytes internal constant G1_GENERATOR = hex"00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002";
    bytes internal constant G2_GENERATOR = hex"198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa";
    bool internal constant SECOND_GROUP_ARITHMETIC = false;
}
