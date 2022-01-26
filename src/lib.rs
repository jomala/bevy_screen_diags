//! A simple library to provide an on-screen FPS display for Bevy projects.

use std::fmt::Write;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    utils::Duration,
};

/// The plugin
pub struct ScreenDiagsPlugin;

impl Plugin for ScreenDiagsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup)
            .add_system(update);
    }
}

/// The marker component for our FPS update interval timer.
///
/// To disable this plugin, pause the timer.
#[derive(Component)]
pub struct ScreenDiagsTimer {
    text_entity: Option<Entity>,
}

#[derive(Component)]
struct ScreenDiagsText;

fn update(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut timer_query: Query<(&mut ScreenDiagsTimer, &mut Timer)>,
    mut text_query: Query<&mut Text, With<ScreenDiagsText>>,
) {
    let (mut marker, mut timer) = timer_query.single_mut();
    if timer.paused() {
        if let Some(entity) = marker.text_entity {
            commands.entity(entity).despawn_recursive();
            marker.text_entity = None;
        }
        return;
    } else if marker.text_entity.is_none() {
        marker.text_entity = Some(spawn_text(
            &mut commands,
            asset_server,
            extract_fps(diagnostics).map(|fps| {
                let mut buffer = String::new();
                format_fps(&mut buffer, fps);
                buffer
            }),
        ));
        return;
    } else if !timer.tick(time.delta()).just_finished() {
        return;
    }

    if let Some(fps) = extract_fps(diagnostics) {
        let mut text = text_query.single_mut();
        format_fps(&mut text.sections[1].value, fps);
    }
}

fn extract_fps(diagnostics: Res<Diagnostics>) -> Option<f64> {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            return Some(average);
        }
    }
    None
}

fn format_fps(s: &mut String, fps: f64) {
    s.clear();
    // SAFETY: Writing to a String never fails
    unsafe {
        write!(s, "{:.2}", fps).unwrap_unchecked();
    }
}

/// Set up the UI camera, the text element and, attached to it, the plugin state.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let entity = spawn_text(&mut commands, asset_server, None);
    commands.spawn_bundle((
        ScreenDiagsTimer {
            text_entity: Some(entity),
        },
        Timer::new(Duration::from_secs(1), true),
    ));
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
                            font_size: 32.0,
                            color: Color::RED,
                        },
                    },
                    TextSection {
                        value: fps.unwrap_or_else(|| "...".to_string()),
                        style: TextStyle {
                            font: handle,
                            font_size: 32.0,
                            color: Color::RED,
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
