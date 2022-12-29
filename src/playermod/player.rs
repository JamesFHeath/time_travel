use std::{cmp::min, f32::consts::PI};

use float_cmp::approx_eq;
use float_ord::FloatOrd;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player.label("playerspawn"))
            .add_system(player_movement.label("movement"))
            .add_system(camera_follow.after("movement"))
            .add_system(rotate_player_direction_indicator.after("movement"))
            .add_system(interact);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MovementDirection {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Neutral = 4,
}

#[derive(Clone, Component, Copy)]
pub enum FacingDirection {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Component)]
pub struct PlayerDirectionIndicator {
    lock_rotation_up: bool,
    lock_rotation_down: bool,
    lock_rotation_left: bool,
    lock_rotation_right: bool,
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    movement_direction: MovementDirection,
}

fn interact(
    player_query: Query<(&GlobalTransform, &FacingDirection), With<PlayerDirectionIndicator>>,
    mut inter_event_writer: EventWriter<InteractionEvent>,
    keyboard: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
) {
    if keyboard.just_pressed(key_bindings.interact) {
        let (pdi_transform, facing_direction) = player_query.single();
        inter_event_writer.send(InteractionEvent::new(
            pdi_transform.translation(),
            *facing_direction,
        ));
    }
}

pub fn get_manual_movement_speed(player_speed: f32, delta_seconds: f32) -> f32 {
    (player_speed * TILE_SIZE * delta_seconds) as i32 as f32
}

fn rotate_player_direction_indicator(
    mut pdi_query: Query<
        (
            &mut Transform,
            &mut FacingDirection,
            &mut PlayerDirectionIndicator,
        ),
        (With<PlayerDirectionIndicator>, Without<Player>),
    >,
    player_query: Query<&Player, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
) {
    let player = player_query.single();
    let (mut pdi_transform, mut facing_direction, mut pdi) = pdi_query.single_mut();
    let rotation_angle: f32;

    if player.movement_direction != MovementDirection::Neutral
        && player.movement_direction as u8 != *facing_direction as u8
    {
        rotation_angle = match player.movement_direction {
            MovementDirection::Up => match *facing_direction {
                FacingDirection::Up => 0.0,
                FacingDirection::Down => PI,
                FacingDirection::Left => 3.0 * PI / 2.0,
                FacingDirection::Right => PI / 2.0,
            },
            MovementDirection::Down => match *facing_direction {
                FacingDirection::Up => PI,
                FacingDirection::Down => 0.0,
                FacingDirection::Left => PI / 2.0,
                FacingDirection::Right => 3.0 * PI / 2.0,
            },
            MovementDirection::Left => match *facing_direction {
                FacingDirection::Up => PI / 2.0,
                FacingDirection::Down => 3.0 * PI / 2.0,
                FacingDirection::Left => 0.0,
                FacingDirection::Right => PI,
            },
            MovementDirection::Right => match *facing_direction {
                FacingDirection::Up => 3.0 * PI / 2.0,
                FacingDirection::Down => PI / 2.0,
                FacingDirection::Left => PI,
                FacingDirection::Right => 0.0,
            },
            MovementDirection::Neutral => 0.0,
        };
        *facing_direction = match player.movement_direction {
            MovementDirection::Up => FacingDirection::Up,
            MovementDirection::Down => FacingDirection::Down,
            MovementDirection::Left => FacingDirection::Left,
            MovementDirection::Right => FacingDirection::Right,
            MovementDirection::Neutral => *facing_direction,
        };
    } else if player.movement_direction == MovementDirection::Neutral {
        if keyboard.pressed(key_bindings.up) && !pdi.lock_rotation_up {
            pdi.lock_rotation_up = true;
            pdi.lock_rotation_down = false;
            pdi.lock_rotation_left = false;
            pdi.lock_rotation_right = false;
            rotation_angle = match *facing_direction {
                FacingDirection::Up => 0.0,
                FacingDirection::Down => PI,
                FacingDirection::Left => 3.0 * PI / 2.0,
                FacingDirection::Right => PI / 2.0,
            };
            *facing_direction = FacingDirection::Up;
        } else if keyboard.pressed(key_bindings.down) && !pdi.lock_rotation_down {
            pdi.lock_rotation_down = true;
            pdi.lock_rotation_up = false;
            pdi.lock_rotation_left = false;
            pdi.lock_rotation_right = false;
            rotation_angle = match *facing_direction {
                FacingDirection::Up => PI,
                FacingDirection::Down => 0.0,
                FacingDirection::Left => PI / 2.0,
                FacingDirection::Right => 3.0 * PI / 2.0,
            };
            *facing_direction = FacingDirection::Down;
        } else if keyboard.pressed(key_bindings.left) && !pdi.lock_rotation_left {
            pdi.lock_rotation_left = true;
            pdi.lock_rotation_up = false;
            pdi.lock_rotation_down = false;
            pdi.lock_rotation_right = false;
            rotation_angle = match *facing_direction {
                FacingDirection::Up => PI / 2.0,
                FacingDirection::Down => 3.0 * PI / 2.0,
                FacingDirection::Left => 0.0,
                FacingDirection::Right => PI,
            };
            *facing_direction = FacingDirection::Left;
        } else if keyboard.pressed(key_bindings.right) && !pdi.lock_rotation_right {
            pdi.lock_rotation_right = true;
            pdi.lock_rotation_up = false;
            pdi.lock_rotation_down = false;
            pdi.lock_rotation_left = false;
            rotation_angle = match *facing_direction {
                FacingDirection::Up => 3.0 * PI / 2.0,
                FacingDirection::Down => PI / 2.0,
                FacingDirection::Left => PI,
                FacingDirection::Right => 0.0,
            };
            *facing_direction = FacingDirection::Right;
        } else {
            rotation_angle = 0.0;
        }
    } else {
        rotation_angle = 0.0;
    }
    pdi_transform.rotate_around(
        Vec3::new(0.0, 0.0, 0.0),
        Quat::from_rotation_z(rotation_angle),
    );
}

