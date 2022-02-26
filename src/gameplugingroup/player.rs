use std::{f32::consts::PI};

use bevy::{prelude::*, math::Vec3Swizzles};
use crate::gameplugingroup::gametypes::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, mut app: &mut App)
    {
        app
            .add_startup_system(player_setup)
            .add_system(player_move);
    }
}

fn player_setup(mut commands: Commands)
{
    commands.spawn_bundle(SpriteBundle 
    {
        sprite: Sprite
        {
            color: Color::RED,
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        transform: Transform
        {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player)
    .insert(Thrust(25.0))
    .insert(TurnSpeed(250.0))
    .insert(Velocity(0.0, 0.0));
}

fn player_move(time: Res<Time>, keys: Res<Input<KeyCode>>, mut player_q: Query<(&Thrust, &TurnSpeed, &mut Transform, &mut Velocity), With<Player>>)
{
    if let Ok((thrust, turn_speed, mut transform, mut velocity)) = player_q.get_single_mut()
    {
        let rotate_dir = -1.0 * ((keys.pressed(KeyCode::D) as i32) - (keys.pressed(KeyCode::A) as i32)) as f32;
        transform.rotation *= Quat::from_rotation_z(turn_speed.0 * rotate_dir * time.delta_seconds() * PI / 180.0);

        let impulse_strength = keys.pressed(KeyCode::Space) as i32 as f32;
        let impulse_dir = transform.local_y().xy();
        velocity.0 += impulse_dir.x * impulse_strength * thrust.0;
        velocity.1 += impulse_dir.y * impulse_strength * thrust.0;

        transform.translation += Vec3::new(velocity.0, velocity.1, 0.0) * time.delta_seconds();

        if keys.just_pressed(KeyCode::Backslash)
        {
            transform.translation = Vec3::ZERO;
            (velocity.0, velocity.1) = (0.0, 0.0);
        }
    }
}

fn player_fire()
{

}