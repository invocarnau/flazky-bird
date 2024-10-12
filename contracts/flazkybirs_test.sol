// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "./flazkybird.sol"; // Adjust the path as necessary

contract FlazkyBirdTest is Test {
    FlazkyBird public flazkyBird;

    function setUp() public {
        flazkyBird = new FlazkyBird();
    }

    function testAddLeaderboardEntry() public {
        address player = address(0x1);
        uint256 score = 100;

        // Simulate adding a leaderboard entry
        vm.prank(player); // Set the caller to the player address
        flazkyBird.addLeaderboardEntry(score); // Replace with the actual function name

        // Check if the entry was added correctly
        LeaderboardEntry memory entry = flazkyBird.leaderboard(player);
        assertEq(entry.player, player);
        assertEq(entry.score, score);
    }

    function testAddExistingPlayer() public {
        address player = address(0x1);
        uint256 score = 100;

        // Simulate adding a leaderboard entry
        vm.prank(player);
        flazkyBird.addLeaderboardEntry(score);

        // Attempt to add the same player again and expect a revert
        vm.expectRevert("Player already exists");
        vm.prank(player);
        flazkyBird.addLeaderboardEntry(score);
    }
}