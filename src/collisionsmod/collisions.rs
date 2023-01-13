use crate::*;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(draw_collidable)
            .add_system(manage_interaction_events.run_on_event::<InteractionEvent>());
    }
}

/// Checks collisions between the player or another
/// moving entity and all collidable entities.
pub fn check_collision(
    mover_translation: &Vec3,
    mover_index: &u32,
    collidables: &[(Vec3, u32)],
    mover_size: Vec2,
) -> Option<Vec3> {
    let tile = Vec2::new(TILE_SIZE, TILE_SIZE);
    for (collidable_translation, collidable_index) in collidables.iter() {
        if mover_index != collidable_index
            && collide(
                *mover_translation,
                mover_size,
                *collidable_translation,
                tile,
            )
            .is_some()
        {
            return Some(*collidable_translation);
        };
    }
    None
}

/// Checks if the player was in range and facing
/// the correct direction to interact with an entity.
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

/// This system checks interaction events between
/// the player and interactable entities.
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
    let locations = vec![
        Vec2::new(5.0 * TILE_SIZE, 0.0 * TILE_SIZE),
        Vec2::new(6.0 * TILE_SIZE, 0.0 * TILE_SIZE),
        Vec2::new(0.0 * TILE_SIZE, 4.0 * TILE_SIZE),
    ];
    for location in locations {
        commands.spawn((
            GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::YELLOW_GREEN),
                    outline_mode: StrokeMode::new(Color::BLACK, TILE_SIZE / 10.0),
                },
                Transform::from_translation(Vec3::new(
                    location.x,
                    location.y,
                    PLAYER_LEVEL,
                )),
            ),
            Collidable(),
            Interactable(),
            Hookshotable(),
            CollidableTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        ));
    }
}
