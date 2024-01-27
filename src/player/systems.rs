use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::*;

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
) {
    let Ok((_, mut player_physics_values)) = player_query.get_single_mut() else { return };
    let Ok(mut player_controller) = player_controller_query.get_single_mut() else { return };
    
    // DEBUG
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

    // if player_physics_values.velocity.y.abs() < 0.001 { player_physics_values.velocity.y = 0f32 }

    player_physics_values.velocity.y -= phys_consts.gravity;

    if let Ok(player_controller_output) = player_controller_output_query.get_single() {
        
        if player_controller_output.grounded {

            // println!("grounded");
            player_physics_values.velocity.y = -0.2;

            if inputs.just_pressed(KeyCode::Space){ 
                player_physics_values.velocity.y = phys_consts.jump_boost;
                
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

    // SET TRANSLATION
    
    player_controller.translation = Some(player_physics_values.velocity  * time.delta_seconds());
    

    // (OPTIONAL) Handle Jumping

}