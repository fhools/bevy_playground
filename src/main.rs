use bevy::prelude::*;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: [WIDTH, HEIGHT].into(),
                    title: "My Shooter".to_string(),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()}))
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
    });
}

fn spawn_basic_scene(mut commands: Commands,
                     mut meshes: ResMut<Assets<Mesh>>,
                     mut materials: ResMut<Assets<StandardMaterial>>) {

    // The camera and cube share components like mesh and material
    // All 3 entities share a tranform component.
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0, ..default() })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0, ..default() })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

