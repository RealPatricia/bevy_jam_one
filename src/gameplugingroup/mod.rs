use bevy::{prelude::*, app::PluginGroupBuilder};

mod gametypes;
mod boringstuff;
mod player;
mod enemy;
mod starfield;

use boringstuff::*;
use player::*;
use enemy::*;
use starfield::*;

pub struct GamePlugins;

impl PluginGroup for GamePlugins
{
    fn build(&mut self, group: &mut PluginGroupBuilder)
    {
        group
            .add(BoringPlugin)
            .add(PlayerPlugin)
            .add(EnemyPlugin)
            .add(StarFieldPlugin);
    }
}