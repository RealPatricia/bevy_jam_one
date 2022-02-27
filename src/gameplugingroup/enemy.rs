use std::f32::consts::PI;

use bevy::{prelude::*};
use crate::gameplugingroup::gametypes::{characters::*, prefabs::*};

use super::gametypes::events::LaserFireEvent;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_startup_system(enemy_setup)
            .add_system(enemy_target)
            .add_system(enemy_velocity)
            .add_system(enemy_fire)
            .add_system(enemy_collision_avoidance);
    }
}

fn enemy_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
)
{
    let ship_num = 4;
    let enemy_sprite_bundle = SpriteBundle 
    {
        sprite: Sprite
        {
            color: Color::RED,
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        texture: asset_server.load("sprites/ship_sidesC.png"),
        ..Default::default()
    };

    let enemy_ship_prefab = ShipPrefab
    {
        sprite_bundle: enemy_sprite_bundle,
        thrust: Thrust(50.0),
        turnspeed: TurnSpeed(300.0),
        motile: MotileType::Ship,
        health: Health::Finite(50),
        ..Default::default()
    };

    for i in 0..ship_num
    {
        commands
            .spawn_bundle(enemy_ship_prefab.clone())
            .insert(Enemy)
            .insert(FireTimer(Timer::from_seconds(0.25, true)))
            .insert(Transform
            {
                translation: Vec3::new(200.0 * (PI * 0.2 * i as f32).sin(), 200.0 * (PI * 0.2 * i as f32).cos(), 0.0),
                ..Default::default()
            });
    }
}

#[allow(dead_code)]
fn enemy_velocity(
    mut enemy_q: Query<(&Target, &Transform, &Thrust, &mut Velocity), With<Enemy>>,
    player_q: Query<&Transform, With<Player>>
)
{
    if let Ok(player_transform) = player_q.get_single()
    {
        let player_location = player_transform.translation.truncate();
        for (target, transform, thrust, mut velocity) in enemy_q.iter_mut()
        {
            if let Some(target_exists) = target.0
            {
                let my_location = transform.translation.truncate();

                let approach_dist = 400.0;
                let retreat_dist = 200.0;
                let center_dist = (approach_dist + retreat_dist) / 2.0;
                let adjustment = (approach_dist - retreat_dist) / 2.0;

                let dist_to_player = (player_location - my_location).length();
                let player_dist_from_center = dist_to_player - center_dist;
                let sign = player_dist_from_center.signum();
                let behavior = (player_dist_from_center.abs() - adjustment).clamp(0.0, 1000.0)/1000.0;
                let thrust_behavior = sign * behavior;

                let angle = (target_exists - transform.translation.truncate()).angle_between(transform.local_y().truncate());
                let thrust_normalize = (0.75 * PI - angle.abs()) / PI;
                velocity.0 += transform.local_y().truncate() * thrust.0 * thrust_normalize * thrust_behavior * 1.5;
            }
        }
    }
}

#[allow(dead_code)]
fn enemy_target(
    time: Res<Time>,
    mut q_set: QuerySet<(
        QueryState<(&Transform, &Velocity), With<Player>>,
        QueryState<(&Transform, &Velocity, &mut Target), With<Enemy>>
    )>
)
{
    let mut my_target: Option<Vec2> = None;

    if let Ok((player_trans, _)) = q_set.q0().get_single()
    {
        my_target = Some(player_trans.translation.truncate());
    }

    for (_transform, enemy_velocity, mut target) in q_set.q1().iter_mut()
    {
        if let Some(player_location) = my_target
        {
           *target = Target(Some(player_location - enemy_velocity.0 * time.delta_seconds())); 
        }
    }

}

#[allow(dead_code)]
fn enemy_fire(
    mut enemy_q: Query<(&Transform, &Velocity, &Target, &mut FireTimer), With<Enemy>>,
    mut ev_laser: EventWriter<LaserFireEvent>,
    time: Res<Time>
)
{
    for (transform, velocity, target, mut fire_timer) in enemy_q.iter_mut()
    {
        if fire_timer.0.tick(time.delta()).just_finished()
        {
            if let Some(target_position) = (*target).0
            {
                let pos = transform.translation.truncate();
                let target_angle = (target_position - pos).angle_between(transform.local_y().truncate()).abs();


                if target_angle < PI * 0.25
                {
                    let laser_velocity = Velocity(transform.local_y().truncate() * 2000.0 + velocity.0 * 2.0);
                    let mut laser_transform = (*transform).clone();
                    laser_transform.translation += laser_transform.local_y() * 20.0 + Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
                    
                    ev_laser.send(LaserFireEvent(laser_transform, laser_velocity, LaserType::EnemyLaser));
                }
            }
        }
    }

}

fn enemy_collision_avoidance(
    mut enemy_q: Query<(&Transform, &mut Velocity, &Enemy)>
)
{
    let mut iter = enemy_q.iter_combinations_mut();
    while let Some([(transform1, mut velocity1, _), (transform2, mut velocity2, _)]) = iter.fetch_next()
    {
        let delta = transform1.translation.truncate() - transform2.translation.truncate();
        let dist = delta.length_squared();

        let impulse = delta.normalize() * 1.0 / dist;

        velocity1.0 -= impulse;
        velocity2.0 += impulse;
    }
}