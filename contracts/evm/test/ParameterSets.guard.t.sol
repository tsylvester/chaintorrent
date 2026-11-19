// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.38;

import {Test} from "forge-std/Test.sol";
import {ParameterSets} from "../src/ParameterSets.sol";
import {IParameterSets} from "../src/IParameterSets.sol";
import {buildParameterSet} from "./ParameterSetsMock.sol";

contract ParameterSetsGuardTest is Test {
    /// Contract: a call from an address other than the deployer reverts with `CallerNotRegistry` carrying that address.
    function test_RegisterRefusesACallerThatIsNotTheRegistry() public {
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        vm.prank(address(0xdead));
        vm.expectRevert(abi.encodeWithSelector(IParameterSets.CallerNotRegistry.selector, address(0xdead)));
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
    }

    // test_RegisterRefusesACallerThatIsNotTheRegistry

    /// Contract: a call from an address other than the deployer reverts with `CallerNotRegistry` carrying that address.
    function test_RetireRefusesACallerThatIsNotTheRegistry() public {
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        vm.prank(address(0xdead));
        vm.expectRevert(abi.encodeWithSelector(IParameterSets.CallerNotRegistry.selector, address(0xdead)));
        sets.retireParameterSet(set.id);
    }

    // test_RetireRefusesACallerThatIsNotTheRegistry

    /// Contract: a zero `id` reverts `ParameterSetIdUnset`.
    function test_RegisterRefusesAnUnsetId() public {
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        set.id = bytes32(0);
        vm.expectRevert(IParameterSets.ParameterSetIdUnset.selector);
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
    }

    // test_RegisterRefusesAnUnsetId

    /// Contract: a zero `asset` reverts `AssetUnset`.
    function test_RegisterRefusesAnUnsetAsset() public {
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        set.asset = bytes32(0);
        vm.expectRevert(IParameterSets.AssetUnset.selector);
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
    }

    // test_RegisterRefusesAnUnsetAsset

    /// Contract: a zero `suiteId` reverts `SuiteUnset`.
    function test_RegisterRefusesAnUnsetSuite() public {
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        set.suiteId = bytes32(0);
        vm.expectRevert(IParameterSets.SuiteUnset.selector);
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
    }

    // test_RegisterRefusesAnUnsetSuite

    /// Contract: empty `material` reverts `MaterialEmpty`.
    function test_RegisterRefusesEmptyMaterial() public {
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        set.material = hex"";
        vm.expectRevert(IParameterSets.MaterialEmpty.selector);
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
    }

    // test_RegisterRefusesEmptyMaterial

    /// Contract: registering an id whose record is live, and one whose record is retired, reverts with `ParameterSetAlreadyRegistered` carrying the id.
    function test_RegisterRefusesAnAlreadyRegisteredId() public {
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
        vm.expectRevert(abi.encodeWithSelector(IParameterSets.ParameterSetAlreadyRegistered.selector, set.id));
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
        sets.retireParameterSet(set.id);
        vm.expectRevert(abi.encodeWithSelector(IParameterSets.ParameterSetAlreadyRegistered.selector, set.id));
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
    }

    // test_RegisterRefusesAnAlreadyRegisteredId

    /// Contract: the register checks run in the specified order: the caller, then `id`, `asset`, `suiteId`, `material`, then the recorded `id`.
    function test_RegisterChecksInTheSpecifiedOrder() public {
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        set.id = bytes32(0);
        set.asset = bytes32(0);
        set.suiteId = bytes32(0);
        set.material = hex"";
        vm.prank(address(0xdead));
        vm.expectRevert(abi.encodeWithSelector(IParameterSets.CallerNotRegistry.selector, address(0xdead)));
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
        vm.expectRevert(IParameterSets.ParameterSetIdUnset.selector);
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
        set.id = bytes32(uint256(1));
        vm.expectRevert(IParameterSets.AssetUnset.selector);
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
        set.asset = bytes32(uint256(2));
        vm.expectRevert(IParameterSets.SuiteUnset.selector);
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
        set.suiteId = bytes32(uint256(3));
        set.material = hex"01";
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
        set.material = hex"";
        vm.expectRevert(IParameterSets.MaterialEmpty.selector);
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
    }

    // test_RegisterChecksInTheSpecifiedOrder

    /// Contract: retiring an id never recorded reverts `ParameterSetNotRegistered` carrying the id.
    function test_RetireRefusesAnUnregisteredId() public {
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        vm.expectRevert(abi.encodeWithSelector(IParameterSets.ParameterSetNotRegistered.selector, set.id));
        sets.retireParameterSet(set.id);
    }

    // test_RetireRefusesAnUnregisteredId

    /// Contract: retiring a retired set reverts `ParameterSetAlreadyRetired` carrying the id.
    function test_RetireRefusesAnAlreadyRetiredId() public {
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
        sets.retireParameterSet(set.id);
        vm.expectRevert(abi.encodeWithSelector(IParameterSets.ParameterSetAlreadyRetired.selector, set.id));
        sets.retireParameterSet(set.id);
    }

    // test_RetireRefusesAnAlreadyRetiredId

    /// Contract: the retire checks run in the specified order: the caller, then the recorded `id`.
    function test_RetireChecksInTheSpecifiedOrder() public {
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        vm.prank(address(0xdead));
        vm.expectRevert(abi.encodeWithSelector(IParameterSets.CallerNotRegistry.selector, address(0xdead)));
        sets.retireParameterSet(set.id);
        vm.expectRevert(abi.encodeWithSelector(IParameterSets.ParameterSetNotRegistered.selector, set.id));
        sets.retireParameterSet(set.id);
    }

    // test_RetireChecksInTheSpecifiedOrder

    /// Contract: a refused call leaves the record it concerned and the records of other ids as they were.
    function test_ARevertLeavesTheRecordsUnchanged() public {
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        IParameterSets.ParameterSet memory other = buildParameterSet();
        other.id = bytes32(uint256(2));
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
        sets.registerParameterSet(other.id, other.asset, other.suiteId, other.material, other.identityScope);
        IParameterSets.ParameterSet memory recordedSet = sets.parameterSet(set.id);
        IParameterSets.ParameterSet memory recordedOther = sets.parameterSet(other.id);
        IParameterSets.ParameterSet memory refused = buildParameterSet();
        refused.id = bytes32(uint256(3));
        IParameterSets.ParameterSet memory zero;
        vm.prank(address(0xdead));
        vm.expectRevert(abi.encodeWithSelector(IParameterSets.CallerNotRegistry.selector, address(0xdead)));
        sets.registerParameterSet(refused.id, refused.asset, refused.suiteId, refused.material, refused.identityScope);
        assertEq(abi.encode(sets.parameterSet(refused.id)), abi.encode(zero));
        assertEq(abi.encode(sets.parameterSet(set.id)), abi.encode(recordedSet));
        assertEq(abi.encode(sets.parameterSet(other.id)), abi.encode(recordedOther));
        refused.asset = bytes32(0);
        vm.expectRevert(IParameterSets.AssetUnset.selector);
        sets.registerParameterSet(refused.id, refused.asset, refused.suiteId, refused.material, refused.identityScope);
        assertEq(abi.encode(sets.parameterSet(refused.id)), abi.encode(zero));
        assertEq(abi.encode(sets.parameterSet(set.id)), abi.encode(recordedSet));
        assertEq(abi.encode(sets.parameterSet(other.id)), abi.encode(recordedOther));
        vm.expectRevert(abi.encodeWithSelector(IParameterSets.ParameterSetAlreadyRegistered.selector, set.id));
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
        assertEq(abi.encode(sets.parameterSet(set.id)), abi.encode(recordedSet));
        assertEq(abi.encode(sets.parameterSet(other.id)), abi.encode(recordedOther));
        vm.expectRevert(abi.encodeWithSelector(IParameterSets.ParameterSetNotRegistered.selector, refused.id));
        sets.retireParameterSet(refused.id);
        assertEq(abi.encode(sets.parameterSet(refused.id)), abi.encode(zero));
        assertEq(abi.encode(sets.parameterSet(set.id)), abi.encode(recordedSet));
        assertEq(abi.encode(sets.parameterSet(other.id)), abi.encode(recordedOther));
        sets.retireParameterSet(set.id);
        IParameterSets.ParameterSet memory retiredSet = sets.parameterSet(set.id);
        vm.expectRevert(abi.encodeWithSelector(IParameterSets.ParameterSetAlreadyRetired.selector, set.id));
        sets.retireParameterSet(set.id);
        assertEq(abi.encode(sets.parameterSet(set.id)), abi.encode(retiredSet));
        assertEq(abi.encode(sets.parameterSet(other.id)), abi.encode(recordedOther));
    }

    // test_ARevertLeavesTheRecordsUnchanged
}
