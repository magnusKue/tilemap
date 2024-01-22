use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
struct CameraMarker {
    zoom_speed: f32,
    move_speed: f32
}

#[derive(Default, Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )
        .add_plugins(LdtkPlugin)

        .add_systems(Startup, setup)
        .add_systems(Update, (
            move_camera,
            change_levels
            ))

        .insert_resource(LevelSelection::Index(0))

        .register_ldtk_entity::<EnemyBundle>("MyEntityIdentifier")
        .register_ldtk_entity::<EnemyBundle>("TestEntity")
        .register_ldtk_entity::<PlayerBundle>("Player")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle{
        transform: Transform::from_xyz(200.0, 0.0, 0.0),
        ..Default::default()
        },
        CameraMarker{
            zoom_speed:1.02,
            move_speed:6.0
        }
    )
    );

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("my_project.ldtk"),
        ..Default::default()
    });
}

fn change_levels(
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

fn move_camera(
    inputs: Res<Input<KeyCode>>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection, &CameraMarker)>,
) {
    let (
        mut transform, 
        mut projection, 
        marker
    )   = camera.get_single_mut().unwrap();

    if inputs.pressed(KeyCode::A) {
        transform.translation.x -= marker.move_speed * marker.zoom_speed;
    }
    else if inputs.pressed(KeyCode::D) {
        transform.translation.x += marker.move_speed * marker.zoom_speed;
    }
    
    if inputs.pressed(KeyCode::S) {
        transform.translation.y -= marker.move_speed * marker.zoom_speed; 
    }
    else if inputs.pressed(KeyCode::W) {
        transform.translation.y += marker.move_speed * marker.zoom_speed;
    }
    
    if inputs.pressed(KeyCode::Q) {
        projection.scale /= marker.zoom_speed;
    }
    else if inputs.pressed(KeyCode::E) {
        projection.scale *= marker.zoom_speed;
    }
}


#[derive(Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    marker: Player,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}