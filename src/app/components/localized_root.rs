use std::str::FromStr;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::app::state::use_store;

use super::*;

pub const APP_MAIN: &str = "app-main";
pub const TOAST_CONTAINER: &str = "toast-container";

#[derive(Clone, PartialEq, Eq, Params)]
struct LocalizeParams {
    lang: Option<String>,
}

pub fn use_lang() -> Signal<Language> {
    let state = use_store();
    Signal::derive(move || state.get().lang)
}

#[derive(
    Clone, Serialize, Deserialize, Debug, PartialEq, Eq, strum::Display, Default, strum::EnumString,
)]
pub enum Language {
    #[default]
    #[strum(to_string = "en")]
    En,
    #[strum(to_string = "ru")]
    Ru,
    #[strum(to_string = "nl")]
    Nl,
    #[strum(to_string = "ja")]
    Ja,
}

#[component]
pub fn LocalizedRootView() -> impl IntoView {
    let route = use_params::<LocalizeParams>();
    let state = use_store();

    let lang = Signal::derive(move || {
        Language::from_str(
            route
                .get()
                .map(|p| p.lang)
                .ok()
                .flatten()
                .unwrap_or_else(|| rust_i18n::available_locales!().first().unwrap().to_string())
                .as_str(),
        )
        .unwrap()
    });

    create_isomorphic_effect(move |_| {
        let lang = lang.get();
        state.update(move |s| s.lang = lang);
    });

    let localized = move || {
        rust_i18n::set_locale(lang.get().to_string().as_str());

        view! {
            <HeaderView/>
            <main class="overflow-auto grow flex flex-col" id=APP_MAIN>
                <div class="grow mx-auto w-full max-w-screen-2xl px-6 md:px-8 lg:px-16 flex flex-col sm:flex-row-reverse lg:flex-col lg:justify-stretch lg:items-stretch">
                    <Outlet/>
                </div>
                <div id=TOAST_CONTAINER class="fixed right-0 bottom-0 flex flex-col-reverse gap-4 items-stretch h-min max-h-96 overflow-auto pr-8 pb-8">
                </div>
            </main>
            <FooterView/>
        }
    };

    view! {
        <Html lang={move || lang.get().to_string()}/>
        {localized}
    }
}
