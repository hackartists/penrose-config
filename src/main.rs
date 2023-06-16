mod hackartist_config;

use hackartist_config::HackartistConfig;
use penrose::{
    core::{bindings::parse_keybindings_with_xmodmap, Config, WindowManager},
    extensions::hooks::{add_ewmh_hooks, SpawnOnStartup},
    x11rb::RustConn,
    Result,
};

use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("trace")
        .finish()
        .init();

    let conf = HackartistConfig {};
    let config = add_ewmh_hooks(Config {
        default_layouts: conf.layouts(),
        startup_hook: Some(SpawnOnStartup::boxed("polybar")),
        ..Config::default()
    });

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(conf.key_bindings())?;
    let wm = WindowManager::new(config, key_bindings, HashMap::new(), conn)?;

    wm.run()
}
