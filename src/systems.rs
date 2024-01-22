use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{components::*, CameraState};

pub fn move_camera(
    inputs: Res<Input<KeyCode>>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection, &CameraMarker)>,
) {
    let (
        mut transform, 
        mut projection, 
        marker
    )   = camera.get_single_mut().unwrap();

    if inputs.pressed(KeyCode::A) {
        transform.translation.x -= marker.move_speed * marker.zoom_speed;
    }
    else if inputs.pressed(KeyCode::D) {
        transform.translation.x += marker.move_speed * marker.zoom_speed;
    }
    
    if inputs.pressed(KeyCode::S) {
        transform.translation.y -= marker.move_speed * marker.zoom_speed; 
    }
    else if inputs.pressed(KeyCode::W) {
        transform.translation.y += marker.move_speed * marker.zoom_speed;
    }
    
    if inputs.pressed(KeyCode::Q) {
        projection.scale /= marker.zoom_speed;
    }
    else if inputs.pressed(KeyCode::E) {
        projection.scale *= marker.zoom_speed;
    }
}

pub fn move_player(
    inputs: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<PlayerMarker>>,
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else { return };
    
    let player_speed: f32 = 1.0;

    let mut direction: Vec3 = Vec3::ZERO;

    if inputs.pressed(KeyCode::A) {
        direction.x = -1.0;
    }
    else if inputs.pressed(KeyCode::D) {
        direction.x = 1.0;
    }
    
    if inputs.pressed(KeyCode::S) {
        direction.y = -1.0;
    }
    else if inputs.pressed(KeyCode::W) {
        direction.y = 1.0;
    }

    player_transform.translation += direction.normalize_or_zero() * player_speed;
}

pub fn sync_player_camera(
    player: Query<&Transform, With<PlayerMarker>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, With<CameraMarker>, Without<PlayerMarker>)>,
) {
    let Ok(player_transform) = player.get_single() else { return };
    let Ok(mut camera_transform) = camera.get_single_mut() else { return };

    let delta = player_transform.translation - camera_transform.translation;

    if delta.length() > 1.0 {
       camera_transform.translation += 0.01 * delta;
    }
    else {
        camera_transform.translation = player_transform.translation;
    }
}

pub fn change_levels(
    inputs: Res<Input<KeyCode>>,
    mut commands: Commands
) {
    if inputs.just_pressed(KeyCode::R) {
        commands.insert_resource(LevelSelection::Index(1));
    }
    else if inputs.just_pressed(KeyCode::T) {
        commands.insert_resource(LevelSelection::Index(0));
    }
}

pub fn switch_cam(
    keyboard: Res<Input<KeyCode>>,
    cam_state: ResMut<State<CameraState>>,
    mut commands: Commands 
) {
    if keyboard.just_pressed(KeyCode::Space){
        if *cam_state == CameraState::FreeCam {
            commands.insert_resource(NextState(Some(CameraState::FollowPlayer)))
        }
        else if *cam_state == CameraState::FollowPlayer {
            commands.insert_resource(NextState(Some(CameraState::FreeCam)))
        }
    }
}

pub fn reset_zoom(
    cam_state: Res<State<CameraState>>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    let Ok(mut proj) = camera_query.get_single_mut() else { return };

    if cam_state.is_changed() && *cam_state == CameraState::FollowPlayer {
        proj.scale = 0.25;
    }
}