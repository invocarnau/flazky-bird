// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";

import "./flazkybird.sol"; // Adjust the path as necessary

contract FlazkyBirdTest is Test {
    FlazkyBird public flazkyBird;

    function setUp() public {
        flazkyBird = new FlazkyBird();
    }

    function testFlazkyy() public {
        address player1 = address(0x1);
        address player2 = address(0x2);
        address player3 = address(0x3);

        vm.prank(player1); 
        flazkyBird.addLeaderboardEntry(100, 0); // index 1, [1: 0(100)]
        vm.prank(player2); 
        flazkyBird.addLeaderboardEntry(200, 0); // index 2, [1: 1(100), 2: 0(200)]
        vm.prank(player3); 
        flazkyBird.addLeaderboardEntry(300, 0); // index 3, [1: 2(100), 2: 1(200), 3: 0(300)]
        vm.prank(player1); 
        flazkyBird.addLeaderboardEntry(200, 2); // index 4, [1: 3(100), 2: 1(200), 3: 0(300), 4 2(200)]
        vm.prank(player2); 
        flazkyBird.addLeaderboardEntry(50, 1); // index 5, [1: 3(100), 2: 1(200), 3: 0(300), 4 2(200), 5 4(50)]
        vm.prank(player3); 
        flazkyBird.addLeaderboardEntry(1000, 0); // index 6, [1: 4(100), 2: 2(200), 3: 1(300), 4 3(200), 5 5(50), 6: 0(1000)]

        (FlazkyBird.LeaderboardEntry[] memory leaderboard, uint256 next) = flazkyBird.getLeaderboard(0, 6);
        assertEq(next, 0);
        // #0
        assertEq(leaderboard[0].player, player3);
        assertEq(leaderboard[0].score, 1000);
        assertEq(leaderboard[0].next, 3);
        // #1
        assert(leaderboard[1].score <= leaderboard[0].score);
        assertEq(leaderboard[1].player, player3);
        assertEq(leaderboard[1].score, 300);
        assertEq(leaderboard[1].next, 2);
        // #2
        assert(leaderboard[2].score <= leaderboard[1].score);
        assertEq(leaderboard[2].player, player2);
        assertEq(leaderboard[2].score, 200);
        assertEq(leaderboard[2].next, 4);
        // #3
        assert(leaderboard[3].score <= leaderboard[2].score);
        assertEq(leaderboard[3].player, player1);
        assertEq(leaderboard[3].score, 200);
        assertEq(leaderboard[3].next, 1);
        // #4
        assert(leaderboard[4].score <= leaderboard[3].score);
        assertEq(leaderboard[4].player, player1);
        assertEq(leaderboard[4].score, 100);
        assertEq(leaderboard[4].next, 5);
        // #5
        assert(leaderboard[5].score <= leaderboard[4].score);
        assertEq(leaderboard[5].player, player2);
        assertEq(leaderboard[5].score, 50);
        assertEq(leaderboard[5].next, 0);
    }
}