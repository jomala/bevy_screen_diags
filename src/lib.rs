//! A simple library to provide an on-screen FPS display for Bevy projects.

use bevy::{
    prelude::*,
    utils::{Duration, Instant},
};

/// The plugin
#[derive(Debug, Default, Clone)]
pub struct ScreenDiagsPlugin {
    /// The configurable settings for the plugin.
    pub settings: ScreenDiagsSettings,
}

impl Plugin for ScreenDiagsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(self.settings.clone())
            .add_startup_system(setup.system())
            .add_system(update.system());
    }
}

/// The settings
#[derive(Debug, Clone)]
pub struct ScreenDiagsSettings {
    /// The interval between screen updates. A balance between being responsive
    /// and easy to read. Defaults to 1 second.
    pub interval: Duration,
    /// Whether the FPS display is enabled.  Any change in status
    /// will be responded to at the end of the `interval`. Defaults to true.
    pub enabled: bool,
}

impl Default for ScreenDiagsSettings {
    fn default() -> Self {
        ScreenDiagsSettings {
            interval: Duration::from_secs(1),
            enabled: true,
        }
    }
}

/// The marker for the text to be updated, and the container for the state
#[derive(Debug, Default, Clone)]
struct ScreenDiagsText {
    state: Option<ScreenDiagsState>,
}

/// The state to be updated
#[derive(Debug, Clone, Copy)]
struct ScreenDiagsState {
    last_time: Instant,
    frame_count: u32,
}

/// Update the FPS state and, if sufficient time has passed, change the display.
fn update(
    settings: Res<ScreenDiagsSettings>,
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut ScreenDiagsText)>,
) {
    let now: Instant = time.last_update().unwrap_or_else(|| time.startup());
    for (mut text, mut marker) in query.iter_mut() {
        if let Some(state) = marker.state.as_mut() {
            state.frame_count += 1;

            let so_far = now - state.last_time;
            if so_far > settings.interval {
                if settings.enabled {
                    let fps = state.frame_count as f64 / so_far.as_secs_f64();
                    text.sections[0].value = format!("FPS: {:4.0}", fps);
                } else {
                    text.sections[0].value = "".to_owned();
                }

                marker.state = None;
            }
        }
        if marker.state.is_none() {
            marker.state = Some(ScreenDiagsState {
                last_time: now,
                frame_count: 0,
            });
        }
    }
}

/// Set up the UI camera, the text element and, attached to it, the plugin state.
fn setup(mut commands: Commands, mut assets: ResMut<Assets<Font>>) {
    // The font file to use is included in this crate so you don't need to access the file at runtime.
    // Here we load it as an asset.
    let font_bytes = include_bytes!("../assets/fonts/FiraSans-Bold.ttf").to_vec();
    let font_struct = Font::try_from_bytes(font_bytes).expect("Font should be present and valid");
    let font = assets.add(font_struct);

    // The UI camera is required to show the text. It can coexist with other cameras.
    commands.spawn_bundle(UiCameraBundle::default());
    // The text is not currently configurable, but could be.
    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "FPS: ...",
                TextStyle {
                    font,
                    font_size: 32.0,
                    color: Color::WHITE,
                },
                TextAlignment::default()
            ),
            ..Default::default()
        })
        // The state is not set up initially. This is to avoid the start-up time being counted as the first frame.
        .insert(ScreenDiagsText::default());
}
