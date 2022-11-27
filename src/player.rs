use std::cmp::min;

use float_cmp::approx_eq;
use float_ord::FloatOrd;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_prototype_lyon::prelude::*;

use crate::{PLAYER_LEVEL, TILE_SIZE};

pub struct PlayerPlugin;

#[derive(PartialEq)]
enum MovementDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NEUTRAL,
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    movement_direction: MovementDirection,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            // .add_system(camera_follow.after("movement"))
            .add_system(player_movement.label("movement"));
    }
}

// fn camera_follow(
//     player_query: Query<&Transform, With<Player>>,
//     mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
// ) {
//     let player_transform = player_query.single();
//     let mut camera_transform = camera_query.single_mut();

//     camera_transform.translation.x = player_transform.translation.x;
//     camera_transform.translation.y = player_transform.translation.y;
// }

fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    // wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    let mut y_delta = 0.0;
    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::W) & [MovementDirection::NEUTRAL, MovementDirection::UP].contains(&player.movement_direction) {
        y_delta += player.speed * TILE_SIZE * time.delta_seconds();
        player.movement_direction = MovementDirection::UP;
    }
    else if keyboard.pressed(KeyCode::S) & [MovementDirection::NEUTRAL, MovementDirection::DOWN].contains(&player.movement_direction){
        y_delta -= player.speed * TILE_SIZE * time.delta_seconds();
        player.movement_direction = MovementDirection::DOWN;
    }
    else if keyboard.pressed(KeyCode::A) & [MovementDirection::NEUTRAL, MovementDirection::LEFT].contains(&player.movement_direction){
        x_delta -= player.speed * TILE_SIZE * time.delta_seconds();
        player.movement_direction = MovementDirection::LEFT;
    }
    else if keyboard.pressed(KeyCode::D) & [MovementDirection::NEUTRAL, MovementDirection::RIGHT].contains(&player.movement_direction){
        x_delta += player.speed * TILE_SIZE * time.delta_seconds();
        player.movement_direction = MovementDirection::RIGHT;
    }
    else if player.movement_direction == MovementDirection::UP {
        let distance_to_tile = transform.translation.y.abs() % TILE_SIZE;
        y_delta += min(FloatOrd((player.speed * TILE_SIZE * time.delta_seconds()).abs()), FloatOrd(distance_to_tile)).0;
        if !(5.0..=95.0).contains(&distance_to_tile) {
            player.movement_direction = MovementDirection::NEUTRAL;
        } 
    }
    else if player.movement_direction == MovementDirection::DOWN {
        let distance_to_tile = transform.translation.y.abs() % TILE_SIZE;
        y_delta -= min(FloatOrd((player.speed * TILE_SIZE * time.delta_seconds()).abs()), FloatOrd(distance_to_tile)).0;
        if !(5.0..=95.0).contains(&distance_to_tile) {
            player.movement_direction = MovementDirection::NEUTRAL;
        } 
    }
    else if player.movement_direction == MovementDirection::LEFT {
        let distance_to_tile = transform.translation.x.abs() % TILE_SIZE;
        x_delta -= min(FloatOrd((player.speed * TILE_SIZE * time.delta_seconds()).abs()), FloatOrd(distance_to_tile)).0;
        if !(5.0..=95.0).contains(&distance_to_tile) {
            player.movement_direction = MovementDirection::NEUTRAL;
        } 
    }
    else if player.movement_direction == MovementDirection::RIGHT {
        let distance_to_tile = transform.translation.x.abs() % TILE_SIZE;
        x_delta += min(FloatOrd((player.speed * TILE_SIZE * time.delta_seconds()).abs()), FloatOrd(distance_to_tile)).0;
        if !(5.0..=95.0).contains(&distance_to_tile) {
            player.movement_direction = MovementDirection::NEUTRAL;
        } 
    }

    // if keyboard.just_released(KeyCode::W) & (player.movement_direction == MovementDirection::UP) {
    //     player.movement_direction = MovementDirection::NEUTRAL;
    // }
    // if keyboard.just_released(KeyCode::S) & (player.movement_direction == MovementDirection::DOWN) {
    //     player.movement_direction = MovementDirection::NEUTRAL;
    // }
    // if keyboard.just_released(KeyCode::A) & (player.movement_direction == MovementDirection::LEFT) {
    //     player.movement_direction = MovementDirection::NEUTRAL;
    // }
    // if keyboard.just_released(KeyCode::D) & (player.movement_direction == MovementDirection::RIGHT) {
    //     player.movement_direction = MovementDirection::NEUTRAL;
    // }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    transform.translation = target;
    // if wall_collision_check(target, &wall_query) {
    //     transform.translation = target;
    // }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    transform.translation = target;
    // if wall_collision_check(target, &wall_query) {
    //     transform.translation = target;
    // }
}

// fn wall_collision_check(
//     target_player_pos: Vec3,
//     wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
// ) -> bool {
//     for wall_transform in wall_query.iter() {
//         let collision = collide(
//             target_player_pos,
//             Vec2::splat(TILE_SIZE * 0.9),
//             wall_transform.translation,
//             Vec2::splat(TILE_SIZE),
//         );
//         if collision.is_some() {
//             return false;
//         }
//     }
//     true
// }

fn spawn_player(mut commands: Commands) {
    let shape = shapes::Circle {
        radius: TILE_SIZE / 2.0,
        center: Vec2::ZERO,
        // feature: shapes::RegularPolygonFeature::Radius(TILE_SIZE / 2.0),
        // ..shapes::RegularPolygon::default()
    };

    commands
        .spawn(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, TILE_SIZE / 10.0),
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, PLAYER_LEVEL)),
        ))
        .insert(Player { speed: 3.0, movement_direction: MovementDirection::NEUTRAL });
}

// fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
//     let player = spawn_ascii_sprite(
//         &mut commands,
//         &ascii,
//         1,
//         Color::rgb(0.3, 0.3, 0.9),
//         Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
//     );

//     commands
//         .entity(player)
//         .insert(Name::new("Player"))
//         .insert(Player { speed: 3.0 })
//         .id();

//     let background = spawn_ascii_sprite(
//         &mut commands,
//         &ascii,
//         0,
//         Color::rgb(0.5, 0.5, 0.5),
//         Vec3::new(0.0, 0.0, -1.0),
//     );

//     commands
//         .entity(background)
//         .insert(Name::new("Background"))
//         .id(); //id() gives back the entity after creation

//     commands.entity(player).push_children(&[background]);
// }