fn get_auto_movement_speed(transform: &Transform, delta_seconds: &f32, player: &mut Player) -> f32 {
    let distance_to_tile;
    match player.movement_direction {
        MovementDirection::Up => {
            let signed_distance = transform.translation.y % TILE_SIZE;
            if FloatOrd(signed_distance) >= FloatOrd(0.0) {
                distance_to_tile = TILE_SIZE - signed_distance;
            } else {
                distance_to_tile = signed_distance.abs();
            }
        }
        MovementDirection::Down => {
            let signed_distance = transform.translation.y % TILE_SIZE;
            if FloatOrd(signed_distance) >= FloatOrd(0.0) {
                distance_to_tile = signed_distance;
            } else {
                distance_to_tile = TILE_SIZE - signed_distance.abs();
            }
        }
        MovementDirection::Left => {
            let signed_distance = transform.translation.x % TILE_SIZE;
            if FloatOrd(signed_distance) >= FloatOrd(0.0) {
                distance_to_tile = signed_distance;
            } else {
                distance_to_tile = TILE_SIZE - signed_distance.abs();
            }
        }
        MovementDirection::Right => {
            let signed_distance = transform.translation.x % TILE_SIZE;
            if FloatOrd(signed_distance) >= FloatOrd(0.0) {
                distance_to_tile = TILE_SIZE - signed_distance;
            } else {
                distance_to_tile = signed_distance.abs();
            }
        }
        MovementDirection::Neutral => distance_to_tile = 0.0,
    }
    let delta = min(
        FloatOrd((player.speed * TILE_SIZE * delta_seconds).abs()),
        FloatOrd(distance_to_tile),
    )
    .0 as i32 as f32;
    if approx_eq!(f32, delta, distance_to_tile) {
        player.movement_direction = MovementDirection::Neutral;
    }
    delta
}

type Pte<'a, 'b> = (&'a mut Player, &'b mut Transform, Entity);

