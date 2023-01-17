use bevy::prelude::*;
use core::time::Duration;

use crate::*;

pub struct ArrowsPlugin;

impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fire_arrow.label("firearrow"))
            .add_startup_system(arrow_cooldown_init)
            .add_system(arrow_move.label("arrowmove"))
            .add_system(despawn_offscreen_arrows.after("arrowmove"))
            .add_system(manage_arrow_collisions.after("arrowmove"))
            .init_resource::<ArrowCooldown>();
    }
}

const ARROW_COOLDOWN: f32 = 0.5;
const ARROW_SPEED: f32 = 7.5;
const ARROW_LENGTH: f32 = TILE_SIZE / 2.0;
const ARROW_WIDTH: f32 = TILE_SIZE / 10.0;

#[derive(Component)]
pub struct Arrow {
    pub facing_direction: FacingDirection,
    pub speed: f32,
    pub size: Vec2,
}

impl Arrow {
    pub fn new(facing_direction: FacingDirection, speed: f32, size: Vec2) -> Self {
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

#[derive(Resource, Deref, DerefMut)]
struct ArrowCooldown(Timer);

impl Default for ArrowCooldown {
    fn default() -> Self {
        Self(Timer::from_seconds(ARROW_COOLDOWN, TimerMode::Once))
    }
}


fn manage_arrow_collisions(
    mut commands: Commands,
    mut arrow_query: Query<(&Transform, Entity, &Arrow), With<Arrow>>,
    collidable_query: Query<(&Transform, Entity), With<Collidable>>,
) {
    let collidables: Vec<(Vec3, u32)> = collidable_query
        .iter()
        .map(|(transform, entity)| (transform.translation, entity.index()))
        .collect();
    for (arrow_transform, arrow_entity, arrow) in arrow_query.iter_mut() {
        if check_collision(
            &arrow_transform.translation,
            &arrow_entity.index(),
            &collidables,
            arrow.size,
        ).is_some() {
            commands.entity(arrow_entity).despawn();
        }
    }
}

fn despawn_offscreen_arrows(
    mut commands: Commands,
    mut arrow_query: Query<(&Transform, Entity), With<Arrow>>,
    camera_query: Query<&mut Transform, (Without<Arrow>, With<Camera>)>,
) {
    let camera_transform = camera_query.single();
    let camera_x = camera_transform.translation.x;
    let camera_y = camera_transform.translation.y;
    for (arrow_transform, arrow) in arrow_query.iter_mut() {
        if out_of_bounds(
            camera_x,
            camera_y,
            arrow_transform.translation.x,
            arrow_transform.translation.y,
        ) {
            commands.entity(arrow).despawn();
        }
    }
}

fn arrow_cooldown_init(mut cooldown_timer: ResMut<ArrowCooldown>) {
    cooldown_timer.tick(Duration::from_secs((ARROW_COOLDOWN + 1.0) as u64));
}

fn arrow_move(mut arrow_query: Query<(&mut Transform, &Arrow)>, time: Res<Time>) {
    for (mut transform, arrow) in arrow_query.iter_mut() {
        let mut delta_x: f32 = 0.0;
        let mut delta_y: f32 = 0.0;
        let movement_speed = get_manual_movement_speed(arrow.speed(), time.delta_seconds());
        match arrow.facing_direction() {
            FacingDirection::Up => delta_y += movement_speed,
            FacingDirection::Down => delta_y -= movement_speed,
            FacingDirection::Left => delta_x -= movement_speed,
            FacingDirection::Right => delta_x += movement_speed,
        };
        transform.translation.x += delta_x;
        transform.translation.y += delta_y;
    }
}

fn fire_arrow(
    mut commands: Commands,
    pdi_query: Query<(&mut GlobalTransform, &mut FacingDirection), With<PlayerDirectionIndicator>>,
    keyboard: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    time: Res<Time>,
    mut arrow_cooldown: ResMut<ArrowCooldown>,
) {
    arrow_cooldown.tick(time.delta());
    if arrow_cooldown.finished() && keyboard.just_pressed(key_bindings.fire) {
        let (pdi_transform, facing_direction) = pdi_query.single();

        let (arrow_x, arrow_y, arrow_length, arrow_width) = match facing_direction {
            FacingDirection::Up => (
                pdi_transform.translation().x,
                pdi_transform.translation().y + TILE_SIZE / 2.0,
                ARROW_LENGTH,
                ARROW_WIDTH,
            ),
            FacingDirection::Down => (
                pdi_transform.translation().x,
                pdi_transform.translation().y - TILE_SIZE / 2.0,
                ARROW_LENGTH,
                ARROW_WIDTH,
            ),
            FacingDirection::Left => (
                pdi_transform.translation().x - TILE_SIZE / 2.0,
                pdi_transform.translation().y,
                ARROW_WIDTH,
                ARROW_LENGTH,
            ),
            FacingDirection::Right => (
                pdi_transform.translation().x + TILE_SIZE / 2.0,
                pdi_transform.translation().y,
                ARROW_WIDTH,
                ARROW_LENGTH,
            ),
        };

        let shape = shapes::Rectangle {
            extents: Vec2::new(arrow_width, arrow_length),
            origin: RectangleOrigin::Center,
        };

        commands.spawn((
            GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::MAROON),
                    outline_mode: StrokeMode::new(Color::BLACK, TILE_SIZE / 10.0),
                },
                Transform::from_translation(Vec3::new(arrow_x, arrow_y, PLAYER_LEVEL - 50.0)),
            ),
            Arrow::new(*facing_direction, ARROW_SPEED, Vec2::new(arrow_width, arrow_length)),
        ));
        arrow_cooldown.reset();
    }
}
