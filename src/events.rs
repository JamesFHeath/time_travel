use bevy::prelude::*;

use crate::player::player::FacingDirection;

pub struct InteractionEvent {
    pdi_translation: Vec3,
    facing_direction: FacingDirection,
}

impl InteractionEvent {
    pub fn new(pdi_translation: Vec3, facing_direction: FacingDirection) -> Self {
        Self {
            pdi_translation,
            facing_direction,
        }
    }

    pub fn pdi_translation(&self) -> Vec3 {
        self.pdi_translation
    }

    pub fn facing_direction(&self) -> FacingDirection {
        self.facing_direction
    }
}

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractionEvent>();
    }
}
