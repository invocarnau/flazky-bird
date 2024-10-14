// SPDX-License-Identifier: AGPL-3.0

pragma solidity ^0.8.20;

import "../interfaces/ISP1Verifier.sol";

contract SP1VerifierMock is ISP1Verifier {
    // SP1 interface
    function verifyProof(bytes32 programVKey, bytes calldata publicValues, bytes calldata proofBytes) public pure {}
}
