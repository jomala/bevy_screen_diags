//! This example illustrates how to enable the FPS text in the upper left hand corner
//! for a blank screen.

use bevy::prelude::*;
use bevy_screen_diags::ScreenDiagsPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // Include the plugin
        .add_plugin(ScreenDiagsPlugin::default())
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    // Add further cameras to test that they interoperate with the one in the plugin.
    commands.spawn_bundle(PerspectiveCameraBundle::default());
    commands.spawn_bundle(UiCameraBundle::default());
}
