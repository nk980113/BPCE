use bevy::prelude::*;
use bevy_simple_scroll_view::{ScrollView, ScrollableContent};

mod components;
use components::*;

mod styles;
use styles::*;

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
            styled_container(ROOT_STYLE),
            FileSelectMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(styled_container(LEFT_CONTAINER))
                .with_children(|left| {
                    left
                        .spawn(ButtonBundle {
                            style: ADD_BUTTON,
                            background_color: BackgroundColor(Color::linear_rgb(0.0, 0.3, 0.0)),
                            border_radius: BorderRadius::all(Val::Px(10.0)),
                            ..Default::default()
                        })
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section("Add Chart...", saira_size(60.0, &asset_server)));
                        });
                    left
                        .spawn(styled_container(LEFT_TEXT_BOX_STYLE))
                        .with_children(|container| {
                            container
                                .spawn(TextBundle::from_section("Recent Charts", saira_size(80.0, &asset_server)));
                        });
                    left
                        .spawn((
                            NodeBundle {
                                style: SONG_SCROLL_VIEW_STYLE,
                                border_color: BorderColor(Color::WHITE),
                                border_radius: BorderRadius::all(Val::Px(10.0)),
                                ..Default::default()
                            },
                            ScrollView::default(),
                        ))
                        .with_children(|area| {
                            area
                                .spawn((
                                    styled_container(SONG_SCROLLABLE_STYLE),
                                    ScrollableContent::default(),
                                ))
                                .with_children(|list| {
                                    // TODO: load real songs here
                                    for i in 1..=20 {
                                        list
                                            .spawn(NodeBundle {
                                                style: SINGLE_SONG_CONTAINER,
                                                border_color: BorderColor(Color::WHITE),
                                                border_radius: BorderRadius::all(Val::Px(15.0)),
                                                ..Default::default()
                                            })
                                            .with_children(|song| {
                                                song
                                                    .spawn(NodeBundle {
                                                        style: Style {
                                                            width: Val::Vw(100.0),
                                                            overflow: Overflow::clip_x(),
                                                            ..Default::default()
                                                        },
                                                        ..Default::default()
                                                    })
                                                    .with_children(|title| {
                                                        title.spawn(TextBundle::from_section(format!("Song {i}"), saira_size(75.0, &asset_server)));
                                                    });
                                                song.spawn(TextBundle::from_section("SP Lv.?", saira_size(35.0, &asset_server)));
                                            });
                                    }
                                });
                        });
                });
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(60.0),
                    ..Default::default()
                },
                border_radius: BorderRadius::all(Val::Px(10.0)),
                background_color: BackgroundColor(Color::linear_rgb(0.0, 1.0, 0.0)),
                ..Default::default()
            });
        });
}
