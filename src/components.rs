use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

// COMPONENTS

#[derive(Default, Component)]
pub struct CameraMarker {
    pub zoom_speed: f32,
    pub fc_move_speed: f32,
    pub fp_move_speed: f32,
    pub cam_offset: Vec3,
}


#[derive(Default, Component)]
pub struct PlayerMarker { }

// BUNDLES

#[derive(Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    marker: PlayerMarker,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}