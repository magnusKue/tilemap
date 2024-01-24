use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::physics::PhysicsObjectBundle;
// COMPONENTS

#[derive(Default, Component)]
pub struct PlayerMarker { }

// BUNDLES

#[derive(Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    physics: PhysicsObjectBundle,

    #[from_entity_instance]
    entity_instance: EntityInstance,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    marker: PlayerMarker,
    
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    
    #[from_entity_instance]
    physics: PhysicsObjectBundle,
    
    #[from_entity_instance]
    entity_instance: EntityInstance,
}