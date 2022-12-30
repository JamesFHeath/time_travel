use bevy::prelude::*;
use core::time::Duration;

use crate::*;

pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fire_projectile.label("fireprojectile"))
            .add_startup_system(projectile_cooldown_init)
            .add_system(projectile_move.label("projectilemove"))
            .add_system(despawn_offscreen_projectiles.after("projectilemove"))
            .init_resource::<ProjectileCooldown>();
    }
}

const PROJECTILE_COOLDOWN: f32 = 0.5;
const PROJECTILE_SPEED: f32 = 6.0;

#[derive(Component)]
pub struct Projectile {
    pub facing_direction: FacingDirection,
    pub speed: f32,
}

#[derive(Resource, Deref, DerefMut)]
struct ProjectileCooldown(Timer);

impl Default for ProjectileCooldown {
    fn default() -> Self {
        Self(Timer::from_seconds(PROJECTILE_COOLDOWN, TimerMode::Once))
    }
}

impl Projectile {
    pub fn new(facing_direction: FacingDirection, speed: f32) -> Self {
        Self {
            facing_direction,
            speed,
        }
    }

    pub fn facing_direction(&self) -> FacingDirection {
        self.facing_direction
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }
}

fn despawn_offscreen_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(&Transform, Entity), With<Projectile>>,
    camera_query: Query<&mut Transform, (Without<Projectile>, With<Camera>)>,
) {
    let camera_transform = camera_query.single();
    let camera_x = camera_transform.translation.x;
    let camera_y = camera_transform.translation.y;
    for (projectile_transform, projectile) in projectile_query.iter_mut() {
        if out_of_bounds(
            camera_x,
            camera_y,
            projectile_transform.translation.x,
            projectile_transform.translation.y,
        ) {
            commands.entity(projectile).despawn();
        }
    }
}

fn projectile_cooldown_init(mut cooldown_timer: ResMut<ProjectileCooldown>) {
    cooldown_timer.tick(Duration::from_secs((PROJECTILE_COOLDOWN + 1.0) as u64));
}

fn projectile_move(mut projectile_query: Query<(&mut Transform, &Projectile)>, time: Res<Time>) {
    for (mut transform, projectile) in projectile_query.iter_mut() {
        let mut delta_x: f32 = 0.0;
        let mut delta_y: f32 = 0.0;
        let movement_speed = get_manual_movement_speed(projectile.speed(), time.delta_seconds());
        match projectile.facing_direction() {
            FacingDirection::Up => delta_y += movement_speed,
            FacingDirection::Down => delta_y -= movement_speed,
            FacingDirection::Left => delta_x -= movement_speed,
            FacingDirection::Right => delta_x += movement_speed,
        };
        transform.translation.x += delta_x;
        transform.translation.y += delta_y;
    }
}

fn fire_projectile(
    mut commands: Commands,
    pdi_query: Query<(&mut GlobalTransform, &mut FacingDirection), With<PlayerDirectionIndicator>>,
    keyboard: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    time: Res<Time>,
    mut projectile_cooldown: ResMut<ProjectileCooldown>,
) {
    projectile_cooldown.tick(time.delta());
    if projectile_cooldown.finished() && keyboard.just_pressed(key_bindings.fire) {
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
            Projectile::new(*facing_direction, PROJECTILE_SPEED),
        ));
        projectile_cooldown.reset();
    }
}
