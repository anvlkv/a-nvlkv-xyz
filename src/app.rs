mod components;
mod pages;
mod process;
mod state;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use pages::*;
use state::StoreProvider;

use crate::app::components::{FooterView, HeaderView};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <StoreProvider>
            <Stylesheet id="leptos" href="/pkg/a_nvlkv_xyz.css"/>

            <Title text="Welcome to Leptos"/>

            <div class="font-sans h-screen w-screen overflow-hidden flex flex-col bg-stone-300 dark:bg-stone-950 text-slate-950 dark:text-slate-50">
                <Router>
                    <Routes>
                        <Route path="/:lang" view=LocalizedView>
                            <Route path="/:step?" view=ProcessView />
                            <Route path="/:step/:example_id" view=ProcessView />
                            <Route path="/projects" view=ContactView />
                            <Route path="/projects/:id" view=ContactView />
                            <Route path="/contact" view=ContactView />
                            <Route path="/resume" view=ContactView />
                            <Route path="/links" view=ContactView />
                        </Route>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </Router>
            </div>
        </StoreProvider>
    }
}

#[derive(Clone, PartialEq, Eq, Params)]
struct LocalizeParams {
    lang: Option<String>,
}

#[derive(Clone)]
pub struct Language(pub String);

#[component]
fn LocalizedView() -> impl IntoView {
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
            <main class="overflow-auto grow">
                <Outlet/>
            </main>
            <FooterView/>
        }
    };

    view! {
        <Html lang={move || lang.get().0}/>
        {localized}
    }
}

#[server(SaveCount, "/api")]
pub async fn save_count(count: u32) -> Result<(), ServerFnError<String>> {
    println!("Saving value {count}");
    let store = spin_sdk::key_value::Store::open_default().map_err(|e| e.to_string())?;
    store
        .set_json("a_nvlkv_xyz_count", &count)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(())
}
