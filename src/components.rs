use bevy::prelude::*;

use crate::playermod::player::FacingDirection;

#[derive(Component)]
pub struct Collidable();

#[derive(Component)]
pub struct Interactable();

#[derive(Component)]
pub struct InteractedWith();

#[derive(Component)]
pub struct Projectile {
    pub facing_direction: FacingDirection,
}

impl Projectile {
    pub fn new(facing_direction: FacingDirection) -> Self {
        Self { facing_direction }
    }

    pub fn facing_direction(&self) -> FacingDirection {
        self.facing_direction
    }
}
