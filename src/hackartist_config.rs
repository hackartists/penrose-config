use penrose::{
    builtin::{
        actions::{exit, log_current_state, modify_with, send_layout_message, spawn},
        layout::{
            messages::{ExpandMain, IncMain, ShrinkMain},
            transformers::{Gaps, ReflectHorizontal, ReserveTop},
            MainAndStack,
        },
    },
    core::{bindings::KeyEventHandler, layout::LayoutStack},
    map, stack,
    x11rb::RustConn,
};

use std::collections::HashMap;

pub struct HackartistConfig {}

impl HackartistConfig {
    pub fn key_bindings(&self) -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
        let mut raw_bindings = map! {
            // map_keys: |k: &str| format!("C-{k}");
            map_keys: |k: &str| k.to_owned();

            "M-j" => modify_with(|cs| cs.focus_down()),
            "M-k" => modify_with(|cs| cs.focus_up()),
            "M-S-j" => modify_with(|cs| cs.swap_down()),
            "M-S-k" => modify_with(|cs| cs.swap_up()),
            "M-S-q" => modify_with(|cs| cs.kill_focused()),
            "M-Tab" => modify_with(|cs| cs.toggle_tag()),
            "M-bracketright" => modify_with(|cs| cs.next_screen()),
            "M-bracketleft" => modify_with(|cs| cs.previous_screen()),
            "M-grave" => modify_with(|cs| cs.next_layout()),
            "M-S-grave" => modify_with(|cs| cs.previous_layout()),
            "M-Up" => send_layout_message(|| IncMain(1)),
            "M-Down" => send_layout_message(|| IncMain(-1)),
            "M-Right" => send_layout_message(|| ExpandMain),
            "M-Left" => send_layout_message(|| ShrinkMain),
            "M-semicolon" => spawn("dmenu_run"),
            "M-S-s" => log_current_state(),
            "M-Return" => spawn("st"),
            "M-A-Escape" => exit(),
        };

        for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
            raw_bindings.extend([
                (
                    format!("M-{tag}"),
                    modify_with(move |client_set| client_set.focus_tag(tag)),
                ),
                (
                    format!("M-S-{tag}"),
                    modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
                ),
            ]);
        }

        raw_bindings
    }

    pub fn layouts(&self) -> LayoutStack {
        let max_main = 1;
        let ratio = 0.6;
        let ratio_step = 0.1;
        let outer_px = 5;
        let inner_px = 5;
        let top_px = 18;

        stack!(
            MainAndStack::side(max_main, ratio, ratio_step),
            ReflectHorizontal::wrap(MainAndStack::side(max_main, ratio, ratio_step)),
            MainAndStack::bottom(max_main, ratio, ratio_step)
        )
        .map(|layout| ReserveTop::wrap(Gaps::wrap(layout, outer_px, inner_px), top_px))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use penrose::core::bindings::parse_keybindings_with_xmodmap;

    #[test]
    fn correctly_binding_keys() {
        let conf = HackartistConfig {};
        let res = parse_keybindings_with_xmodmap(conf.key_bindings());

        if let Err(e) = res {
            panic!("{e}");
        }
    }
}
