//! This example illustrates how to enable the FPS text in the upper left hand corner
//! for a blank screen.

use bevy::prelude::*;
use bevy_screen_diags::ScreenDiagsPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ScreenDiagsPlugin::default())
        .add_startup_system(setup.system())
        .run();
}

fn setup(commands: &mut Commands) {
    commands
        .spawn(CameraUiBundle::default());
}
