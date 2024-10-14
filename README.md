# FlaZKy Bird

## Proving instructions

1. If you haven't already, [install SP1](https://docs.succinct.xyz/getting-started/install.html)
2. Clone this repo!
3. `cd` into `prover/host`
4. If you have access to the [Succint proving network](https://docs.succinct.xyz/generating-proofs/prover-network.html), copy the `.env.example` to `.env` and add your private key. If not, don't worry, the proof will be generated on your machine, it just will take some time
5. Run the following command: `cargo run --release -- --file A --prove --eth-address B`, where:
    - `A` is the path where you have downloaded the trace file from the web UI
    - `B` is any valid Ethereum address, that will receive the NFT on the smart contract (doesn't need to be the address that will send the tx)
6. Once the proof is generated, there should be a new file `prover/fixtures/flazky.json`, with all the info needed to build the tx
7. Go to the [etherscan smart contract](https://sepolia.etherscan.io/address/0x5a2f7933f312476af5eec0972a6b6c6c09cebfdc#writeContract#F1) page, on and go to the `addLeaderboardEntry`. After connecting your wallet, fill the fields:
   - `_publicValues (bytes)`: `prover/fixtures/flazky.json` / `public_values`
   - `_proofBytes (bytes)`: `prover/fixtures/flazky.json` / `proof`
   - `_previousTokenID`: 0 if you have the new highscore, 1 otherwhise. You can get the current highscore by calling [getLeaderboard](https://sepolia.etherscan.io/address/0x5a2f7933f312476af5eec0972a6b6c6c09cebfdc#readContract#F3) with `from (uint256) -> 0` and `items (uint256) -> 1`. (we should improve this in the proof generation script and add the correct value on flazky.josn...)
8. Once the tx goes through (and it succeeds), the specified address will be awarded with an NFT, and be includded in the leaderboard forever. You can check the [NFTs on opensea](https://testnets.opensea.io/assets/sepolia/0x5a2f7933f312476af5eec0972a6b6c6c09cebfdc)