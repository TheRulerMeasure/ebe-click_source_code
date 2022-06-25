use bevy_kira_audio::AudioPlugin;
use bevy::prelude::{App, DefaultPlugins, WindowDescriptor};
use ebe::{EbeGamePlugin, GAME_SIZE};

mod ebe;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "ebe click".to_string(),
            width: GAME_SIZE.0,
            height: GAME_SIZE.1,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(EbeGamePlugin)
        .run();
}
