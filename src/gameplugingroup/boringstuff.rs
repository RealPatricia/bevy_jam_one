use bevy::prelude::*;
use crate::gameplugingroup::gametypes::{characters::*, utilities::*};
use std::{f32::consts::PI};

pub struct BoringPlugin;

impl Plugin for BoringPlugin
{
    fn build(&self, app: &mut App)
    {
        app.insert_resource(ClearColor(Color::rgb(0.01, 0.01, 0.01)))
            .insert_resource(WindowDescriptor
            {
                title: "Untitled Game".to_string(),
                width: 1600.0,
                height: 900.0,
                ..Default::default()
            })
            .add_startup_system(camera_setup)
            .add_system(camera_follow_player)
            .add_system(movement)
            .add_system(aim)
            .add_system(drag);
    }
}

fn camera_setup(mut commands: Commands)
{
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(PlayerCameraTag);
    commands.spawn_bundle(UiCameraBundle::default()).insert(UiCameraTag);
}

fn camera_follow_player(
    mut q_set: QuerySet<(
        QueryState<&mut Transform, With<PlayerCameraTag>>, 
        QueryState<&Transform, With<Player>>)>,
        time: Res<Time>
    )
{
    let mut player_pos: Vec2 = Vec2::ZERO;
    let mut cam_pos: Vec2 = Vec2::ZERO;

    if let Ok(player_trans) = q_set.q1().get_single()
    {
        player_pos = Vec2::new( player_trans.translation.x, player_trans.translation.y);
    }

    if let Ok(mut cam_trans) = q_set.q0().get_single_mut()
    {
        cam_pos = Vec2::new(cam_trans.translation.x, cam_trans.translation.y);
        let diff = player_pos - cam_pos;
        let mag = (diff * 0.1).length_squared();
        let adjustment = Vec3::new(diff.x, diff.y, 0.0) * time.delta_seconds() * mag; 

        if diff.length() > 0.01
        {
            cam_trans.translation += adjustment;
        }
        else
        {
            cam_trans.translation = Vec3::new(player_pos.x, player_pos.y, cam_trans.translation.z);
        }
    }
}

fn movement(
    time: Res<Time>,
    mut motile_q: Query<(&mut Transform, &Velocity)>
)
{
    for (mut transform, velocity) in motile_q.iter_mut()
    {
        transform.translation += Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
    }
}

fn drag(
    mut fast_q: Query<(&mut Velocity, &MotileType)>
)
{
    for (mut velocity, motile) in fast_q.iter_mut()
    {
        if *motile == MotileType::Ship
        {
            let force = 0.0001 * (velocity.0 * 0.01).length().powf(2.0).abs();
            velocity.0 *= 1.0 - (force.clamp(0.0, 1.0))
        }
    }
}

fn aim(
    time: Res<Time>,
    mut shooter_q: Query<(&mut Transform, &Target, &TurnSpeed), Or<(With<Player>, With<Enemy>)>>
)
{
    for (mut transform, target, turnspeed) in shooter_q.iter_mut()
    {
        if let Some(target_exists) = (*target).0
        {
            let pos = transform.translation.truncate();
            let angle = (target_exists - pos).angle_between(transform.local_y().truncate());
            if angle > 0.05 || angle < -0.05
            {
                transform.rotation *= Quat::from_rotation_z(-1.0 * angle * time.delta_seconds() * turnspeed.0 * PI / 180.0)
            }
        }
    }
}