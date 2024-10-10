use bevy::prelude::*;
use rand::Rng;
use bevy::math::UVec2;
use bevy_asset::AssetMetaCheck;
use flazky_bird_lib::FlazkyBird;
// use bincode;
// use std::fs::File;
// use std::io::Write;
use hex;
use wasm_bindgen::prelude::*;



#[derive(Component)]
struct GameLogic {
    flazky_bird: FlazkyBird,
}

#[derive(Component, Deref, DerefMut)]
struct GameLogicTimerPhysics(Timer);

#[derive(Component, Deref, DerefMut)]
struct GameLogicTimerJump(Timer);

#[derive(Component, Deref, DerefMut)]
struct GameLogicTimerCollisions(Timer);

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Base;

#[derive(Component)]
struct Pipes;


#[derive(Component)]
struct PressSpace;

#[derive(Component)]
struct ScoreDisplay;

#[derive(Resource, Default)]
struct Score {
    value: u32,
}

#[derive(Resource, Default)]
struct GameState {
    game_over: bool,
    first_start: bool,
}

#[derive(Component)]
struct GameOverDisplay;


#[derive(Component, Deref, DerefMut)]
struct GravityTimer(Timer);

#[derive(Component)]
struct BirdAnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Event)]
struct GameOverEvent();


const WINDOW_Y: f32 = 512.;
const WINDOW_X: f32 = 800.;

fn main() {
    App::new()
        .init_resource::<Score>()
        .init_resource::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, display_score)
        .add_systems(Update, jump)
        .add_systems(Update, animate_bird.run_if(game_is_active))
        .add_systems(Update, animate_press_space.run_if(game_is_not_active))
        .add_systems(Update, physics.run_if(game_is_active))
        .add_systems(Update, move_bg.run_if(game_is_active))
        .add_systems(Update, move_base.run_if(game_is_active))
        .add_systems(Update, move_pipes_and_game_logic.run_if(game_is_active))
        .add_event::<GameOverEvent>()
        .add_systems(Update, game_over_event)
        .add_plugins(
            DefaultPlugins
                .set(
                    WindowPlugin {
                        primary_window: Some(Window {
                            title: "Rusty Bird".to_string(),
                            resolution: (WINDOW_X, WINDOW_Y).into(),
                            resizable: false,
                            ..default()
                        }),
                        ..default()
                    }
                ).set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut game_over: ResMut<GameState>,
) {
    game_over.game_over = true;
    game_over.first_start = true;
    let background_image = asset_server.load("sprites/background-day.png");
    let number_image = asset_server.load("sprites/numbers.png");
    let base_image = asset_server.load("sprites/base.png");
    let game_over_image = asset_server.load("sprites/game-over.png");
    let pipe = asset_server.load("sprites/pipe.png");
    let bird = asset_server.load("sprites/bluebird2.png");
    let space = asset_server.load("sprites/space.png");

    let pos = Vec3::new(0., 0., 0.);
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: background_image,
            transform: Transform::from_translation(pos),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1400.0, 512.0)),
                ..default()
            },
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: false,
            stretch_value: 1.,
        },
        Background,
    ));

    commands.spawn((
        SpriteBundle {
            texture: base_image,
            transform: Transform::from_xyz(-0., -230., 5.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1400.0, 112.0)),
                ..default()
            },
            ..Default::default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: false,
            stretch_value: 1.,
        },
        Base,
    ));

    commands.spawn((
        SpriteBundle {
            texture: game_over_image,
            transform: Transform::from_xyz(0., 0., 10.),
            visibility: Visibility::Hidden,
            ..default()
        },
        GameOverDisplay,
    ));

    commands.spawn((
        SpriteBundle {
            texture: space,
            transform: Transform::from_xyz(0., -50., 10.),
            ..default()
        },
        PressSpace,
        AnimationTimer(Timer::from_seconds(0.75, TimerMode::Repeating)),
    ));

    let mut x = -250.;
    for _i in 0..=4 {
        commands.spawn((
            SpriteBundle {
                texture: number_image.clone(),
                transform: Transform::from_xyz(x, 220., 10.),
                sprite: Sprite {
                    rect: Some(Rect::new(0., 0., 24., 36.)),
                    ..default()
                },
                ..Default::default()
            },
            ScoreDisplay,
        ));

        x -= 26.;
    }

    let layout = TextureAtlasLayout::from_grid(UVec2::new(34, 24), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = BirdAnimationIndices {
        first: 0,
        last: 2,
    };
    commands.spawn((
        GameLogic {flazky_bird: FlazkyBird::new(false)},
        GameLogicTimerPhysics(Timer::from_seconds(0.03, TimerMode::Repeating)), // ~30 fps
        GameLogicTimerJump(Timer::from_seconds(0.03, TimerMode::Repeating)), // ~30 fps
        GameLogicTimerCollisions(Timer::from_seconds(0.03, TimerMode::Repeating)), // ~30 fps
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture: bird,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_xyz(0., 0., 4.),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        GravityTimer(Timer::from_seconds(0.02, TimerMode::Repeating)),
    ));

    x = 300.;
    for _ in 1..=5 {
        let mut transform = Transform::from_xyz(x, -100., 3.);
        commands.spawn((
            SpriteBundle {
                texture: pipe.clone(),
                transform: transform.clone(),
                ..default()
            },
            Pipes,
        ));

        transform.rotate_local_x(std::f32::consts::PI);
        transform.translation.y += 450.;
        commands.spawn((
            SpriteBundle {
                texture: pipe.clone(),
                transform,
                ..default()
            },
            Pipes,
        ));
        x += 200.;
    }
}

