// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.38;

import {IParameterSets} from "./IParameterSets.sol";

contract ParameterSets is IParameterSets {
    address private immutable _registry;
    mapping(bytes32 => ParameterSet) private _parameterSets;

    constructor() {
        _registry = msg.sender;
    }

    function registerParameterSet(
        bytes32 id,
        bytes32 asset,
        bytes32 suiteId,
        bytes calldata material,
        uint8 identityScope
    ) external override {
        if (msg.sender != _registry) {
            revert CallerNotRegistry(msg.sender);
        }
        if (id == bytes32(0)) {
            revert ParameterSetIdUnset();
        }
        if (asset == bytes32(0)) {
            revert AssetUnset();
        }
        if (suiteId == bytes32(0)) {
            revert SuiteUnset();
        }
        if (material.length == 0) {
            revert MaterialEmpty();
        }
        if (_parameterSets[id].asset != bytes32(0)) {
            revert ParameterSetAlreadyRegistered(id);
        }
        _parameterSets[id] = ParameterSet({
            id: id, asset: asset, suiteId: suiteId, material: material, identityScope: identityScope, live: true
        });
        emit ParameterSetRegistered(id, asset, suiteId, identityScope);
    }

    function retireParameterSet(bytes32 id) external override {
        if (msg.sender != _registry) {
            revert CallerNotRegistry(msg.sender);
        }
        ParameterSet storage set = _parameterSets[id];
        if (set.asset == bytes32(0)) {
            revert ParameterSetNotRegistered(id);
        }
        if (!set.live) {
            revert ParameterSetAlreadyRetired(id);
        }
        set.live = false;
        emit ParameterSetRetired(id);
    }

    function parameterSet(bytes32 id) external view override returns (ParameterSet memory set) {
        return _parameterSets[id];
    }

    function isParameterSetLive(bytes32 id) external view override returns (bool live) {
        return _parameterSets[id].live;
    }
}
