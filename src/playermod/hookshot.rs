use bevy::prelude::*;
use std::f32::consts::PI;

use crate::*;

pub struct HookshotPlugin;

impl Plugin for HookshotPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fire_hookshot.label("firehookshot"))
            .add_system(hookshot_move.label("hookshotmove"));
    }
}

#[derive(Component)]
pub struct Hookshot {
    pub facing_direction: FacingDirection,
    pub speed: f32,
    pub size: f32,
}

const HOOKSHOT_SPEED: f32 = 7.5;
const HOOKSHOT_SIZE: f32 = TILE_SIZE / 2.0;

impl Hookshot {
    pub fn new(facing_direction: FacingDirection, speed: f32, size: f32) -> Self {
        Self {
            facing_direction,
            speed,
            size,
        }
    }

    pub fn facing_direction(&self) -> FacingDirection {
        self.facing_direction
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }
}

fn fire_hookshot(
    mut commands: Commands,
    pdi_query: Query<(&mut GlobalTransform, &mut FacingDirection), With<PlayerDirectionIndicator>>,
    keyboard: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
) {
    if keyboard.just_pressed(key_bindings.hookshot) {
        let (pdi_transform, facing_direction) = pdi_query.single();

        let (hookshot_x, hookshot_y, roation_angle) = match facing_direction {
            FacingDirection::Up => (
                pdi_transform.translation().x,
                pdi_transform.translation().y + TILE_SIZE / 2.0,
                0.0,
            ),
            FacingDirection::Down => (
                pdi_transform.translation().x,
                pdi_transform.translation().y - TILE_SIZE / 2.0,
                PI,
            ),
            FacingDirection::Left => (
                pdi_transform.translation().x - TILE_SIZE / 2.0,
                pdi_transform.translation().y,
                PI / 2.0,
            ),
            FacingDirection::Right => (
                pdi_transform.translation().x + TILE_SIZE / 2.0,
                pdi_transform.translation().y,
                3.0 * PI / 2.0,
            ),
        };

        let pdi_shape = shapes::RegularPolygon {
            sides: 3,
            feature: shapes::RegularPolygonFeature::Radius(TILE_SIZE / 4.0),
            ..Default::default()
        };

        commands.spawn((
            GeometryBuilder::build_as(
                &pdi_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GREEN),
                    outline_mode: StrokeMode::new(Color::BLACK, TILE_SIZE / 10.0),
                },
                Transform {
                    translation: Vec3::new(hookshot_x, hookshot_y, PLAYER_LEVEL),
                    rotation: Quat::from_rotation_z(roation_angle),
                    ..Default::default()
                },
            ),
            Hookshot::new(*facing_direction, HOOKSHOT_SPEED, HOOKSHOT_SIZE),
        ));
    }
}

fn hookshot_move(mut hookshot_query: Query<(&mut Transform, &Hookshot)>, time: Res<Time>) {
    for (mut transform, hookshot) in hookshot_query.iter_mut() {
        let mut delta_x: f32 = 0.0;
        let mut delta_y: f32 = 0.0;
        let movement_speed = get_manual_movement_speed(hookshot.speed(), time.delta_seconds());
        match hookshot.facing_direction() {
            FacingDirection::Up => delta_y += movement_speed,
            FacingDirection::Down => delta_y -= movement_speed,
            FacingDirection::Left => delta_x -= movement_speed,
            FacingDirection::Right => delta_x += movement_speed,
        };
        transform.translation.x += delta_x;
        transform.translation.y += delta_y;
    }
}
