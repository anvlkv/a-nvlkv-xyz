use leptos::*;
use leptos_router::*;

use crate::app::Language;

#[component]
pub fn FooterView() -> impl IntoView {
    let lang = use_context::<Signal<Language>>().unwrap();

    view! {
        <footer class="bg-stone-100 dark:bg-stone-900 shadow-sm flex justify-center">
            <nav class="flex shrink-0 grow gap-2 flex-col sm:flex-row justify-between px-6 md:px-8 lg:px-16 py-6 max-w-screen-2xl">
                <A href={move || format!("/{}/resume", lang.get().0)}>{ t!("menu.resume") }</A>
                <A href={move || format!("/{}/projects", lang.get().0)}>{ t!("menu.projects") }</A>
                <A href={move || format!("/{}/1", lang.get().0)}>{ t!("menu.process") }</A>
                <A href={move || format!("/{}/links", lang.get().0)}>{ t!("menu.links") }</A>
                <A href={move || format!("/{}/contact", lang.get().0)}>{ t!("menu.contact") }</A>
            </nav>
        </footer>
    }
}
