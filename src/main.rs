use bevy::prelude::*;

use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

// MODULES

pub mod systems;
// use crate::systems::*;

pub mod components;
use crate::components::*;

pub mod camera;
use crate::camera::*;

pub mod physics;
// use crate::physics::*;

pub mod player;
use crate::player::*;

pub mod level;



fn main() {
    // App builder
    App::new()
        // PLUGINS
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)

        .add_plugins(player::PlayerPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(level::LevelPlugin)
        // ------

        // Systems
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, (
            setup,
        ))
        // ------

        // LDtk entitys
        .register_ldtk_entity::<EnemyBundle>("MyEntityIdentifier")
        // ------
        
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_project.ldtk"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert(Name::new("TileMap".to_string()));
    
    commands.spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(2000f32, 20f32))
        .insert(Name::new("test_collider"));
}