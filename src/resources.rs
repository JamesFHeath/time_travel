use bevy::prelude::*;

#[derive(Resource)]
pub struct CurrentEra {
    pub current_era: Entity,
}

#[derive(Resource)]
pub struct KeyBindings {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub interact: KeyCode,
    pub fire: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            interact: KeyCode::I,
            fire: KeyCode::F,
        }
    }
}
