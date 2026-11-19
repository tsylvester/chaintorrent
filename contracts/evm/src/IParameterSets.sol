// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.38;

interface IParameterSets {
    struct ParameterSet {
        bytes32 id;
        bytes32 asset;
        bytes32 suiteId;
        bytes material;
        uint8 identityScope;
        bool live;
    }

    event ParameterSetRegistered(bytes32 indexed id, bytes32 indexed asset, bytes32 suiteId, uint8 identityScope);
    event ParameterSetRetired(bytes32 indexed id);

    error CallerNotRegistry(address caller);
    error ParameterSetIdUnset();
    error AssetUnset();
    error SuiteUnset();
    error MaterialEmpty();
    error ParameterSetAlreadyRegistered(bytes32 id);
    error ParameterSetNotRegistered(bytes32 id);
    error ParameterSetAlreadyRetired(bytes32 id);

    function registerParameterSet(
        bytes32 id,
        bytes32 asset,
        bytes32 suiteId,
        bytes calldata material,
        uint8 identityScope
    ) external;

    function retireParameterSet(bytes32 id) external;

    function parameterSet(bytes32 id) external view returns (ParameterSet memory set);

    function isParameterSetLive(bytes32 id) external view returns (bool live);
}
