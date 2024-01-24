use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct PhysicsObjectBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl From<&EntityInstance> for PhysicsObjectBundle {
    fn from(entity_instance: &EntityInstance) -> PhysicsObjectBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => PhysicsObjectBundle {
                collider: Collider::cuboid(6., 14.),
                rigid_body: RigidBody::KinematicPositionBased,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                gravity_scale: GravityScale(0.0),
                rotation_constraints,
                ..Default::default()
            },
            "MyEntityIdentifier" => PhysicsObjectBundle {
                collider: Collider::cuboid(5., 5.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                gravity_scale: GravityScale(1.0),
                ..Default::default()
            },
            _ => PhysicsObjectBundle::default(),
        }
    }
}