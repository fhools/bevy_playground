use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::f32::consts::PI;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

// Components
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    shooting_timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)] // Not sure what this does? I forgot to put it and it still worked in world
                      // inspector
pub struct Lifetime {
    timer: Timer,
}

#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}

static SHOOTING_FREQ: f32 = 8.0;
static BULLET_LIFETIME: f32 = 30.0;

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
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<Tower>()
        .add_systems(Startup, spawn_basic_scene)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, asset_loading)
        .add_systems(Update, tower_shooting)
        .add_systems(Update, bullet_despawn)
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
    
    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0, ..default() })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // Main Tower
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(Tower {
        shooting_timer: Timer::from_seconds(SHOOTING_FREQ, TimerMode::Repeating)
    })
    .insert(Name::new("Tower"));

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight { 
            illuminance:50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    })
    .insert(Name::new("Light"));

}

fn tower_shooting(mut commands: Commands,
                  mut towers: Query<&mut Tower>,
                  bullet_assets: Res<GameAssets>,
                  time: Res<Time>) {

    // iterate through all Towers and increment timer and
    // then spawn a bullet 
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());

        if tower.shooting_timer.just_finished() {
            let  spawn_transform = Transform::from_xyz(0.0, 0.7, 0.6)
                .with_rotation(Quat::from_rotation_y(-PI / 2.0));

            commands
                .spawn(SceneBundle {
                    scene: bullet_assets.bullet_scene.clone(),
                    transform: spawn_transform,
                    ..default()
                })
            .insert(Lifetime { timer: Timer::from_seconds(BULLET_LIFETIME, TimerMode::Once) })
            .insert(Name::new("Bullet"));
        }
    }
}

fn bullet_despawn(mut commands: Commands,
                  mut bullets: Query<(Entity, &mut Lifetime)>,
                  time: Res<Time>,) {

    // Loop through all entities that have Lifetime and Entity components
    // Our bullets are the only thing with lifetime component
    for (entity, mut lifetime) in &mut bullets {
        // Update bullet's lifetime timer 
        lifetime.timer.tick(time.delta());

        // If it finished 
        if lifetime.timer.just_finished() {
            // Remove it from the world, depsawn_recursive also removes it's children (none in this
            // case)
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn asset_loading(mut commands: Commands,
              assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet7.glb#Scene0"),
    });
}

