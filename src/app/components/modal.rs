use leptos::*;

use crate::app::{components::ButtonView, STYLED_ROOT};

#[component]
pub fn ModalView(
    #[prop(into)] when: MaybeSignal<bool>,
    #[prop(into, optional)] cancel_btn: MaybeSignal<bool>,
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

    let cta_1 = Signal::derive(move || {
        if cancel_btn.try_get().unwrap_or(false) {
            2
        } else {
            1
        }
    });

    view! {
        <Show when=move || when.get()>
            <Portal mount={document().fullscreen_element().unwrap_or_else(|| document().body().unwrap().into())}>
                <Show when=move || curtain.get()>
                    <div role="presentation" class="absolute min-w-screen min-h-dvh h-full w-full bg-gray-950 opacity-75 top-0 left-0"></div>
                </Show>
                <div class="fixed top-0 left-0 min-w-screen min-h-dvh w-full h-full flex justify-center items-center font-sans text-slate-950 dark:text-slate-50">
                    <div role="dialog" class="p-8 pb-4 bg-stone-200 dark:bg-stone-800 rounded-xl shadow-lg max-h-svh overflow-y-auto animate__animated animate__fadeInDown">
                        {children_view}
                        <hr class="border-t border-slate-400 mt-8 mb-4"/>
                        <div class="flex w-full justify-end">
                            <ButtonView
                                attr:class="mr-4 min-w-20 md:min-w-28"
                                on:click={move |_| on_resolve(true)}
                                cta=cta_1
                            >
                                {t!("util.ok")}
                            </ButtonView>
                            <Show when=move || cancel_btn.get()>
                                <ButtonView
                                    attr:class="min-w-20 md:min-w-28"
                                    on:click={move |_| on_resolve(false)}
                                >
                                    {t!("util.cancel")}
                                </ButtonView>
                            </Show>
                        </div>
                    </div>
                </div>
            </Portal>
        </Show>
    }
}
