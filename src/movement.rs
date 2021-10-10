use bevy::prelude::*;
use bevy_mod_picking::{SelectionEvent, *};

use crate::grid::{ELEVATION, GridSquare}; 
use crate::player::Player; 

pub struct MovementUiState {
    enabled: bool,
}

impl Default for MovementUiState {
    fn default() -> Self { MovementUiState { 
        enabled: false
    } }
}
pub struct MovementHoverIndicator;

// Toggle the player movement UI on and off
pub fn toggle_movement_ui (
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

pub fn movement_ui (
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

pub fn movement (
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