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
        // site head
        <Stylesheet id="leptos" href="/pkg/a_nvlkv_xyz.css"/>
        <Link rel="icon" attr:type="image/ico" href="/pkg/favicon.ico" />
        <Link rel="icon" attr:type="image/png" href="/pkg/favicon-32x32.png" sizes="32x32" />
        <Link rel="icon" attr:type="image/png" href="/pkg/favicon-16x16.png" sizes="16x16" />
        // js libs
        <Script src="https://unpkg.com/@rive-app/canvas@2.15.2"/>

        // app
        <StoreProvider>
            <div class="font-sans md:h-screen w-screen overflow-auto md:overflow-hidden flex flex-col bg-stone-300 dark:bg-stone-950 text-slate-950 dark:text-slate-50">
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
