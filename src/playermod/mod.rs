pub mod player;
pub mod skills;

use crate::*;
use bevy::app::PluginGroupBuilder;

pub struct PlayerModPluginGroup;

impl PluginGroup for PlayerModPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerPlugin)
            .add(SkillPlugin)
    }
}
