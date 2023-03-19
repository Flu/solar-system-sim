use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub enum SystemTypes {
    CameraLabel = 0,
    PhysicsLabel
}