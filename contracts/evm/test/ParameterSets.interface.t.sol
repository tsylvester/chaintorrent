// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.38;

import {Test} from "forge-std/Test.sol";
import {IParameterSets} from "../src/IParameterSets.sol";

contract ParameterSetsInterfaceTest is Test {
    /// Contract: `ParameterSet` has the members `id`, `asset`, `suiteId`, `material`, `identityScope`, and `live`.
    function test_ParameterSetMembers() public pure {
        IParameterSets.ParameterSet memory set = IParameterSets.ParameterSet({
            id: bytes32(uint256(0x01)),
            asset: bytes32(uint256(0x02)),
            suiteId: bytes32(uint256(0x03)),
            material: hex"04",
            identityScope: 5,
            live: true
        });
        assertEq(set.id, bytes32(uint256(0x01)));
        assertEq(set.asset, bytes32(uint256(0x02)));
        assertEq(set.suiteId, bytes32(uint256(0x03)));
        assertEq(set.material, hex"04");
        assertEq(set.identityScope, 5);
        assertEq(set.live, true);
    }

    // test_ParameterSetMembers

    /// Contract: `registerParameterSet` takes `bytes32`, `bytes32`, `bytes32`, `bytes`, and `uint8`, in that order.
    function test_RegisterParameterSetSelector() public pure {
        assert(
            IParameterSets.registerParameterSet.selector
                == bytes4(keccak256("registerParameterSet(bytes32,bytes32,bytes32,bytes,uint8)"))
        );
    }

    // test_RegisterParameterSetSelector

    /// Contract: `retireParameterSet` takes one `bytes32`.
    function test_RetireParameterSetSelector() public pure {
        assert(IParameterSets.retireParameterSet.selector == bytes4(keccak256("retireParameterSet(bytes32)")));
    }

    // test_RetireParameterSetSelector

    /// Contract: `parameterSet` takes one `bytes32`.
    function test_ParameterSetSelector() public pure {
        assert(IParameterSets.parameterSet.selector == bytes4(keccak256("parameterSet(bytes32)")));
    }

    // test_ParameterSetSelector

    /// Contract: `isParameterSetLive` takes one `bytes32`.
    function test_IsParameterSetLiveSelector() public pure {
        assert(IParameterSets.isParameterSetLive.selector == bytes4(keccak256("isParameterSetLive(bytes32)")));
    }

    // test_IsParameterSetLiveSelector

    /// Contract: `registerParameterSet` and `retireParameterSet` are `external` and change state, returning nothing; `parameterSet` and `isParameterSetLive` are `external view`, returning a `ParameterSet` and a `bool`.
    function test_StateChangingAndViewFunctionTypes() public pure {
        function(bytes32, bytes32, bytes32, bytes memory, uint8) external registerParameterSetFunction =
        IParameterSets(address(0)).registerParameterSet;
        function(bytes32) external retireParameterSetFunction = IParameterSets(address(0)).retireParameterSet;
        function(bytes32) external view returns (IParameterSets.ParameterSet memory) parameterSetFunction =
        IParameterSets(address(0)).parameterSet;
        function(bytes32) external view returns (bool) isParameterSetLiveFunction =
        IParameterSets(address(0)).isParameterSetLive;
    }

    // test_StateChangingAndViewFunctionTypes

    /// Contract: the events `ParameterSetRegistered(bytes32 indexed id, bytes32 indexed asset, bytes32 suiteId, uint8 identityScope)` and `ParameterSetRetired(bytes32 indexed id)` are exported.
    function test_EventSelectors() public pure {
        assert(
            IParameterSets.ParameterSetRegistered.selector
                == keccak256("ParameterSetRegistered(bytes32,bytes32,bytes32,uint8)")
        );
        assert(IParameterSets.ParameterSetRetired.selector == keccak256("ParameterSetRetired(bytes32)"));
    }

    // test_EventSelectors

    /// Contract: the errors `CallerNotRegistry(address)`, `ParameterSetIdUnset()`, `AssetUnset()`, `SuiteUnset()`, `MaterialEmpty()`, `ParameterSetAlreadyRegistered(bytes32)`, `ParameterSetNotRegistered(bytes32)`, and `ParameterSetAlreadyRetired(bytes32)` are exported.
    function test_ErrorSelectors() public pure {
        assert(IParameterSets.CallerNotRegistry.selector == bytes4(keccak256("CallerNotRegistry(address)")));
        assert(IParameterSets.ParameterSetIdUnset.selector == bytes4(keccak256("ParameterSetIdUnset()")));
        assert(IParameterSets.AssetUnset.selector == bytes4(keccak256("AssetUnset()")));
        assert(IParameterSets.SuiteUnset.selector == bytes4(keccak256("SuiteUnset()")));
        assert(IParameterSets.MaterialEmpty.selector == bytes4(keccak256("MaterialEmpty()")));
        assert(
            IParameterSets.ParameterSetAlreadyRegistered.selector
                == bytes4(keccak256("ParameterSetAlreadyRegistered(bytes32)"))
        );
        assert(
            IParameterSets.ParameterSetNotRegistered.selector == bytes4(keccak256("ParameterSetNotRegistered(bytes32)"))
        );
        assert(
            IParameterSets.ParameterSetAlreadyRetired.selector
                == bytes4(keccak256("ParameterSetAlreadyRetired(bytes32)"))
        );
    }

    // test_ErrorSelectors

    /// Contract: the `abi/IParameterSets.json` entries of `type` `function` state the interface's functions.
    function test_AbiDescriptionFunctions() public view {
        string memory json = vm.readFile("abi/IParameterSets.json");
        assertEq(vm.parseJsonString(json, "$[0].type"), "function");
        assertEq(vm.parseJsonString(json, "$[0].name"), "registerParameterSet");
        assertEq(vm.parseJsonString(json, "$[0].stateMutability"), "nonpayable");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].name"), "id");
        assertEq(vm.parseJsonString(json, "$[0].inputs[0].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].name"), "asset");
        assertEq(vm.parseJsonString(json, "$[0].inputs[1].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[0].inputs[2].name"), "suiteId");
        assertEq(vm.parseJsonString(json, "$[0].inputs[2].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[0].inputs[3].name"), "material");
        assertEq(vm.parseJsonString(json, "$[0].inputs[3].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[0].inputs[4].name"), "identityScope");
        assertEq(vm.parseJsonString(json, "$[0].inputs[4].type"), "uint8");
        assertFalse(vm.keyExistsJson(json, "$[0].outputs[0]"));
        assertEq(vm.parseJsonString(json, "$[1].type"), "function");
        assertEq(vm.parseJsonString(json, "$[1].name"), "retireParameterSet");
        assertEq(vm.parseJsonString(json, "$[1].stateMutability"), "nonpayable");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].name"), "id");
        assertEq(vm.parseJsonString(json, "$[1].inputs[0].type"), "bytes32");
        assertFalse(vm.keyExistsJson(json, "$[1].outputs[0]"));
        assertEq(vm.parseJsonString(json, "$[2].type"), "function");
        assertEq(vm.parseJsonString(json, "$[2].name"), "parameterSet");
        assertEq(vm.parseJsonString(json, "$[2].stateMutability"), "view");
        assertEq(vm.parseJsonString(json, "$[2].inputs[0].name"), "id");
        assertEq(vm.parseJsonString(json, "$[2].inputs[0].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].name"), "set");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].type"), "tuple");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].internalType"), "struct IParameterSets.ParameterSet");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].components[0].name"), "id");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].components[0].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].components[1].name"), "asset");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].components[1].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].components[2].name"), "suiteId");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].components[2].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].components[3].name"), "material");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].components[3].type"), "bytes");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].components[4].name"), "identityScope");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].components[4].type"), "uint8");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].components[5].name"), "live");
        assertEq(vm.parseJsonString(json, "$[2].outputs[0].components[5].type"), "bool");
        assertEq(vm.parseJsonString(json, "$[3].type"), "function");
        assertEq(vm.parseJsonString(json, "$[3].name"), "isParameterSetLive");
        assertEq(vm.parseJsonString(json, "$[3].stateMutability"), "view");
        assertEq(vm.parseJsonString(json, "$[3].inputs[0].name"), "id");
        assertEq(vm.parseJsonString(json, "$[3].inputs[0].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[3].outputs[0].name"), "live");
        assertEq(vm.parseJsonString(json, "$[3].outputs[0].type"), "bool");
    }

    // test_AbiDescriptionFunctions

    /// Contract: the `abi/IParameterSets.json` entries of `type` `event` and `error` state the interface's events and errors.
    function test_AbiDescriptionEventsAndErrors() public view {
        string memory json = vm.readFile("abi/IParameterSets.json");
        assertEq(vm.parseJsonString(json, "$[4].type"), "event");
        assertEq(vm.parseJsonString(json, "$[4].name"), "ParameterSetRegistered");
        assertEq(vm.parseJsonString(json, "$[4].inputs[0].name"), "id");
        assertEq(vm.parseJsonString(json, "$[4].inputs[0].type"), "bytes32");
        assertEq(vm.parseJsonBool(json, "$[4].inputs[0].indexed"), true);
        assertEq(vm.parseJsonString(json, "$[4].inputs[1].name"), "asset");
        assertEq(vm.parseJsonString(json, "$[4].inputs[1].type"), "bytes32");
        assertEq(vm.parseJsonBool(json, "$[4].inputs[1].indexed"), true);
        assertEq(vm.parseJsonString(json, "$[4].inputs[2].name"), "suiteId");
        assertEq(vm.parseJsonString(json, "$[4].inputs[2].type"), "bytes32");
        assertEq(vm.parseJsonBool(json, "$[4].inputs[2].indexed"), false);
        assertEq(vm.parseJsonString(json, "$[4].inputs[3].name"), "identityScope");
        assertEq(vm.parseJsonString(json, "$[4].inputs[3].type"), "uint8");
        assertEq(vm.parseJsonBool(json, "$[4].inputs[3].indexed"), false);
        assertEq(vm.parseJsonString(json, "$[5].type"), "event");
        assertEq(vm.parseJsonString(json, "$[5].name"), "ParameterSetRetired");
        assertEq(vm.parseJsonString(json, "$[5].inputs[0].name"), "id");
        assertEq(vm.parseJsonString(json, "$[5].inputs[0].type"), "bytes32");
        assertEq(vm.parseJsonBool(json, "$[5].inputs[0].indexed"), true);
        assertEq(vm.parseJsonString(json, "$[6].type"), "error");
        assertEq(vm.parseJsonString(json, "$[6].name"), "CallerNotRegistry");
        assertEq(vm.parseJsonString(json, "$[6].inputs[0].name"), "caller");
        assertEq(vm.parseJsonString(json, "$[6].inputs[0].type"), "address");
        assertEq(vm.parseJsonString(json, "$[7].type"), "error");
        assertEq(vm.parseJsonString(json, "$[7].name"), "ParameterSetIdUnset");
        assertFalse(vm.keyExistsJson(json, "$[7].inputs[0]"));
        assertEq(vm.parseJsonString(json, "$[8].type"), "error");
        assertEq(vm.parseJsonString(json, "$[8].name"), "AssetUnset");
        assertFalse(vm.keyExistsJson(json, "$[8].inputs[0]"));
        assertEq(vm.parseJsonString(json, "$[9].type"), "error");
        assertEq(vm.parseJsonString(json, "$[9].name"), "SuiteUnset");
        assertFalse(vm.keyExistsJson(json, "$[9].inputs[0]"));
        assertEq(vm.parseJsonString(json, "$[10].type"), "error");
        assertEq(vm.parseJsonString(json, "$[10].name"), "MaterialEmpty");
        assertFalse(vm.keyExistsJson(json, "$[10].inputs[0]"));
        assertEq(vm.parseJsonString(json, "$[11].type"), "error");
        assertEq(vm.parseJsonString(json, "$[11].name"), "ParameterSetAlreadyRegistered");
        assertEq(vm.parseJsonString(json, "$[11].inputs[0].name"), "id");
        assertEq(vm.parseJsonString(json, "$[11].inputs[0].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[12].type"), "error");
        assertEq(vm.parseJsonString(json, "$[12].name"), "ParameterSetNotRegistered");
        assertEq(vm.parseJsonString(json, "$[12].inputs[0].name"), "id");
        assertEq(vm.parseJsonString(json, "$[12].inputs[0].type"), "bytes32");
        assertEq(vm.parseJsonString(json, "$[13].type"), "error");
        assertEq(vm.parseJsonString(json, "$[13].name"), "ParameterSetAlreadyRetired");
        assertEq(vm.parseJsonString(json, "$[13].inputs[0].name"), "id");
        assertEq(vm.parseJsonString(json, "$[13].inputs[0].type"), "bytes32");
    }

    // test_AbiDescriptionEventsAndErrors
}
