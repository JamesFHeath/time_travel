use std::{cmp::min, f32::consts::PI};

use float_cmp::approx_eq;
use float_ord::FloatOrd;

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

#[derive(PartialEq, Debug)]
struct Rotation(f32);

const PLAYER_SPEED: f32 = 4.0;
const PI_OVER_TWO: Rotation = Rotation(PI / 2.0);
const THREE_PI_OVER_TWO: Rotation = Rotation(3.0 * PI / 2.0);
const ZERO_PI: Rotation = Rotation(0.0);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MovementDirection {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Neutral = 4,
}

#[derive(Clone, Component, Copy, PartialEq, Debug)]
pub enum FacingDirection {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Component)]
pub struct PlayerDirectionIndicator();

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

#[cfg(test)]
mod test_get_manual_movement_speed {
    use super::*;

    #[test]
    fn test_get_manual_movement_speed() {
        assert_eq!(get_manual_movement_speed(1.0, 1.0), TILE_SIZE);
    }
}

fn rotate_player_direction_indicator(
    mut pdi_query: Query<
        (
            &mut Transform,
            &mut FacingDirection,
        ),
        (With<PlayerDirectionIndicator>, Without<Player>),
    >,
    player_query: Query<&Player, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
) {
    let player = player_query.single();
    let (mut pdi_transform, mut facing_direction) = pdi_query.single_mut();

    let key_pressed: KeyCode;

    if keyboard.pressed(key_bindings.up) {
        key_pressed = key_bindings.up;
    } else if keyboard.pressed(key_bindings.down) {
        key_pressed = key_bindings.down;
    } else if keyboard.pressed(key_bindings.left) {
        key_pressed = key_bindings.left;
    } else if keyboard.pressed(key_bindings.right) {
        key_pressed = key_bindings.right;
    } else {
        return;
    }

    let (rotation_angle, new_facing_direction) = get_new_angle_and_facing_direction_for_pdi(
        player.movement_direction,
        *facing_direction,
        key_pressed,
        key_bindings.into_inner(),
    );

    *facing_direction = new_facing_direction;

    pdi_transform.rotate_around(
        Vec3::new(0.0, 0.0, 0.0),
        Quat::from_rotation_z(rotation_angle.0),
    );
}

