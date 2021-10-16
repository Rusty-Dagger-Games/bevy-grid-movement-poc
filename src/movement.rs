use bevy::prelude::*;
use bevy_mod_picking::{SelectionEvent, *};

use crate::grid::GridSquare;
use crate::player::Player;

pub struct MovementUiState {
    enabled: bool,
}

impl Default for MovementUiState {
    fn default() -> Self {
        MovementUiState { enabled: false }
    }
}
pub struct MovementHoverIndicator;

pub fn clean_up_movement_indicators(
    mut commands: Commands,
    mut q_indicators: Query<Entity, With<MovementHoverIndicator>>,
    movement_ui_state: Res<MovementUiState>,
) {
    if movement_ui_state.enabled == false {
        for indicator in q_indicators.iter_mut() {
            commands.entity(indicator).despawn();
        }
    }
}

// Toggle the player movement UI on and off
pub fn toggle_movement_ui(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut movement_ui_state: ResMut<MovementUiState>,
    q_camera: Query<&PickingCamera>,
) {
    if keyboard_input.just_pressed(KeyCode::LShift) && !movement_ui_state.enabled {
        info!("Left-Shift Pressed - Enabling Movement UI.");
        movement_ui_state.enabled = true;
        if let Ok(camera) = q_camera.single() {
            if let Some((_intersected_grid_square, intersection)) = camera.intersect_top() {
                // spawn new entity with mesh as indicator
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                        material: materials.add(Color::rgb(10.2, 10.2, 10.2).into()),
                        transform: Transform::from_xyz(
                            intersection.position().x.round(),
                            intersection.position().y.round(),
                            intersection.position().z.round(),
                        ),
                        ..Default::default()
                    })
                    // insert into Entity the MovementHoverIndicator component
                    .insert(MovementHoverIndicator);
            }
        };
    } else if keyboard_input.just_released(KeyCode::LShift) && movement_ui_state.enabled {
        info!("Left-Shift Released - Disabling Movement UI.");
        movement_ui_state.enabled = false;
    }
}

pub fn movement_ui(
    mut commands: Commands,
    movement_ui_state: Res<MovementUiState>,
    mut q_indicators: Query<(Entity, &Transform, &mut MovementHoverIndicator)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_camera: Query<&PickingCamera>,
) {
    if movement_ui_state.enabled == false {
        return;
    }
    if let Ok(camera) = q_camera.single() {
        if let Some((_entity, intersection)) = camera.intersect_top() {
            // Spawn MovementHoverIndicator at this but slightly higher
            let hovered_square_transform = intersection.position();
            // spawn new entity with mesh as indicator
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                    material: materials.add(Color::rgb(10.2, 10.2, 10.2).into()),
                    transform: Transform::from_xyz(
                        hovered_square_transform.x.round(),
                        hovered_square_transform.y.round(),
                        hovered_square_transform.z.round(),
                    ),
                    ..Default::default()
                })
                // insert into Entity the MovementHoverIndicator component
                .insert(MovementHoverIndicator);
            for (indicator, transform, _) in q_indicators.iter_mut() {
                if transform.translation != hovered_square_transform {
                    commands.entity(indicator).despawn();
                }
            }
        }
    }
}

pub fn movement(
    mut events: EventReader<PickingEvent>,
    mut q_player: Query<&mut Transform, With<Player>>,
    mut q_selected: Query<&mut Selection, With<GridSquare>>,
    movement_ui_state: Res<MovementUiState>,
    q_camera: Query<&PickingCamera>,
) {
    if !movement_ui_state.enabled {
        return;
    }
    if let Ok(camera) = q_camera.single() {
        if let Some((_entity, intersection)) = camera.intersect_top() {
            for event in events.iter() {
                match &event {
                    &PickingEvent::Selection(SelectionEvent::JustSelected(_entity)) => {
                        let player_translation =
                            Vec3::new(intersection.position().x, 0.5, intersection.position().z);
                        let mut player = q_player.single_mut().unwrap();
                        player.translation = player_translation;
                        let mut selected = q_selected.single_mut().unwrap();
                        selected.set_selected(false);
                    }
                    PickingEvent::Selection(_) => (),
                    PickingEvent::Hover(_) => (),
                }
            }
        }
    }
}
