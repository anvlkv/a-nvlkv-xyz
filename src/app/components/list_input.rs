use form_signal::FormState;
use leptos::*;
use leptos_use::{
    use_draggable_with_options, use_event_listener, use_mouse_in_element_with_options,
    UseDraggableOptions, UseDraggableReturn, UseMouseInElementOptions, UseMouseInElementReturn,
};
use uuid::Uuid;

use crate::app::components::{ButtonView, IconView, StringInputView};

#[component]
pub fn ListInputView(
    #[prop(into)] input_type: String,
    #[prop(into)] data: Signal<Vec<FormState<String>>>,
    #[prop(into)] add_value: Callback<(String, Option<usize>), Uuid>,
    #[prop(into)] remove_value: Callback<Uuid>,
    #[prop(into, optional)] drop_target_name: MaybeSignal<String>,
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
        create_memo(move |_| data.get().iter().take(1).find(|v| v.get().is_empty()).map(|v| v.id));

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
        #[cfg(feature = "client")]
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
    let drop_target_name = Signal::derive(move || drop_target_name.get());
    let delete_allowed = Signal::derive(move || data.try_get().unwrap_or_default().len() > 1);

    view! {
        <div class="flex flex-col items-stretch" node_ref=element>
            <ol class="contents">
                <Show
                    when={move || data.try_get().unwrap_or_default().len() <= 1}
                >
                    <ListDropTarget
                        item_after=FormState::new(Default::default())
                        drop_target_name=drop_target_name
                    />
                </Show>
                <For
                    each=move || data.try_get().unwrap_or_default().into_iter().enumerate()
                    key=|(index, state)| (state.id, *index)
                    let:child
                >
                    <ListItemView
                        input_type=input_type.clone()
                        placeholder=placeholder.clone()
                        autocomplete=autocomplete.clone()
                        with_placeholder_id
                        focused_id=focused_id
                        on_blur=on_blur_item
                        on_focus={move |_| focused_id.set(Some(child.1.id))}
                        item=child.1.clone()
                        remove_value
                        delete_allowed
                    />
                    <ListDropTarget
                        item_after=child.1.clone()
                        drop_target_name=drop_target_name
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
                    attr:tabindex={2}
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
    #[prop(into)] item: FormState<String>,
    #[prop(into)] delete_allowed: Signal<bool>,
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

    let dragable_ref = create_node_ref::<html::Div>();
    let drag_ctx = use_context::<DragListCtx>();

    let has_drag_ctx = Signal::derive({
        let drag_ctx = drag_ctx.clone();
        move || drag_ctx.is_some()
    });

    let UseDraggableReturn {
        is_dragging, style, ..
    } = use_draggable_with_options(
        dragable_ref,
        UseDraggableOptions::default()
            .on_start({
                let drag_ctx = drag_ctx.clone();
                move |_| {
                    if let Some(ctx) = drag_ctx.as_ref() {
                        let value = value.get();
                        ctx.0.set(Some(value));
                        true
                    } else {
                        false
                    }
                }
            })
            .on_end({
                let drag_ctx = drag_ctx.clone();
                move |_| {
                    if let Some(ctx) = drag_ctx.as_ref() {
                        if let Some((insert_after, drop_target)) = ctx.1.get() {
                            ctx.2.call((value.get(), drop_target, insert_after));
                        } else {
                            log::debug!("drop restore position")
                        }
                        ctx.0.set(None);
                    }
                }
            }),
    );

    let style = Signal::derive(move || {
        let style = style.get();
        if is_dragging.get() {
            Some(format!("position: fixed; {style}"))
        } else {
            None
        }
    });

    let class = Signal::derive(move || {
        format!(
            "flex mb-4 {} {}",
            match (has_drag_ctx.get(), delete_allowed.get()) {
                (true, true) => "rounded-full",
                (true, false) => "rounded-l-full rounded-r",
                (false, true) => "rounded-l rounded-r-full",
                (false, false) => "rounded",
            },
            if is_dragging.get() {
                "shadow-lg scale-110"
            } else {
                ""
            }
        )
    });

    let input_class = Signal::derive(move || {
        format!(
            "{} focus:z-10",
            match (has_drag_ctx.get(), delete_allowed.get()) {
                (true, true) => "rounded-none",
                (true, false) => "rounded-l-none rounded-r",
                (false, true) => "rounded-l rounded-r-none",
                (false, false) => "rounded",
            }
        )
    });

    view! {
        <li class={class} style={style}>
            <Show when=move || has_drag_ctx.get()>
                <div
                    class="grow-0 shrink-0 pl-4 pr-2 pb-0.5 flex items-center border border-r-0 border-slate-400 bg-stone-50 dark:bg-stone-950 text-stone-950 dark:text-stone-50 hover:text-purple-600 dark:hover:text-purple-600 text-xs focus:outline-purple-400 hover:outline active:outline rounded-l-full cursor-move"
                    title={t!("util.reorder")}
                    node_ref={dragable_ref}
                    dragable=true
                >
                    <IconView icon="Drag"/>
                </div>
            </Show>
            <StringInputView
                class=input_class
                autocomplete
                auto_focus
                on_blur=on_blur_item
                on_focus=on_focus_item
                input_type
                value
                placeholder />
            <Show when=move || delete_allowed.get()>
                <button
                    tabindex="2"
                    on:click={on_remove}
                    title=t!("util.delete")
                    class="grow-0 shrink-0 px-4 pb-0.5 border border-l-0 border-slate-400 bg-stone-50 dark:bg-stone-950 text-stone-950 dark:text-stone-50 hover:text-red-600 dark:hover:text-red-600 text-xs focus:outline-purple-400 focus:outline rounded-r-full"
                >
                    <IconView icon="Delete"/>
                </button>
            </Show>
        </li>
    }
}

#[derive(Clone, Debug)]
pub struct DragListCtx(
    RwSignal<Option<FormState<String>>>,
    RwSignal<Option<(Uuid, String)>>,
    Callback<(FormState<String>, String, Uuid)>,
);

impl DragListCtx {
    pub fn provide(cb: Callback<(FormState<String>, String, Uuid)>) {
        let origin = create_rw_signal(None);
        let target = create_rw_signal(None);
        provide_context(DragListCtx(origin, target, cb));
    }
}

const TOLLERANCE: f64 = 15_f64;

#[component]
fn ListDropTarget(
    #[prop(into)] item_after: FormState<String>,
    #[prop(into)] drop_target_name: Signal<String>,
) -> impl IntoView {
    let drag_ctx = use_context::<DragListCtx>();

    if let Some(ctx) = drag_ctx {
        let el = create_node_ref::<html::Li>();
        let id_after = item_after.id.clone();

        let UseMouseInElementReturn {
            is_outside,
            element_x,
            element_y,
            element_width,
            element_height,
            ..
        } = use_mouse_in_element_with_options(
            el,
            UseMouseInElementOptions::default().handle_outside(true),
        );

        let is_in_tolerance_bounds = Signal::derive(move || {
            (element_x.get() >= -TOLLERANCE && element_y.get() >= -TOLLERANCE)
                && (element_x.get() - element_width.get() <= TOLLERANCE
                    && element_y.get() - element_height.get() <= TOLLERANCE)
        });

        let class = Signal::derive(move || {
            format!(
                "transition-all duration-100 mx-4 mb-4 rounded {}",
                if !is_outside.get() || is_in_tolerance_bounds.get() {
                    "h-8 bg-purple-400 dark:bg-purple-600"
                } else {
                    "h-2 bg-purple-200 dark:bg-purple-900"
                }
            )
        });

        create_render_effect(move |_| {
            let drop_target = drop_target_name.get();
            if !is_outside.get() || is_in_tolerance_bounds.get() {
                ctx.1.set(Some((id_after, drop_target)))
            } else {
                ctx.1.update(|d| match d.as_ref() {
                    Some(id) => {
                        if id == &(id_after, drop_target) {
                            *d = None
                        }
                    }
                    _ => {}
                })
            }
        });

        view! {
            <Show
                when=move || ctx.0.get().filter(|f| f.id != id_after).is_some()
            >
                <li
                    id={format!("drop-target-{id_after}")}
                    class=class
                    node_ref={el}
                />
            </Show>
        }
        .into_view()
    } else {
        ().into_view()
    }
}
