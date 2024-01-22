use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{components::*, CameraState, FpsCounter};

pub fn move_camera(
    inputs: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection, &CameraMarker)>,
) {
    let (
        mut transform, 
        mut projection, 
        marker
    )   = camera.get_single_mut().unwrap();

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
    transform.translation += direction.normalize_or_zero() * marker.fc_move_speed * projection.scale * time.delta_seconds();


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

pub fn camera_follow_player(
    player: Query<&Transform, With<PlayerMarker>>,
    mut camera: Query<(&mut Transform, &CameraMarker), (With<Camera2d>, Without<PlayerMarker>)>,
) {
    let Ok(player_transform) = player.get_single() else { return };
    let Ok((mut camera_transform, marker)) = camera.get_single_mut() else { return };

    let delta = player_transform.translation - camera_transform.translation;

    if delta.length() > 1.0 {
       camera_transform.translation += marker.fp_move_speed * delta;
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

    if cam_state.is_changed() {
        if *cam_state == CameraState::FollowPlayer {
            proj.scale = 0.25;
        }
        else if *cam_state == CameraState::FreeCam {
            proj.scale = 0.45;
        }
    }
}

pub fn update_fps(
    mut fps_counter: ResMut<FpsCounter>,
    time: Res<Time>
) {
    fps_counter.fps = 1.0 / time.delta_seconds();
}