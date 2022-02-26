use bevy::prelude::*;

pub struct BoringPlugin;

impl Plugin for BoringPlugin
{
    fn build(&self, mut app: &mut App)
    {
        app.insert_resource(ClearColor(Color::rgb(0.01, 0.01, 0.01)))
            .insert_resource(WindowDescriptor
            {
                title: "Untitled Game".to_string(),
                width: 1600.0,
                height: 900.0,
                ..Default::default()
            })
            .add_startup_system(camera_setup);
    }
}

fn camera_setup(mut commands: Commands)
{
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}