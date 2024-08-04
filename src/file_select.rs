use bevy::prelude::*;

mod components;
use components::*;

pub struct FileSelectPlugin;

impl Plugin for FileSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_menu);
    }
}

fn spawn_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            FileSelectMenu,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Hello, world!",
                        TextStyle {
                            font: asset_server.load("fonts/Saira-Regular.ttf"),
                            font_size: 64.0,
                            color: Color::WHITE,
                        },
                    )],
                    justify: JustifyText::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
