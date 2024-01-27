use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_inspector_egui::inspector_options::ReflectInspectorOptions;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use bevy_inspector_egui::InspectorOptions;
use bevy_rapier2d::render::DebugRenderContext;

use crate::player::components::PlayerMarker;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        // EGUI
        .add_plugins(ResourceInspectorPlugin::run_if(ResourceInspectorPlugin::<FpsCounter>::default(), in_state(CameraState::FreeCam)))
        
        .add_plugins(WorldInspectorPlugin::run_if(WorldInspectorPlugin::new(), in_state(CameraState::FreeCam)))
        
        .init_resource::<FpsCounter>() // `ResourceInspectorPlugin` won't initialize the resource
        .register_type::<FpsCounter>() // you need to register your type to display it 
        // ------

        // STATES
        .add_state::<CameraState>()
        // ------

        // SYSTEMS
        .add_systems(Startup, (
                setup_camera,
            ))
            .add_systems(Update, (
                move_camera.run_if(in_state(CameraState::FreeCam)),
                camera_follow_player.run_if(in_state(CameraState::FollowPlayer)),
                switch_cam,
                reset_zoom,
                update_fps,
            ));
        // -------
    }
}

//


#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum CameraState {
    FreeCam,
    #[default]
    FollowPlayer,
}

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct FpsCounter {
    fps: f32,
}

#[derive(Default, Component)]
pub struct CameraMarker {
    pub zoom_speed: f32,
    pub fc_move_speed: f32,
    pub fp_move_speed: f32,
    pub cam_offset: Vec3,
}



// SYSTEMS

pub fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            camera_2d: Camera2d { clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0))},
            ..default()
        },
        CameraMarker {
            zoom_speed:1.02,
            fc_move_speed:1000.0,
            fp_move_speed:0.018,
            cam_offset: Vec3::ZERO,
        }
    ));
}

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
        direction.x = -1f32;
    }
    else if inputs.pressed(KeyCode::D) {
        direction.x = 1f32;
    }
    
    if inputs.pressed(KeyCode::S) {
        direction.y = -1f32;
    }
    else if inputs.pressed(KeyCode::W) {
        direction.y = 1f32;
    }
    transform.translation += direction.normalize_or_zero() * marker.fc_move_speed * projection.scale * time.delta_seconds();


    if inputs.pressed(KeyCode::E) {
        projection.scale /= marker.zoom_speed;
    }
    else if inputs.pressed(KeyCode::Q) {
        projection.scale *= marker.zoom_speed;
    }
}

pub fn camera_follow_player (
    time: Res<Time>,
    player: Query<&Transform, With<PlayerMarker>>,
    mut camera: Query<(&mut Transform, &CameraMarker), (With<Camera2d>, Without<PlayerMarker>)>,
) {
    let Ok(player_transform) = player.get_single() else { return };
    let Ok((mut camera_transform, marker)) = camera.get_single_mut() else { return };

    let delta = player_transform.translation - camera_transform.translation;

    if delta.length() > 1.0 {
       camera_transform.translation += marker.fp_move_speed * delta * time.delta_seconds() * 100.0;
    }
    else {
        camera_transform.translation = player_transform.translation;
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

pub fn switch_cam(
    keyboard: Res<Input<KeyCode>>,
    cam_state: ResMut<State<CameraState>>,
    mut commands: Commands,
    mut physics_debugger: ResMut<DebugRenderContext>
) {
    if keyboard.just_pressed(KeyCode::Tab){
        if *cam_state == CameraState::FreeCam {
            commands.insert_resource(NextState(Some(CameraState::FollowPlayer)));
            physics_debugger.enabled = false;
        }
        else if *cam_state == CameraState::FollowPlayer {
            commands.insert_resource(NextState(Some(CameraState::FreeCam)));
            physics_debugger.enabled = true;
        }
    }
}

pub fn update_fps(
    mut fps_counter: ResMut<FpsCounter>,
    time: Res<Time>
) {
    fps_counter.fps = 1.0 / time.delta_seconds();
}