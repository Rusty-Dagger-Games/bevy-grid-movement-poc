use bevy::prelude::*;
use bevy_mod_picking::{ *, SelectionEvent};

fn main() {
    // Lots derived from
    // https://caballerocoll.com/blog/bevy-chess-tutorial/
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(move_player.system())
        // https://github.com/aevyrie/bevy_mod_picking/
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(HighlightablePickingPlugin)
        // .add_plugin(DebugCursorPickingPlugin)
        // .add_plugin(DebugEventsPickingPlugin)
        .add_system(tile_selection_reader.system())
        .run();
}

// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // set up the camera
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 13.0;
    camera.transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);

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
            // Add Camera to stay relative to Player
            parent
                .spawn_bundle(camera)
                .insert_bundle(PickingCameraBundle::default());
        });

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

struct Player;
struct GridSquare;

const BOARD_SIZE: f32 = 14.0;
const ELEVATION: f32 = 1.0;
const PLAYER_ELEVATION: f32 = ELEVATION + 1.0;
const MAX_ELEVATION: f32 = 28.0;

// control the game character
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform, With<Player>)>,
) {
    let mut moved = false;
    let mut player = match player.single_mut() {
        Ok((player, _)) => player,
        Err(_) => return,
    };
    // -x -z
    if keyboard_input.just_pressed(KeyCode::W) {
        if player.translation.z > -BOARD_SIZE + 1. {
            player.translation.z -= 1.0;
        }
        if player.translation.x > -BOARD_SIZE + 1. {
            player.translation.x -= 1.0;
        }
        moved = true;
    }
    // -z
    if keyboard_input.just_pressed(KeyCode::E) {
        if player.translation.z > -BOARD_SIZE + 1. {
            player.translation.z -= 1.0;
        }
        moved = true;
    }
    // -z +x
    if keyboard_input.just_pressed(KeyCode::D) {
        if player.translation.z > -BOARD_SIZE {
            player.translation.z -= 1.0;
        }
        if player.translation.x < BOARD_SIZE {
            player.translation.x += 1.0;
        }
        moved = true;
    }
    // +x
    if keyboard_input.just_pressed(KeyCode::C) {
        if player.translation.x < BOARD_SIZE - 1. {
            player.translation.x += 1.0;
        }
        moved = true;
    }
    // +x +z
    if keyboard_input.just_pressed(KeyCode::X) {
        if player.translation.z < BOARD_SIZE - 1. {
            player.translation.z += 1.0;
        }
        if player.translation.x < BOARD_SIZE - 1. {
            player.translation.x += 1.0;
        }
        moved = true;
    }
    // +z
    if keyboard_input.just_pressed(KeyCode::Z) {
        if player.translation.z < BOARD_SIZE - 1. {
            player.translation.z += 1.0;
        }
        moved = true;
    }
    // -x +z
    if keyboard_input.just_pressed(KeyCode::A) {
        if player.translation.z < BOARD_SIZE - 1. {
            player.translation.z += 1.0;
        }
        if player.translation.x > -BOARD_SIZE {
            player.translation.x -= 1.0;
        }
        moved = true;
    }
    // -x
    if keyboard_input.just_pressed(KeyCode::Q) {
        if player.translation.x > -BOARD_SIZE + 1. {
            player.translation.x -= 1.0;
        }
        moved = true;
    }
    // +y
    if keyboard_input.just_pressed(KeyCode::Up) {
        if player.translation.y < MAX_ELEVATION {
            player.translation.y += 1.0;
        }
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        if player.translation.y > ELEVATION {
            player.translation.y -= 1.0;
        }
        moved = true;
    }

    // move on the board
    if moved {
        *player = Transform {
            translation: Vec3::new(
                player.translation.x,
                player.translation.y,
                player.translation.z,
            ),
            ..Default::default()
        };
    }
}

// listen for selected event.
fn tile_selection_reader(
    mut events: EventReader<PickingEvent>,
    mut queries: QuerySet<(
        Query<&mut Transform, With<Player>>,
        Query<(&Transform, &Selection), With<GridSquare>>,
    )>
) {
    for event in events.iter() {
        match &event {
            &PickingEvent::Selection(SelectionEvent::JustSelected(entity)) => {
                let mut player_translation = Vec3::new(1.0,1.0,1.0);
                for (picked_transform, is_selected) in queries.q1().clone().iter() {
                    if is_selected.selected() {
                        player_translation = Vec3::new(
                            picked_transform.translation.x,
                            ELEVATION+0.5,
                            picked_transform.translation.z,
                        )
                    }
                }
                let mut player = queries.q0_mut().single_mut().unwrap();
                player.translation = player_translation;
            }
            PickingEvent::Selection(_) => (),
            PickingEvent::Hover(_) => (),
        }
    }
}
