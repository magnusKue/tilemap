use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

// PLUGIN

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            // RAPIER
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin{enabled:true, ..default()})
            // ------

            // Resources
            .insert_resource(RapierConfiguration::default());
            // ------
    }
}


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
            collider: Collider::cuboid(14.0, 14.0),
            controller: KinematicCharacterController {
                snap_to_ground: Some(CharacterLength::Relative(0.1)),
                max_slope_climb_angle: 0.8, // ~47° in radians
                offset: CharacterLength::Absolute(0.1),
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Absolute(2.0),
                    min_width: CharacterLength::Relative(1.0),
                    ..Default::default()
                }),
                up: Vec2::new(0.0, 1.0),
                ..default()
            },
            gravity_scale: GravityScale(0f32),
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