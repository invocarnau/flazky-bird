// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

contract FlazkyBird {
    struct LeaderboardEntry {
        address player;
        uint256 score;
        uint256 next;
    }

    mapping(uint256 => LeaderboardEntry) public leaderboard;
    uint256 public leader;
    uint256 counter;

    function addLeaderboardEntry(uint256 score, uint256 previous) public {
        // TODO: verify ZKP
        counter++;
        uint256 next;
        if (previous == 0) { // player claims to be the new leader
            if (counter != 1) {
                LeaderboardEntry memory currentLeader = leaderboard[leader];
                require(currentLeader.score < score, "score should be higher than current lead");
                next = leader;
            }
            // update leader, GG
            leader = counter;
        } else {
            LeaderboardEntry memory previousEntry = leaderboard[previous];
            require(previousEntry.score >= score, "previous entry has lower score");
            next = previousEntry.next;
            if (next != 0) { // player is at the bottom
                LeaderboardEntry memory nextEntry = leaderboard[next];
                require(previousEntry.next == next, "prev/next are inconsistent");
                require(nextEntry.score < score);
            }
            previousEntry.next = counter;
            leaderboard[previous] = previousEntry;
        }

        // insert score
        leaderboard[counter] = LeaderboardEntry(msg.sender, score, next);
    }

    function getLeaderboard(uint256 from, uint256 items) public view returns (LeaderboardEntry[] memory, uint256 nextIndex) {
        LeaderboardEntry[] memory entries = new LeaderboardEntry[](items);
        uint256 next = from;
        if (next == 0) {
            next = leader;
        }
        for (uint i = 0; i < items; i++) {
            entries[i] = leaderboard[next];
            next = entries[i].next;
        }
        return (entries, next);
    }
}
