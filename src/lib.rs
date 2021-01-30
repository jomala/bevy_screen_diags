use bevy::{
    utils::{Duration, Instant},
    prelude::*,
};

/// The plugin
#[derive(Debug, Default, Clone)]
pub struct ScreenDiagsPlugin {
    settings: ScreenDiagsSettings,
}

impl Plugin for ScreenDiagsPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
            .add_resource(self.settings.clone())
            .add_startup_system(setup.system())
            .add_system(update.system());
        }
}

/// The settings
#[derive(Debug, Clone)]
pub struct ScreenDiagsSettings {
    interval: Duration,
    enabled: bool,
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

fn update(settings: Res<ScreenDiagsSettings>, time: Res<Time>, mut query: Query<(&mut Text, &mut ScreenDiagsText)>) {
    for (mut text, mut marker) in query.iter_mut() {
        if let Some(state) = marker.state.as_mut() {
            state.frame_count += 1;

            let so_far = Instant::now() - state.last_time;
            if so_far > settings.interval {
                if settings.enabled {
                    let fps = state.frame_count as f64 / so_far.as_secs_f64();
                    text.value = format!("FPS: {:4.0}", fps);
                } else {
                    text.value = "".to_owned();
                }

                marker.state = None;
            }
        }
        if marker.state.is_none() {
            marker.state = Some(ScreenDiagsState {
                last_time: Instant::now(),
                frame_count: 0,
            });
        }
    }
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS: ...".to_owned(),
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            },
            ..Default::default()
        })
        .with(ScreenDiagsText::default());
}
