// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/utils/Base64.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "./interfaces/ISP1Verifier.sol";

contract FlazkyBird is ERC721Enumerable {
    struct PublicValuesStruct {
        address player;
        uint256 score;
        bytes32 nullifier;
    }

    struct LeaderboardEntry {
        uint64 score;
        uint64 nextTokenID;
    }

    ISP1Verifier public immutable verifier;
    bytes32 public immutable vKey;

    mapping(uint256 tokenId => LeaderboardEntry entry) public leaderboard;
    mapping(bytes32 => bool) public nullifierMap;

    uint256 public leader;

    constructor(ISP1Verifier _verifier, bytes32 _vKey) ERC721("FlazkyBird", "ZKB") {
        verifier = _verifier;
        vKey = _vKey;
    }

    function addLeaderboardEntry(bytes calldata _publicValues, bytes calldata _proofBytes, uint256 _previousTokenID)
        public
    {
        // Verifier trace of flazkybird
        verifier.verifyProof(vKey, _publicValues, _proofBytes);

        // Decode public values
        PublicValuesStruct memory publicValues = abi.decode(_publicValues, (PublicValuesStruct));

        // check nullifier:
        require(!nullifierMap[publicValues.nullifier], "nullifier already used");

        uint256 score = publicValues.score;

        // First tokenID is 1
        uint64 currentTokenID = uint64(totalSupply()) + 1;
        _safeMint(publicValues.player, currentTokenID);

        // Search what will be the next token ID
        uint256 previousTokenID = _previousTokenID;
        uint256 nextTokenID;

        // claim leadership
        if (_previousTokenID == 0) {
            if (leader != 0) {
                // if there's a leader
                require(score > leaderboard[leader].score, "score should be higher than current lead");
                nextTokenID = leader;
            }
            leader = currentTokenID;
        } else {
            // load previous entry
            LeaderboardEntry storage previousEntry = leaderboard[_previousTokenID];
            require(previousEntry.score >= score, "previous entry has lower score");

            if (previousEntry.nextTokenID == 0) {
                // player is at the bottom
                previousEntry.nextTokenID = currentTokenID;
            } else {
                nextTokenID = previousEntry.nextTokenID;
                while (true) {
                    LeaderboardEntry storage nextEntry = leaderboard[nextTokenID];
                    uint64 nextEntryScore = nextEntry.score;
                    if (nextEntryScore < score) {
                        leaderboard[previousTokenID].nextTokenID = currentTokenID;
                        break;
                    } else {
                        previousTokenID = nextTokenID;
                        nextTokenID = nextEntry.nextTokenID;
                    }
                }
            }
        }

        // insert score
        leaderboard[currentTokenID] = LeaderboardEntry(uint64(score), uint64(nextTokenID));
        // set nullifier
        nullifierMap[publicValues.nullifier] = true;
    }

    function getLeaderboard(uint256 from, uint256 items)
        public
        view
        returns (LeaderboardEntry[] memory, uint256 nextIndex)
    {
        LeaderboardEntry[] memory entries = new LeaderboardEntry[](items);
        uint256 next = from;
        if (next == 0) {
            next = leader;
        }
        for (uint256 i = 0; i < items; i++) {
            entries[i] = leaderboard[next];
            next = uint256(entries[i].nextTokenID);
        }
        return (entries, next);
    }

    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        _requireOwned(tokenId);
        LeaderboardEntry memory nftData = leaderboard[tokenId];

        return string(
            abi.encodePacked(
                "data:application/json;base64,",
                Base64.encode(
                    bytes(
                        abi.encodePacked(
                            '{"name":"FlaZKy Bird score",',
                            '"description": "NFT obtained playing FlaZKy bird and proving the score on-chain",',
                            '"attributes": ["score": ',
                            Strings.toString(nftData.score),
                            "],",
                            '"image":"https://toppng.com/uploads/preview/flappy-bird-pixel-art-flappy-bird-1156289438531sspmvwnk.png"}'
                        )
                    )
                )
            )
        );
    }
}
