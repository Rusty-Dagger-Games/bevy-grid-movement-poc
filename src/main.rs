use bevy::prelude::*;
use bevy_mod_picking::{SelectionEvent, *};
use grid::{ELEVATION, GridSquare};
use pan_orbit_camera::PanOrbitCamera;
use player::{PLAYER_ELEVATION, Player};

mod pan_orbit_camera;
mod movement;
pub mod grid;
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
        .add_system(movement::movement.system())
        .insert_resource(movement::MovementUiState::default())
        .add_system(pan_orbit_camera::pan_orbit_camera.system())
        .run();
}

const BOARD_SIZE: f32 = 6.0;

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
                }).insert(PanOrbitCamera {
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
    let white_material = materials.add(Color::rgb(0.9, 0.9, 0.9).into());
    for i in (-BOARD_SIZE) as i32..(BOARD_SIZE) as i32 {
        for j in (-BOARD_SIZE) as i32..(BOARD_SIZE) as i32 {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: 1. })),
                    material: black_material.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        (i as f32) + 0.5,
                        ELEVATION,
                        (j as f32) + 0.5,
                    )),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(GridSquare);

            // Lines along the Z axis
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 1. })),
                material: white_material.clone(),
                transform: Transform {
                    translation: Vec3::new(i as f32, ELEVATION + 0.001, j as f32),
                    scale: Vec3::new(0.1, 0.1, 1.),
                    ..Default::default()
                },
                ..Default::default()
            });
            // Lines along the X axis
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 1. })),
                material: white_material.clone(),
                transform: Transform {
                    translation: Vec3::new(i as f32, ELEVATION + 0.001, j as f32),
                    scale: Vec3::new(1., 0.1, 0.1),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

