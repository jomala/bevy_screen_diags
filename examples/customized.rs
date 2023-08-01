//! This example illustrates how to customize the font or position of the FPS display.

use bevy::prelude::*;

use bevy_screen_diags::{ScreenDiagsText, ScreenDiagsTextPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Include the plugin
        .add_plugins(ScreenDiagsTextPlugin)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, (tweak_fps_font, tweak_fps_position))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn tweak_fps_font(mut text_query: Query<&mut Text, With<ScreenDiagsText>>) {
    let mut text = text_query.single_mut();
    text.sections[0].style.color = Color::GREEN;
    text.sections[0].style.font_size = 92.0;
}

fn tweak_fps_position(mut style_query: Query<&mut Style, With<ScreenDiagsText>>) {
    let mut style = style_query.single_mut();
    style.position_type = PositionType::Absolute;
    style.right = Val::Percent(0.0);
    style.bottom = Val::Percent(0.0);
}
