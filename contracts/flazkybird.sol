// SPDX-License-Identifier: MIT

pragma solidity ^0.8.28;

contract FlazkyBird {
    struct LeaderboardEntry {
        address player;
        uint256 score;
        address next;
    }

    mapping(address => LeaderboardEntry) public leaderboard;
    address public leader;

    // constructor() public {
        
    // }

    function addLeaderboardEntry(uint256 score, address previous) public {
        // TODO: verify ZKP
        address next;
        if (previous == address(0)) { // player claims to be the new leader
            if (leader != address(0)) {
                LeaderboardEntry memory currentLeader = leaderboard[leader];
                require(currentLeader.score < score, "score should be higher than current lead");
                next = currentLeader.player;
            }
            // update leader, GG
            leader = msg.sender;
        } else {
            LeaderboardEntry memory previousEntry = leaderboard[previous];
            require(previousEntry.score >= score, "previous entry has lower score");
            next = previousEntry.next;
            if (next != address(0)) { // player is at the bottom
                LeaderboardEntry memory nextEntry = leaderboard[next];
                require(previousEntry.next == nextEntry.player, "prev/next are inconsistent");
                require(nextEntry.score < score);
            }
            previousEntry.next = msg.sender;
            leaderboard[previousEntry.player] = previousEntry;
        }

        // insert score
        leaderboard[msg.sender] = LeaderboardEntry(msg.sender, score, next);
    }

    function getLeaderboard(address from, uint256 items) public view returns (LeaderboardEntry[] memory) {
        LeaderboardEntry[] memory entries = new LeaderboardEntry[](items);
        address next = from;
        for (uint i = 0; i < items; i++) {
            entries[i] = leaderboard[next];
            next = entries[i].next;
        }
        return entries;
    }
}
