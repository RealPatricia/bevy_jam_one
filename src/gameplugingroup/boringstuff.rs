use bevy::{prelude::*, sprite::collide_aabb::collide};
use crate::gameplugingroup::gametypes::{characters::*, utilities::*, events::*};
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
            .add_event::<LaserFireEvent>()
            .add_startup_system(camera_setup)
            .add_system(camera_follow_player)
            .add_system(movement)
            .add_system(aim)
            .add_system(drag)
            .add_system(fire_laser)
            .add_system(self_destruct)
            .add_system(death)
            .add_system(collision);
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

    if let Ok(player_trans) = q_set.q1().get_single()
    {
        player_pos = Vec2::new( player_trans.translation.x, player_trans.translation.y);
    }

    if let Ok(mut cam_trans) = q_set.q0().get_single_mut()
    {
        let cam_pos = Vec2::new(cam_trans.translation.x, cam_trans.translation.y);
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
            let force = 0.0001 * (velocity.0 * 0.01).length().powf(2.8).abs();
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

fn fire_laser(
    mut commands: Commands,
    mut ev_laser: EventReader<LaserFireEvent>
)
{
    for laser in ev_laser.iter()
    {
        let laser_transform = laser.0;
        let laser_velocity = laser.1;
        let laser_type = laser.2;
        let laser_sprite_bundle = SpriteBundle 
        {
            sprite: Sprite
            {
                color: Color::rgb(0.9, 0.9, 1.5),
                custom_size: Some(Vec2::new(5.0, 50.0)),
                ..Default::default()
            },
            ..Default::default()
        };

        commands
            .spawn_bundle(laser_sprite_bundle)
            .insert(laser_transform)
            .insert(laser_velocity)
            .insert(SelfDestructTimer(Timer::from_seconds(3.0, false)))
            .insert(Health::Finite(2))
            .insert(laser_type);
    }
}

fn self_destruct(
    time: Res<Time>,
    mut commands: Commands,
    mut sdq: Query<(Entity, &mut SelfDestructTimer)>
)
{
    for (entity, mut sdt) in sdq.iter_mut()
    {
        if sdt.0.tick(time.delta()).just_finished()
        {
            commands.entity(entity).despawn();
        }
    }
}

fn collision(
    mut health_q: Query<(&mut Health, &Transform, &Sprite)>
)
{
    let mut iter = health_q.iter_combinations_mut();
    while let Some([(mut health1, transform1, sprite1), (mut health2, transform2, sprite2)]) =
        iter.fetch_next()
    {
        let scale1 = Vec2::from(transform1.scale.truncate());
        let scale2 = Vec2::from(transform2.scale.truncate());

        let collision = collide(
            transform1.translation, 
            sprite1.custom_size.unwrap() * scale1 * 0.6, 
            transform2.translation, 
            sprite2.custom_size.unwrap() * scale2 * 0.6);

        if let Some(_) = collision
        {
            match *health1
            {
                Health::Finite(x) =>
                {
                    *health1 = Health::Finite(x - 1);
                }

                _ => {}
            }
            match *health2
            {
                Health::Finite(x) =>
                {
                    *health2 = Health::Finite(x - 1);
                }

                _ => {}
            }
        }
    }
}

fn death(
    mut commands: Commands,
    health_q: Query<(Entity, &Health)>
)
{
    for (entity, health) in health_q.iter()
    {
        match health
        {
            Health::Finite(x) =>
            {
                if *x <= 0
                {
                    commands.entity(entity).despawn();
                }
            }

            _ => {}
        }
    }
}