fn player_movement(
    mut player_query: Query<Pte, (With<Player>, Without<Collidable>)>,
    collidable_query: Query<(&Transform, Entity), With<Collidable>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    key_bindings: Res<KeyBindings>,
) {
    let (mut player, mut transform, entity) = player_query.single_mut();

    let mut y_delta = 0.0;
    let mut x_delta = 0.0;
    if keyboard.pressed(key_bindings.up)
        & [MovementDirection::Neutral, MovementDirection::Up].contains(&player.movement_direction)
    {
        y_delta += get_manual_movement_speed(player.speed, time.delta_seconds());
        player.movement_direction = MovementDirection::Up;
    } else if keyboard.pressed(key_bindings.down)
        & [MovementDirection::Neutral, MovementDirection::Down].contains(&player.movement_direction)
    {
        y_delta -= get_manual_movement_speed(player.speed, time.delta_seconds());
        player.movement_direction = MovementDirection::Down;
    } else if keyboard.pressed(key_bindings.left)
        & [MovementDirection::Neutral, MovementDirection::Left].contains(&player.movement_direction)
    {
        x_delta -= get_manual_movement_speed(player.speed, time.delta_seconds());
        player.movement_direction = MovementDirection::Left;
    } else if keyboard.pressed(key_bindings.right)
        & [MovementDirection::Neutral, MovementDirection::Right]
            .contains(&player.movement_direction)
    {
        x_delta += get_manual_movement_speed(player.speed, time.delta_seconds());
        player.movement_direction = MovementDirection::Right;
    } else if player.movement_direction == MovementDirection::Up {
        y_delta += get_auto_movement_speed(&transform, &time.delta_seconds(), &mut player);
    } else if player.movement_direction == MovementDirection::Down {
        y_delta -= get_auto_movement_speed(&transform, &time.delta_seconds(), &mut player);
    } else if player.movement_direction == MovementDirection::Left {
        x_delta -= get_auto_movement_speed(&transform, &time.delta_seconds(), &mut player);
    } else if player.movement_direction == MovementDirection::Right {
        x_delta += get_auto_movement_speed(&transform, &time.delta_seconds(), &mut player);
    }
    let target = transform.translation + Vec3::new(x_delta, y_delta, 0.0);
    let collidable_entity: Vec<(Vec3, Entity)> = collidable_query
        .iter()
        .map(|(t, e)| (t.translation, e))
        .collect();
    if !check_collision(&target, &entity, &collidable_entity) {
        transform.translation = target;
    } else {
        player.movement_direction = MovementDirection::Neutral;
    }
}

fn camera_follow(
    player_query: Query<(&Transform, &Player), With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
    time: Res<Time>,
) {
    let (player_transform, player) = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    let (player_x, player_y) = (
        player_transform.translation.x,
        player_transform.translation.y,
    );

    let delta_x = (camera_transform.translation.x - player_x).abs();
    let delta_y = (camera_transform.translation.y - player_y).abs();

    let mut camera_new_x: f32 = 0.0;
    let mut camera_new_y: f32 = 0.0;

    let catchup_mult: f32 = 1.1;

    match player.movement_direction {
        MovementDirection::Up => {
            if delta_y > TILE_SIZE * 2.0 {
                camera_new_y +=
                    get_manual_movement_speed(player.speed * catchup_mult, time.delta_seconds());
            }
        }
        MovementDirection::Down => {
            if delta_y > TILE_SIZE * 2.0 {
                camera_new_y -=
                    get_manual_movement_speed(player.speed * catchup_mult, time.delta_seconds());
            }
        }
        MovementDirection::Left => {
            if delta_x > TILE_SIZE * 2.0 {
                camera_new_x -=
                    get_manual_movement_speed(player.speed * catchup_mult, time.delta_seconds());
            }
        }
        MovementDirection::Right => {
            if delta_x > TILE_SIZE * 2.0 {
                camera_new_x +=
                    get_manual_movement_speed(player.speed * catchup_mult, time.delta_seconds());
            }
        }
        MovementDirection::Neutral => {}
    }

    camera_transform.translation.x += camera_new_x;
    camera_transform.translation.y += camera_new_y;
}

