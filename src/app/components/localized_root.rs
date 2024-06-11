use std::str::FromStr;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{storage::use_storage, utils::JsonCodec};
use serde::{Deserialize, Serialize};

use crate::app::{
    state::{use_store, StorageMode},
    STYLED_ROOT,
};

use super::*;

pub const APP_MAIN: &str = "app-main";
pub const TOAST_CONTAINER: &str = "toast-container";
pub const DARK_MODE_STORAGE: &str = "dark-mode";

#[derive(Clone, PartialEq, Eq, Params)]
pub struct LocalizeParams {
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
    view! {
        <div class="font-sans min-h-screen w-screen overflow-auto md:overflow-hidden flex flex-col bg-stone-300 dark:bg-stone-950 text-slate-950 dark:text-slate-50" id={STYLED_ROOT}>
            <Localized>
                <ThemedHtml/>
                <HeaderView/>
                <main class="grow flex flex-col" id={APP_MAIN}>
                    <Outlet/>
                    <div id=TOAST_CONTAINER class="fixed right-0 bottom-0 flex flex-col-reverse gap-4 items-stretch h-min max-h-96 overflow-auto pr-8 pb-8">
                    </div>
                </main>
                <FooterView/>
                <PrivacyNoticeView/>
            </Localized>
        </div>
    }
}

#[component]
pub fn Localized(children: ChildrenFn) -> impl IntoView {
    let params = use_params::<LocalizeParams>();
    let store = use_store();

    let lang = Signal::derive(move || {
        Language::from_str(
            params
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
        store.update(move |s| s.lang = lang);
    });

    let localized = move || {
        rust_i18n::set_locale(lang.get().to_string().as_str());

        children()
    };

    view! {
        <div class="contents">
            {localized}
        </div>
    }
}

#[component]
fn ThemedHtml() -> impl IntoView {
    let store = use_store();

    let storage_type = create_read_slice(store, |s| {
        s.storage_preference.get().unwrap_or(StorageMode::None)
    });

    let themed = move || {
        let storage_type = storage_type.get();

        let (dark_setting, _, _) =
            use_storage::<Option<bool>, JsonCodec>((&storage_type).into(), DARK_MODE_STORAGE);

        let dark_mode_class = Signal::derive(move || {
            dark_setting
                .get()
                .map(|setting| if setting { "theme-dark" } else { "theme-light" }.to_string())
        });

        let lang = use_lang();

        view! {
            <Html
                lang={move || lang.get().to_string()}
                attr:class=dark_mode_class
            />
        }
    };

    themed.into_view()
}
