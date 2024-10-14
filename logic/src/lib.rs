use alloy_primitives::{Address};
use serde::{Serialize, Deserialize};

const GRAVITY: f32 = -30.;
const WINDOW_Y: i32 = 512;
const BIRD_WIDTH: f32 = 20.;
const BIRD_HEIGHT: i32 = 32;
const PIPE_WIDTH: f32 = 48.;
const PIPE_HEIGHT: i32 = 316;
const MAGIC_NUMBER: i32 = 435885720;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub encoded_trace: Vec<u8>,
    pub player: Address,
}

pub struct FlazkyBird {
    prover_mode: bool,
    game_is_over: bool,
    bird: Bird,
    pipes: Vec<Pipe>,
    score: u32,
    high_score: u32,
    current_treacer: Vec<TraceItem>,
    high_score_treacer: Vec<TraceItem>,
    rand: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceItem {
    pub action: Action,
    pub data: [u8; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Action {
    NewPlay,
    Jump,
    ApplyPhysics,
    CheckCollisionAndMovePipes,
    GameOver,
}

fn new_rand(current_rand: i32) -> i32 {
    // return a value between -315 and -60
    let mut input = current_rand * MAGIC_NUMBER;
    if input < 0 {
        input = input * -1;
    }
    (input % 255) -315
}

impl FlazkyBird {
    pub fn new(prover_mode: bool) -> Self {
        let mut pipes = Vec::new();
        let mut x = 300.;
        for _ in 1..=5 {
            let lower = Pipe {
                position: Coord { x, y: -100 },
            };
            pipes.push(lower);
            let upper = Pipe {
                position: Coord { x, y: 350 },
            };
            pipes.push(upper);
            x += 200.;
        }
        Self {
            prover_mode,
            game_is_over: true,
            bird: Bird::new(),
            pipes,
            score: 0,
            high_score: 0,
            current_treacer: Vec::new(),
            high_score_treacer: Vec::new(),
            rand: 0,
        }
    }

    pub fn new_play(&mut self, rand_seed: i32) {
        self.bird.position.y = 0;
        // self.bird.position.rotation = Quat::from_rotation_x(0.);
        let mut mutable_seed = rand_seed;
        self.bird.speed = 0.;
        for (i, pipe) in self.pipes.iter_mut().enumerate() {
            if i % 2 == 0 {
                mutable_seed = new_rand(mutable_seed);
            }
            pipe.position.x = 300. + (i/2 * 200) as f32;
            pipe.position.y = (i as i32%2 * 450) + mutable_seed;
        }
        self.score = 0;
        if !self.game_is_over {
            panic!("game is not over noooo");
        }
        self.game_is_over = false;
        self.rand = rand_seed;
        if !self.prover_mode {
            self.current_treacer = Vec::new();
            self.current_treacer.push(TraceItem {
                action: Action::NewPlay,
                data: rand_seed.to_le_bytes(),
            });
        }
    }

    pub fn apply_physics(&mut self, delta_seconds: f32) -> bool {
        if self.game_is_over {
            return true;
        }
        if !self.prover_mode {
            self.current_treacer.push(TraceItem {
                action: Action::ApplyPhysics,
                data: delta_seconds.to_le_bytes(),
            });
        }
        self.bird.position.y += (self.bird.speed + (0.5 * GRAVITY * delta_seconds * delta_seconds)) as i32;
        self.bird.speed += GRAVITY * delta_seconds;
        // self.bird.rotation = Quat::from_rotation_z((self.bird.speed.max(0.).abs() / 50.) as f32);
        if self.bird.position.y < -174 {
            self.game_over();
            self.bird.position.y = -174;
            return true;
        }
        false
    }

    pub fn jump(&mut self) {
        if self.game_is_over {
            return;
        }
        if !self.prover_mode {
            self.current_treacer.push(TraceItem {
                action: Action::Jump,
                data: [0;4],
            });
        }
        if self.bird.position.y < WINDOW_Y / 2 {
            if self.bird.speed > 7. {
                self.bird.speed = 22.;
            } else {
                self.bird.speed = 14.;
            }
        }
    }

    pub fn bird_position(&self) -> Coord {
        Coord { x: self.bird.position.x, y: self.bird.position.y }
    }

    pub fn get_pipe_positions(&self) -> Vec<Coord> {
        self.pipes.iter().map(|pipe| Coord { x: pipe.position.x, y: pipe.position.y }).collect()
    }

    pub fn check_collision_and_move_pipes(&mut self, delta_seconds: f32) -> (bool, bool) {
        if self.game_is_over {
            return (true, false);
        }
        if !self.prover_mode {
            let trace =TraceItem {
                action: Action::CheckCollisionAndMovePipes,
                data: delta_seconds.to_le_bytes(),
            };
            self.current_treacer.push(trace);
        }
        // check for collision
        for pipe in self.pipes.iter() {
            let half_width1 = BIRD_WIDTH / 2.;
            let half_height1 = BIRD_HEIGHT / 2;
            let half_width2 = PIPE_WIDTH / 2.;
            let half_height2 = PIPE_HEIGHT / 2;

            let x1_min = self.bird.position.x - half_width1;
            let x1_max = self.bird.position.x + half_width1;
            let y1_min = self.bird.position.y - half_height1;
            let y1_max = self.bird.position.y + half_height1;

            let x2_min = pipe.position.x - half_width2;
            let x2_max = pipe.position.x + half_width2;
            let y2_min = pipe.position.y - half_height2;
            let y2_max = pipe.position.y + half_height2;

            let collision_x = x1_max >= x2_min && x2_max >= x1_min;
            let collision_y = y1_max >= y2_min && y2_max >= y1_min;

            if collision_x && collision_y {
                self.game_over();
                return (true, false);
            }
        }
        // move pipes
        let mut level_up = false;
        let mut init_score = 0;
        for (i, pipe) in self.pipes.iter_mut().enumerate() {
            pipe.position.x -= delta_seconds * 200.;
            if self.score < 4 && i%2 == 0 && pipe.position.x < -PIPE_WIDTH {
                init_score += 1;
            }
            if pipe.position.x <= -500. {
                pipe.position.x = 500.0;
                if i%2 == 0 { // lower pipe
                    self.score += 1;
                    level_up = true;
                    pipe.position.y = self.rand;
                } else { // upper pipe
                    pipe.position.y = 450 + self.rand;
                }
            }
        }
        if self.score < 4 {
            self.score = init_score;
        }
        if level_up {
            self.rand = new_rand(self.rand);
        }
        (false, level_up)
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn game_over(&mut self) {
        if !self.prover_mode && !self.game_is_over {
            self.current_treacer.push(TraceItem {
                action: Action::GameOver,
                data: [0;4],
            });
            if self.score > self.high_score {
                self.high_score = self.score;
                self.high_score_treacer = self.current_treacer.clone();
            }
            self.game_is_over = true;
        }
    }

    pub fn get_high_score_treacer(&self) -> Vec<TraceItem> {
        self.high_score_treacer.clone()
    }

    pub fn get_high_score(&self) -> u32 {
        self.high_score
    }
}

pub struct Coord {
    pub x: f32,
    pub y: i32,
}


struct Bird {
    position: Coord,
    speed: f32,
}

impl Bird {
    pub fn new() -> Self {
        Self {
            speed: 0.,
            position: Coord { x: 0., y: 0 },
        }
    }
}

struct Pipe {
    position: Coord,
}
