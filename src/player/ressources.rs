
use bevy::prelude::*;

use bevy_inspector_egui::inspector_options::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;


// PHYSICS CONSTANTS

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct PlayerPhysicsConstants {
    pub player_speed: f32,
    pub player_max_speed: f32,
    pub player_max_falling_speed: f32,
    pub falling_gravity_scaler: f32,
    pub coyote_time: f32,
    pub jump_boost: f32,
    pub friction: Vec2,
    pub acceleration: f32,
    pub gravity: f32,
    /// time the jump input will be buffered in secs
    pub jump_inp_buffering: f32,
}

impl Default for PlayerPhysicsConstants {
    fn default() -> Self {
        PlayerPhysicsConstants {
            player_speed: 0.1f32,
            player_max_speed: 4f32,
            player_max_falling_speed: 1.4f32,
            falling_gravity_scaler: 1.1,
            coyote_time: 0.1,
            jump_boost: 270f32,
            friction: Vec2::new(0.1f32, 0f32),
            acceleration: 1.2f32,
            gravity: 4f32,
            jump_inp_buffering: 0.1,
        }
    }
}


// PLAYER ANIMATION STATES

#[derive(Reflect, Default)]
pub enum PlayerAnimationStates{
    #[default]
    Idle,
    Running,
    Jumping,
    Falling,
    Landing,
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct PlayerAnimation {
    pub animation: PlayerAnimationStates,
    pub facing_left: bool,
    pub animate: bool,
    pub change_animations: bool,
    pub speed: f32,
}

impl Default for PlayerAnimation {
    fn default() -> PlayerAnimation {
        PlayerAnimation {
            animation: PlayerAnimationStates::Idle,
            facing_left: false,
            animate: true,
            change_animations: true,
            speed: 6.0,
        }
    }
}