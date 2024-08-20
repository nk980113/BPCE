use bevy::prelude::*;

use crate::into_const;

pub const ROOT_STYLE: Style = into_const!(Style {
    width: Val::Percent(100.0),
    height: Val::Vh(100.0),
    padding: UiRect::all(Val::Px(20.0)),
    row_gap: Val::Px(20.0),
    ..Style::DEFAULT
});

pub const LEFT_CONTAINER: Style = into_const!(Style {
    width: Val::Percent(40.0),
    height: Val::Percent(100.0),
    margin: into_const!(UiRect {
        right: Val::Px(20.0),
        ..UiRect::DEFAULT
    }),
    flex_direction: FlexDirection::Column,
    ..Style::DEFAULT
});

pub const ADD_BUTTON: Style = into_const!(Style {
    height: Val::Vh(15.0),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    margin: into_const!(UiRect {
        bottom: Val::Px(20.0),
        ..UiRect::DEFAULT
    }),
    ..Style::DEFAULT
});

pub const LEFT_TEXT_BOX_STYLE: Style = into_const!(Style {
    height: Val::Vh(10.0),
    ..Style::DEFAULT
});

pub const SONG_SCROLL_VIEW_STYLE: Style = into_const!(Style {
    border: UiRect::all(Val::Px(5.0)),
    padding: UiRect::all(Val::Px(20.0)),
    height: Val::Vh(75.0),
    ..Style::DEFAULT
});

pub const SONG_SCROLLABLE_STYLE: Style = into_const!(Style {
    flex_direction: FlexDirection::Column,
    width: Val::Percent(100.0),
    ..Style::DEFAULT
});

pub const SINGLE_SONG_CONTAINER: Style = into_const!(Style {
    border: UiRect::all(Val::Px(10.0)),
    height: Val::Px(160.0),
    padding: UiRect::all(Val::Px(10.0)),
    width: Val::Percent(100.0),
    flex_direction: FlexDirection::Column,
    overflow: Overflow::clip_x(),
    margin: into_const!(UiRect {
        bottom: Val::Px(20.0),
        ..UiRect::DEFAULT
    }),
    ..Style::DEFAULT
});

pub fn styled_container(style: Style) -> NodeBundle {
    NodeBundle {
        style,
        ..Default::default()
    }
}

pub fn saira_size(font_size: f32, asset_server: &AssetServer) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Saira-Regular.ttf"),
        font_size,
        color: Color::WHITE,
    }
}
