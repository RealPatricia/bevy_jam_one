use bevy::prelude::*;
use crate::gameplugingroup::gametypes::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin
{
    fn build(&self, mut app: &mut App)
    {
        app
            .add_startup_system(enemy_setup);
    }
}

fn enemy_setup(mut commands: Commands)
{
    commands.spawn_bundle(SpriteBundle 
        {
            sprite: Sprite
            {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            transform: Transform
            {
                translation: Vec3::new(400.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });
        
        commands.spawn_bundle(SpriteBundle 
        {
            sprite: Sprite
            {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            transform: Transform
            {
                translation: Vec3::new(0.0, 200.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });

        commands.spawn_bundle(SpriteBundle 
        {
            sprite: Sprite
            {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            transform: Transform
            {
                translation: Vec3::new(0.0, -200.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });

        commands.spawn_bundle(SpriteBundle 
        {
            sprite: Sprite
            {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            transform: Transform
            {
                translation: Vec3::new(-400.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn enemy_move()
{

}

fn enemy_fire()
{

}