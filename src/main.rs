use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
struct CameraMarker {
    zoom_speed: f32,
    move_speed: f32
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()), // prevents blurry sprites
        )
        .add_plugins(LdtkPlugin)

        .add_systems(Startup, setup)
        .add_systems(Update, move_camera)

        .insert_resource(LevelSelection::Index(0))

        .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .register_ldtk_entity::<MyBundle>("TestEntity")
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
        transform.translation.x -= marker.move_speed;
    }
    else if inputs.pressed(KeyCode::D) {
        transform.translation.x += marker.move_speed;
    }
    
    if inputs.pressed(KeyCode::S) {
        transform.translation.y -= marker.move_speed;
    }
    else if inputs.pressed(KeyCode::W) {
        transform.translation.y += marker.move_speed;
    }
    
    if inputs.pressed(KeyCode::Q) {
        projection.scale /= marker.zoom_speed;
    }
    else if inputs.pressed(KeyCode::E) {
        projection.scale *= marker.zoom_speed;
    }
}

#[derive(Default, Component)]
struct ComponentA;

#[derive(Default, Component)]
struct ComponentB;

#[derive(Default, Bundle, LdtkEntity)]
pub struct MyBundle {
    a: ComponentA,
    b: ComponentB,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
}