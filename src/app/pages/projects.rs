use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::projects::ProjectsGridDummy;

#[component]
pub fn ProjectsView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {}", t!("projects.title"), t!("name"))}/>
        <Transition fallback=ProjectsGridDummy>
        <Outlet/>
        </Transition>
        // <div class="mx-auto max-w-screen-2xl px-6 md:px-8 lg:px-16 min-h-full flex flex-col justify-center items-center">
        // </div>
    }
}
