use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerLaser;

#[derive(Component)]
pub struct EnemyLaser;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Thrust(pub f32);

#[derive(Component)]
pub struct TurnSpeed(pub f32);

#[derive(Component)]
pub struct Velocity(pub f32, pub f32);