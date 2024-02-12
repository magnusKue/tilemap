use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{player::components::*, Enemy, EnemyBundle};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                handle_player_collision,
            ))
            .register_ldtk_entity::<EnemyBundle>("Enemy");
        }
}

fn handle_player_collision(
    rapier_context: Res<RapierContext>,
    query_player: Query<Entity, With<PlayerMarker>>,
    query_enemy: Query<Entity, With<Enemy>>,
    mut commands: Commands
) {
    for entity_enemy in query_enemy.iter() {
        for entity_player in query_player.iter() {
            if rapier_context.intersection_pair(entity_enemy, entity_player) == Some(true) {
                commands.entity(entity_player).despawn();
                println!("collision");
            }
        }
    }
}