fn get_new_angle_and_facing_direction_for_pdi(
    player_movement_direction: MovementDirection,
    pdi_facing_direction: FacingDirection,
    key_pressed: KeyCode,
    key_bindings: &KeyBindings,
) -> (Rotation, FacingDirection) {
    let new_rotation_angle: Rotation;
    let new_facing_direction: FacingDirection;

    if player_movement_direction != MovementDirection::Neutral
        && player_movement_direction as u8 != pdi_facing_direction as u8
    {
        new_rotation_angle = match player_movement_direction {
            MovementDirection::Up => match pdi_facing_direction {
                FacingDirection::Up => ZERO_PI,
                FacingDirection::Down => Rotation(PI),
                FacingDirection::Left => THREE_PI_OVER_TWO,
                FacingDirection::Right => PI_OVER_TWO,
            },
            MovementDirection::Down => match pdi_facing_direction {
                FacingDirection::Up => Rotation(PI),
                FacingDirection::Down => ZERO_PI,
                FacingDirection::Left => PI_OVER_TWO,
                FacingDirection::Right => THREE_PI_OVER_TWO,
            },
            MovementDirection::Left => match pdi_facing_direction {
                FacingDirection::Up => PI_OVER_TWO,
                FacingDirection::Down => THREE_PI_OVER_TWO,
                FacingDirection::Left => ZERO_PI,
                FacingDirection::Right => Rotation(PI),
            },
            MovementDirection::Right => match pdi_facing_direction {
                FacingDirection::Up => THREE_PI_OVER_TWO,
                FacingDirection::Down => PI_OVER_TWO,
                FacingDirection::Left => Rotation(PI),
                FacingDirection::Right => ZERO_PI,
            },
            MovementDirection::Neutral => ZERO_PI,
        };
        new_facing_direction = match player_movement_direction {
            MovementDirection::Up => FacingDirection::Up,
            MovementDirection::Down => FacingDirection::Down,
            MovementDirection::Left => FacingDirection::Left,
            MovementDirection::Right => FacingDirection::Right,
            MovementDirection::Neutral => pdi_facing_direction,
        };
    } else if player_movement_direction == MovementDirection::Neutral {
        if key_pressed == key_bindings.up {
            new_rotation_angle = match pdi_facing_direction {
                FacingDirection::Up => ZERO_PI,
                FacingDirection::Down => Rotation(PI),
                FacingDirection::Left => THREE_PI_OVER_TWO,
                FacingDirection::Right => PI_OVER_TWO,
            };
            new_facing_direction = FacingDirection::Up;
        } else if key_pressed == key_bindings.down {
            new_rotation_angle = match pdi_facing_direction {
                FacingDirection::Up => Rotation(PI),
                FacingDirection::Down => ZERO_PI,
                FacingDirection::Left => PI_OVER_TWO,
                FacingDirection::Right => THREE_PI_OVER_TWO,
            };
            new_facing_direction = FacingDirection::Down;
        } else if key_pressed == key_bindings.left {
            new_rotation_angle = match pdi_facing_direction {
                FacingDirection::Up => PI_OVER_TWO,
                FacingDirection::Down => THREE_PI_OVER_TWO,
                FacingDirection::Left => ZERO_PI,
                FacingDirection::Right => Rotation(PI),
            };
            new_facing_direction = FacingDirection::Left;
        } else if key_pressed == key_bindings.right {
            new_rotation_angle = match pdi_facing_direction {
                FacingDirection::Up => THREE_PI_OVER_TWO,
                FacingDirection::Down => PI_OVER_TWO,
                FacingDirection::Left => Rotation(PI),
                FacingDirection::Right => ZERO_PI,
            };
            new_facing_direction = FacingDirection::Right;
        } else {
            new_rotation_angle = ZERO_PI;
            new_facing_direction = pdi_facing_direction;
        }
    } else {
        new_rotation_angle = ZERO_PI;
        new_facing_direction = pdi_facing_direction;
    }
    (new_rotation_angle, new_facing_direction)
}

#[cfg(test)]
mod test_get_new_angle_and_facing_direction_for_pdi {
    use super::*;

    #[test]
    fn test_player_moving_neutral_up_pressed_pdi_facing_up() {
        let player_movement_direction = MovementDirection::Neutral;
        let pdi_facing_direction = FacingDirection::Up;
        let key_pressed = KeyCode::Up;
        let key_bindings = KeyBindings::default();
        let (new_rotation_angle, new_facing_direction) = get_new_angle_and_facing_direction_for_pdi(
            player_movement_direction,
            pdi_facing_direction,
            key_pressed,
            &key_bindings,
        );
        assert_eq!(new_rotation_angle, ZERO_PI);
        assert_eq!(new_facing_direction, FacingDirection::Up);
    }
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

#[cfg(test)]
mod tests_get_auto_movement_speed {
    use super::*;

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

type Pte<'a, 'b> = (&'a mut Player, &'b mut Transform, Entity);

fn player_movement(
    mut player_query: Query<Pte, (With<Player>, Without<Collidable>)>,
    collidable_query: Query<(&Transform, Entity), With<Collidable>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    key_bindings: Res<KeyBindings>,
    hookshot_firing: Res<HookshotFiring>,
) {
    if hookshot_firing.0 {
        return;
    }
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
    let collidable_entity: Vec<(Vec3, u32)> = collidable_query
        .iter()
        .map(|(t, e)| (t.translation, e.index()))
        .collect();
    if check_collision(
        &target,
        &entity.index(),
        &collidable_entity,
        Vec2::new(TILE_SIZE, TILE_SIZE),
    )
    .is_none()
    {
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
                speed: PLAYER_SPEED,
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
                PlayerDirectionIndicator(),
            ));
        });
}
