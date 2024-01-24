use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::physics::ObjectPhysicsBundle;
// COMPONENTS

// BUNDLES

#[derive(Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    physics: ObjectPhysicsBundle,

    #[from_entity_instance]
    entity_instance: EntityInstance,
}

