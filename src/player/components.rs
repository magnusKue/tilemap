use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::physics::*;

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