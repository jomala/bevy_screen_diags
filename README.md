Adds a very simple frames-per-second (FPS) display to your screen in Bevy.

*Inspired by the Bevy example `ui/text.rs`.*

Put the crate into your `Cargo.toml`.
```
bevy_screen_diags = "*"
```

Include the plugin when you build your `App`.
```
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_screen_diags::ScreenDiagsPlugin)
        :
        :
}
```

Put the font you want to use in `assets/fonts/screen-diags-font.ttf`. If you want, you can use
the font at that path in this project (which is FiraSans-Bold).

![Example screen showing the FPS](docs/fps.png)

There are configuration options but you probably don't want them.

The `basic` example just shows the FPS count on a grey background, but you can click your mouse on
the window to add or remove the display.

Contributions to the crate are welcome.
