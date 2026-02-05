#![deny(missing_docs)]

//! Add a diagnostics overlay in Bevy.
//!
//! This crate provides a Bevy plugin, [ScreenDiagsPlugin] to add a resource, [ScreenDiagsState], containing the information
//! diagnostic, and the more comprehensive [ScreenDiagsTextPlugin] to create and update the diagnostics text overlay.
//!
//! Currently the only diagnostic show is FPS (frames per second).

use std::time::Duration;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

/// Font size used by [ScreenDiagsTextPlugin].
const FONT_SIZE: f32 = 32.0;
/// Font color used by [ScreenDiagsTextPlugin].
const FONT_COLOR: Color = Color::Srgba(Srgba::RED);

/// The update interval used.
const UPDATE_INTERVAL: Duration = Duration::from_secs(1);
/// The prefix of the string to display the FPS.
const STRING_FORMAT: &str = "FPS: ";
/// The string used when the FPS is unavailable.
const STRING_INITIAL: &str = "FPS: ...";

/// A plugin that collect diagnostics and updates any `Text` marked as [ScreenDiagsText].
/// Currently only the FPS is displayed.
///
/// Use the [resource](ScreenDiagsState) to control its behaviour.
pub struct ScreenDiagsPlugin;

impl Plugin for ScreenDiagsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Update, update_frame_rate)
            .init_resource::<ScreenDiagsState>()
            .init_resource::<FrameRate>()
            .add_systems(Update, update_text);
    }
}

/// A plugin that draws diagnostics on-screen with Bevy UI.
/// Currently only the FPS is displayed.
///
/// This plugin builds on [ScreenDiagsPlugin] and adds a default [ScreenDiagsText] to display
/// the diagnostics using the font defined in `assets/fonts/screen-diags-font.ttf`,
/// [FONT_SIZE] and [FONT_COLOR].
pub struct ScreenDiagsTextPlugin;

impl Plugin for ScreenDiagsTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ScreenDiagsPlugin)
            .add_systems(Startup, spawn_text);
    }
}

/// The diagnostics state resource.
///
/// To disable the FPS rate, get a [ResMut](bevy::prelude::ResMut) reference to this struct and
/// pause the timer. Unpause the timer to re-enable the rate.
#[derive(Resource)] // Must use 'static lifetime
pub struct ScreenDiagsState {
    /// The timer that triggers a diagnostics reading.
    ///
    /// Disabling the timer disables the collection of the diagnostics and stops the display.
    ///
    /// This is public, to allow flexible use, but in general you should use the methods
    /// [enable] and [disable] to interact with it.
    pub timer: Timer,
    /// A flag to indicate to update the display, even if the timer has not popped.
    ///
    /// This is public, to allow flexible use, but in general you should use the methods
    /// [enable] and [disable] to interact with it.
    pub update_now: bool,
}

impl Default for ScreenDiagsState {
    fn default() -> Self {
        Self {
            timer: Timer::new(UPDATE_INTERVAL, TimerMode::Repeating),
            update_now: true,
        }
    }
}

impl ScreenDiagsState {
    /// Enable the FPS collection and display.
    pub fn enable(&mut self) {
        self.timer.unpause();
        self.update_now = true;
    }

    /// Disable the FPS collection and display.
    pub fn disable(&mut self) {
        self.timer.pause();
        self.update_now = true;
    }

    /// Whether the FPS collection and display enabled.
    pub fn enabled(&self) -> bool {
        !self.timer.is_paused()
    }
}

/// Resource containing the FPS (frames per second) diagnostic.
#[derive(Resource, Default)]
pub struct FrameRate(pub f64);

// Updates the frame_rate measure in the resource.
fn update_frame_rate(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    state_resource: Option<ResMut<ScreenDiagsState>>,
    mut frame_rate: ResMut<FrameRate>,
) {
    if let Some(mut state) = state_resource {
        if state.update_now || state.timer.tick(time.delta()).just_finished() {
            if state.timer.is_paused() {
                return;
            } else {
                let fps_diags = extract_fps(&diagnostics);

                if let Some(fps) = fps_diags {
                    frame_rate.0 = fps;
                } else {
                    frame_rate.0 = 0.0;
                }
            }
        }
    }
}

/// The marker on the text to be updated.
#[derive(Component)]
pub struct ScreenDiagsText;

/// The Bevy system to update the text marked with [ScreenDiagsText].
fn update_text(
    time: Res<Time>,
    state_resource: Option<ResMut<ScreenDiagsState>>,
    mut text_query: Query<&mut Text, With<ScreenDiagsText>>,
    frame_rate: Res<FrameRate>,
) {
    if let Some(mut state) = state_resource {
        if state.update_now || state.timer.tick(time.delta()).just_finished() {
            if state.timer.is_paused() {
                // Time is paused so remove text
                for mut text in text_query.iter_mut() {
                    text.clear();
                }
            } else {
                for mut text in text_query.iter_mut() {
                    *text = Text::new(format!("{}{:.0}", STRING_FORMAT, frame_rate.0));
                }
            }
        }
    }
}

/// Utility function to get the current fps from the FrameTimeDiagnosticsPlugin
fn extract_fps(diagnostics: &DiagnosticsStore) -> Option<f64> {
    diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
}

/// Function to spawn the text that will be updated to the current FPS.
fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/screen-diags-font.ttf");
    commands
        .spawn((
            Text::new(STRING_INITIAL),
            TextFont {
                font,
                font_size: FONT_SIZE,
                ..Default::default()
            },
            TextColor(FONT_COLOR),
            ScreenDiagsText,
        ));
}
