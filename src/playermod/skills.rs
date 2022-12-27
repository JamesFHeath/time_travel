use bevy::prelude::*;

use crate::*;

// fn projectile_move(Query<&Transform>, With<>) {

// }

pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fire_projectile);
    }
}

fn fire_projectile(
    mut commands: Commands,
    pdi_query: Query<(&mut GlobalTransform, &mut FacingDirection), With<PlayerDirectionIndicator>>,
    keyboard: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
) {
    if keyboard.just_pressed(key_bindings.fire) {
        let (pdi_transform, facing_direction) = pdi_query.single();

        let (projectile_x, projectile_y, projectile_height, projectile_width) =
            match facing_direction {
                FacingDirection::Up => (
                    pdi_transform.translation().x,
                    pdi_transform.translation().y + TILE_SIZE / 2.0,
                    TILE_SIZE / 2.0,
                    TILE_SIZE / 10.0,
                ),
                FacingDirection::Down => (
                    pdi_transform.translation().x,
                    pdi_transform.translation().y - TILE_SIZE / 2.0,
                    TILE_SIZE / 2.0,
                    TILE_SIZE / 10.0,
                ),
                FacingDirection::Left => (
                    pdi_transform.translation().x - TILE_SIZE / 2.0,
                    pdi_transform.translation().y,
                    TILE_SIZE / 10.0,
                    TILE_SIZE / 2.0,
                ),
                FacingDirection::Right => (
                    pdi_transform.translation().x + TILE_SIZE / 2.0,
                    pdi_transform.translation().y,
                    TILE_SIZE / 10.0,
                    TILE_SIZE / 2.0,
                ),
            };

        let shape = shapes::Rectangle {
            extents: Vec2::new(projectile_width, projectile_height),
            origin: RectangleOrigin::Center,
        };

        commands.spawn((
            GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::MAROON),
                    outline_mode: StrokeMode::new(Color::BLACK, TILE_SIZE / 10.0),
                },
                Transform::from_translation(Vec3::new(
                    projectile_x,
                    projectile_y,
                    PLAYER_LEVEL + 50.0,
                )),
            ),
            Projectile::new(*facing_direction),
        ));
    }
}
