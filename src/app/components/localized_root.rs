use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::*;

pub const APP_MAIN: &str = "app-main";
pub const TOAST_CONTAINER: &str = "toast-container";

#[derive(Clone, PartialEq, Eq, Params)]
struct LocalizeParams {
    lang: Option<String>,
}

#[derive(Clone)]
pub struct Language(pub String);

#[component]
pub fn LocalizedRootView() -> impl IntoView {
    let route = use_params::<LocalizeParams>();

    let lang = Signal::derive(move || {
        Language(
            route
                .get()
                .map(|p| p.lang)
                .ok()
                .flatten()
                .unwrap_or_else(|| rust_i18n::available_locales!().first().unwrap().to_string()),
        )
    });

    provide_context(lang);

    let localized = move || {
        rust_i18n::set_locale(lang.get().0.as_str());

        view! {
            <HeaderView/>
            <main class="overflow-auto grow flex flex-col" id=APP_MAIN>
                <div class="grow mx-auto w-full max-w-screen-2xl px-6 md:px-8 lg:px-16 flex flex-col sm:flex-row-reverse lg:flex-col lg:justify-stretch lg:items-stretch">
                    <Outlet/>
                </div>
                <div id=TOAST_CONTAINER class="fixed right-0 bottom-0 flex flex-col-reverse gap-4 items-stretch h-min max-h-full overflow-auto pr-8 pb-8">
                </div>
            </main>
            <FooterView/>
        }
    };

    view! {
        <Html lang={move || lang.get().0}/>
        {localized}
    }
}
