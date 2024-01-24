use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::PlayerMarker;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
}

pub fn configure_player(
    mut commands: Commands,
    player_query: Query<(Entity, &PlayerMarker), Without<RigidBody>>, 
) {
    let Ok((player_entity, _)) = player_query.get_single() else { return; };
    commands.entity(player_entity)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}