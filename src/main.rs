use bevy::prelude::*;
use bevy_mod_picking::*;
use grid::GridSquare;
use pan_orbit_camera::PanOrbitCamera;
use player::{Player, PLAYER_ELEVATION};

pub mod grid;
mod movement;
mod pan_orbit_camera;
pub mod player;

fn main() {
    // Lots derived from
    // https://caballerocoll.com/blog/bevy-chess-tutorial/
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        // ***** bevy mod picking *****
        // https://github.com/aevyrie/bevy_mod_picking/
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        // .add_plugin(HighlightablePickingPlugin)
        // .add_plugin(DebugCursorPickingPlugin)
        // .add_plugin(DebugEventsPickingPlugin)
        // ***** END bevy mod picking *****
        .add_system(movement::toggle_movement_ui.system())
        .add_system(movement::movement_ui.system())
        .add_system(movement::clean_up_movement_indicators.system())
        .add_system(movement::movement.system())
        .insert_resource(movement::MovementUiState::default())
        .add_system(pan_orbit_camera::pan_orbit_camera.system())
        .run();
}

const BOARD_SIZE: f32 = 2000.0;

// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // player
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.2, 0.2, 1.0).into()),
            transform: Transform::from_xyz(0.0, PLAYER_ELEVATION, 0.0),
            ..Default::default()
        })
        .insert(Player)
        .with_children(|parent| {
            // spawn camera
            let translation = Vec3::new(-2.0, 2.5, 5.0);
            let radius = translation.length();
            // Add Camera to stay relative to Player
            parent
                .spawn_bundle(PerspectiveCameraBundle {
                    transform: Transform::from_translation(translation)
                        .looking_at(Vec3::ZERO, Vec3::Y),
                    ..Default::default()
                })
                .insert(PanOrbitCamera {
                    radius,
                    ..Default::default()
                })
                .insert_bundle(PickingCameraBundle::default());
        });

    // Light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(0., 14.0, 0.)),
        ..Default::default()
    });

    // Grid!
    let black_material = materials.add(Color::rgb(0.1, 0.1, 0.1).into());
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: BOARD_SIZE })),
            material: black_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(GridSquare);
}
