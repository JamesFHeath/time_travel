use crate::*;
use bevy::app::PluginGroupBuilder;

pub mod player;
pub mod arrows;

pub struct PlayerModPluginGroup;

impl PluginGroup for PlayerModPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerPlugin)
            .add(ArrowsPlugin)
    }
}
