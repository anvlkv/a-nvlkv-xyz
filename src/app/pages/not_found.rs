use leptos::*;
use leptos_meta::*;

use crate::app::components::{ButtonSize, ButtonView, RvArtboardView};

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        if let Some(resp) = use_context::<leptos_spin::ResponseOptions>() {
            resp.set_status(404);
        }
    }

    view! {
        <Title text={move || t!("not_found.title", fallback="en")}/>
        <div class="flex flex-col items-center">
            <h1 class="text-6xl my-16">{t!("not_found.title", fallback="en")}</h1>
            <RvArtboardView
                attr:class="w-64 h-64"
                state_machine="Inquire State Machine"
                name="Inquire"
            />
            <ButtonView
                attr:class="mt-8"
                cta=2
                link="/"
                on:click={move |_| {
                    _ = window().location().set_href("/");
                }}
                size=ButtonSize::Lg
            >
                {t!("not_found.cta", fallback="en")}
            </ButtonView>
        </div>
    }
}
