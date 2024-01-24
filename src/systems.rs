use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::components::*;


pub fn move_player(
    inputs: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<PlayerMarker>>,
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else { return };
    
    let player_speed: f32 = 1.0;

    let mut direction: Vec3 = Vec3::ZERO;

    if inputs.pressed(KeyCode::A) {
        direction.x = -1.0;
    }
    else if inputs.pressed(KeyCode::D) {
        direction.x = 1.0;
    }
    
    if inputs.pressed(KeyCode::S) {
        direction.y = -1.0;
    }
    else if inputs.pressed(KeyCode::W) {
        direction.y = 1.0;
    }

    player_transform.translation += direction.normalize_or_zero() * player_speed;
}


pub fn change_levels(
    inputs: Res<Input<KeyCode>>,
    mut commands: Commands
) {
    if inputs.just_pressed(KeyCode::R) {
        commands.insert_resource(LevelSelection::Index(1));
    }
    else if inputs.just_pressed(KeyCode::T) {
        commands.insert_resource(LevelSelection::Index(0));
    }
}