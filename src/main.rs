use bevy::prelude::*;
use bevy_mod_picking::{SelectionEvent, *};

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
        .add_system(toggle_movement_ui.system())
        .add_system(movement_ui.system())
        .add_system(movement.system())
        .insert_resource(MovementUiState { enabled: false })
        .run();
}

const BOARD_SIZE: f32 = 6.0;
const ELEVATION: f32 = 1.0;
const PLAYER_ELEVATION: f32 = ELEVATION + 1.0;
const MAX_ELEVATION: f32 = 28.0;

// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // set up the camera
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 5.0;
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

struct Player;
struct GridSquare;
struct MovementUiState {
    enabled: bool,
}
struct MovementHoverIndicator;

// Toggle the player movement UI on and off
fn toggle_movement_ui (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard_input: Res<Input<KeyCode>>, 
    mut movement_ui_state: ResMut<MovementUiState>,
    q_camera: Query<&PickingCamera>,
    q_grid_squares: Query<&Transform, With<GridSquare>>,
    mut q_indicators: Query<Entity, With<MovementHoverIndicator>>,
) {
    if keyboard_input.just_pressed(KeyCode::LShift) && !movement_ui_state.enabled {
        info!("Left-Shift Pressed - Enabling Movement UI.");
        movement_ui_state.enabled = true;
        if let Ok(camera) = q_camera.single() {
            if let Some((intersected_grid_square, _)) = camera.intersect_top() {
                if let Ok(hovered_square_transform) = q_grid_squares.get(intersected_grid_square) {
                    // spawn new entity with mesh as indicator
                    commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                            material: materials.add(Color::rgb(10.2, 10.2, 10.2).into()),
                            transform: Transform::from_xyz(
                                hovered_square_transform.translation.x, 
                                hovered_square_transform.translation.y, 
                                hovered_square_transform.translation.z
                            ),
                            ..Default::default()
                        })
                        // insert into Entity the MovementHoverIndicator component
                        .insert(MovementHoverIndicator);
                }
            }
        };
    } else if keyboard_input.just_released(KeyCode::LShift) && movement_ui_state.enabled {
        info!("Left-Shift Released - Disabling Movement UI.");
        movement_ui_state.enabled = false;
        // Clean up all the indicators that may be left.
        for indicator in q_indicators.iter_mut() {
            commands.entity(indicator).despawn();
        }
    }
}

fn movement_ui (
    mut commands: Commands,
    movement_ui_state: Res<MovementUiState>,
    mut events: EventReader<PickingEvent>,
    mut q_indicators: Query<(Entity, &Transform, &mut MovementHoverIndicator)>,
    q_grid_squares: Query<&Transform, With<GridSquare>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if movement_ui_state.enabled == false {
        return;
    }
    for event in events.iter() {
        match &event {
            &PickingEvent::Hover(HoverEvent::JustEntered(entity)) => {
                // Spawn MovementHoverIndicator at this but slightly higher
                // get Transform from current GridSquare
                let hovered_square_transform = q_grid_squares.get(*entity).unwrap().translation;
                // spawn new entity with mesh as indicator
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                        material: materials.add(Color::rgb(10.2, 10.2, 10.2).into()),
                        transform: Transform::from_xyz(
                            hovered_square_transform.x, 
                            hovered_square_transform.y, 
                            hovered_square_transform.z
                        ),
                        ..Default::default()
                    })
                    // insert into Entity the MovementHoverIndicator component
                    .insert(MovementHoverIndicator);
            },
            &PickingEvent::Hover(HoverEvent::JustLeft(entity)) => {
                // Despawn MovementHoverIndicator at this Transform
                let just_left_translation = q_grid_squares.get(*entity).unwrap().translation;
                for (indicator_entity, transform, _) in q_indicators.iter_mut() {
                    let translation = transform.translation;
                    if translation.x == just_left_translation.x && translation.z == just_left_translation.z {
                        commands.entity(indicator_entity).despawn();
                    }
                }
            },
            _ => ()
        }

    }
}

fn movement (
    mut events: EventReader<PickingEvent>,
    mut queries: QuerySet<(
        Query<&mut Transform, With<Player>>,
        Query<(&Transform, &Selection), With<GridSquare>>,
    )>,
    movement_ui_state: Res<MovementUiState>,
) {
    if !movement_ui_state.enabled {
        return;
    }
    for event in events.iter() {
        match &event {
            &PickingEvent::Selection(SelectionEvent::JustSelected(_entity)) => {
                let mut player_translation = Vec3::new(1.0, 1.0, 1.0);
                for (picked_transform, is_selected) in queries.q1().clone().iter() {
                    if is_selected.selected() {
                        player_translation = Vec3::new(
                            picked_transform.translation.x,
                            ELEVATION + 0.5,
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
