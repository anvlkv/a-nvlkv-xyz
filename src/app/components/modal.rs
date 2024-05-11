use leptos::*;

use crate::app::STYLED_ROOT;

#[component]
pub fn ModalView(
    #[prop(into)] when: MaybeSignal<bool>,
    #[prop(into, optional)] curtain: MaybeSignal<bool>,
    #[prop(into)] on_resolve: Callback<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children_view = Signal::derive(move || children().into_view());

    create_render_effect(move |_| {
        if let Some(root_el) = document().get_element_by_id(STYLED_ROOT) {
            if when.get() {
                root_el.class_list().add_1("blur").unwrap();
            } else {
                root_el.class_list().remove_1("blur").unwrap();
            }
        }
    });

    view! {
        <Show when=move || when.get()>
            <Portal>
                <Show when=move || curtain.get()>
                    <div role="presentation" class="absolute min-w-screen min-h-screen h-full w-full bg-gray-950 opacity-75 top-0 left-0"></div>
                </Show>
                <div class="fixed top-0 left-0 min-w-screen min-h-screen w-full h-full flex justify-center items-center font-sans text-slate-950 dark:text-slate-50">
                    <div role="dialog" class="p-8 pb-4 bg-stone-200 dark:bg-stone-800 rounded-xl shadow-lg max-w-prose animate__animated animate__fadeInDown">
                        {children_view}
                        <hr class="border-t border-slate-400 mt-8 mb-4"/>
                        <div class="flex w-full justify-end">
                            <button class="mr-4 px-2 py-1 md:px-3 md:py-2 md:min-w-28 rounded-full bg-purple-900 hover:bg-purple-800 text-stone-100 active:bg-purple-950 border-2 border-solid border-slate-50 drop-shadow-sm" on:click={move |_| on_resolve.call(true)}>{t!("util.ok")}</button>
                            <button class="px-2 py-1 md:px-3 md:py-2 md:min-w-28 rounded-full bg-stone-300 dark:bg-stone-950 hover:bg-stone-200 dark:hover:bg-stone-800 active:bg-stone-300 dark:active:bg-stone:700 border-2 border-solid border-slate-50 drop-shadow-sm" on:click={move |_| on_resolve.call(false)}>{t!("util.cancel")}</button>
                        </div>
                    </div>
                </div>
            </Portal>
        </Show>
    }
}
