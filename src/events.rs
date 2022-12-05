use bevy::prelude::*;

pub struct InteractionEvent {
    player_translation: Vec3,
}

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractionEvent>();
    }
}