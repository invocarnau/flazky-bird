use std::fs;
use std::path::PathBuf; 
use clap::Parser;
use sp1_sdk::{HashableKey, utils, ProverClient, SP1Stdin, SP1ProofWithPublicValues};
use flazky_bird_lib::Input;
use alloy::hex;
use alloy_primitives::Address;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use alloy_sol_types::SolType;
use alloy_sol_types::sol;

#[derive(Parser, Debug)]
struct Args {
    /// Whether or not to generate a proof.
    #[arg(long, default_value_t = false)]
    prove: bool,

    #[clap(long)]
    file: String,

    #[clap(long)]
    eth_address: String,
}


sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct PublicValuesStruct {
        address player;
        uint256 score;
        bytes32 nullifier;
    }
}


const ELF_FLAZKY_BIRD: &[u8] = include_bytes!("../../../elf/flazky-bird");

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Initialize the logger.
    dotenv::dotenv().ok();
    utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();
    let trace_file = args.file;
    let trace_data = fs::read(trace_file)?;
    let eth_address: Address = Address::from_str(&args.eth_address).expect("Invalid address");

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF_FLAZKY_BIRD);
    let mut stdin = SP1Stdin::new();
    stdin.write(&Input{
        encoded_trace: trace_data,
        player: eth_address,
    });
    let (mut public_values, execution_report) =
        client.execute(&pk.elf, stdin.clone()).run().unwrap();
    println!(
        "Finished executing the block in {} cycles",
        execution_report.total_instruction_count()
    );
    // let prover_high_score = public_values.read::<u32>();
    // println!("Prover high score: {}", prover_high_score);

    if args.prove {
        println!("Starting proof generation.");
        let proof: SP1ProofWithPublicValues = client.prove(&pk, stdin.clone()).plonk().run().expect("Proving should work.");
        println!("Proof generation finished.");

        // Handle the result of the save operation
        let public_values_solidity_encoded = proof.public_values.as_slice();
        let decoded_values = PublicValuesStruct::abi_decode(public_values_solidity_encoded, true).unwrap();
        let fixture = ProofFixture {
            player: decoded_values.player.to_string(),
            score: decoded_values.score.to_string(),
            nullifier: decoded_values.nullifier.to_string(),
            vkey: vk.bytes32().to_string(),
            public_values: format!("0x{}", hex::encode(public_values_solidity_encoded)),
            proof: format!("0x{}", hex::encode(proof.bytes())),
        };

        let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../fixtures");
        std::fs::create_dir_all(&fixture_path).expect("failed to create fixture path");
        std::fs::write(
            fixture_path.join(format!("flazky.json").to_lowercase()),
            serde_json::to_string_pretty(&fixture).unwrap(),
        )
        .expect("failed to write fixture");
        println!("proof generated, output stored at ../fixtures/flazky.json")
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProofFixture {
    pub player: String,
    pub score: String,
    pub nullifier: String,
    pub public_values: String,
    pub proof: String,
    pub vkey: String,
}  
