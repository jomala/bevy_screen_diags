//! This example illustrates how to enable and disable the FPS text in the bottom left hand corner
//! for a blank screen.

use bevy::prelude::*;

use bevy_screen_diags::{ScreenDiagsState, ScreenDiagsTextPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Include the plugin
        .add_plugin(ScreenDiagsTextPlugin)
        .add_startup_system(setup)
        .add_system(mouse_handler)
        .run();
}

fn setup(mut commands: Commands) {
    // Add further cameras to test that they interoperate with the one in the plugin.
    commands.spawn(Camera2dBundle::default());
}

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
