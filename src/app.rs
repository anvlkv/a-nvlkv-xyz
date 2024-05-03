mod components;
mod pages;
mod process;
mod state;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use pages::*;
use state::StoreProvider;

pub use components::Language;
use components::LocalizedView;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <StoreProvider>
            <Stylesheet id="leptos" href="/pkg/a_nvlkv_xyz.css"/>

            <div class="font-sans h-screen w-screen overflow-hidden flex flex-col bg-stone-300 dark:bg-stone-950 text-slate-950 dark:text-slate-50">
                <Router>
                    <Routes>
                        <Route path="/:lang" view=LocalizedView>
                            <Route path="/:step?" view=ProcessView />
                            <Route path="/:step/:example_id" view=ProcessView />
                            <Route path="/projects" view=ProjectsView />
                            <Route path="/projects/:id" view=CaseView />
                            <Route path="/contact" view=ContactView />
                            <Route path="/resume" view=ResumeView />
                            <Route path="/links" view=LinksView />
                        </Route>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </Router>
            </div>
        </StoreProvider>
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
