use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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

pub mod parallax;


fn main() {
    
    // App builder
    App::new()

        // PLUGINS
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Tilemap test".into(),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    window_theme: Some(bevy::window::WindowTheme::Dark),
                    // mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(LdtkPlugin)

        .add_plugins(player::PlayerPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(parallax::ParallaxPlugin)
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
    _asset_server: Res<AssetServer>,
) {
    println!("\n\nKeybindings:\n- W,A,S,D :: Movement\n- R,T :: Switch demo level\n- TAB :: Debug mode\n- ESC :: Close window\n\nHave fun\n");
    commands.spawn(RigidBody::Fixed)
        .insert(Collider::ball(1.))
        .insert(TransformBundle::default())
        .insert(Name::new("test_collider"));
}