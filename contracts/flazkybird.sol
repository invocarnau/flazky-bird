// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/utils/Base64.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

contract FlazkyBird is ERC721 {
    struct LeaderboardEntry {
        address player;
        uint256 score;
        uint256 next;
    }

    mapping(uint256 => LeaderboardEntry) public leaderboard;
    uint256 public leader;
    uint256 counter;

    constructor() ERC721("FlazkyBird", "ZKB") {

    }

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
        _safeMint(msg.sender, counter);
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

    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        require(tokenId <= counter, "this NFT does not exist");
        LeaderboardEntry memory nftData = leaderboard[tokenId];
        
        return string(
            abi.encodePacked(
                "data:application/json;base64,",
                Base64.encode(
                    bytes(
                        abi.encodePacked(
                            '{"name":"FlaZKy Bird score",',
                            '"description": "NFT obtained playing FlaZKy bird and proving the score on-chain",',
                            '"attributes": ["score": ', Strings.toString(nftData.score),'],',
                            '"image":"https://toppng.com/uploads/preview/flappy-bird-pixel-art-flappy-bird-1156289438531sspmvwnk.png"}'
                        )
                    )
                )
            )
        );
    }
}
