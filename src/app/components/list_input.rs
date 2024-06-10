use form_signal::FormState;
use leptos::*;
use leptos_use::use_event_listener;
use uuid::Uuid;

use crate::app::components::{ButtonView, IconView, StringInputView};

#[component]
pub fn ListInputView(
    #[prop(into)] input_type: String,
    #[prop(into)] data: Signal<Vec<FormState<String>>>,
    #[prop(into)] add_value: Callback<(String, Option<usize>), Uuid>,
    #[prop(into)] remove_value: Callback<Uuid>,
    #[prop(into, optional)] placeholder: MaybeSignal<String>,
    #[prop(into, optional)] add_entry_text: MaybeSignal<String>,
    #[prop(into, optional)] autocomplete: MaybeSignal<Vec<String>>,
) -> impl IntoView {
    let focused_id = create_rw_signal::<Option<Uuid>>(None);

    let on_add = move || {
        let data = data.get();
        let next_pos = focused_id
            .get()
            .map(|id| data.iter().position(|d| d.id == id).map(|p| p + 1))
            .flatten();
        let next_id = add_value.call((Default::default(), next_pos));
        focused_id.set(Some(next_id));
    };

    let with_placeholder_id =
        create_memo(move |_| data.get().iter().find(|v| v.get().is_empty()).map(|v| v.id));

    let element = create_node_ref::<html::Div>();
    #[cfg_attr(feature = "ssr", allow(unused_variables))]
    let button_element = create_node_ref::<html::AnyElement>();

    let is_multiline = input_type == "textarea".to_string();

    let cleanup_listener = use_event_listener(element, ev::keydown, move |e: ev::KeyboardEvent| {
        if e.key().to_lowercase().as_str() == "enter" {
            if is_multiline && e.shift_key() {
                return;
            }
            e.prevent_default();
            e.stop_propagation();
            on_add();
        } else if e.key().to_lowercase().as_str() == "backspace" {
            if let Some((id, next)) = focused_id
                .get()
                .map(|id| {
                    let data = data.get();
                    data.iter()
                        .enumerate()
                        .find(|(_, d)| d.id == id)
                        .filter(|(_, d)| d.get().is_empty())
                        .map(|(i, d)| (d.id, data.iter().nth(i.saturating_sub(1)).map(|d| d.id)))
                })
                .flatten()
            {
                e.prevent_default();
                remove_value.call(id);
                focused_id.set(next);
            }
        }
    });

    #[cfg_attr(feature = "ssr", allow(unused_variables))]
    let on_blur_item = move |e: ev::FocusEvent| {
        #[cfg(any(feature = "hydrate", feature = "csr"))]
        {
            use wasm_bindgen::JsCast;

            let element = button_element.get();
            let element = element.as_deref();

            if e.related_target()
                .map(|t| t.dyn_ref::<web_sys::HtmlElement>().as_deref() == element)
                .unwrap_or(false)
            {
                e.prevent_default();
            } else {
                focused_id.set(None);
            }
        }
    };

    on_cleanup(move || {
        cleanup_listener();
    });

    let add_entry_text = Signal::derive(move || add_entry_text.get());

    view! {
        <div class="flex flex-col" node_ref=element>
            <ol class="contents">
                <For
                    each=move || data.try_get().unwrap_or_default()
                    key=|state| state.id
                    let:child
                >
                    <ListItemView
                        input_type=input_type.clone()
                        placeholder=placeholder.clone()
                        autocomplete=autocomplete.clone()
                        with_placeholder_id
                        focused_id=focused_id
                        on_blur=on_blur_item
                        on_focus={move |_| focused_id.set(Some(child.id))}
                        item=child
                        remove_value
                    />
                </For>
            </ol>
            <div class="contents">
                <ButtonView
                    cta=1
                    attr:class="mt-4 mx-8 lg:mx-15 xl:mx-20 md:px-3 md:py-2 md:min-w-28"
                    on:click={move |e| {
                        e.prevent_default();
                        on_add()
                    }}
                    node_ref={button_element}
                >
                    {add_entry_text}
                </ButtonView>
            </div>
        </div>
    }
}

#[component]
fn ListItemView(
    #[prop(into)] input_type: String,
    #[prop(into, optional)] placeholder: MaybeSignal<String>,
    #[prop(into)] with_placeholder_id: Signal<Option<Uuid>>,
    #[prop(into, optional)] autocomplete: MaybeSignal<Vec<String>>,
    #[prop(into)] focused_id: Signal<Option<Uuid>>,
    #[prop(into)] remove_value: Callback<Uuid>,
    #[prop(into)] on_blur: Callback<ev::FocusEvent>,
    #[prop(into)] on_focus: Callback<ev::FocusEvent>,
    item: FormState<String>,
) -> impl IntoView {
    let value = Signal::derive(move || item.clone());

    let on_remove = move |e: ev::MouseEvent| {
        e.prevent_default();
        remove_value.call(value.get().id);
    };

    let auto_focus = Signal::derive(move || {
        value
            .try_get()
            .zip(focused_id.get())
            .map(|(v, f)| v.id == f)
            .unwrap_or(false)
    });

    let on_blur_item = Callback::new(move |e| {
        on_blur.call(e);
    });
    let on_focus_item = Callback::new(move |e| {
        on_focus.call(e);
    });

    let placeholder = Signal::derive(move || {
        if with_placeholder_id.get() == Some(value.get().id) {
            placeholder.get()
        } else {
            Default::default()
        }
    });

    view! {
        <li class="flex mb-4">
            <StringInputView
                class="rounded-r-none focus:z-10"
                autocomplete
                auto_focus
                on_blur=on_blur_item
                on_focus=on_focus_item
                input_type
                value
                placeholder />
            <button
                tabindex="2"
                on:click={on_remove}
                title=t!("util.delete")
                class="grow-0 shrink-0 px-4 pb-0.5 border border-l-0 border-slate-400 bg-stone-50 dark:bg-stone-950 text-stone-950 dark:text-stone-50 hover:text-red-600 dark:hover:text-red-600 text-xs focus:outline-purple-400 focus:outline rounded-r-full"
            >
                <IconView icon="Delete"/>
            </button>
        </li>
    }
}
