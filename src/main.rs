use bevy::prelude::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy_ecs_ldtk::prelude::*;

pub mod systems;
use crate::systems::*;

pub mod components;
use crate::components::*;

pub mod states;
use crate::states::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )
        .add_plugins(LdtkPlugin)

        .add_state::<CameraState>()

        .add_systems(Startup, setup)
        .add_systems(Update, (
            move_camera.run_if(in_state(CameraState::FreeCam)),
            move_player.run_if(in_state(CameraState::FollowPlayer)),
            sync_player_camera.run_if(in_state(CameraState::FollowPlayer)),
            change_levels,
            switch_cam,
            reset_zoom
            ))

        .insert_resource(LevelSelection::Index(0))

        .register_ldtk_entity::<EnemyBundle>("MyEntityIdentifier")
        .register_ldtk_entity::<EnemyBundle>("TestEntity")
        .register_ldtk_entity::<PlayerBundle>("Player")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            camera_2d: Camera2d { clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0))},
            ..default()
        },
        CameraMarker {
            zoom_speed:1.02,
            move_speed:8.0,
            cam_offset: Vec3::ZERO,
        }
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_project.ldtk"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}