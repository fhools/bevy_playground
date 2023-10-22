use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::f32::consts::PI;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

// Components


// for: Tower
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    shooting_timer: Timer,
}


// For: Bullets, 
#[derive(Reflect, Component, Default)]
#[reflect(Component)] // Not sure what this does? I forgot to put it and it still worked in world
                      // inspector
pub struct Lifetime {
    timer: Timer,
}

// For: Target spawning

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TargetSpawnTimer {
    timer: Timer,
}


// For: Targets

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    health: f32,
}


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target {
    speed: f32,
}
// Resources
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
        .add_systems(Update, target_spawning)
        .add_systems(Update, target_move)
        .run();
}


// Startup system to place camera 
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
    });
}


// Startup system to load level
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

    // Target Spawning Entity
    // using the anonymous tuple of components
    commands.spawn((TargetSpawnTimer { timer: Timer::from_seconds(5.0, TimerMode::Repeating) },))
        .insert(Name::new("Target Spawning Entity"));
}


// System that spawns the targets base on a timer
fn target_spawning(mut commands: Commands,
                   mut target_spawner: Query<&mut TargetSpawnTimer>,
                   mut meshes: ResMut<Assets<Mesh>>,
                   mut materials: ResMut<Assets<StandardMaterial>>,
                   time: Res<Time>) {
    // TODO: this seems weird making a loop when we know the query should return just one
    // entity
    for mut t in &mut target_spawner {
        t.timer.tick(time.delta());
        if t.timer.just_finished() {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {  radius: 0.2, ..default()})),
                material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                transform: Transform::from_xyz(-2.5, 0.5, 2.0),
                ..default()
            })
            .insert(Health { health: 10.0 })
            .insert(Target { speed: 0.7 })
            .insert(Name::new("Target"));
        }
    }

}

fn target_move(mut targets: Query<(&Target, &mut Transform)>,
               time: Res<Time>) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}
// System that shoots bullets from the tower
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


// System to destroy bullets after lifetime expire
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


// Startup system to load assets
fn asset_loading(mut commands: Commands,
              assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet7.glb#Scene0"),
    });
}

