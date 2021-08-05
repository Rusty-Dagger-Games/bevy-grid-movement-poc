use bevy::{prelude::*};

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(move_player.system())
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
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.2, 0.2, 1.0).into()),
        transform: Transform::from_xyz(0.0, ELEVATION + 1.0, 0.0),
        ..Default::default()
    })
    .insert(Player)
    .with_children(|parent| {
        // Add Camera to stay relative to Player
        parent.spawn_bundle(camera);
    });

    // Grid!
    let black_material = materials.add(Color::rgb(0.1, 0.1, 0.1).into());
    let white_material = materials.add(Color::rgb(0.9, 0.9, 0.9).into());
    for i in (-BOARD_SIZE) as i32..(BOARD_SIZE) as i32 {
        for j in (-BOARD_SIZE) as i32..(BOARD_SIZE) as i32 {
            commands.spawn_bundle(
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: 1. })),
                    material: black_material.clone(),
                    transform: Transform::from_translation(Vec3::new(i as f32, ELEVATION, j as f32)),
                    ..Default::default()
                }
            );

            // Lines along the Z axis
            commands.spawn_bundle(
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: 1. })),
                    material: white_material.clone(),
                    transform: Transform { 
                        translation: Vec3::new(i as f32, ELEVATION + 0.001, j as f32),
                        scale: Vec3::new(0.1, 0.1, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            );
            // Lines along the X axis
            commands.spawn_bundle(
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: 1. })),
                    material: white_material.clone(),
                    transform: Transform { 
                        translation: Vec3::new(i as f32, ELEVATION + 0.001, j as f32),
                        scale: Vec3::new(1., 0.1, 0.1),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            );
        }
    }
}

struct Player;

const BOARD_SIZE: f32 = 14.0;
const ELEVATION: f32 = 1.0;
const MAX_ELEVATION: f32 = 28.0;

// control the game character
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut transforms: Query<(&mut Transform, With<Player>)>,
) {
    let mut moved = false;
    let mut player = match transforms.single_mut() {
        Ok((player, _)) => player,
        Err(_) => return
    };
    // -x -z
    if keyboard_input.just_pressed(KeyCode::W) {
        eprintln!("W was pressed");
        if player.translation.z > -BOARD_SIZE + 1. {
            eprintln!("z: {:?}", player.translation.z);
            player.translation.z -= 1.0;
        }
        if player.translation.x > -BOARD_SIZE + 1. {            
            eprintln!("x: {:?}", player.translation.x);
            player.translation.x -= 1.0;
        }
        moved = true;
    }
    // -z
    if keyboard_input.just_pressed(KeyCode::E) {
        eprintln!("E was pressed");
        if player.translation.z > -BOARD_SIZE + 1. {
            eprintln!("z: {:?}", player.translation.z);
            player.translation.z -= 1.0;
        }
        moved = true;
    }
    // -z +x
    if keyboard_input.just_pressed(KeyCode::D) {
        eprintln!("D was pressed");
        if player.translation.z > -BOARD_SIZE {
            eprintln!("z: {:?}", player.translation.z);
            player.translation.z -= 1.0;
        }
        if player.translation.x < BOARD_SIZE {            
            eprintln!("x: {:?}", player.translation.x);
            player.translation.x += 1.0;
        }
        moved = true;
    }
    // +x
    if keyboard_input.just_pressed(KeyCode::C) {
        eprintln!("C was pressed");
        if player.translation.x < BOARD_SIZE - 1. {            
            eprintln!("x: {:?}", player.translation.x);
            player.translation.x += 1.0;
        }
        moved = true;
    }
    // +x +z
    if keyboard_input.just_pressed(KeyCode::X) {
        eprintln!("X was pressed");
        if player.translation.z < BOARD_SIZE - 1. {
            eprintln!("z: {:?}", player.translation.z);
            player.translation.z += 1.0;
        }
        if player.translation.x < BOARD_SIZE - 1. {            
            eprintln!("x: {:?}", player.translation.x);
            player.translation.x += 1.0;
        }
        moved = true;
    }
    // +z
    if keyboard_input.just_pressed(KeyCode::Z) {
        eprintln!("Z was pressed");
        if player.translation.z < BOARD_SIZE - 1. {
            eprintln!("z: {:?}", player.translation.z);
            player.translation.z += 1.0;
        }
        moved = true;
    }
    // -x +z
    if keyboard_input.just_pressed(KeyCode::A) {
        eprintln!("A was pressed");
        if player.translation.z < BOARD_SIZE - 1. {
            eprintln!("z: {:?}", player.translation.z);
            player.translation.z += 1.0;
        }
        if player.translation.x > -BOARD_SIZE {            
            eprintln!("x: {:?}", player.translation.x);
            player.translation.x -= 1.0;
        }
        moved = true;
    }
    // -x
    if keyboard_input.just_pressed(KeyCode::Q) {
        eprintln!("Q was pressed");
        if player.translation.x > -BOARD_SIZE + 1. {            
            eprintln!("x: {:?}", player.translation.x);
            player.translation.x -= 1.0;
        }
        moved = true;
    }
    // +y
    if keyboard_input.just_pressed(KeyCode::Up) {
        eprintln!("+ was pressed");
        if player.translation.y < MAX_ELEVATION {            
            eprintln!("y: {:?}", player.translation.y);
            player.translation.y += 1.0;
        }
        moved = true;
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        eprintln!("- was pressed");
        if player.translation.y > ELEVATION {            
            eprintln!("y: {:?}", player.translation.y);
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
