pub mod components;
pub mod pages;
pub mod process;
pub mod projects;
pub mod resume;
pub mod state;
pub mod util;

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
        <Link
            rel="stylesheet"
            href="https://cdnjs.cloudflare.com/ajax/libs/paper-css/0.4.1/paper.css"
            integrity="sha384-Velkkr4y29T3b+5t49UmQaVHkJrr1GJRHHq1BG3nSpmQrdf5Dv525IDQRdqkxZpd"
            crossorigin="anonymous"
        />
        <Link rel="manifest" href="/pkg/manifest.json" />
        <Link rel="icon" attr:type="image/ico" href="/pkg/favicon.ico" />
        <Link rel="icon" attr:type="image/png" href="/pkg/favicon-32x32.png" sizes="32x32" />
        <Link rel="icon" attr:type="image/png" href="/pkg/favicon-16x16.png" sizes="16x16" />
        <Link rel="preload" href="/pkg/anvlkv-done-button.riv" attr:as="fetch" crossorigin="anonymous"/>

        <Meta attr:http-equiv="Content-Security-Policy" content="
          default-src;
          script-src 'self' unpkg.com cdn.jsdelivr.net cdnjs.cloudflare.com 'unsafe-inline' 'wasm-unsafe-eval' 'unsafe-eval';
          style-src 'self' cdnjs.cloudflare.com 'unsafe-inline';
          img-src 'self' data: *.xata.sh;
          font-src 'self';
          connect-src 'self' unpkg.com cdn.jsdelivr.net;
          media-src 'self';
          object-src 'none';
          child-src;
          frame-src 'self';
          form-action 'self';
          base-uri;
          manifest-src 'self';
        "/>

        // cdn libs
        <Script
            src="https://unpkg.com/@rive-app/canvas@2.17.3"
            integrity="sha384-S6ym4vurJm2ceWBIlCGYq8aG9EJvqCthpBoVP40V0gJ3QS+TkORGHohsYBNpYzTT"
            crossorigin="anonymous"
        />
        <Link
            rel="stylesheet"
            href="https://cdnjs.cloudflare.com/ajax/libs/animate.css/4.1.1/animate.min.css"
            integrity="sha384-Gu3KVV2H9d+yA4QDpVB7VcOyhJlAVrcXd0thEjr4KznfaFPLe0xQJyonVxONa4ZC"
            crossorigin="anonymous"
        />

        // app
        <div class="contents">
            <StoreProvider>
                    <Router>
                        <Routes>
                            <Route path=":lang/process/download" view=process::WorksheetsDownload/>
                            <Route path=":lang" view=LocalizedRootView>
                                <Route path="" view=LandingView/>
                                <Route path="process" view=ProcessView ssr=SsrMode::PartiallyBlocked>
                                    <Route path=":step/:example?" view=process::ProcessSwitchView/>
                                </Route>
                                <Route path="projects" view=ProjectsView>
                                    <Route path="" view=projects::ProjectsGridView />
                                    <Route path=":id" view=projects::CaseView />
                                </Route>
                                <Route path="contact" view=ContactView />
                                <Route path="resume" view=ResumeView />
                                <Route path="links" view=LinksView />
                            </Route>
                            <Route path="*any" view=NotFound/>
                        </Routes>
                    </Router>
            </StoreProvider>
        </div>
    }
}
