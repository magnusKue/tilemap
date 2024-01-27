use bevy::prelude::*;

use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

// MODULES

pub mod systems;
// use crate::systems::*;

pub mod components;
use crate::components::*;

pub mod camera;

pub mod physics;
// use crate::physics::*;

pub mod player;

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
) {
    commands.spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(2000f32, 20f32))
        .insert(Name::new("test_collider"));
}