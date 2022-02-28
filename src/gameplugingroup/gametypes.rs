
pub mod characters
{
    use bevy::prelude::*;

    #[derive(Component, PartialEq, Eq, Default, Clone)]
    pub enum MotileType
    {
        Ship,
        #[default]
        Laser,
    }
    
    #[derive(Component, Default, Clone)]
    pub struct Laser;
    
    #[derive(Component, Default, Clone, Copy)]
    pub enum LaserType
    {
        #[default]
        EnemyLaser,
        PlayerLaser,
    }

    #[derive(Component, Default, Clone)]
    pub struct Player;

    #[derive(Component, Default, Clone)]
    pub struct Enemy;
    
    #[derive(Component, Default, Clone)]
    pub struct Thrust(pub f32);
    
    #[derive(Component, Default, Clone)]
    pub struct TurnSpeed(pub f32);
    
    #[derive(Component, Default, Clone, Copy)]
    pub struct Velocity(pub Vec2);
    
    #[derive(Component, Default, Clone)]
    pub struct Target(pub Option<Vec2>);

    #[derive(Component, Default, Clone)]
    pub enum Health
    {
        #[default]
        Infinite,
        Finite(i32),
    }

    #[derive(Component)]
    pub struct SelfDestructTimer(pub Timer);
    
    #[derive(Component)]
    pub struct FireTimer(pub Timer);
}

pub mod utilities
{
    use bevy::prelude::Component;

    #[derive(Component)]
    pub struct PlayerCameraTag;

    #[derive(Component)]
    pub struct UiCameraTag;
}

pub mod prefabs
{
    use bevy::{prelude::Bundle, sprite::SpriteBundle};

    use super::characters::*;

    #[derive(Bundle, Default, Clone)]
    pub struct ShipPrefab
    {
        #[bundle]
        pub sprite_bundle: SpriteBundle,
        pub thrust: Thrust,
        pub turnspeed: TurnSpeed,
        pub velocity: Velocity,
        pub motile: MotileType,
        pub target: Target,
        pub health: Health        
    }
}

pub mod events
{
    use bevy::prelude::Transform;

    use super::characters::{Velocity, LaserType};

    pub struct LaserFireEvent(pub Transform, pub Velocity, pub LaserType);
}