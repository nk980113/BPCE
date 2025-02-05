use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "BPCE".to_owned(),
                    mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        ))
        .add_systems(Startup, (
            spawn_camera,
            configure_buttons,
        ))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn configure_buttons(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window.single_mut();
    window.enabled_buttons.maximize = false;
    window.enabled_buttons.minimize = false;
    window.enabled_buttons.close = true;
}