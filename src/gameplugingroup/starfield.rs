use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::gameplugingroup::gametypes::{utilities::*};

pub struct StarFieldPlugin;

impl Plugin for StarFieldPlugin
{
    fn build(&self, app: &mut App)
    {
        app
        .insert_resource(ArenaSize 
        {
            width: 20_000.0,
            height: 20_000.0
        })
        .add_startup_system(asteroid_field_setup);
    }
}

fn asteroid_field_setup(
    mut commands: Commands,
    arena_size: Res<ArenaSize>,
    asset_server: Res<AssetServer>
)
{
    let mut layers: [Vec<SpriteBundle>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for i in 0..layers.len()
    {
        for _ in 0..250
        {
            let mut rng = thread_rng();
            let dist = rng.gen_range(300.0..(arena_size.width));
            let mut dir = Vec3::ZERO;
            while dir == Vec3::ZERO
            {
                dir = Vec3::from([rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0]);
            }
            dir = dir.normalize();

            let asteroid_transform = Transform
            {
                translation: dir * dist,
                ..Default::default()
            };
            let asteroid_bundle = SpriteBundle
            {
                transform: asteroid_transform,
                texture: asset_server.load("sprites/meteor_large.png"),
                ..Default::default()
            };
            layers[i].push(asteroid_bundle);
        }
    }

    commands.spawn_batch(layers[0].clone());
    commands.spawn_batch(layers[1].clone());
    commands.spawn_batch(layers[2].clone());
}
