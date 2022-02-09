# Bevy diagnostics overlay

[![Crates.io](https://img.shields.io/crates/v/bevy_screen_diags)](https://crates.io/crates/bevy_screen_diags)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

`bevy_screen_diags` adds a very simple frames-per-second (FPS) display to your screen in Bevy.

## Usage

Put the crate into your `Cargo.toml`.

```toml
bevy_screen_diags = "*"
```

Include the plugin when you build your `App`.

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_screen_diags::ScreenDiagsPlugin)
        //If a UI camera is already in your game remove the next line
        .add_startup_system(|mut commands: Commands| {commands.spawn_bundle(UiCameraBundle::default());})
        :
        :
}
```

Put the font you want to use in `assets/fonts/screen-diags-font.ttf`. If you want, you can use
the font at that path in this project (which is FiraSans-Bold).

![Example screen showing the FPS](docs/fps.png)

The `basic` example just shows the FPS count on a grey background, but you can click your mouse on
the window to add or remove the display.

Contributions to the crate are welcome.
