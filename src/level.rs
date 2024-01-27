use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app 
            // SYSTEMS
            .add_systems(Update, build_wall_colliders)
            .add_systems(Update, (
                change_levels,
            ))
            
            // Resources
            .insert_resource(LevelSelection::Identifier("World_Level_0".to_string()));
            // ------
    }
}

pub fn build_wall_colliders (
    mut commands: Commands,
    tiles: Query<(Entity, &TileEnumTags), Added<TileEnumTags>>,
) {
    for (entity, enum_tags) in tiles.iter() {
        if enum_tags.tags.contains(&String::from("Wall")) {
            commands.entity(entity).insert(Collider::cuboid(8.0, 8.0));
        }
    }
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