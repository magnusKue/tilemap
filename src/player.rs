use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::control::KinematicCharacterController;
use crate::physics::PlayerPhysicsBundle;
use crate::CameraMarker;
use crate::CameraState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (move_player.run_if(in_state(CameraState::FollowPlayer)),))
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

#[derive(Default, Component)]
pub struct PlayerMarker { }

#[derive(Component)]
pub struct PlayerPhysicsValues { 
    pub velocity: Vec2,
}

impl Default for PlayerPhysicsValues {
    fn default() -> PlayerPhysicsValues {
        PlayerPhysicsValues { velocity: Vec2::ZERO }
    }
}


#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    marker: PlayerMarker,

    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    
    physics: PlayerPhysicsBundle,
    physics_values: PlayerPhysicsValues
}


// SYSTEMS

pub fn move_player(
    inputs: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&Transform, &mut PlayerPhysicsValues), With<PlayerMarker>>,
    mut player_controller_query: Query<&mut KinematicCharacterController>,
    camera_query: Query<(&mut Transform, &CameraMarker), Without<PlayerMarker>>,
) {
    let Ok((player_transform, mut player_physics_values)) = player_query.get_single_mut() else { return };
    let Ok(mut player_controller) = player_controller_query.get_single_mut() else { return };

    let Ok((camera_transform, _)) = camera_query.get_single() else { return };
    
    if (camera_transform.translation - player_transform.translation).length() > 50f32 { return };

    let player_speed: Vec2 = Vec2 { x: 0.1f32, y: 1f32 };
    let jump_boost: f32 = 25f32;

    let friction = Vec2::new(0.2f32, 0.2f32);
    let acceleration = 1.2f32;
    let gravity = 0.4f32;
    
    player_physics_values.velocity *= Vec2::new(1f32, 1f32) - friction;
    player_physics_values.velocity.y -= gravity;

    let mut direction: Vec2 = Vec2::ZERO;

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

    if inputs.just_pressed(KeyCode::Space) {
        player_physics_values.velocity.y += jump_boost;
    }

    player_physics_values.velocity += direction.normalize_or_zero() * acceleration;

    player_controller.translation = Some(player_physics_values.velocity * player_speed * time.delta_seconds() * 100f32);
}