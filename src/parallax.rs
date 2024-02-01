use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;

use crate::camera::{CameraMarker, move_camera};

// PLUGIN
pub struct ParallaxPlugin;

impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_default)
            .add_systems(Update, set_paralax_positions.after(move_camera))
            .register_type::<ParallaxObject>();
    }
}

// COMPONENT

#[derive(Component)]
pub struct ParallaxParent;


#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
#[derive(Component)]
pub struct ParallaxObject {
    pub layer: u32,
    /// the layers offset is relative to this point. Its zero on top of it
    pub origin: Vec2,
    //  Optional offset
    // pub offset: Vec3,
}

impl Default for ParallaxObject{
    fn default() -> ParallaxObject {
        ParallaxObject{
            layer: 2,
            origin: Vec2::new(0.0, 0.0),
            // offset: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

// SYSTEMS

pub fn set_paralax_positions(
    mut targets_query: Query<(&mut Transform, &ParallaxObject), Without<CameraMarker>>,
    camera_query: Query<&Transform, With<CameraMarker>>,
) {
    if let Ok(camera) = camera_query.get_single() {
        for (mut target_transform, target_vals) in targets_query.iter_mut(){
            // target.translation.x = camera.translation.x; 
            // target.translation.y = camera.translation.y;
            let mut oc: Vec2 = camera.translation.xy() - target_vals.origin;
            oc *= (0.1 * target_vals.layer as f32).max(0.001);
            target_transform.translation.x = target_vals.origin.x + oc.x;
            target_transform.translation.y = target_vals.origin.y + oc.y;
            target_transform.translation.z = 1.0 - (0.1 * (target_vals.layer as f32));
        }
    }
}

pub fn spawn_default(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let glob_y = 130.0;
    let glob_x = 630.0;

    commands.spawn((ParallaxParent, VisibilityBundle::default(), TransformBundle::default()))
        .with_children(|parent| {
            
            // -- First __

            parent.spawn(SpriteBundle {
                texture: asset_server.load("paralax_layers/parallax-demon-woods-close-trees.png"),
                transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)),
                ..default()
            }).insert(ParallaxObject { origin: Vec2::new(glob_x, glob_y), layer: 1})
            .insert(Name::new("Parallax Layer 1".to_string()));
            
            parent.spawn(SpriteBundle {
                texture: asset_server.load("paralax_layers/parallax-demon-woods-mid-trees.png"),
                transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)),
                ..default()
            }).insert(ParallaxObject { origin: Vec2::new(glob_x, glob_y), layer: 2})
            .insert(Name::new("Parallax Layer 2".to_string()));
            
            parent.spawn(SpriteBundle {
                texture: asset_server.load("paralax_layers/parallax-demon-woods-far-trees.png"),
                transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)),
                ..default()
            }).insert(ParallaxObject { origin: Vec2::new(glob_x, glob_y), layer: 3})
            .insert(Name::new("Parallax Layer 3".to_string()));
            
            // ---
        
            parent.spawn(SpriteBundle {
                texture: asset_server.load("paralax_layers/parallax-demon-woods-close-trees.png"),
                transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)),
                ..default()
            }).insert(ParallaxObject { origin: Vec2::new(-327.0, glob_y), layer: 1})
            .insert(Name::new("Parallax Layer 1".to_string()));
            
            parent.spawn(SpriteBundle {
                texture: asset_server.load("paralax_layers/parallax-demon-woods-mid-trees.png"),
                transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)),
                ..default()
            }).insert(ParallaxObject { origin: Vec2::new(-327.0, glob_y), layer: 2})
            .insert(Name::new("Parallax Layer 2".to_string()));
            
            parent.spawn(SpriteBundle {
                texture: asset_server.load("paralax_layers/parallax-demon-woods-far-trees.png"),
                transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.5)),
                ..default()
            }).insert(ParallaxObject { origin: Vec2::new(-327.0, glob_y), layer: 3})
            .insert(Name::new("Parallax Layer 3".to_string()));
        }
    ).insert(Name::new("Parallax Layers".to_string()));
}