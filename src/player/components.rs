use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy::time::Stopwatch;

use crate::physics::*;

#[derive(Default, Component)]
pub struct PlayerMarker { }



#[derive(Component)]
pub struct PlayerPhysicsValues { 
    pub velocity: Vec2,
    pub last_frame_grounded: bool,
}

impl Default for PlayerPhysicsValues {
    fn default() -> PlayerPhysicsValues {
        PlayerPhysicsValues { velocity: Vec2::ZERO, last_frame_grounded: false }
    }
}



#[derive(Component)]
pub struct CoyoteWatch {
    pub timer: Stopwatch,
}

impl Default for CoyoteWatch {
    fn default() -> CoyoteWatch {
        CoyoteWatch { timer: Stopwatch::new() }
    }
}

/// Input buffer for Jumping input
#[derive(Component)]
pub struct JumpBuffer {
    pub timer: Stopwatch,
}

impl Default for JumpBuffer {
    fn default() -> JumpBuffer {
        JumpBuffer { timer: Stopwatch::new() }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    marker: PlayerMarker,

    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    
    physics: PlayerPhysicsBundle,
    physics_values: PlayerPhysicsValues,
    coyote_watch: CoyoteWatch,
    jump_buffer: JumpBuffer,
}