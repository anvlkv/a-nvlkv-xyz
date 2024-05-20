use leptos::*;
use leptos_use::{storage::use_local_storage, use_preferred_dark, utils::JsonCodec};

use crate::app::components::{RvArtboardView, DARK_MODE_STORAGE};

#[component]
pub fn DarkModeToggleView() -> impl IntoView {
    let dark_preference = use_preferred_dark();
    let (dark_setting, set_dark_setting, del_dark_setting) =
        use_local_storage::<Option<bool>, JsonCodec>(DARK_MODE_STORAGE);

    let dark_mode = Signal::derive(move || {
        if let Some(setting) = dark_setting.get() {
            setting
        } else {
            dark_preference.get()
        }
    });

    let set_dark_mode = Callback::new(move |dark| {
        if dark == dark_preference.get() {
            del_dark_setting();
        } else {
            set_dark_setting.set(Some(dark));
        }
    });

    let on_change = move |_| {
        let is_dark = dark_mode.get();
        set_dark_mode.call(!is_dark);
    };

    let title = Signal::derive(move || {
        let is_dark = dark_mode.get();
        let state = if is_dark {
            t!("util.off")
        } else {
            t!("util.on")
        };
        t!("util.dark", state = state).to_string()
    });

    let dark_mode_input = Signal::derive(move || Some(("IsDark".to_string(), dark_mode.get())));

    view! {
        <label class="relative cursor-pointer" attr:title=title>
            <RvArtboardView
                attr:class="w-8 h-8"
                state_machine="Sun-Moon State Machine"
                name="Sun-Moon"
                input_bool=dark_mode_input
            />
            <input
            class="hidden"
            attr:name="dark-mode"
            attr:type="checkbox"
            attr:checked=dark_mode
            on:change=on_change/>
        </label>
    }
}