fn game_is_active(game_over: Res<GameState>) -> bool {
    return !game_over.game_over;
}

fn game_is_not_active(game_over: Res<GameState>) -> bool {
    return game_over.game_over;
}

fn animate_bird(
    time: Res<Time>,
    mut query: Query<(
        &mut BirdAnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
    )>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn animate_press_space(
    time: Res<Time>,
    mut query: Query<(&mut Visibility, &mut AnimationTimer), With<PressSpace>>,
) {
    for (mut vis, mut timer) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if matches!(*vis, Visibility::Visible) {
                *vis = Visibility::Hidden;
            } else {
                *vis = Visibility::Visible;
            }
        }
    }
}

fn physics(
    game_over: ResMut<GameState>,
    time: Res<Time>,
    mut game_logic_query: Query<(&mut GameLogic,&mut GameLogicTimerPhysics)>,
    mut bird_query: Query<&mut Transform, (With<BirdAnimationIndices>, Without<Pipes>)>,
    mut ev_game_over: EventWriter<GameOverEvent>,
) {
    if game_over.game_over {
        return;
    }
    let (mut gl, mut timer) = game_logic_query.single_mut();
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }
    let mut bird = bird_query.single_mut();
    if gl.flazky_bird.apply_physics(0.03+timer.elapsed_secs()) {
        ev_game_over.send(GameOverEvent());
    }
    let bird_position = gl.flazky_bird.bird_position();
    bird.translation.y = bird_position.y;
    // bird.rotation = bird_position.rotation;
}

fn game_over_event(
    mut game_over: ResMut<GameState>,
    mut game_over_and_space_query: Query<
        &mut Visibility,
        Or<(With<GameOverDisplay>, With<PressSpace>)>,
    >,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_game_over: EventReader<GameOverEvent>,
    game_logic_query: Query<&mut GameLogic>,
) {
    for _ in ev_game_over.read() {
        if !game_over.game_over {
            let gl = game_logic_query.single();
            let score = gl.flazky_bird.score();
            let high_score = gl.flazky_bird.get_high_score();
            if score == high_score && high_score > 0 {
                alert("New high score! Go to logs to grab the trace");
                let high_score_treacer = gl.flazky_bird.get_high_score_treacer();
                // let file_name = format!("trace_{}.bin", high_score);
                // let mut file = File::create(file_name).unwrap();
                let serialized = bincode::serialize(&high_score_treacer).unwrap();
                
                println!("trace for score {}: {}", high_score, hex::encode(&serialized));
                log(&format!("trace for score {}: {}", high_score, hex::encode(&serialized)));
                // file.write_all(&serialized).unwrap();
            }
        }
        game_over.game_over = true;
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/hit.ogg"),
            settings: PlaybackSettings::DESPAWN,
            ..default()
        });
        for mut vis in game_over_and_space_query.iter_mut() {
            *vis = Visibility::Visible;
        }
    }
}

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn display_score(
    mut score: ResMut<Score>,
    mut sprites: Query<&mut Sprite, With<ScoreDisplay>>,
    game_logic_query: Query<&mut GameLogic>,
) {
    let mut current_score = game_logic_query.single().flazky_bird.score();
    score.value = current_score;
    // if score.is_changed() {
    //     return;
    // }

    let mut digits = Vec::new();
    if current_score == 0 {
        for _i in 0..=4 {
            digits.push(0);
        }
    } else {
        while current_score > 0 {
            let digit = current_score % 10;
            digits.push(digit);
            current_score /= 10;
        }
    }
    for (i, mut sprite) in sprites.iter_mut().enumerate() {
        if let Some(digit) = digits.get(i) {
            sprite.rect = Some(Rect::new(
                0.,
                (*digit as f32) * 36.,
                24.,
                ((*digit as f32) + 1.) * 36.,
            ));
        }
    }
}

