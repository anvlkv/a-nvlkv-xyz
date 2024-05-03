use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn LinksView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {}", t!("links.title"), t!("name"))}/>
        <div class="mx-auto max-w-screen-2xl px-6 md:px-8 lg:px-16 min-h-full flex justify-center items-center">
            {"links"}
        </div>
    }
}
