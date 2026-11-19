// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.38;

import {Test} from "forge-std/Test.sol";
import {ParameterSets} from "../src/ParameterSets.sol";
import {IParameterSets} from "../src/IParameterSets.sol";
import {buildParameterSet} from "./ParameterSetsMock.sol";

contract ParameterSetsTest is Test {
    /// Contract: the address that deployed the contract may register and retire; another address may not.
    /// Arrange: a `ParameterSets` deployed from a pranked literal address, and a built set.
    /// Act:     `registerParameterSet` and `retireParameterSet` from the deploying address, then `registerParameterSet` from the test contract.
    /// Assert:  the deploying address's calls record and retire the set; the test contract's call reverts `CallerNotRegistry` carrying `address(this)`.
    function test_OnlyTheDeployingAddressMayRegisterAndRetire() public {
        // Arrange
        vm.prank(address(0xbeef));
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();

        // Act
        vm.prank(address(0xbeef));
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
        vm.prank(address(0xbeef));
        sets.retireParameterSet(set.id);
        vm.expectRevert(abi.encodeWithSelector(IParameterSets.CallerNotRegistry.selector, address(this)));
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);

        // Assert
        IParameterSets.ParameterSet memory recorded = sets.parameterSet(set.id);
        assertEq(recorded.id, bytes32(uint256(1)));
        assertFalse(recorded.live);
        assertFalse(sets.isParameterSetLive(set.id));
    }

    // test_OnlyTheDeployingAddressMayRegisterAndRetire

    /// Contract: an accepted registration stores `id`, `asset`, `suiteId`, `material`, `identityScope`, and `live` true, and emits `ParameterSetRegistered(id, asset, suiteId, identityScope)`.
    /// Arrange: a built set with `id`, `asset`, `suiteId`, `material`, and `identityScope` each assigned a literal differing from the builder's default.
    /// Act:     `registerParameterSet` with the set's members.
    /// Assert:  `ParameterSetRegistered` is emitted carrying the four literal arguments; `parameterSet` returns each assigned member and `live` true; `isParameterSetLive` returns true.
    function test_RegisterRecordsTheSetLiveAndEmitsTheEvent() public {
        // Arrange
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        set.id = bytes32(uint256(0x11));
        set.asset = bytes32(uint256(0x22));
        set.suiteId = bytes32(uint256(0x33));
        set.material = hex"a5a5";
        set.identityScope = 7;

        // Act
        vm.expectEmit(true, true, false, true);
        emit IParameterSets.ParameterSetRegistered(
            bytes32(uint256(0x11)), bytes32(uint256(0x22)), bytes32(uint256(0x33)), 7
        );
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);

        // Assert
        IParameterSets.ParameterSet memory recorded = sets.parameterSet(set.id);
        assertEq(recorded.id, bytes32(uint256(0x11)));
        assertEq(recorded.asset, bytes32(uint256(0x22)));
        assertEq(recorded.suiteId, bytes32(uint256(0x33)));
        assertEq(recorded.material, hex"a5a5");
        assertEq(recorded.identityScope, 7);
        assertTrue(recorded.live);
        assertTrue(sets.isParameterSetLive(set.id));
    }

    // test_RegisterRecordsTheSetLiveAndEmitsTheEvent

    /// Contract: `material` and `identityScope` are stored and returned unread.
    /// Arrange: one built set with `material` assigned `hex"00ff"` and `identityScope` `0`, a second with `identityScope` `255`.
    /// Act:     `registerParameterSet` for each set.
    /// Assert:  each record's `material` and `identityScope` return the literal assigned.
    function test_TheMaterialAndTheIdentityScopeAreStoredUnread() public {
        // Arrange
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        set.id = bytes32(uint256(0x41));
        set.material = hex"00ff";
        set.identityScope = 0;
        IParameterSets.ParameterSet memory other = buildParameterSet();
        other.id = bytes32(uint256(0x42));
        other.identityScope = 255;

        // Act
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
        sets.registerParameterSet(other.id, other.asset, other.suiteId, other.material, other.identityScope);

        // Assert
        IParameterSets.ParameterSet memory recorded = sets.parameterSet(set.id);
        assertEq(recorded.material, hex"00ff");
        assertEq(recorded.identityScope, 0);
        IParameterSets.ParameterSet memory recordedOther = sets.parameterSet(other.id);
        assertEq(recordedOther.identityScope, 255);
    }

    // test_TheMaterialAndTheIdentityScopeAreStoredUnread

    /// Contract: sets registered for one asset are separate records, each live; retiring one leaves the others live.
    /// Arrange: two built sets assigned the same literal `asset` and distinct literal `id`s, both registered.
    /// Act:     `retireParameterSet` of the first id.
    /// Assert:  both ids are live before the retirement; after it the first record's `live` is false and the second's record is unchanged and live.
    function test_SetsOfOneAssetAreRecordedIndependently() public {
        // Arrange
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        set.id = bytes32(uint256(0x101));
        set.asset = bytes32(uint256(0xaa));
        IParameterSets.ParameterSet memory other = buildParameterSet();
        other.id = bytes32(uint256(0x102));
        other.asset = bytes32(uint256(0xaa));
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
        sets.registerParameterSet(other.id, other.asset, other.suiteId, other.material, other.identityScope);
        IParameterSets.ParameterSet memory otherBefore = sets.parameterSet(other.id);
        bool setLiveBefore = sets.isParameterSetLive(set.id);
        bool otherLiveBefore = sets.isParameterSetLive(other.id);

        // Act
        sets.retireParameterSet(set.id);

        // Assert
        assertTrue(setLiveBefore);
        assertTrue(otherLiveBefore);
        IParameterSets.ParameterSet memory setAfter = sets.parameterSet(set.id);
        IParameterSets.ParameterSet memory otherAfter = sets.parameterSet(other.id);
        assertFalse(setAfter.live);
        assertFalse(sets.isParameterSetLive(set.id));
        assertEq(setAfter.id, bytes32(uint256(0x101)));
        assertEq(setAfter.asset, bytes32(uint256(0xaa)));
        assertEq(abi.encode(otherAfter), abi.encode(otherBefore));
        assertTrue(otherAfter.live);
        assertTrue(sets.isParameterSetLive(other.id));
    }

    // test_SetsOfOneAssetAreRecordedIndependently

    /// Contract: an accepted retirement sets `live` false, keeps every other member, and emits `ParameterSetRetired(id)`.
    /// Arrange: a built set assigned literal members, registered.
    /// Act:     `retireParameterSet` of its id.
    /// Assert:  `ParameterSetRetired` is emitted carrying the id; the record keeps `id`, `asset`, `suiteId`, `material`, and `identityScope` with `live` false; `isParameterSetLive` returns false.
    function test_RetireMarksTheSetNotLiveAndKeepsTheRecord() public {
        // Arrange
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        set.id = bytes32(uint256(0x51));
        set.asset = bytes32(uint256(0x52));
        set.suiteId = bytes32(uint256(0x53));
        set.material = hex"54";
        set.identityScope = 85;
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);

        // Act
        vm.expectEmit(true, false, false, true);
        emit IParameterSets.ParameterSetRetired(bytes32(uint256(0x51)));
        sets.retireParameterSet(set.id);

        // Assert
        IParameterSets.ParameterSet memory recorded = sets.parameterSet(set.id);
        assertEq(recorded.id, bytes32(uint256(0x51)));
        assertEq(recorded.asset, bytes32(uint256(0x52)));
        assertEq(recorded.suiteId, bytes32(uint256(0x53)));
        assertEq(recorded.material, hex"54");
        assertEq(recorded.identityScope, 85);
        assertFalse(recorded.live);
        assertFalse(sets.isParameterSetLive(set.id));
    }

    // test_RetireMarksTheSetNotLiveAndKeepsTheRecord

    /// Contract: registering an id after its retirement reverts `ParameterSetAlreadyRegistered(id)`; the record stays retired with its material.
    /// Arrange: a built set assigned a literal `id` and `material`, registered and retired; a second built set assigned the same `id` and a different literal `material`.
    /// Act:     `registerParameterSet` with the second set's members.
    /// Assert:  the call reverts `ParameterSetAlreadyRegistered` carrying the id; the record keeps the retired `material` and `live` false.
    function test_ARetiredIdIsNeverRegisteredAgain() public {
        // Arrange
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();
        set.id = bytes32(uint256(0x61));
        set.material = hex"ab";
        sets.registerParameterSet(set.id, set.asset, set.suiteId, set.material, set.identityScope);
        sets.retireParameterSet(set.id);
        IParameterSets.ParameterSet memory again = buildParameterSet();
        again.id = bytes32(uint256(0x61));
        again.material = hex"cd";

        // Act
        vm.expectRevert(
            abi.encodeWithSelector(IParameterSets.ParameterSetAlreadyRegistered.selector, bytes32(uint256(0x61)))
        );
        sets.registerParameterSet(again.id, again.asset, again.suiteId, again.material, again.identityScope);

        // Assert
        IParameterSets.ParameterSet memory recorded = sets.parameterSet(bytes32(uint256(0x61)));
        assertEq(recorded.material, hex"ab");
        assertFalse(recorded.live);
        assertFalse(sets.isParameterSetLive(bytes32(uint256(0x61))));
    }

    // test_ARetiredIdIsNeverRegisteredAgain

    /// Contract: `parameterSet` of an id no registration recorded returns every member zero; `isParameterSetLive` returns false.
    /// Arrange: a fresh `ParameterSets` and a built set whose `id` was never registered.
    /// Act:     `parameterSet` and `isParameterSetLive` of the unrecorded `id`.
    /// Assert:  every member is zero, `material` empty, `live` false; the liveness answer is false.
    function test_ParameterSetReturnsAnEmptyRecordForAnUnrecordedId() public {
        // Arrange
        ParameterSets sets = new ParameterSets();
        IParameterSets.ParameterSet memory set = buildParameterSet();

        // Act
        IParameterSets.ParameterSet memory record = sets.parameterSet(set.id);
        bool live = sets.isParameterSetLive(set.id);

        // Assert
        assertEq(record.id, bytes32(0));
        assertEq(record.asset, bytes32(0));
        assertEq(record.suiteId, bytes32(0));
        assertEq(record.material, hex"");
        assertEq(record.identityScope, 0);
        assertFalse(record.live);
        assertFalse(live);
    }

    // test_ParameterSetReturnsAnEmptyRecordForAnUnrecordedId
}
