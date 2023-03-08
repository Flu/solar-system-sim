use bevy::prelude::*;

#[derive(Component)]
pub struct FocusableEntity {
    pub is_focused: bool
}

impl Default for FocusableEntity {
    fn default() -> Self {
        FocusableEntity { is_focused: false }
    }
}