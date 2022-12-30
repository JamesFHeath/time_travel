#![allow(clippy::redundant_field_names, clippy::type_complexity)]
#![allow(unused_imports)]
use bevy::sprite::collide_aabb::collide;
use bevy::{prelude::*, window::close_on_esc};
use bevy_prototype_lyon::prelude::*;
use iyes_loopless::prelude::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 100.0;
pub const SCREEN_WIDTH: f32 = TILE_SIZE * 16.0;
pub const SCREEN_HEIGHT: f32 = TILE_SIZE * 9.0;
pub const BACKGROUND_ONE: f32 = 0.0;
pub const PLAYER_LEVEL: f32 = 200.0;

mod background;
mod camera;
mod collisionsmod;
mod events;
mod playermod;
mod resources;
mod systemsmod;

use background::BackgroundPlugin;
use camera::CameraPlugin;
use collisionsmod::collisions::*;
use collisionsmod::components::*;
use collisionsmod::*;
use events::{EventPlugin, InteractionEvent};
use playermod::player::*;
use playermod::skills::*;
use playermod::*;
use resources::KeyBindings;
use systemsmod::general_systems::*;
use systemsmod::*;

fn main() {
    let height = 540.0;
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: height * RESOLUTION,
                height: height,
                title: "Time Travel".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..Default::default()
        }))
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(ShapePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(BackgroundPlugin)
        .add_plugins(PlayerModPluginGroup)
        .add_plugins(SystemsModPluginGroup)
        .add_plugins(CollisionsModPluginGroup)
        .add_plugin(EventPlugin)
        .add_system(close_on_esc)
        .init_resource::<KeyBindings>()
        .run();
}
