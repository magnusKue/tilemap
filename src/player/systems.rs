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
    mut animator: ResMut<PlayerAnimation>,
    phys_consts: Res<PlayerPhysicsConstants>,
    mut player_query: Query<(&Transform, &mut PlayerPhysicsValues, &mut CoyoteWatch, &JumpBuffer), With<PlayerMarker>>,
    mut player_controller_query: Query<&mut KinematicCharacterController>,
    player_controller_output_query: Query<&KinematicCharacterControllerOutput>,
) {
    let Ok((_, mut player_phys_vals, mut coy_timer, jump_buffer)) = player_query.get_single_mut() else { return };
    let Ok(mut player_controller) = player_controller_query.get_single_mut() else { return };
    
    // DEBUG
    // println!("{}", player_phys_vals.velocity);
    
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
        
        let coyote_active: bool =  coy_timer.timer.elapsed_secs() < phys_consts.coyote_time;
        let grounded: bool = player_controller_output.grounded;
        let input: bool = inputs.just_pressed(KeyCode::Space);
        let input_buffer: bool = jump_buffer.timer.elapsed_secs() < phys_consts.jump_inp_buffering;

        
        if grounded {
            // slightly push player down to keep detecting ground collision
            player_phys_vals.velocity.y = -0.2;
        }

        if (grounded || coyote_active) && (input || input_buffer)  {
            // JUMP
            player_phys_vals.velocity.y = phys_consts.jump_boost;

            coy_timer.timer.tick(Duration::from_secs_f32(200.0f32));
        }


        // RESET COYOTE TIMER WHEN LEAVING GROUNDED STATE
        if player_phys_vals.last_frame_grounded != player_controller_output.grounded && !player_controller_output.grounded  && player_phys_vals.velocity.y < 30.{
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


    // FALLING SPEED 
    applied_velocity.y = applied_velocity.y.clamp(-phys_consts.player_max_falling_speed, phys_consts.player_max_falling_speed);
    

    // ADJUST ANIMATIONS BASED ON VELOCITY
    if animator.change_animations {
        if applied_velocity.y < -150.0 {
            animator.animation = PlayerAnimationStates::Falling;
        }
        else if applied_velocity.y > 3.0 {
            animator.animation = PlayerAnimationStates::Jumping;
        }
        else if applied_velocity.x.abs() > 65.0 {
            animator.animation = PlayerAnimationStates::Running;
        }
        else {
            animator.animation = PlayerAnimationStates::Idle;
        }

        animator.facing_left = applied_velocity.x < 0.0;
    }

    // SET TRANSLATION
    player_controller.translation = Some(applied_velocity  * time.delta_seconds());
    

    // (OPTIONAL) Handle Jumping

}

pub fn tick_timers(
    mut player_query: Query<(&mut CoyoteWatch, &mut JumpBuffer), With<PlayerMarker>>,
    time: Res<Time>,
    inputs: Res<Input<KeyCode>>,
) {
    for (mut coyote_timer, mut jump_buffer_timer) in player_query.iter_mut() {
        coyote_timer.timer.tick(time.delta());
        jump_buffer_timer.timer.tick(time.delta());
        // println!("{}", jump_buffer_timer.timer.elapsed_secs());
        
        if inputs.just_pressed(KeyCode::Space) {
            jump_buffer_timer.timer.reset();
        }
    }

}

pub fn animate_player_sprite(
    animation: Res<PlayerAnimation>,
    time: Res<Time>,
    mut atlas_handle_query: Query<&mut TextureAtlasSprite, With<PlayerMarker>>,
) {
    if let Ok(mut atlas_handle) = atlas_handle_query.get_single_mut() {
        if animation.animate {
            atlas_handle.index = match animation.animation  {
                PlayerAnimationStates::Idle => 0,
                PlayerAnimationStates::Running => (((time.elapsed_seconds() * animation.speed) %  2.0) + 2.0) as usize , // swich 2, 3
                PlayerAnimationStates::Jumping => 4,
                PlayerAnimationStates::Falling => 5,
                _ => 0,
            };

            atlas_handle.flip_x = animation.facing_left;
        }
    }
}