use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

// MODULES

pub mod systems;
// use crate::systems::*;

pub mod components;
use crate::{components::*, paralax::ParallaxObject};

pub mod camera;

pub mod physics;
// use crate::physics::*;

pub mod player;

pub mod level;

pub mod paralax;


fn main() {
    // App builder
    App::new()
        // PLUGINS
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)

        .add_plugins(player::PlayerPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(paralax::ParallaxPlugin)
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
    asset_server: Res<AssetServer>,
) {
    println!("\n\nKeybindings:\n- W,A,S,D :: Movement\n- R,T :: Switch demo level\n- TAB :: Debug mode\n- ESC :: Close window\n\nHave fun\n");
    // commands.spawn(RigidBody::Fixed)
    //     .insert(Collider::cuboid(20000f32, 20f32))
    //     .insert(Name::new("test_collider"));

    let glob_y = 130.0;
    commands.spawn(SpriteBundle {
        texture: asset_server.load("paralax_layers/parallax-demon-woods-close-trees.png"),
        transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)),
        ..default()
    }).insert(ParallaxObject { origin: Vec2::new(630.0, glob_y), layer: 1})
    .insert(Name::new("Parallax Layer 1".to_string()));
    
    commands.spawn(SpriteBundle {
        texture: asset_server.load("paralax_layers/parallax-demon-woods-mid-trees.png"),
        transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)),
        ..default()
    }).insert(ParallaxObject { origin: Vec2::new(630.0, glob_y), layer: 2})
    .insert(Name::new("Parallax Layer 2".to_string()));
    
    commands.spawn(SpriteBundle {
        texture: asset_server.load("paralax_layers/parallax-demon-woods-far-trees.png"),
        transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)),
        ..default()
    }).insert(ParallaxObject { origin: Vec2::new(630.0, glob_y), layer: 3})
    .insert(Name::new("Parallax Layer 3".to_string()));




    commands.spawn(SpriteBundle {
        texture: asset_server.load("paralax_layers/parallax-demon-woods-close-trees.png"),
        transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)),
        ..default()
    }).insert(ParallaxObject { origin: Vec2::new(-327.0, glob_y), layer: 1})
    .insert(Name::new("Parallax Layer 1".to_string()));
    
    commands.spawn(SpriteBundle {
        texture: asset_server.load("paralax_layers/parallax-demon-woods-mid-trees.png"),
        transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)),
        ..default()
    }).insert(ParallaxObject { origin: Vec2::new(-327.0, glob_y), layer: 2})
    .insert(Name::new("Parallax Layer 2".to_string()));
    
    commands.spawn(SpriteBundle {
        texture: asset_server.load("paralax_layers/parallax-demon-woods-far-trees.png"),
        transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)),
        ..default()
    }).insert(ParallaxObject { origin: Vec2::new(-327.0, glob_y), layer: 3})
    .insert(Name::new("Parallax Layer 3".to_string()));
}