fn spawn_player(mut commands: Commands) {
    let shape = shapes::Circle {
        radius: TILE_SIZE / 2.0,
        center: Vec2::ZERO,
    };

    let pdi_shape = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(TILE_SIZE / 4.0),
        ..Default::default()
    };

    commands
        .spawn((
            GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::CYAN),
                    outline_mode: StrokeMode::new(Color::BLACK, TILE_SIZE / 10.0),
                },
                Transform::from_translation(Vec3::new(0.0, 0.0, PLAYER_LEVEL)),
            ),
            Player {
                speed: 3.0,
                movement_direction: MovementDirection::Neutral,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                GeometryBuilder::build_as(
                    &pdi_shape,
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::OLIVE),
                        outline_mode: StrokeMode::new(Color::BLACK, TILE_SIZE / 10.0),
                    },
                    Transform::from_translation(Vec3::new(0.0, TILE_SIZE / 6.0, 50.0)),
                ),
                FacingDirection::Up,
                PlayerDirectionIndicator {
                    lock_rotation_up: false,
                    lock_rotation_down: false,
                    lock_rotation_left: false,
                    lock_rotation_right: false,
                },
            ));
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_manual_movement_speed() {
        assert_eq!(get_manual_movement_speed(1.0, 1.0), TILE_SIZE);
    }

    #[test]
    fn test_get_auto_movement_speed_up() {
        let mut player = Player {
            speed: 3.0,
            movement_direction: MovementDirection::Up,
        };
        let delta_seconds = 0.022913124;
        let transform = Transform::from_translation(Vec3::new(97.0, 55.0, 0.0));
        assert_eq!(
            get_auto_movement_speed(&transform, &delta_seconds, &mut player),
            6.0
        );
        assert_eq!(player.movement_direction, MovementDirection::Up);
    }

    #[test]
    fn test_get_auto_movement_speed_down() {
        let mut player = Player {
            speed: 3.0,
            movement_direction: MovementDirection::Down,
        };
        let delta_seconds = 0.022913124;
        let transform = Transform::from_translation(Vec3::new(97.0, 55.0, 0.0));
        assert_eq!(
            get_auto_movement_speed(&transform, &delta_seconds, &mut player),
            6.0
        );
        assert_eq!(player.movement_direction, MovementDirection::Down);
    }

    #[test]
    fn test_get_auto_movement_speed_left() {
        let mut player = Player {
            speed: 3.0,
            movement_direction: MovementDirection::Left,
        };
        let delta_seconds = 0.022913124;
        let transform = Transform::from_translation(Vec3::new(97.0, 55.0, 0.0));
        assert_eq!(
            get_auto_movement_speed(&transform, &delta_seconds, &mut player),
            6.0
        );
        assert_eq!(player.movement_direction, MovementDirection::Left);
    }

    #[test]
    fn test_get_auto_movement_speed_right() {
        let mut player = Player {
            speed: 3.0,
            movement_direction: MovementDirection::Right,
        };
        let delta_seconds = 0.022913124;
        let transform = Transform::from_translation(Vec3::new(97.0, 55.0, 0.0));
        assert_eq!(
            get_auto_movement_speed(&transform, &delta_seconds, &mut player),
            3.0
        );
        assert_eq!(player.movement_direction, MovementDirection::Neutral);
    }

    #[test]
    fn test_get_auto_movement_speed_up_negative() {
        let mut player = Player {
            speed: 3.0,
            movement_direction: MovementDirection::Up,
        };
        let delta_seconds = 0.022913124;
        let transform = Transform::from_translation(Vec3::new(97.0, -295.0, 0.0));
        assert_eq!(
            get_auto_movement_speed(&transform, &delta_seconds, &mut player),
            6.0
        );
        assert_eq!(player.movement_direction, MovementDirection::Up);
    }

    #[test]
    fn test_get_auto_movement_speed_down_negative() {
        let mut player = Player {
            speed: 3.0,
            movement_direction: MovementDirection::Down,
        };
        let delta_seconds = 0.022913124;
        let transform = Transform::from_translation(Vec3::new(97.0, -295.0, 0.0));
        assert_eq!(
            get_auto_movement_speed(&transform, &delta_seconds, &mut player),
            5.0
        );
        assert_eq!(player.movement_direction, MovementDirection::Neutral);
    }
}