fn jump(
    mut game_over: ResMut<GameState>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pipe_query: Query<&mut Transform, (Without<BirdAnimationIndices>, With<Pipes>)>,
    mut bird_query: Query<&mut Transform, (With<BirdAnimationIndices>, Without<Pipes>)>,
    mut game_logic_query: Query<&mut GameLogic>,
    mut game_over_and_space_query: Query<
        &mut Visibility,
        Or<(With<GameOverDisplay>, With<PressSpace>)>,
    >,
) {
    let mut gl = game_logic_query.single_mut();
    if input.just_pressed(KeyCode::Space) {
        if !game_over.game_over {
            let mut bird = bird_query.single_mut();
            gl.flazky_bird.jump();
            let bird_position = gl.flazky_bird.bird_position();
            bird.translation.y = bird_position.y;
            // bird.rotation = bird_position.rotation;
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/wing.ogg"),
                settings: PlaybackSettings::DESPAWN,
                ..default()
            });
        } else {
            if game_over.first_start {
                game_over.first_start = false;
                let mut rand = [0; 5];
                let mut rng = rand::thread_rng();
                for i in 0..rand.len() {
                    rand[i] = rng.gen_range(-200..75);
                }
                gl.flazky_bird.new_play(rand);
            } else {
                let mut bird = bird_query.single_mut();
                let mut rand = [0; 5];
                let mut rng = rand::thread_rng();
                for i in 0..rand.len() {
                    rand[i] = rng.gen_range(-200..75);
                }
                gl.flazky_bird.new_play(rand);
                let bird_position = gl.flazky_bird.bird_position();
                bird.translation.y = bird_position.y;
                // bird.rotation = bird_position.rotation;
                let i = 0;
                let pipe_positions = gl.flazky_bird.get_pipe_positions();
                for mut pipe in pipe_query.iter_mut() {
                    pipe.translation.x = pipe_positions[i].x;
                }
            }

            for mut vis in game_over_and_space_query.iter_mut() {
                *vis = Visibility::Hidden;
            }
            game_over.game_over = false;
        }
    }
}

fn move_bg(
    time: Res<Time>,
    game_logic_query: Query<&mut GameLogic>,
    mut bg_query: Query<&mut Transform, With<Background>>,
) {
    let delta_seconds = time.delta_seconds();
    let score = game_logic_query.single().flazky_bird.score();
    for mut transform in bg_query.iter_mut() {
        transform.translation.x -= delta_seconds * (100. + score.min(100) as f32);
        if transform.translation.x <= -288. {
            transform.translation.x = 0.0;
        }
    }
}

fn move_base(
    time: Res<Time>,
    game_logic_query: Query<&mut GameLogic>,
    mut base_query: Query<&mut Transform, With<Base>>,
) {
    let score = game_logic_query.single().flazky_bird.score();
    let delta_seconds = time.delta_seconds();
    for mut transform in base_query.iter_mut() {
        transform.translation.x -= delta_seconds * 2. * (100. + score.min(100) as f32);
        if transform.translation.x <= -288. {
            transform.translation.x = 0.0;
        }
    }
}

fn move_pipes_and_game_logic(
    game_over: ResMut<GameState>,
    time: Res<Time>,
    mut pipe_query: Query<&mut Transform, (Without<BirdAnimationIndices>, With<Pipes>)>,
    mut game_logic_query: Query<(&mut GameLogic, &mut GameLogicTimerCollisions)>,
    mut ev_game_over: EventWriter<GameOverEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if game_over.game_over {
        return;
    }
    let (mut gl, mut timer) = game_logic_query.single_mut();
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }
    let mut rand = [0; 5];
    let mut rng = rand::thread_rng();
    for i in 0..rand.len() {
        rand[i] = rng.gen_range(-200..75);
    }
    let (game_over, level_up) = gl.flazky_bird.check_collision_and_move_pipes(0.03+timer.elapsed_secs(), rand);
    if game_over {
        ev_game_over.send(GameOverEvent());
    }
    if level_up {
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/point.ogg"),
            settings: PlaybackSettings::DESPAWN,
            ..default()
        });
    }
    // update pipe graphics
    let pipe_positions = gl.flazky_bird.get_pipe_positions();
    for (i, mut pipe) in pipe_query.iter_mut().enumerate() {
        pipe.translation.x = pipe_positions[i].x;
        pipe.translation.y = pipe_positions[i].y;
    }
}
