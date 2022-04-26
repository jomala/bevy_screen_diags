//! This example illustrates how to enable and disable the FPS text in the bottom left hand corner
//! for a blank screen.

use bevy::prelude::*;

use bevy_screen_diags::{ScreenDiagsPlugin, ScreenDiagsTimer};

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
    mut query: Query<&mut ScreenDiagsTimer>,
    mut timer: ResMut<Timer>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        //let mut timer = query.single_mut();
        if timer.paused() {
            timer.unpause();
        } else {
            timer.pause();
        }
    }
}
