use leptos::*;
use leptos_use::{storage::use_local_storage, use_preferred_dark, utils::JsonCodec};

use crate::app::components::DARK_MODE_STORAGE;

#[cfg(any(feature = "csr", feature = "hydrate"))]
mod rv_animation {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/app/components/dark_mode_toggle.mjs")]
    extern "C" {

        #[wasm_bindgen(js_name=darkModeAnimation)]
        pub fn dark_mode_animation(is_dark_mode_enabled: bool);

        #[wasm_bindgen(js_name=cleanUp)]
        pub fn clean_up();

        #[wasm_bindgen(js_name=setDark)]
        pub fn set_dark_mode(is_dark_mode_enabled: bool);
    }
}

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

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        create_effect(move |mount| {
            let is_dark = dark_mode.get();
            if mount.is_some() {
                rv_animation::set_dark_mode(is_dark);
            } else {
                rv_animation::dark_mode_animation(is_dark);
            }
        });

        on_cleanup(move || {
            rv_animation::clean_up();
        });
    }

    view! {
        <label class="relative cursor-pointer" attr:title=title>
            <canvas id="dark-mode_animation" class="w-8 h-8"/>
            <input
            class="hidden"
            attr:name="dark-mode"
            attr:type="checkbox"
            attr:checked=dark_mode
            on:change=on_change/>
            // <span>
            //     {on.children}
            // </span>
            // <span
            //     attr:title=title
            //     class="absolute"
            // />
            // <span>
            //     {off.children}
            // </span>
        </label>
    }
}

// name="dark-theme"
// title={t!("util.dark").to_string()}
// checked=dark_mode
// on_change=set_dark_mode
// >
// <ToggleOnLabel slot:on>
//     {"☽"}
// </ToggleOnLabel>
// <ToggleOffLabel slot:off>
//     {"☀︎"}
// </ToggleOffLabel>
// </ToggleView>
