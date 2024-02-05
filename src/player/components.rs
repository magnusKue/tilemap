use bevy::prelude::*;
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
        PlayerPhysicsValues { 
            velocity: Vec2::ZERO, 
            last_frame_grounded: false,
        }
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

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    pub marker: PlayerMarker,

    pub sprite_bundle: SpriteSheetBundle,

    pub physics: PlayerPhysicsBundle,
    pub physics_values: PlayerPhysicsValues,
    pub coyote_watch: CoyoteWatch,
    pub jump_buffer: JumpBuffer,
}