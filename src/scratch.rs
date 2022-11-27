
fn draw_background(commands: &mut Commands, color: Color) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(TILE_SIZE, TILE_SIZE),
        origin: RectangleOrigin::TopLeft,
        // sides: 4,
        // feature: shapes::RegularPolygonFeature::Radius(TILE_SIZE / 2.0),
        // ..shapes::RegularPolygon::default()
    };
    // commands
    //     .spawn((
    //         BackgroundParent {},
    //         SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
    //             0.0,
    //             0.0,
    //             BACKGROUND_ONE,
    //         ))),
    //     ))
    //     .with_children(|parent| {
    for i in 0..3 {
        for j in 0..3 {
            commands.spawn(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(color),
                    outline_mode: StrokeMode::new(Color::BLACK, TILE_SIZE / 10.0),
                },
                Transform::from_translation(Vec3::new(
                    i as f32 * TILE_SIZE,
                    j as f32 * TILE_SIZE,
                    BACKGROUND_ONE,
                )),
            ));
        }
    }
    // })
    // .id()
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

            // GeometryBuilder::build_as(
            //     &parent,
            //     DrawMode::Outlined {
            //         fill_mode: FillMode::color(Color::YELLOW),
            //         outline_mode: StrokeMode::new(Color::BLACK, TILE_SIZE / 10.0),
            //     },
            //     Transform::from_translation(Vec3::new(
            //         0.0  * TILE_SIZE,
            //         0.0 * TILE_SIZE,
            //         0.0,
            //     )),
            // ),