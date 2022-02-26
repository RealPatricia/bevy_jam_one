use std::f32::consts::PI;

use bevy::prelude::*;
use crate::gameplugingroup::gametypes::{characters::*, prefabs::*};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_startup_system(enemy_setup);
    }
}

fn enemy_setup(mut commands: Commands)
{
    let enemy_sprite_bundle = SpriteBundle 
    {
        sprite: Sprite
        {
            color: Color::RED,
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        ..Default::default()
    };

    let enemy_ship_prefab = ShipPrefab
    {
        sprite_bundle: enemy_sprite_bundle,
        thrust: Thrust(45.0),
        turnspeed: TurnSpeed(500.0),
        motile: MotileType::Ship,
        ..Default::default()
    };

    for i in 0..4 
    {
        let enemy = commands.spawn_bundle(enemy_ship_prefab.clone()).insert(Enemy).id();
        commands.entity(enemy).insert(Transform
        {
            translation: Vec3::new(400.0 * (PI * 0.5 * i as f32).sin(), 200.0 * (PI * 0.5 * i as f32).cos(), 0.0),
            ..Default::default()
        });
    }
}

#[allow(dead_code)]
fn enemy_velocity()
{

}

#[allow(dead_code)]
fn enemy_aim()
{

}

#[allow(dead_code)]
fn enemy_fire()
{

}