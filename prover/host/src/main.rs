use std::fs;
use std::io;
use clap::Parser;
use sp1_sdk::{SP1Proof, HashableKey, utils, ProverClient, SP1Stdin, SP1ProofWithPublicValues, SP1VerifyingKey};

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    file: String,
}

const ELF_FLAZKY_BIRD: &[u8] = include_bytes!("../../../elf/flazky-bird");

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Intialize the environment variables.
    // dotenv::dotenv().ok();

    // // Fallback to 'info' level if RUST_LOG is not set
    // if std::env::var("RUST_LOG").is_err() {
    //     std::env::set_var("RUST_LOG", "info");
    // }

    // Initialize the logger.
    utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();
    let trace_file = args.file;
    let trace_data = fs::read(trace_file)?;

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF_FLAZKY_BIRD);
    let mut stdin = SP1Stdin::new();
    stdin.write_vec(trace_data);
    let (mut public_values, execution_report) =
        client.execute(&pk.elf, stdin.clone()).run().unwrap();
    println!(
        "Finished executing the block in {} cycles",
        execution_report.total_instruction_count()
    );
    let prover_high_score = public_values.read::<u32>();
    println!("Prover high score: {}", prover_high_score);

    Ok(())
}