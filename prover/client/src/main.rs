#![no_main]
sp1_zkvm::entrypoint!(main);

use flazky_bird_lib::{FlazkyBird,TraceItem,Action,Input};
use bincode;
use alloy_primitives::U256;
use alloy_sol_types::sol;
use serde::{Serialize, Deserialize};
use tiny_keccak::{Keccak,Hasher};
use alloy_sol_types::SolType;

sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct PublicValuesStruct {
        address player;
        uint256 score;
        bytes32 nullifier;
    }
}

pub fn main() {
    // Read the input
    let input: Input = sp1_zkvm::io::read::<Input>();
    let trace = bincode::deserialize::<Vec<TraceItem>>(&input.encoded_trace).unwrap();

    // Generate nullifier
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(&input.encoded_trace);
    hasher.finalize(&mut output);
    let nullifier = output.into();


    // assert trace is valid
    assert!(trace.len() > 2);
    assert!(trace[0].action == Action::NewPlay);
    assert!(trace.last().unwrap().action == Action::GameOver);

   // Execute the game
   let mut game = FlazkyBird::new(true);
   let mut is_first_game_over = true;
   let mut is_first_collision = true;
   for item in trace {
       match item.action {
           Action::NewPlay => {
                let rand = i32::from_le_bytes(item.data); 
                game.new_play(rand);
           }
           Action::Jump => {
               game.jump();
           }
           Action::ApplyPhysics => {
                let delta_seconds = f32::from_le_bytes(item.data);
                if game.apply_physics(delta_seconds) {
                    assert!(is_first_collision);
                    is_first_collision = false;
                }
           }
           Action::CheckCollisionAndMovePipes => {
                let delta_seconds = f32::from_le_bytes(item.data);
                let (collision, _) = game.check_collision_and_move_pipes(delta_seconds);
                if collision {
                    assert!(is_first_collision);
                    is_first_collision = false;
                }
           }
           Action::GameOver => {
                assert!(is_first_game_over);
                is_first_game_over = false;
           }
       }
   }
    
    // Commit  
    let public_values_solidity_encoded = PublicValuesStruct::abi_encode(&PublicValuesStruct {
        score: U256::from(game.score()),
        player: input.player,
        nullifier,
    });
    sp1_zkvm::io::commit_slice(&public_values_solidity_encoded);
}