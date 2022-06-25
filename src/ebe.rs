use bevy::prelude::*;
use bevy_kira_audio::{AudioSource, Audio};
use rand::{thread_rng, Rng};

pub const GAME_SIZE: (f32, f32) = (256.0, 256.0);
const TIME_STEP: f32 = 1.0 / 60.0;

const PLAYER_SPRITE: &str = "farm.png";
const CHICKEN_SPRITE: &str = "chicken.png";
const CHICKEN_SOUND: &str = "killChicken.wav";
const DOG_SPRITE: &str = "dog.png";
const DOG_SOUND: &str = "dog03.wav";

pub struct GameTextures {
    player: Handle<Image>,
    chicken: Handle<Image>,
    chicken_sound: Handle<AudioSource>,
    dog: Handle<Image>,
    dog_sound: Handle<AudioSource>,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Moveable {
    auto_despawn: bool,
    vel_x: f32,
    vel_y: f32,
}

pub struct EbeGamePlugin;

impl Plugin for EbeGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(ClearColor(Color::OLIVE))
            .add_startup_system(setup_system)
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                spawn_player_system
            )
            .add_system(moveable_system)
            .add_system(mouse_events);
    }
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(
        GameTextures {
            player: asset_server.load(PLAYER_SPRITE),
            chicken: asset_server.load(CHICKEN_SPRITE),
            chicken_sound: asset_server.load(CHICKEN_SOUND),
            dog: asset_server.load(DOG_SPRITE),
            dog_sound: asset_server.load(DOG_SOUND),
        }
    );
}

fn spawn_player_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.player.clone(),
            ..Default::default()
        })
        .insert(Player);
}

fn mouse_events(
    mut commands: Commands,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    game_textures: Res<GameTextures>,
    audio: Res<Audio>,
) {
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let window = windows.get_primary().unwrap();
    if let Some(position) = window.cursor_position() {
        x = position.x - (window.width()/2.0);
        y = position.y - (window.height()/2.0);
    }
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        let mut rng = thread_rng();
        let vel_x: f32 = rng.gen_range(-1.0..1.0);
        let vel_y: f32 = rng.gen_range(-1.0..-0.2);
        let is_chicken: bool = rng.gen_bool(1.0/3.0);
        let sp_texture = if is_chicken {
            game_textures.chicken.clone()
        } else {
            game_textures.dog.clone()
        };
        commands
            .spawn_bundle(SpriteBundle {
                texture: sp_texture,
                transform: Transform {
                    translation: Vec3::new(x, y, 10.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Moveable {
                auto_despawn: true,
                vel_x: vel_x,
                vel_y: vel_y,
            });
        if is_chicken {
            audio.play(game_textures.chicken_sound.clone());
        } else {
            audio.play(game_textures.dog_sound.clone());
        }
    }
}

fn moveable_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Moveable), With<Moveable>>
) {
    for (ent, mut tf, moveable) in query.iter_mut() {
        let translation = &mut tf.translation;
        translation.x += 50.0*moveable.vel_x*TIME_STEP;
        translation.y += 100.0*moveable.vel_y*TIME_STEP;
        if moveable.auto_despawn {
            const MARGIN: f32 = 200.0;
            if translation.y > (GAME_SIZE.1/2.0) + MARGIN
                || translation.y < -(GAME_SIZE.1/2.0) - MARGIN
                || translation.x > (GAME_SIZE.0/2.0) + MARGIN
                || translation.x < -(GAME_SIZE.0/2.0) - MARGIN
            {
                // println!("killed {:?}", ent);
                commands.entity(ent).despawn();
            }
        }
    }
}
