//! This example illustrates how to enable and disable the FPS text in a user-defined font and position
//! for a blank screen.
//!
//! Click the screen to toggle whether the diagnostic text is enabled.

use bevy::prelude::*;

use bevy_screen_diags::{ScreenDiagsPlugin, ScreenDiagsState, ScreenDiagsText};

/// Enable the plug-ins.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Include the plugin
        .add_plugins(ScreenDiagsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_handler)
        .run();
}

/// Initial set-up of the camera and the text top-left.
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 50.0,
                ..default()
            },
        ),
        ScreenDiagsText,
    ));
}

/// The mouse click handler.
fn mouse_handler(
    mouse_button_input: Res<Input<MouseButton>>,
    mut diags_state: ResMut<ScreenDiagsState>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        if diags_state.enabled() {
            diags_state.disable();
        } else {
            diags_state.enable();
        }
    }
}
