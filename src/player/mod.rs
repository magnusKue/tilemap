use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::camera::CameraState;

pub mod systems;
pub mod components;
pub mod ressources;
pub mod spawnpoint;

use systems::*;
use components::*;
use ressources::*;
use spawnpoint::*;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, add_controller_output.before(move_player))
            .add_systems(Update, (
                move_player.run_if(in_state(CameraState::FollowPlayer)),
                tick_timers,
                animate_player_sprite,
                spawnpoint::respawn_player
            ))
            .register_ldtk_entity::<PlayerSpawnPoin>("PlayerSpawn")
            
            .init_resource::<PlayerPhysicsConstants>()
            .register_type::<PlayerPhysicsConstants>()
            
            .init_resource::<PlayerAnimation>()
            .register_type::<PlayerAnimation>();
    }
}