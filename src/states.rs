use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum CameraState {
    #[default]
    FreeCam,
    FollowPlayer,
}