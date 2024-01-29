use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app 
            // SYSTEMS
            .add_systems(Startup, spawn_level)
            .add_systems(Update, (
                change_levels,
                build_wall_colliders
            ))
            
            // Resources
            .insert_resource(LevelSelection::Identifier("World_Level_0".to_string()));
            // ------
    }
}

pub fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("platformer.ldtk"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert(Name::new("TileMap".to_string()));
}

pub fn build_wall_colliders (
    mut commands: Commands,
    tiles: Query<(Entity, &TileEnumTags), Added<TileEnumTags>>,
) {
    for (entity, enum_tags) in tiles.iter() {
        // half tile size for calculations:
        let ts = 0.5 * Vec2::new(16.0, 16.0);
        if enum_tags.tags.contains(&String::from("Wall")) {
            commands.entity(entity).insert(Collider::cuboid(ts.x, ts.y));
        }
        else if enum_tags.tags.contains(&String::from("FullSlope")) {
            commands.entity(entity).insert(Collider::triangle(Vec2::new(-ts.x, -ts.y -0.5), Vec2::new(ts.x, -ts.y - 0.5), Vec2::new(ts.x, ts.y)));
        }
        else if enum_tags.tags.contains(&String::from("FullSlopeInv")) {
            commands.entity(entity).insert(Collider::triangle(Vec2::new(-ts.x, -ts.y -0.5), Vec2::new(ts.x, -ts.y - 0.5), Vec2::new(-ts.x, ts.y)));
        }
        else if enum_tags.tags.contains(&String::from("Circle")) {
            commands.entity(entity).insert(Collider::cuboid(ts.x, ts.y));
        }
        // CIRCLES DONT GIVE NICE COLLISIONS. DONT USE THEM!!
        // else if enum_tags.tags.contains(&String::from("Circle")) {
        //     commands.entity(entity).insert(Collider::ball(ts.x));
        // }
        else if enum_tags.tags.contains(&String::from("Platform")) {
            commands.entity(entity)
                .with_children(|children| {
                    children.spawn(Collider::cuboid(ts.x, ts.y*0.4))
                        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
                });
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