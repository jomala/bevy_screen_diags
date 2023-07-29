#![deny(missing_docs)]

//! Add a diagnostics overlay (with an FPS counter) in Bevy.
//!
//! This crate provides a Bevy [plugin](ScreenDiagsPlugin) to add the diagnostics overlay.

use std::fmt::Write;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore},
    prelude::*,
    utils::Duration,
};

const FONT_SIZE: f32 = 32.0;
const FONT_COLOR: Color = Color::RED;
const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

const STRING_FORMAT: &str = "FPS: ";
const STRING_INITIAL: &str = "FPS: ...";

/// A plugin that draws diagnostics on-screen with Bevy UI.
/// Currently only the FPS is displayed.
///
/// Use the [marker struct](ScreenDiagsText) to customise the FPS counter appearance,
/// and the [resource](ScreenDiagsState) to control its behaviour.
pub struct ScreenDiagsPlugin;

impl Plugin for ScreenDiagsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Update, update_frame_counter)
            .init_resource::<ScreenDiagsState>()
            .init_resource::<FrameCounter>();
    }
}

/// A plugin to write the FPS counter to the screen
///
/// Use the [marker struct](ScreenDiagsText) to customise the FPS counter appearance,
/// and the [resource](ScreenDiagsState) to control its behaviour.
pub struct ScreenDiagsTextPlugin;

impl Plugin for ScreenDiagsTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ScreenDiagsPlugin)
            .add_systems(Startup, spawn_text)
            .add_systems(Update, update_text);
    }
}

/// The diagnostics state resource.
///
/// To disable the FPS counter, get a [ResMut](bevy::prelude::ResMut) reference to this struct and
/// pause the timer. Unpause the timer to re-enable the counter.
#[derive(Resource)]
pub struct ScreenDiagsState {
    /// The timer that triggers a diagnostics reading.
    /// Public, to allow flexible use, but in general use the methods to interact.
    pub timer: Timer,
    /// A flag to indicate to update the display, even if the timer has not popped.
    /// Public, to allow flexible use, but in general use the methods to interact.
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
    /// Enable the FPS display.
    pub fn enable(&mut self) {
        self.timer.unpause();
        self.update_now = true;
    }

    /// Disable the FPS display.
    pub fn disable(&mut self) {
        self.timer.pause();
        self.update_now = true;
    }

    /// Is the FPS display enabled.
    pub fn enabled(&self) -> bool {
        !self.timer.paused()
    }
}

/// Resource to get the current FPS.
#[derive(Resource, Default)]
pub struct FrameCounter(pub f64);

// Updates the frame_counter
fn update_frame_counter(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    state_resource: Option<ResMut<ScreenDiagsState>>,
    mut frame_counter: ResMut<FrameCounter>,
) {
    if let Some(mut state) = state_resource {
        if state.update_now || state.timer.tick(time.delta()).just_finished() {
            if state.timer.paused() {
                return;
            } else {
                let fps_diags = extract_fps(&diagnostics);

                if let Some(fps) = fps_diags {
                    frame_counter.0 = fps;
                } else {
                    frame_counter.0 = 0.0;
                }
            }
        }
    }
}

/// The marker on the text to be updated.
#[derive(Component)]
pub struct ScreenDiagsText;

fn update_text(
    time: Res<Time>,
    state_resource: Option<ResMut<ScreenDiagsState>>,
    mut text_query: Query<&mut Text, With<ScreenDiagsText>>,
    frame_counter: Res<FrameCounter>,
) {
    if let Some(mut state) = state_resource {
        if state.update_now || state.timer.tick(time.delta()).just_finished() {
            if state.timer.paused() {
                // Time is paused so remove text
                for mut text in text_query.iter_mut() {
                    let value = &mut text.sections[0].value;
                    value.clear();
                }
            } else {
                for mut text in text_query.iter_mut() {
                    let value = &mut text.sections[0].value;
                    value.clear();

                    write!(value, "{}{:.0}", STRING_FORMAT, frame_counter.0).unwrap();
                }
            }
        }
    }
}

// Get the current fps
fn extract_fps(diagnostics: &DiagnosticsStore) -> Option<f64> {
    diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/screen-diags-font.ttf");
    commands
        .spawn(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: STRING_INITIAL.to_string(),
                    style: TextStyle {
                        font,
                        font_size: FONT_SIZE,
                        color: FONT_COLOR,
                    },
                }],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScreenDiagsText);
}
