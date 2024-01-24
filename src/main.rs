use bevy::prelude::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy_ecs_ldtk::prelude::*;

use bevy_inspector_egui::quick::{WorldInspectorPlugin, ResourceInspectorPlugin};

use bevy_rapier2d::prelude::*;

pub mod systems;
use crate::systems::*;

pub mod components;
use crate::components::*;

pub mod camera;
use crate::camera::*;

pub mod physics;
// use crate::physics::*;



fn main() {
    // App builder
    App::new()
        // PLUGINS
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        // ------

        // EGUI
        .init_resource::<FpsCounter>() // `ResourceInspectorPlugin` won't initialize the resource
        .register_type::<FpsCounter>() // you need to register your type to display it
        .add_plugins(ResourceInspectorPlugin::run_if(ResourceInspectorPlugin::<FpsCounter>::default(), in_state(CameraState::FreeCam)))
        .add_plugins(WorldInspectorPlugin::run_if(WorldInspectorPlugin::new(), in_state(CameraState::FreeCam)))
        // ------

        // RAPIER
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        // ------

        // STATES
        .add_state::<CameraState>()
        // ------

        // Systems
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, (
            setup,
        ))
        .add_systems(Update, (
            move_camera.run_if(in_state(CameraState::FreeCam)),
            move_player.run_if(in_state(CameraState::FollowPlayer)),
            camera_follow_player.run_if(in_state(CameraState::FollowPlayer)),
            change_levels,
            switch_cam,
            reset_zoom,
            update_fps
        ))
        // ------

        // Resources
        .insert_resource(LevelSelection::Identifier("World_Level_0".to_string()))
        .insert_resource(RapierConfiguration {
            ..Default::default()
        })
        // ------

        // LDtk entitys
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<EnemyBundle>("MyEntityIdentifier")
        // ------
        
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
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

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_project.ldtk"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    
    commands.spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(20f32, 20f32))
        .insert(Name::new("test_collider"));
}