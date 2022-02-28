//good luck

#![feature(derive_default_enum)]
use bevy::{prelude::*};

mod gameplugingroup;
use gameplugingroup::*;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugins)
        .run();
}