use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;


#[derive(Debug, Bundle)]
pub struct PlayerPhysicsBundle {
    pub collider: Collider,
    pub controller: KinematicCharacterController,
    pub gravity_scale: GravityScale,
    pub rot_constraints: LockedAxes,
}

impl Default for PlayerPhysicsBundle {
    fn default() -> PlayerPhysicsBundle {
        PlayerPhysicsBundle {
            collider: Collider::cuboid(15.0, 15.0),
            controller: KinematicCharacterController::default(),
            gravity_scale: GravityScale(2f32),
            rot_constraints: LockedAxes::ROTATION_LOCKED,
        }
    }
}


#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ObjectPhysicsBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}


impl From<&EntityInstance> for ObjectPhysicsBundle {
    fn from(entity_instance: &EntityInstance) -> ObjectPhysicsBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "MyEntityIdentifier" => ObjectPhysicsBundle {
                collider: Collider::cuboid(5., 5.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                gravity_scale: GravityScale(1.0),
                ..Default::default()
            },
            _ => ObjectPhysicsBundle::default(),
        }
    }
}



pub fn build_wall_colliders (
    mut commands: Commands,
    tiles: Query<(Entity, &TileEnumTags), Added<TileEnumTags>>,
) {
    for (entity, enum_tags) in tiles.iter() {
        println!("contains enum");
        if enum_tags.tags.contains(&String::from("Wall")) {
            commands.entity(entity).insert(Collider::cuboid(8.0, 8.0));
        }
    }
}   