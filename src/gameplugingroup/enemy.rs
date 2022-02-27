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
            .add_system(enemy_fire);
    }
}

fn enemy_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
)
{
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
        thrust: Thrust(60.0),
        turnspeed: TurnSpeed(400.0),
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
fn enemy_velocity(
    mut enemy_q: Query<(&Target, &Transform, &Thrust, &mut Velocity), With<Enemy>>
)
{
    for (target, transform, thrust, mut velocity) in enemy_q.iter_mut()
    {
        if let Some(has_target) = target.0
        {
            let pos = transform.translation.truncate();
            let target_dir = (has_target - pos).normalize();
            let target_dist = (has_target - pos).length();
            let target_angle = (has_target - pos).angle_between(transform.local_y().truncate()).signum();
            
            let thrust_dir = target_dir * thrust.0;

            if target_dist > 500.0
            {
                velocity.0 += thrust_dir;
            }
            else if target_dist < 250.0
            {
                velocity.0 -= thrust_dir;
            }
            else
            {
                velocity.0 += transform.local_x().truncate() * target_angle * thrust.0 * -2.0;
            }
        }
    }
}

#[allow(dead_code)]
fn enemy_target(
    mut q_set: QuerySet<(
        QueryState<&Transform, With<Player>>,
        QueryState<(&Transform, &mut Target), With<Enemy>>
    )>
)
{
    let mut my_target: Option<Vec2> = None;

    if let Ok(player_trans) = q_set.q0().get_single()
    {
        my_target = Some(player_trans.translation.truncate());
    }

    for (transform, mut target) in q_set.q1().iter_mut()
    {
        if let Some(player_location) = my_target
        {
            let my_location = transform.translation.truncate();
            let dist = (my_location - player_location).length();

            if dist < 500.0
            {
               *target = Target(Some(player_location)); 
            }
            else if dist > 4000.0 
            {
                *target = Target(None);
            }
        }
    }

}

#[allow(dead_code)]
fn enemy_fire(
    enemy_q: Query<(&Transform, &Velocity, &Target), With<Enemy>>,
    mut ev_laser: EventWriter<LaserFireEvent>,
    time: Res<Time>
)
{
    for (transform, velocity, target) in enemy_q.iter()
    {
        if let Some(target_position) = (*target).0
        {
            let pos = transform.translation.truncate();
            let target_angle = (target_position - pos).angle_between(transform.local_y().truncate());


            if target_angle < 0.1
            {
                let laser_velocity = Velocity(transform.local_y().truncate() * 2000.0 + velocity.0 * 2.0);
                let mut laser_transform = (*transform).clone();
                laser_transform.translation += laser_transform.local_y() * 20.0 + Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
                
                ev_laser.send(LaserFireEvent(laser_transform, laser_velocity));
            }
        }
    }
}