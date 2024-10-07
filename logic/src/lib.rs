use serde::{Serialize, Deserialize};
use bincode::serialize;

const GRAVITY: f32 = -30.;
const WINDOW_Y: f32 = 512.;
const BIRD_WIDTH: f32 = 20.;
const BIRD_HEIGHT: f32 = 32.;
const PIPE_WIDTH: f32 = 48.;
const PIPE_HEIGHT: f32 = 316.;

pub struct FlazkyBird {
    prover_mode: bool,
    bird: Bird,
    pipes: Vec<Pipe>,
    score: u32,
    high_score: u32,
    current_treacer: Vec<TraceItem>,
    high_score_treacer: Vec<TraceItem>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TraceItem {
    pub action: Action,
    pub data: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum Action {
    NewPlay,
    Jump,
    ApplyPhysics,
    CheckCollisionAndMovePipes,
    GameOver,
}

impl FlazkyBird {
    pub fn new(prover_mode: bool) -> Self {
        let mut pipes = Vec::new();
        let mut x = 300.;
        for _ in 1..=5 {
            let lower = Pipe {
                position: Coord { x, y: -100. },
            };
            pipes.push(lower);
            let upper = Pipe {
                position: Coord { x, y: 350. },
            };
            pipes.push(upper);
            x += 200.;
        }
        Self {
            prover_mode,
            bird: Bird::new(),
            pipes,
            score: 0,
            high_score: 0,
            current_treacer: Vec::new(),
            high_score_treacer: Vec::new(),
        }
    }

    pub fn new_play(&mut self, rand: [i32; 5]) {
        self.bird.position.y = 0.;
        // self.bird.position.rotation = Quat::from_rotation_x(0.);
        self.bird.speed = 0.;
        for (i, pipe) in self.pipes.iter_mut().enumerate() {
            pipe.position.x = 300. + (i/2 * 200) as f32;
            pipe.position.y = -100. + (i%2 * 450) as f32 + rand[i/2] as f32;
        }
        self.score = 0;
        if !self.prover_mode {
            self.current_treacer = Vec::new();
            self.current_treacer.push(TraceItem {
                action: Action::NewPlay,
                data: serialize(&rand).unwrap(),
            });
        }
    }

    pub fn apply_physics(&mut self, delta_seconds: f32) -> bool {
        if !self.prover_mode {
            self.current_treacer.push(TraceItem {
                action: Action::ApplyPhysics,
                data: delta_seconds.to_le_bytes().to_vec(),
            });
        }
        self.bird.position.y += (self.bird.speed + (0.5 * GRAVITY * delta_seconds * delta_seconds)) as f32;
        self.bird.speed += GRAVITY * delta_seconds;
        // self.bird.rotation = Quat::from_rotation_z((self.bird.speed.max(0.).abs() / 50.) as f32);
        if self.bird.position.y < -174. {
            self.game_over();
            self.bird.position.y = -174.;
            return true;
        }
        false
    }

    pub fn jump(&mut self) {
        if !self.prover_mode {
            self.current_treacer.push(TraceItem {
                action: Action::Jump,
                data: Vec::new(),
            });
        }
        if self.bird.position.y < WINDOW_Y / 2. {
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

    pub fn check_collision_and_move_pipes(&mut self, delta_seconds: f32, rand: [i32; 5]) -> (bool, bool) {
        if !self.prover_mode {
            let mut trace =TraceItem {
                action: Action::CheckCollisionAndMovePipes,
                data: delta_seconds.to_le_bytes().to_vec(),
            };
            let rand_bytes = serialize(&rand).unwrap();
            for b in &rand_bytes {
                trace.data.push(*b);
            }
            self.current_treacer.push(trace);
        }
        // check for collision
        for pipe in self.pipes.iter() {
            let half_width1 = BIRD_WIDTH / 2.;
            let half_height1 = BIRD_HEIGHT / 2.;
            let half_width2 = PIPE_WIDTH / 2.;
            let half_height2 = PIPE_HEIGHT / 2.;

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
            pipe.position.x -= delta_seconds * 2. * (100. + self.score.min(100) as f32);
            if self.score < 4 && i%2 == 0 && pipe.position.x < -PIPE_WIDTH {
                init_score += 1;
            }
            if pipe.position.x <= -500. {
                pipe.position.x = 500.0;
                if i%2 == 0 { // lower pipe
                    self.score += 1;
                    level_up = true;
                    pipe.position.y = -100. +  self.score.min(100) as f32 + rand[i/2] as f32;
                } else { // upper pipe
                    pipe.position.y = 350. +  self.score.min(100) as f32 + rand[i/2] as f32;
                }
            }
        }
        if self.score < 4 {
            self.score = init_score;
        }
        (false, level_up)
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn game_over(&mut self) {
        if !self.prover_mode {
            self.current_treacer.push(TraceItem {
                action: Action::GameOver,
                data: Vec::new(),
            });
        }
        if !self.prover_mode {
            if self.score > self.high_score {
                self.high_score = self.score;
                self.high_score_treacer = self.current_treacer.clone();
            }
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
    pub y: f32,
}


struct Bird {
    position: Coord,
    speed: f32,
}

impl Bird {
    pub fn new() -> Self {
        Self {
            speed: 0.,
            position: Coord { x: 0., y: 0. },
        }
    }
}

struct Pipe {
    position: Coord,
}
