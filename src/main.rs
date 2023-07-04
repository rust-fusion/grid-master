use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Resource)]
struct GridSize {
    width: usize,
    height: usize,
}

#[derive(Component)]
struct GameCamera;

fn setup(mut commands: Commands) {
    /*commands.spawn(Window {
    title: "Grid".to_string(),
    resizable: true,
    resolution: WindowResolution::new(1080.0, 600.0),
    ..Default::default()
    });*/
    commands.spawn((Camera2dBundle::default(), GameCamera));
}

fn draw_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    /*let grid_width = grid_size.width;
    let grid_height = grid_size.height;
    let cell_size = 50.0; // Size of each cell
    for x in 0..=grid_width {
        let start = Vec2::new(x as f32 * cell_size, 0.0); // => (0,0), (50,0), (100,0)
        let end = Vec2::new(x as f32 * cell_size, grid_height as f32 * cell_size); //(0,5000)
                                                                                   // commands.spawn(GeometryBuilder::build_as(&shapes::Line(start, end)));
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Line(start, end)),
                ..default()
            },
            Fill::color(Color::BLUE),
            Stroke::new(Color::YELLOW, 10.0),
        ));
    }

    for y in 0..=grid_height {
        let start = Vec2::new(0.0, y as f32 * cell_size);
        let end = Vec2::new(grid_width as f32 * cell_size, y as f32 * cell_size);
        // commands.spawn(GeometryBuilder::build_as(&shapes::Line(start, end)));
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Line(start, end)),
                ..default()
            },
            Fill::color(Color::BLUE),
            Stroke::new(Color::YELLOW, 10.0),
        ));
    }*/
    for i in 0..10 {
        create_square(
            commands,
            meshes,
            materials,
            (50., 50.),
            (i as f32 * 10., 0.),
        );
    }
}

fn create_square(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    size: (f32, f32),
    position: (f32, f32),
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(size.0, size.1)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        transform: Transform::from_translation(Vec3::new(position.0, position.1, 1.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(size.0 + 2., size.1 + 2.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(position.0, position.1, 0.)),
        ..default()
    });
}

/*fn zoom_2d(mut q: Query<&mut OrthographicProjection, With<GameCamera>>) {
    let mut projection = q.single_mut();

    // example: zoom in
    // projection.scale *= 1.25;
    // example: zoom out
    projection.scale *= 0.75;

    // always ensure you end up with sane values
    // (pick an upper and lower bound for your application)
    projection.scale = projection.scale.clamp(0.5, 5.0);
}*/

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(GridSize {
            width: 100,
            height: 100,
        })
        .add_startup_system(setup)
        .add_startup_system(draw_grid)
        // .add_startup_system(draw_grid)
        .run();
}
