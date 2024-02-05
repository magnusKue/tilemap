use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::PlayerBundle;

use super::PlayerMarker;

#[allow(dead_code)]
#[derive(Default, Component, LdtkEntity)]
pub struct PlayerSpawnPoin {
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    entity_instance: EntityInstance,
}

pub fn respawn_player(
    spawnpoint_query: Query<&Transform, With<PlayerSpawnPoin>>,
    mut commands: Commands,
    player_query: Query<&PlayerMarker, With<Transform>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let Ok(spawnpoint_pos) = spawnpoint_query.get_single() else { return };
    
    if player_query.get_single().is_ok() {
        return;
    }
    else{
        println!("player spawned");

        let texture_handle = asset_server.load("atlas/player.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 7, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands.spawn((
            PlayerBundle {
                sprite_bundle: SpriteSheetBundle {
                    transform: Transform::from_translation(spawnpoint_pos.translation),
                    texture_atlas: texture_atlas_handle,
                    ..default()
                },
                ..default()
            },
            Name::new("Player".to_string()),
        ));
    }
}