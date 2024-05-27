use form_signal::FormState;
use leptos::{
    html::{div, input, textarea},
    *,
};
use leptos_use::{use_element_bounding, UseElementBoundingReturn};

use crate::app::components::APP_MAIN;

const AUTOCOMPLETE_OPTION: &str = "_autocomplete-option";

#[component]
pub fn StringInputView(
    #[prop(into)] input_type: String,
    #[prop(into, optional)] placeholder: MaybeSignal<String>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into, optional)] auto_focus: MaybeSignal<bool>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into)] value: Signal<FormState<String>>,
    #[prop(into, optional)] autocomplete: MaybeSignal<Vec<String>>,
    #[prop(attrs, optional)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let (autocomplete_options, set_autocomplete_options) = create_signal::<Vec<String>>(Vec::new());
    let (focused, set_focused) = create_signal(false);
    let (may_autocomplete, set_may_autocomplete) = create_signal(false);

    let element = create_node_ref::<html::AnyElement>();

    let on_input = move |event| {
        let next = value.get();
        let input_value = event_target_value(&event);
        next.update(move |v| *v = input_value);

        let input_value = event_target_value(&event);
        let autocomplete_options = autocomplete
            .get()
            .into_iter()
            .filter(|val| {
                val.to_lowercase()
                    .contains(input_value.to_lowercase().as_str())
                    && val != &input_value
            })
            .collect::<Vec<_>>();
        set_may_autocomplete.set(autocomplete_options.len() > 0);
        set_autocomplete_options.set(autocomplete_options);
    };

    let on_focus = move |e: ev::FocusEvent| {
        let next = value.get();
        next.touch();
        set_focused.set(true);
        if let Some(cb) = on_focus {
            cb.call(e);
        }
    };

    let value_src = Signal::derive(move || {
        value
            .try_get()
            .map(|v| v.try_get())
            .flatten()
            .unwrap_or_default()
    });

    #[cfg_attr(feature = "ssr", allow(unused_variables))]
    let may_blure = move |e: ev::FocusEvent| {
        if !focused.get() {
            if let Some(cb) = on_blur {
                cb.call(e);
            }

            set_may_autocomplete.set(false);
        }
    };

    #[cfg_attr(feature = "ssr", allow(unused_variables))]
    let on_blur_input_el = move |e: ev::FocusEvent| {
        #[cfg(any(feature = "hydrate", feature = "csr"))]
        {
            use wasm_bindgen::JsCast;
            if e.related_target()
                .map(|t| {
                    t.dyn_ref::<web_sys::HtmlLiElement>()
                        .map(|t| t.class_list().contains(AUTOCOMPLETE_OPTION))
                })
                .flatten()
                .unwrap_or(false)
            {
                e.prevent_default();
            } else {
                set_focused.set(false);
                may_blure(e);
            }
        }
    };

    #[cfg_attr(feature = "ssr", allow(unused_variables))]
    let on_blur_autocomplete_el = move |e: ev::FocusEvent| {
        #[cfg(any(feature = "hydrate", feature = "csr"))]
        {
            use wasm_bindgen::JsCast;

            let element = element.get();
            let element = element.as_deref();

            if e.related_target()
                .map(|t| t.dyn_ref::<web_sys::HtmlElement>().as_deref() == element)
                .unwrap_or(false)
            {
                e.prevent_default();
            } else {
                set_focused.set(false);
                may_blure(e);
            }
        }
    };

    let on_select_autocomplete = Callback::new(move |input_value: String| {
        let next = value.get();
        next.update(move |v| *v = input_value);
        set_may_autocomplete.set(false);
    });

    let UseElementBoundingReturn {
        x,
        y,
        width,
        height,
        ..
    } = use_element_bounding(element);

    create_effect(move |_| {
        if !may_autocomplete.get() && focused.get() {
            if let Some(el) = element.get().as_deref() {
                el.focus().unwrap();
            }
        }
    });

    create_effect(move |_| {
        if auto_focus.get() {
            if let Some(el) = element.get().as_deref() {
                el.focus().unwrap();
            }
        }
    });

    let class = move || {
        format!("w-full px-4 py-2 rounded border border-slate-400 bg-stone-50 dark:bg-stone-950 text-stone-950 dark:text-stone-50 text-lg focus:outline-purple-400 focus:outline {}", class.get())
    };

    div()
        .attr("class", "contents")
        .child(
            match input_type.as_str() {
                "textarea" => textarea().child(value_src.get_untracked()).into_any(),
                "text" | "url" | "date" | "time" | "week" | "datetime-local" | "password"
                | "email" | "tel" | "search" | "color" => input().into_any(),
                _ => {
                    unreachable!("input type {input_type} not supported")
                }
            }
            .node_ref(element)
            .attrs(attrs)
            .attr("id", move || value.get().id.to_string())
            .attr("class", class)
            .attr("placeholder", placeholder)
            .prop("value", value_src)
            .on(ev::focus, on_focus)
            .on(ev::blur, on_blur_input_el)
            .on(ev::input, on_input),
        )
        .child(view! {
            <AutoCompleteList
                options=autocomplete_options
                may_autocomplete=may_autocomplete
                on_select=on_select_autocomplete
                x=x
                y=y
                width=width
                height=height
                on_blur=on_blur_autocomplete_el
            />
        })
}

#[component]
fn AutoCompleteList(
    #[prop(into)] options: Signal<Vec<String>>,
    #[prop(into)] may_autocomplete: Signal<bool>,
    #[prop(into)] x: Signal<f64>,
    #[prop(into)] y: Signal<f64>,
    #[prop(into)] width: Signal<f64>,
    #[prop(into)] height: Signal<f64>,
    #[prop(into)] on_select: Callback<String>,
    #[prop(into)] on_blur: Callback<ev::FocusEvent>,
) -> impl IntoView {
    let options_view = move || {
        options.get().into_iter().map(|value| {
                let clicked_value = value.clone();
                let on_click = move |_| {
                    on_select.call(clicked_value.clone());
                };

                view!{
                    <li role="option"
                        tabindex="1"
                        class=format!("{AUTOCOMPLETE_OPTION} cursor-pointer px-4 py-2 border-b border-slate-400 last:border-b-0 focus:outline focus:outline-purple-400")
                        on:click=on_click
                        on:blur=move |e| on_blur.call(e)
                    >
                        {value}
                    </li>
                }
            }).collect_view()
    };

    let style = move || {
        format!(
            "left: {}px; top: {}px; width: {}px;",
            x.get(),
            y.get() + height.get(),
            width.get()
        )
    };

    view! {
        <Show when={move|| may_autocomplete.get() && options.get().len() > 0}>
            <Portal mount={document().get_element_by_id(APP_MAIN).unwrap()}>
                <ul role="listbox" class="fixed overflow-x-visible overflow-y-auto max-h-80 border border-slate-400 rounded bg-stone-50 dark:bg-stone-950 text-stone-950 dark:text-stone-50 text-lg shadow mt-2" style=style>
                    {options_view}
                </ul>
            </Portal>
        </Show>
    }
}
