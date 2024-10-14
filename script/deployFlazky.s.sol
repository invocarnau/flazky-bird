// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {FlazkyBird} from "../contracts/FlazkyBird.sol";
import {SP1VerifierMock} from "../contracts/mocks/SP1VerifierMock.sol";
import {ISP1Verifier} from "../contracts/interfaces/ISP1Verifier.sol";

contract DeployMock is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        SP1VerifierMock mockVerifier = new SP1VerifierMock();
        FlazkyBird flazkyBird = new FlazkyBird(mockVerifier, bytes32(type(uint256).max));

        vm.stopBroadcast();
    }
}

contract DeploySepolia is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        bytes32 vkey = bytes32(type(uint256).max);
        ISP1Verifier sp1Gateway = ISP1Verifier(0x3B6041173B80E77f038f3F2C0f9744f04837185e);
        FlazkyBird flazkyBird = new FlazkyBird(sp1Gateway, vkey);

        vm.stopBroadcast();
    }
}
