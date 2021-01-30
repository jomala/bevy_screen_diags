Adds a very simple frames-per-second (FPS) display to your screen in Bevy.

*Inspired by the Bevy example `ui/text.rs`.*

Put the crate into your `Cargo.toml`.
```
bevy_screen_diags = "*"
```

Include the plugin when you build your `App`.
```
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_screen_diags::ScreenDiagsPlugin::default())
        :
        :
}
```

![Example screen showing the FPS](docs/fps.png)

There are configuration options but you probably don't want them.

Contributions to the crate are welcome.
