#![allow(clippy::redundant_field_names, clippy::type_complexity)]
use bevy::ecs::component::ComponentId;
use bevy::ecs::query::WorldQuery;
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
mod components;
mod events;
mod playermod;
mod resources;

use background::BackgroundPlugin;
use camera::CameraPlugin;
use components::*;
use events::{EventPlugin, InteractionEvent};
use playermod::player::*;
use playermod::skills::*;
use playermod::*;
use resources::KeyBindings;

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
        .add_plugin(EventPlugin)
        .add_startup_system(draw_collidable)
        .add_system(close_on_esc)
        .add_system(manage_interaction_events.run_on_event::<InteractionEvent>())
        .init_resource::<KeyBindings>()
        .run();
}

fn check_collision(
    player_translation: &Vec3,
    player_entity: &Entity,
    collidable_entity: &[(Vec3, Entity)],
) -> bool {
    let tile = Vec2::new(TILE_SIZE, TILE_SIZE);
    for (collidable_translation, collidable_entity) in collidable_entity.iter() {
        if player_entity.index() != collidable_entity.index()
            && collide(*player_translation, tile, *collidable_translation, tile).is_some()
        {
            return true;
        };
    }
    false
}

fn check_interaction(
    mut pdi_translation: Vec3,
    facing_direction: FacingDirection,
    interaction_translation: Vec3,
) -> bool {
    let tile = Vec2::new(TILE_SIZE, TILE_SIZE);
    let scaled_tile_size = TILE_SIZE * 0.5;
    let interaction_box = match facing_direction {
        FacingDirection::Up => {
            pdi_translation.y += scaled_tile_size;
            Vec2::new(scaled_tile_size, scaled_tile_size)
        }
        FacingDirection::Down => {
            pdi_translation.y -= scaled_tile_size;
            Vec2::new(scaled_tile_size, scaled_tile_size)
        }
        FacingDirection::Left => {
            pdi_translation.x -= scaled_tile_size;
            Vec2::new(scaled_tile_size, scaled_tile_size)
        }
        FacingDirection::Right => {
            pdi_translation.x += scaled_tile_size;
            Vec2::new(scaled_tile_size, scaled_tile_size)
        }
    };
    collide(
        pdi_translation,
        interaction_box,
        interaction_translation,
        tile,
    )
    .is_some()
}

fn manage_interaction_events(
    mut commands: Commands,
    query: Query<(&Transform, Entity), With<Interactable>>,
    mut event: EventReader<InteractionEvent>,
) {
    for interaction_event in event.iter() {
        for (transform, entity) in query.iter() {
            let is_interacted = check_interaction(
                interaction_event.pdi_translation(),
                interaction_event.facing_direction(),
                transform.translation,
            );
            if is_interacted {
                commands.entity(entity).insert(InteractedWith());
                println!("INTERACTION HIT");
            }
        }
    }
}

#[derive(Component)]
struct CollidableTimer(Timer);

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
        Interactable(),
        CollidableTimer(Timer::from_seconds(1.0, TimerMode::Once)),
    ));
}
