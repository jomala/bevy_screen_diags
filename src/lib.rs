#![deny(missing_docs)]

//! Add a diagnostics overlay (with an FPS counter) in Bevy.
//!
//! This crate provides a Bevy [plugin](ScreenDiagsPlugin) to add the diagnostics overlay.

use std::fmt::Write;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    utils::Duration,
};

const FONT_SIZE: f32 = 32.0;
const FONT_COLOR: Color = Color::RED;
const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

/// A plugin that draws diagnostics on-screen with Bevy UI.
///
/// Use our [marker struct](ScreenDiagsTimer) to manage the FPS counter.
pub struct ScreenDiagsPlugin;

impl Plugin for ScreenDiagsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_system(update)
            .init_resource::<ScreenDiagsState>();
    }
}

/// The diagnostics state resource.
///
/// To disable the FPS counter, get a [ResMut](bevy::prelude::ResMut) reference to this struct and
/// pause the timer. Unpause the timer to re-enable the counter.
pub struct ScreenDiagsState {
    /// The timer that triggers a diagnostics reading.
    pub timer: Timer,
    text_entity: Option<Entity>,
    /// Ignore the timer until we get an FPS reading (which will be the second frame after
    /// diagnostics are initialized).
    fps_initialized: bool,
}

impl Default for ScreenDiagsState {
    fn default() -> Self {
        Self {
            timer: Timer::new(UPDATE_INTERVAL, true),
            text_entity: None,
            fps_initialized: false,
        }
    }
}

#[derive(Component)]
struct ScreenDiagsText;

fn update(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    asset_server: Res<AssetServer>,
    state: Option<ResMut<ScreenDiagsState>>,
    mut commands: Commands,
    mut text_query: Query<&mut Text, With<ScreenDiagsText>>,
) {
    let mut state = match state {
        Some(s) => s,
        None => return,
    };

    if state.timer.paused() {
        if let Some(entity) = state.text_entity {
            commands.entity(entity).despawn_recursive();
            state.text_entity = None;
        }
        return;
    } else if state.text_entity.is_none() {
        state.text_entity = Some(spawn_text(
            &mut commands,
            asset_server,
            extract_fps(diagnostics).map(|fps| {
                state.fps_initialized = true;

                let mut buffer = String::new();
                format_fps(&mut buffer, fps);
                buffer
            }),
        ));
        return;
    } else if !state.timer.tick(time.delta()).just_finished() && state.fps_initialized {
        return;
    }

    if let Some(fps) = extract_fps(diagnostics) {
        state.fps_initialized = true;

        let mut text = text_query.single_mut();
        format_fps(&mut text.sections[1].value, fps);
    }
}

fn extract_fps(diagnostics: Res<Diagnostics>) -> Option<f64> {
    diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
}

fn format_fps(s: &mut String, fps: f64) {
    s.clear();
    write!(s, "{:.0}", fps).unwrap();
}

fn spawn_text(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    fps: Option<String>,
) -> Entity {
    let handle = asset_server.load("fonts/screen-diags-font.ttf");
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: handle.clone(),
                            font_size: FONT_SIZE,
                            color: FONT_COLOR,
                        },
                    },
                    TextSection {
                        value: fps.unwrap_or_else(|| "...".to_string()),
                        style: TextStyle {
                            font: handle,
                            font_size: FONT_SIZE,
                            color: FONT_COLOR,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScreenDiagsText)
        .id()
}
