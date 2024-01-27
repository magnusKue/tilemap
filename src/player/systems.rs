use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::*;
use crate::player::components::CoyoteWatch;

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
    mut player_query: Query<(&Transform, &mut PlayerPhysicsValues, &mut CoyoteWatch), With<PlayerMarker>>,
    mut player_controller_query: Query<&mut KinematicCharacterController>,
    player_controller_output_query: Query<&KinematicCharacterControllerOutput>,
) {
    let Ok((_, mut player_phys_vals, mut coy_timer)) = player_query.get_single_mut() else { return };
    let Ok(mut player_controller) = player_controller_query.get_single_mut() else { return };
    
    // DEBUG
    // println!("{}", coy_timer.timer.elapsed().as_secs_f32());
    
    // X-COMPONENT:

    if player_phys_vals.velocity.x.abs() < 0.0001 { player_phys_vals.velocity.x = 0f32 }
    player_phys_vals.velocity.x *= 1.0 - phys_consts.friction.x;

    let mut direction: f32 = 0.0;
    
    if inputs.pressed(KeyCode::A) {
        direction = -1.0;
    }
    else if inputs.pressed(KeyCode::D) {
        direction = 1.0;
    }
    
    if let Ok(player_controller_output) = player_controller_output_query.get_single() {
        for collision in player_controller_output.collisions.iter() {
            if collision.toi.normal1.y == 0.0  {
                // println!("hit ceiling");
                player_phys_vals.velocity.x = 0.0; //collision.toi.normal1.x * 70.0;
                direction = 0.0;
                // println!("hit wall");
            }
        }    
    }

    player_phys_vals.velocity.x += direction * phys_consts.acceleration * phys_consts.player_speed* 100f32;






    // Y-COMPONENT

    // if player_physics_values.velocity.y.abs() < 0.001 { player_physics_values.velocity.y = 0f32 }

    player_phys_vals.velocity.y -= phys_consts.gravity;

    if let Ok(player_controller_output) = player_controller_output_query.get_single() {
        
        if player_controller_output.grounded || coy_timer.timer.elapsed_secs() < phys_consts.coyote_time{

            // println!("grounded");
            if player_controller_output.grounded {
                player_phys_vals.velocity.y = -0.2;
            }

            if inputs.just_pressed(KeyCode::Space){ 
                player_phys_vals.velocity.y = phys_consts.jump_boost;
                coy_timer.timer.tick(Duration::from_secs_f32(200.0f32));
            }
        }
        
        // RESET COYOTE TIMER WHEN leaving grounded state
        if player_phys_vals.last_frame_grounded != player_controller_output.grounded && !player_controller_output.grounded {
            coy_timer.timer.reset();
        }
        
        // RESET VELOCITY WHEN BUMPING INTO SOMETHING
        for collision in player_controller_output.collisions.iter() {
            // If the y component of the collision normal is facing downwards then weve collided with an object above us            
            if collision.toi.normal1.y == -1f32 {
                // println!("hit ceiling");
                player_phys_vals.velocity.y = 0.0;
            }
        }    
        

        player_phys_vals.last_frame_grounded = player_controller_output.grounded;
    }

    if inputs.just_released(KeyCode::Space) && player_phys_vals.velocity.y > 0f32{
        player_phys_vals.velocity.y *= 0.5;
    }

    // MAKE FALLING FASTER THEN RISING 
    let mut applied_velocity = player_phys_vals.velocity;
    if applied_velocity.y < 0.0 { applied_velocity.y *= phys_consts.falling_gravity_scaler };

    // SET TRANSLATION
    
    player_controller.translation = Some(applied_velocity  * time.delta_seconds());
    

    // (OPTIONAL) Handle Jumping

}

pub fn tick_coyote_timer(
    mut player_query: Query<&mut CoyoteWatch, With<PlayerMarker>>,
    time: Res<Time>,
) {
    for mut player_timer in player_query.iter_mut() {
        player_timer.timer.tick(time.delta());
    }
}