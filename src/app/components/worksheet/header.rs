use leptos::*;
use leptos_router::*;

use crate::app::components::use_wk_ctx;

use super::Tab;

pub struct WorksheetHeaderDescriptionProps {
    pub toggle_hidden: Callback<()>,
    pub hidden: Signal<bool>,
}

#[component]
pub fn WorksheetHeader(
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into, optional)] description_id: MaybeSignal<String>,
    #[prop(into, optional)] tabs: MaybeSignal<Vec<Tab>>,
) -> impl IntoView {
    let ctx = use_wk_ctx();

    create_render_effect({
        let id = description_id.clone();
        move |_| {
            let id = id.get();
            ctx.set_current_description.set(id);
        }
    });

    on_cleanup(move || {
        let id = description_id.get();
        _ = ctx.set_current_description.try_update(move |t| {
            if t == &id {
                *t = Default::default();
            }
        });
    });

    let description_hidden = ctx.description_hidden.clone();
    let toggle_description_hidden = ctx.toggle_description_hidden.clone();
    let title = Signal::derive(move || title.get());
    let tabs = Signal::derive(move || tabs.get());

    view! {
        <div class="grow-0 flex items-end flex-wrap w-full mb-6">
            <h2 class="shrink-0 max-w-full text-2xl md:text-3xl xl:text-4xl block mr-3">
                {title}
            </h2>
            <div class="flex flex-wrap justify-end grow items-end h-full">
                <div class="border-b-2 px-2 border-slate-400 grow rounded-t-lg after:content-[' ']">
                    <Show when={move || description_hidden.get()}>
                        <button on:click={move |_| toggle_description_hidden.call(())} title=t!("util.info") class="text-2xl -mb-0.5 text-sky-800 dark:text-sky-200">{"ⓘ"}</button>
                    </Show>
                </div>
                <For each=move || tabs.get()
                    key=|state| state.href.clone()
                    let:child>
                    <A href={child.href} exact={true} class={ format!("worksheet-tab block rounded-t px-4 pt-3 pb-1 ml-0 mr-px border border-slate-400 border-b-2 hover:text-purple-800 hover:border-purple-800 active:text-purple-950 {}", if child.is_example { "italic"} else { "non-italic" })} active_class="pointer-events-none -mb-px border-b-transparent">
                        {child.title}
                    </A>
                </For>
            </div>
        </div>
    }
}
