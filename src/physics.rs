use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;


#[derive(Debug, Bundle)]
pub struct PlayerPhysicsBundle {
    pub collider: Collider,
    pub controller: KinematicCharacterController,
    pub rb: RigidBody,
    pub gravity_scale: GravityScale,
    pub rot_constraints: LockedAxes,
}

impl Default for PlayerPhysicsBundle {
    fn default() -> PlayerPhysicsBundle {
        PlayerPhysicsBundle {
            collider: Collider::cuboid(16.0, 16.0),
            controller: KinematicCharacterController::default(),
            rb: RigidBody::Dynamic,
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