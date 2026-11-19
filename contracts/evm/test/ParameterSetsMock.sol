// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.38;

import {IParameterSets} from "../src/IParameterSets.sol";

function buildParameterSet() pure returns (IParameterSets.ParameterSet memory) {
    return IParameterSets.ParameterSet({
        id: bytes32(uint256(1)),
        asset: bytes32(uint256(2)),
        suiteId: bytes32(uint256(3)),
        material: hex"01",
        identityScope: 1,
        live: true
    });
}

contract MockIParameterSets is IParameterSets {
    function parameterSet(bytes32) external pure override returns (ParameterSet memory set) {
        return buildParameterSet();
    }

    function isParameterSetLive(bytes32) external pure override returns (bool live) {
        return buildParameterSet().live;
    }

    function registerParameterSet(bytes32, bytes32, bytes32, bytes calldata, uint8) external pure override {}

    function retireParameterSet(bytes32) external pure override {}
}
