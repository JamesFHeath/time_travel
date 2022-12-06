#![allow(clippy::redundant_field_names, clippy::type_complexity)]
use bevy::sprite::collide_aabb::collide;
use bevy::{prelude::*, window::close_on_esc};
use bevy_prototype_lyon::prelude::*;
use iyes_loopless::prelude::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 100.0;
pub const BACKGROUND_ONE: f32 = 0.0;
pub const PLAYER_LEVEL: f32 = 200.0;

mod background;
mod camera;
mod components;
mod events;
mod player;
mod resources;

use background::BackgroundPlugin;
use camera::CameraPlugin;
use components::{Collidable, Interactable};
use events::{EventPlugin, InteractionEvent};
use player::{FacingDirection, PlayerPlugin};
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
        .add_plugin(ShapePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(BackgroundPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EventPlugin)
        .add_startup_system(draw_collidable)
        .add_system(close_on_esc)
        .add_system(some_interaction.run_on_event::<InteractionEvent>())
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

fn check_interaction(
    mut pdi_translation: Vec3,
    facing_direction: FacingDirection,
    interaction_translation: Vec3,
) {
    // let interaction_box = Vec2::new(TILE_SIZE * 2.0, TILE_SIZE);
    let tile = Vec2::new(TILE_SIZE, TILE_SIZE);
    let scaled_tile_size = TILE_SIZE * 0.5;
    // println!("PDI: {}", pdi_translation);
    // println!("BOX: {}", interaction_translation);
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
    if collide(
        pdi_translation,
        interaction_box,
        interaction_translation,
        tile,
    )
    .is_some()
    {
        println!("INTERACTION HIT");
    }
}

fn some_interaction(
    mut commands: Commands,
    query: Query<&Transform, With<Interactable>>,
    mut event: EventReader<InteractionEvent>,
) {
    for interaction_event in event.iter() {
        for transform in query.iter() {
            check_interaction(
                interaction_event.pdi_translation(),
                interaction_event.facing_direction(),
                transform.translation,
            );
        }
    }
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
        Interactable(),
    ));
}
