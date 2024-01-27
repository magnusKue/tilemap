use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::inspector_options::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
use bevy_rapier2d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};
use crate::physics::PlayerPhysicsBundle;
use crate::CameraMarker;
use crate::CameraState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, add_controller_output.before(move_player))
            .add_systems(Update, (move_player.run_if(in_state(CameraState::FollowPlayer)),))
            .register_ldtk_entity::<PlayerBundle>("Player")
            
            .init_resource::<PlayerPhysicsConstants>()
            .register_type::<PlayerPhysicsConstants>();
    }
}

//

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct PlayerPhysicsConstants {
    player_speed: f32,
    player_max_speed: f32,
    jump_boost: f32,
    friction: Vec2,
    acceleration: f32,
    gravity: f32,
}

impl Default for PlayerPhysicsConstants {
    fn default() -> Self {
        PlayerPhysicsConstants {
            player_speed: 0.1f32,
            player_max_speed: 4f32,
            jump_boost: 250f32,
            friction: Vec2::new(0.1f32, 0f32),
            acceleration: 1.2f32,
            gravity: 4f32,
        }
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

pub fn add_controller_output(
    mut player_controller_query: Query<&mut KinematicCharacterController>,
) {
    let Ok(mut player_controller) = player_controller_query.get_single_mut() else { return };
    player_controller.translation = Some(Vec2::ZERO);
}

pub fn move_player(
    inputs: Res<Input<KeyCode>>,
    time: Res<Time>,
    phys_consts: Res<PlayerPhysicsConstants>,
    mut player_query: Query<(&Transform, &mut PlayerPhysicsValues), With<PlayerMarker>>,
    mut player_controller_query: Query<&mut KinematicCharacterController>,
    player_controller_output_query: Query<&KinematicCharacterControllerOutput>,
    camera_query: Query<(&mut Transform, &CameraMarker), Without<PlayerMarker>>,
) {
    let Ok((_player_transform, mut player_physics_values)) = player_query.get_single_mut() else { return };
    let Ok(mut player_controller) = player_controller_query.get_single_mut() else { return };
    
    let Ok((_camera_transform, _)) = camera_query.get_single() else { return };
    
    // if (camera_transform.translation - player_transform.translation).length() > 50f32 { return };    

    // println!("{}", player_physics_values.velocity);
    

    // X-COMPONENT:

    if player_physics_values.velocity.x.abs() < 0.0001 { player_physics_values.velocity.x = 0f32 }
    player_physics_values.velocity.x *= 1.0 - phys_consts.friction.x;

    let mut direction: f32 = 0.0;
    
    if inputs.pressed(KeyCode::A) {
        direction = -1.0;
    }
    else if inputs.pressed(KeyCode::D) {
        direction = 1.0;
    }
    
    player_physics_values.velocity.x += direction * phys_consts.acceleration * phys_consts.player_speed* 100f32;


    // Y-COMPONENT

    if player_physics_values.velocity.y.abs() < 0.001 { player_physics_values.velocity.y = 0f32 }

    player_physics_values.velocity.y -= phys_consts.gravity;

    if let Ok(player_controller_output) = player_controller_output_query.get_single() {
        if player_controller_output.grounded {
            // println!("grounded");
            player_physics_values.velocity.y = 0.0;
            
            if inputs.just_pressed(KeyCode::Space) {
                player_physics_values.velocity.y = phys_consts.jump_boost;
                // println!("jumped");
            }
            
        }
        
        // RESET Y WHEN BUMPING INTO CEILING
        for collision in player_controller_output.collisions.iter() {
            // If the y component of the collision normal is facing downwards then weve collided with an object above us            
            if collision.toi.normal1.y == -1f32 {
                // println!("hit ceiling");
                player_physics_values.velocity.y = 0.0;
            }
        }    
        
    }
    else {
        println!("no controller output!");
    }
    // SET TRANSLATION

    player_controller.translation = Some(player_physics_values.velocity  * time.delta_seconds());
    

    // (OPTIONAL) Handle Jumping

}