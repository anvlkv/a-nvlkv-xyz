use std::collections::BTreeMap;

use form_signal::{AllSignalTraits, FormSignal};
use leptos::*;
use leptos_use::{
    core::Position, use_draggable_with_options, use_event_listener,
    use_mouse_in_element_with_options, UseDraggableOptions, UseDraggableReturn,
    UseMouseInElementOptions, UseMouseInElementReturn,
};
use uuid::Uuid;

use crate::app::components::{ButtonView, IconView, StringInputView};

#[derive(Debug, Clone)]
struct FocusCtx(RwSignal<Option<Uuid>>, RwSignal<BTreeMap<usize, Uuid>>);

#[component]
pub fn ListInputView<T, Rw, R, W>(
    #[prop(into)] input_type: String,
    #[prop(into)] value: FormSignal<T, Vec<String>, Rw, R, W>,
    #[prop(into, optional)] drop_target_name: MaybeSignal<String>,
    #[prop(into, optional)] placeholder: MaybeSignal<String>,
    #[prop(into, optional)] add_entry_text: MaybeSignal<String>,
    #[prop(into, optional)] autocomplete: MaybeSignal<Vec<String>>,
) -> impl IntoView
where
    Rw: AllSignalTraits<T>,
    T: std::fmt::Debug + Default + PartialEq + Clone + 'static,
    R: Fn(T) -> Vec<String> + Clone + 'static,
    W: Fn(&mut T, Vec<String>) + Clone + 'static,
{
    let focused_id = create_rw_signal::<Option<Uuid>>(None);
    let data_ids = create_rw_signal::<BTreeMap<usize, Uuid>>(BTreeMap::new());

    let on_add = {
        let value = value.clone();
        move || {
            let all_ids = data_ids.get();
            let next_pos = focused_id
                .get()
                .map(|id| {
                    all_ids
                        .iter()
                        .find_map(|(pos, d)| if d == &id { Some(pos + 1) } else { None })
                })
                .flatten();
            // let next_id = add_value.call((Default::default(), next_pos));
            // focused_id.set(Some(next_id));
            let new_val = String::default();
            value.update(|v| {
                if let Some(pos) = next_pos {
                    v.insert(pos, new_val);
                } else {
                    v.push(new_val);
                }
            });
        }
    };

    let remove_value = Callback::new({
        let value = value.clone();
        move |pos: usize| {
            value.update(|v| {
                _ = v.remove(pos);
            })
        }
    });

    let with_placeholder_id =
        create_memo(move |_| data_ids.get().values().next().map(|d| d.clone()));

    let element = create_node_ref::<html::Div>();
    #[cfg_attr(feature = "ssr", allow(unused_variables))]
    let button_element = create_node_ref::<html::AnyElement>();

    let is_multiline = input_type == "textarea".to_string();
    let delete_allowed = Signal::derive({
        let value = value.clone();
        move || value.try_get().unwrap_or_default().len() > 1
    });

    // let cleanup_listener = use_event_listener(element, ev::keydown, move |e: ev::KeyboardEvent| {
    //     let key = e.key().to_lowercase().as_str();
    //     if key == "enter" {
    //         if is_multiline && e.shift_key() {
    //             return;
    //         }
    //         e.prevent_default();
    //         e.stop_propagation();
    //         on_add();
    //     } else if key == "backspace" && delete_allowed.get() {
    //         if let Some((id, next)) = focused_id
    //             .get()
    //             .map(|id| {
    //                 let data = data_ids.get();
    //                 data.iter()
    //                     .find(|(_, d)| d == id)
    //                     .filter(|(_, d)| d.get().is_empty())
    //                     .map(|(i, d)| (d.id, data.iter().nth(i.saturating_sub(1)).map(|d| d.id)))
    //             })
    //             .flatten()
    //         {
    //             e.prevent_default();
    //             remove_value.call(id);
    //             focused_id.set(next);
    //         }
    //     }
    // });

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

    // on_cleanup(move || {
    //     cleanup_listener();
    // });

    let add_entry_text = Signal::derive(move || add_entry_text.get());
    let drop_target_name = Signal::derive(move || drop_target_name.get());
    let values_range = Signal::derive({
        let value = value.clone();
        move || {
            let total = value.get().len();
            (0..total).map(move |i| (i, total))
        }
    });
    provide_context(FocusCtx(focused_id, data_ids));

    log::debug!("render ListInputView");

    view! {
        <div class="flex flex-col items-stretch" node_ref=element>
            <ol class="contents">
                <Show
                    when={move || values_range.get().len() <= 1}
                >
                    <ListDropTarget
                        item_after={0 as usize}
                        drop_target_name=drop_target_name
                    />
                </Show>
                <For each={move || values_range.get()}
                    key=|(i, t)| (*i, *t)
                    let:child
                >
                    <ListItemView
                        input_type=input_type.clone()
                        placeholder=placeholder.clone()
                        autocomplete=autocomplete.clone()
                        with_placeholder_id
                        index=child.0
                        values={value.clone()}
                        delete_allowed
                        remove_value
                        drop_target_name
                    />
                    <ListDropTarget
                        item_after=child.0
                        drop_target_name
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
fn ListItemView<T, Rw, R, W>(
    #[prop(into)] input_type: String,
    #[prop(into)] index: MaybeSignal<usize>,
    #[prop(into, optional)] placeholder: MaybeSignal<String>,
    #[prop(into)] with_placeholder_id: Signal<Option<Uuid>>,
    #[prop(into, optional)] autocomplete: MaybeSignal<Vec<String>>,
    #[prop(into)] values: FormSignal<T, Vec<String>, Rw, R, W>,
    #[prop(into)] delete_allowed: Signal<bool>,
    #[prop(into)] remove_value: Callback<usize>,
    #[prop(into)] drop_target_name: Signal<String>,
) -> impl IntoView
where
    Rw: AllSignalTraits<T>,
    T: std::fmt::Debug + Default + PartialEq + Clone + 'static,
    R: Fn(T) -> Vec<String> + Clone + 'static,
    W: Fn(&mut T, Vec<String>) + Clone + 'static,
{
    let value = values.derive(
        move |v| {
            let i = index.get();
            v[i].clone()
        },
        move |v, n| {
            let i = index.get();
            v[i] = n;
        },
    );
    let FocusCtx(focused_id, all_ids) = use_context::<FocusCtx>().unwrap();

    create_render_effect(move |_| {
        let i = index.get();
        all_ids.update(|d| {
            _ = d.insert(i, value.id);
        });
    });

    on_cleanup(move || {
        let i = index.get();
        let id = value.id;
        all_ids.update(|d| {
            if d.get(&i) == Some(&id) {
                _ = d.remove(&i);
            }
        });
    });

    let on_remove = move |e: ev::MouseEvent| {
        e.prevent_default();
        let at = index.get();
        remove_value(at);
    };

    let auto_focus = Signal::derive(move || focused_id.get() == Some(value.id));

    let on_blur_item = Callback::new(move |_| {
        if focused_id.get() == Some(value.id) {
            focused_id.set(None)
        }
    });
    let on_focus_item = Callback::new(move |_| focused_id.set(Some(value.id)));

    let placeholder = Signal::derive(move || {
        if with_placeholder_id.get() == Some(value.id) {
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
                let value = value.clone();
                move |_| {
                    if let Some(ctx) = drag_ctx.as_ref() {
                        let value = value.get();
                        let index = index.get();
                        let name = drop_target_name.get();
                        ctx.origin.set(Some((index, value, name)));
                        true
                    } else {
                        false
                    }
                }
            })
            .on_end({
                let drag_ctx = drag_ctx.clone();
                let value = value.clone();
                move |_| {
                    if let Some(ctx) = drag_ctx.as_ref() {
                        if let Some((insert_after, drop_target)) = ctx.target.get() {
                            let cb = ctx.on_drop;
                            log::debug!("insert_after: {insert_after}");
                            let inner = value.get();
                            remove_value(index.get());
                            cb((inner, drop_target, insert_after));
                        } else {
                            log::debug!("drop restore position")
                        }
                        ctx.origin.set(None);
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
pub struct DragListCtx {
    on_drop: Callback<(String, String, usize)>,
    origin: RwSignal<Option<(usize, String, String)>>,
    target: RwSignal<Option<(usize, String)>>,
}

impl DragListCtx {
    pub fn provide(on_drop: Callback<(String, String, usize)>) {
        let origin = create_rw_signal(None);
        let target = create_rw_signal(None);
        provide_context(DragListCtx {
            origin,
            target,
            on_drop,
        });
    }
}

const TOLLERANCE: f64 = 15_f64;

#[component]
fn ListDropTarget(
    #[prop(into)] item_after: usize,
    #[prop(into)] drop_target_name: Signal<String>,
) -> impl IntoView {
    let drag_ctx = use_context::<DragListCtx>();

    if let Some(ctx) = drag_ctx {
        let el = create_node_ref::<html::Li>();

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
                ctx.target.set(Some((item_after, drop_target)))
            } else {
                ctx.target.update(|d| match d.as_ref() {
                    Some(id) => {
                        if id == &(item_after, drop_target) {
                            *d = None
                        }
                    }
                    _ => {}
                })
            }
        });

        view! {
            <Show
                when=move || {
                    let name = drop_target_name.get();
                    ctx.origin.get().filter(|f| f.0 != item_after || f.2 != name).is_some()
                }
            >
                <li
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
