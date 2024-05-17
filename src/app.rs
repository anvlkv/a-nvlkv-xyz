pub mod components;
pub mod pages;
pub mod process;
pub mod state;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use pages::*;
use state::StoreProvider;

use components::LocalizedRootView;
pub use components::{use_lang, Language};

pub const STYLED_ROOT: &str = "app-styled-root";

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // site head
        <Stylesheet id="leptos" href="/pkg/a_nvlkv_xyz.css"/>
        <Link rel="icon" attr:type="image/ico" href="/pkg/favicon.ico" />
        <Link rel="icon" attr:type="image/png" href="/pkg/favicon-32x32.png" sizes="32x32" />
        <Link rel="icon" attr:type="image/png" href="/pkg/favicon-16x16.png" sizes="16x16" />
        // cdn libs
        <Script src="https://unpkg.com/@rive-app/canvas@2.15.2"/>
        <Link
            rel="stylesheet"
            href="https://cdnjs.cloudflare.com/ajax/libs/animate.css/4.1.1/animate.min.css"
        />

        // app
        <StoreProvider>
            <div class="font-sans min-h-screen w-screen overflow-auto md:overflow-hidden flex flex-col bg-stone-300 dark:bg-stone-950 text-slate-950 dark:text-slate-50" id={STYLED_ROOT}>
                <Router>
                    <Routes>
                        <Route path=":lang" view=LocalizedRootView>
                            <Route path="" view=LandingView/>
                            <Route path="process" view=ProcessView ssr=SsrMode::PartiallyBlocked>
                                <Route path="0" view=process::AboutView/>
                                <Route path="1/:example?" view=process::ProblemView/>
                                <Route path="2/:example?" view=process::SolutionView/>
                                <Route path="3/:example?" view=process::CompromiseView/>
                                <Route path="4/:example?" view=process::ImplementView/>
                                <Route path="5/:example?" view=process::IterateView/>
                                <Route path="*" view=process::InquireView/>
                            </Route>
                            <Route path="projects" view=ProjectsView />
                            <Route path="projects/:id" view=CaseView />
                            <Route path="contact" view=ContactView />
                            <Route path="resume" view=ResumeView />
                            <Route path="links" view=LinksView />
                        </Route>
                        <Route path="*any" view=NotFound/>
                    </Routes>
                </Router>
            </div>
        </StoreProvider>
    }
}
