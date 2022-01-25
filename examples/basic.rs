//! This example illustrates how to enable the FPS text in the upper left hand corner
//! for a blank screen.

use bevy::prelude::*;

use bevy_screen_diags::ScreenDiagsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Include the plugin
        .add_plugin(ScreenDiagsPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Add further cameras to test that they interoperate with the one in the plugin.
    commands.spawn_bundle(PerspectiveCameraBundle::default());
    commands.spawn_bundle(UiCameraBundle::default());
}
