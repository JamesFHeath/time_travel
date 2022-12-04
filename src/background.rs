use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::resources::CurrentEra;
use crate::{BACKGROUND_ONE, TILE_SIZE};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(draw_backgrounds)
            .add_system(toggle_background);
    }
}

#[derive(Component)]
struct BackgroundParent;

fn draw_backgrounds(mut commands: Commands) {
    let background = draw_background_with_children(&mut commands, Color::RED);
    commands.insert_resource(CurrentEra {
        current_era: background,
    });
}

fn draw_background_with_children(commands: &mut Commands, color: Color) -> Entity {
    let shape = shapes::Rectangle {
        extents: Vec2::new(TILE_SIZE, TILE_SIZE),
        origin: RectangleOrigin::Center,
    };
    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0,
                0.0,
                BACKGROUND_ONE,
            ))),
            BackgroundParent {},
        ))
        .with_children(|parent| {
            for i in -10..10 {
                for j in -10..10 {
                    let mut shape_color = Color::ORANGE;
                    if (i + j) % 2 == 0 {
                        shape_color = color;
                    }
                    parent.spawn(GeometryBuilder::build_as(
                        &shape,
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(shape_color),
                            outline_mode: StrokeMode::new(Color::BLACK, TILE_SIZE / 10.0),
                        },
                        // Transform::default(),
                        Transform::from_translation(Vec3::new(
                            i as f32 * TILE_SIZE,
                            j as f32 * TILE_SIZE,
                            BACKGROUND_ONE,
                        )),
                    ));
                }
            }
        })
        .id()
}

fn toggle_background(
    mut query: Query<&mut Visibility>,
    keyboard: Res<Input<KeyCode>>,
    current_era: Res<CurrentEra>,
) {
    if keyboard.just_released(KeyCode::T) {
        let mut background_visibility =
            query.get_component_mut::<Visibility>(current_era.current_era);
        background_visibility.as_mut().unwrap().is_visible =
            !background_visibility.as_ref().unwrap().is_visible;
    }
}
