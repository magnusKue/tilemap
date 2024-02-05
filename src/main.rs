use bevy::prelude::*;
use bevy_framepace::{FramepaceSettings, Limiter};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};
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

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct DebugSettings {
    pub limit_framerate: bool,
    pub clamped_framerate: u32, 
}

impl Default for DebugSettings {
    fn default() -> Self {
        DebugSettings { limit_framerate: false, clamped_framerate: 60 }
    }
}

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
        .add_plugins(bevy_framepace::FramepacePlugin)


        .add_plugins(player::PlayerPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(parallax::ParallaxPlugin)
        .add_plugins(level::LevelPlugin)
        // ------


        // Systems
        .add_systems(Update, (
            bevy::window::close_on_esc,
            toggle_debug,
        ))
        .add_systems(Startup, (
            setup,
        ))
        // ------


        // LDtk entitys
        .register_ldtk_entity::<EnemyBundle>("MyEntityIdentifier")
        // ------
        
        .init_resource::<DebugSettings>()
        .register_type::<DebugSettings>()

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

fn toggle_debug(
    settings: Res<DebugSettings>,
    mut frame_pace: ResMut<FramepaceSettings>,
) {
    if settings.is_changed() {
        match settings.limit_framerate {
            true => frame_pace.limiter = Limiter::from_framerate(settings.clamped_framerate as f64),
            false=> frame_pace.limiter = Limiter::Off,
        }   
    }
}