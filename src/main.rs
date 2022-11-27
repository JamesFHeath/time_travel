#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_prototype_lyon::prelude::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
// pub const TILE_SIZE: f32 = 0.3;
pub const TILE_SIZE: f32 = 100.0;
pub const BACKGROUND_ONE: f32 = 100.0;
pub const PLAYER_LEVEL: f32 = 200.0;

// mod ascii;
// mod debug;
mod player;
// mod tilemap;

// use ascii::AsciiPlugin;
// use debug::DebugPlugin;
use player::PlayerPlugin;
// use tilemap::TileMapPlugin;

#[derive(Resource)]
struct CurrentEra {
    current_era: Entity,
}

#[derive(Component)]
struct BackgroundParent();

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
        .add_plugin(PlayerPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(draw_backgrounds)
        .add_system(toggle_background)
        // .add_plugin(AsciiPlugin)
        // .add_plugin(DebugPlugin)
        // .add_plugin(TileMapPlugin)
        .run();
}

fn draw_backgrounds(mut commands: Commands) {
    // draw_background(&mut commands, Color::RED);
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
                            0.0,
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

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    // camera.projection.top = 1.0;
    // camera.projection.bottom = -1.0;
    camera.projection.top = TILE_SIZE * 9.0;
    camera.projection.bottom = -TILE_SIZE * 9.0;

    // camera.projection.right = 1.0 * RESOLUTION;
    // camera.projection.left = -1.0 * RESOLUTION;
    camera.projection.right = TILE_SIZE * 16.0;
    camera.projection.left = -TILE_SIZE * 16.0;

    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn(camera);
}
