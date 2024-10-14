// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";

import "./FlazkyBird.sol"; // Adjust the path as necessary
import "./mocks/SP1VerifierMock.sol"; // Adjust the path as necessary
import {console} from "forge-std/console.sol";

contract FlazkyBirdTest is Test {
    FlazkyBird public flazkyBird;
    SP1VerifierMock public mockVerifier;

    function setUp() public {
        vm.prank(address(0xf00));
        mockVerifier = new SP1VerifierMock();
        flazkyBird = new FlazkyBird(mockVerifier, bytes32(type(uint256).max));
    }

    function testFlazkyy() public {
        address player1 = address(0x1);
        address player2 = address(0x2);
        address player3 = address(0x3);

        bytes memory proofBytes = new bytes(0);
        FlazkyBird.PublicValuesStruct memory publicValues =
            FlazkyBird.PublicValuesStruct({player: player1, score: 100, nullifier: bytes32(0)});

        flazkyBird.addLeaderboardEntry(abi.encode(publicValues), proofBytes, 0); // index 1, [1: 0(100)]
        publicValues.player = player2;
        publicValues.score = 200;
        publicValues.nullifier = bytes32(uint256(1));
        flazkyBird.addLeaderboardEntry(abi.encode(publicValues), proofBytes, 0); // index 2, [1: 1(100), 2: 0(200)]
        publicValues.player = player3;
        publicValues.score = 300;
        publicValues.nullifier = bytes32(uint256(2));
        flazkyBird.addLeaderboardEntry(abi.encode(publicValues), proofBytes, 0); // index 3, [1: 2(100), 2: 1(200), 3: 0(300)]
        publicValues.player = player1;
        publicValues.score = 200;
        publicValues.nullifier = bytes32(uint256(3));
        flazkyBird.addLeaderboardEntry(abi.encode(publicValues), proofBytes, 2); // index 4, [1: 3(100), 2: 1(200), 3: 0(300), 4 2(200)]
        publicValues.player = player2;
        publicValues.score = 50;
        publicValues.nullifier = bytes32(uint256(4));
        flazkyBird.addLeaderboardEntry(abi.encode(publicValues), proofBytes, 1); // index 5, [1: 3(100), 2: 1(200), 3: 0(300), 4 2(200), 5 4(50)]
        publicValues.player = player3;
        publicValues.score = 1000;
        publicValues.nullifier = bytes32(uint256(5));
        flazkyBird.addLeaderboardEntry(abi.encode(publicValues), proofBytes, 0); // index 6, [1: 4(100), 2: 2(200), 3: 1(300), 4 3(200), 5 5(50), 6: 0(1000)]

        (FlazkyBird.LeaderboardEntry[] memory leaderboard,) = flazkyBird.getLeaderboard(0, 6);

        // #0
        assertEq(flazkyBird.ownerOf(flazkyBird.leader()), player3);
        assertEq(leaderboard[0].score, 1000);
        assertEq(leaderboard[0].nextTokenID, 3);
        // #1
        assert(leaderboard[1].score <= leaderboard[0].score);
        assertEq(flazkyBird.ownerOf(leaderboard[0].nextTokenID), player3);
        assertEq(leaderboard[1].score, 300);
        assertEq(leaderboard[1].nextTokenID, 2);
        // #2
        assert(leaderboard[2].score <= leaderboard[1].score);
        assertEq(flazkyBird.ownerOf(leaderboard[1].nextTokenID), player2);
        assertEq(leaderboard[2].score, 200);
        assertEq(leaderboard[2].nextTokenID, 4);
        // #3
        assert(leaderboard[3].score <= leaderboard[2].score);
        assertEq(flazkyBird.ownerOf(leaderboard[2].nextTokenID), player1);
        assertEq(leaderboard[3].score, 200);
        assertEq(leaderboard[3].nextTokenID, 1);
        // #4
        assert(leaderboard[4].score <= leaderboard[3].score);
        assertEq(flazkyBird.ownerOf(leaderboard[3].nextTokenID), player1);
        assertEq(leaderboard[4].score, 100);
        assertEq(leaderboard[4].nextTokenID, 5);
        // #5
        assert(leaderboard[5].score <= leaderboard[4].score);
        assertEq(flazkyBird.ownerOf(leaderboard[4].nextTokenID), player2);
        assertEq(leaderboard[5].score, 50);
        assertEq(leaderboard[5].nextTokenID, 0);
        string memory tokenURI = flazkyBird.tokenURI(1);
        console.log(tokenURI);
    }
}
