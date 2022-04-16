//! This example illustrates how to enable and disable the FPS text in the bottom left hand corner
//! for a blank screen.

use bevy::prelude::*;

use bevy_screen_diags::{ScreenDiagsPlugin, ScreenDiagsState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Include the plugin
        .add_plugin(ScreenDiagsPlugin)
        .add_startup_system(setup)
        .add_system(mouse_handler)
        .run();
}

fn setup(mut commands: Commands) {
    // Add further cameras to test that they interoperate with the one in the plugin.
    commands.spawn_bundle(PerspectiveCameraBundle::default());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn mouse_handler(
    mouse_button_input: Res<Input<MouseButton>>,
    mut diags_state: ResMut<ScreenDiagsState>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        if diags_state.timer.paused() {
            diags_state.timer.unpause();
        } else {
            diags_state.timer.pause();
        }
    }
}
