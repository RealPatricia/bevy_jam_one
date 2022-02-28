
use bevy::{prelude::*};
use crate::gameplugingroup::gametypes::{characters::*, utilities::*, prefabs::*, events::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_startup_system(player_setup)
            .add_system(player_velocity)
            .add_system(player_target)
            .add_system(player_fire);
    }
}

fn player_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
)
{
    let player_sprite_bundle = SpriteBundle 
    {
        sprite: Sprite
        {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        transform: Transform
        {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        texture: asset_server.load("sprites/ship_L.png"),
        ..Default::default()
    };

    let player_ship_prefab = ShipPrefab
    {
        sprite_bundle: player_sprite_bundle,
        thrust: Thrust(15.0),
        turnspeed: TurnSpeed(600.0),
        motile: MotileType::Ship,
        health: Health::Infinite,
        ..Default::default()
    };

    commands.spawn_bundle(player_ship_prefab)
        .insert(Player)
        .insert(FireTimer(Timer::from_seconds(0.1, true)));
}

fn player_velocity(
    keys: Res<Input<KeyCode>>, 
    mut player_q: Query<(&Thrust, &Transform, &mut Velocity), With<Player>>
)
{
    if let Ok((thrust, transform, mut velocity)) = player_q.get_single_mut()
    { 

        let impulse_strength = (keys.pressed(KeyCode::W) as i32 - keys.pressed(KeyCode::S) as i32)as f32;
        let strafe_strength = ((keys.pressed(KeyCode::A) as i32) - (keys.pressed(KeyCode::D) as i32)) as f32;
        let impulse_dir = transform.up().truncate();
        let strafe_dir = transform.left().truncate();
        let mut thrust_direction = impulse_dir * impulse_strength + strafe_dir * strafe_strength;
        if thrust_direction != Vec2::ZERO
        {
            thrust_direction = thrust_direction.normalize();
        }

        velocity.0 += thrust_direction * thrust.0;

    }
}

fn player_target(
    windows: Res<Windows>, 
    camera_q: Query<(&Camera, &GlobalTransform), With<PlayerCameraTag>>, 
    mut player_q: Query<&mut Target, With<Player>>
)
{
    
        let (camera, camera_transform) = camera_q.single();
        let mut target = player_q.single_mut();

        let window = windows.get(camera.window).unwrap();

        if let Some(mouse_screen_pos) = window.cursor_position()
        {
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
            let ndc = (mouse_screen_pos / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
            let mouse_world_position = ndc_to_world.project_point3(ndc.extend(-1.0));
            let mouse_world_position: Vec2 = mouse_world_position.truncate();

            *target = Target(Some(mouse_world_position));
        }
}

fn player_fire(
    time: Res<Time>,
    mut player_q: Query<(&Transform, &Velocity, &mut FireTimer), With<Player>>,
    keys: Res<Input<MouseButton>>,
    mut ev_laser: EventWriter<LaserFireEvent>
)
{
    if keys.pressed(MouseButton::Left)
    {
        if let Ok((transform, velocity, mut fire_timer)) = player_q.get_single_mut()
        {
            if fire_timer.0.tick(time.delta()).just_finished()
            {
                let laser_velocity = Velocity(transform.local_y().truncate() * 2000.0 + velocity.0);
                let mut laser_transform = (*transform).clone();
                laser_transform.translation += laser_transform.local_y() * 20.0 + Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
                ev_laser.send(LaserFireEvent(laser_transform, laser_velocity, LaserType::PlayerLaser));
            }
        }
    }
} 