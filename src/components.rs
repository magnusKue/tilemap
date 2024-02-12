use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
// use bevy_rapier2d::prelude::*;

use crate::physics::ObjectPhysicsBundle;

// --- COMPONENTS

#[derive(Component, Default)]
pub struct Cheese;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Component, Default)]
pub struct DamageTrigger;

// --- BUNDLES

// ENEMY
#[derive(Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    physics: ObjectPhysicsBundle,

    #[from_entity_instance]
    entity_instance: EntityInstance,

    enemy_marker: Enemy,
    // sensor: Sensor,
}


// TARGET
#[derive(Default, Bundle, LdtkEntity)]
pub struct CheeseBundle {
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    entity_instance: EntityInstance,

    cheese: Cheese,
}