use bevy::{prelude::*, window::PrimaryWindow};

mod file_select;
use bevy_simple_scroll_view::ScrollViewPlugin;
use file_select::FileSelectPlugin;

mod utils;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "BPCE".to_owned(),
                    mode: bevy::window::WindowMode::BorderlessFullscreen,
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ScrollViewPlugin,
            FileSelectPlugin,
        ))
        .add_systems(Startup, (
            spawn_camera,
            configure_buttons,
        ))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn configure_buttons(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window.single_mut();
    window.enabled_buttons.maximize = false;
    window.enabled_buttons.minimize = false;
    window.enabled_buttons.close = true;
}
