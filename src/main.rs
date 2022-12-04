#![allow(clippy::redundant_field_names, clippy::type_complexity)]
use bevy::sprite::collide_aabb::collide;
use bevy::{prelude::*, render::camera::ScalingMode, window::close_on_esc};
use bevy_prototype_lyon::prelude::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 100.0;
pub const BACKGROUND_ONE: f32 = 0.0;
pub const PLAYER_LEVEL: f32 = 200.0;

mod background;
mod player;
mod resources;

use background::BackgroundPlugin;
use player::PlayerPlugin;

#[derive(Component)]
struct Collidable();

#[derive(Resource)]
struct KeyBindings {
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
    interact: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            interact: KeyCode::I,
        }
    }
}

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
        .add_plugin(ShapePlugin)
        .add_plugin(BackgroundPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(draw_collidable)
        .add_system(close_on_esc)
        .init_resource::<KeyBindings>()
        .run();
}

fn check_collision(
    player_translation: Vec3,
    player_entity: Entity,
    collidable_entity: Vec<(Vec3, Entity)>,
) -> bool {
    let tile = Vec2::new(TILE_SIZE, TILE_SIZE);
    for (collidable_translation, collidable_entity) in collidable_entity.iter() {
        if player_entity.index() != collidable_entity.index()
            && collide(player_translation, tile, *collidable_translation, tile).is_some()
        {
            return true;
        };
    }
    false
}

fn draw_collidable(mut commands: Commands) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(TILE_SIZE, TILE_SIZE),
        origin: RectangleOrigin::Center,
    };
    commands.spawn((
        GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::YELLOW_GREEN),
                outline_mode: StrokeMode::new(Color::BLACK, TILE_SIZE / 10.0),
            },
            Transform::from_translation(Vec3::new(5.0 * TILE_SIZE, 0.0 * TILE_SIZE, PLAYER_LEVEL)),
        ),
        Collidable(),
    ));
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    // camera.projection.top = 1.0;
    // camera.projection.bottom = -1.0;
    camera.projection.top = TILE_SIZE * 4.5;
    camera.projection.bottom = -TILE_SIZE * 4.5;

    // camera.projection.right = 1.0 * RESOLUTION;
    // camera.projection.left = -1.0 * RESOLUTION;
    camera.projection.right = TILE_SIZE * 8.0;
    camera.projection.left = -TILE_SIZE * 8.0;

    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn(camera);
}
