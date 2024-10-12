#![no_main]
sp1_zkvm::entrypoint!(main);

use flazky_bird_lib::{FlazkyBird,TraceItem,Action};
use bincode;

pub fn main() {
    // Read the input
    let input = sp1_zkvm::io::read_vec();
    let trace = bincode::deserialize::<Vec<TraceItem>>(&input).unwrap();

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
    let high_score = game.score();
    sp1_zkvm::io::commit(&high_score);
}