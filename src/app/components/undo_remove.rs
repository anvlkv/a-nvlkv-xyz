use std::{
    fmt::{Debug, Display},
    hash::Hash,
    time::Duration,
};

use leptos::*;
use leptos_use::{use_timestamp_with_options, UseTimestampOptions};

use crate::app::components::TOAST_CONTAINER;

#[component]
pub fn UndoRemove<T>(
    #[prop(into)] history: RwSignal<Vec<(T, usize)>>,
    #[prop(into)] on_restore: Callback<(T, usize)>,
) -> impl IntoView
where
    T: Display + Debug + Clone + PartialEq + Eq + Hash + 'static,
{
    let child_view = move |child: &(T, usize)| {
        let action = {
            let child = child.clone();
            move |_| {
                on_restore.call(child.clone());
                history.update(|h| {
                    h.retain(|v| v != &child);
                    log::debug!("history restore: {h:?}");
                });
            }
        };
        let on_timeout = {
            let child = child.clone();
            move |_| {
                history.update(|h| {
                    h.retain(|v| v != &child);
                    log::debug!("history ontimeout: {h:?}");
                })
            }
        };

        view! {
            <TimedEntry
                title=child.0.to_string()
                action
                timeout=15
                on_timeout
            />
        }
    };

    view! {
        <Show when={move || history.get().len() > 0}>
            <Portal mount=document().get_element_by_id(TOAST_CONTAINER).unwrap()>
                <For
                    each={move || history.get().into_iter().collect::<Vec<_>>()}
                    key=|state| state.clone()
                    let:child
                >
                    {
                        child_view(&child)
                    }
                </For>
            </Portal>
        </Show>
    }
}

#[component]
fn TimedEntry(
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into)] action: Callback<()>,
    #[prop(into)] timeout: i32,
    #[prop(into)] on_timeout: Callback<()>,
) -> impl IntoView {
    let cancel_timeout = set_timeout_with_handle(
        move || {
            on_timeout.call(());
        },
        Duration::from_secs(timeout as u64),
    )
    .unwrap();

    let ts = use_timestamp_with_options(UseTimestampOptions::default().interval(1000));

    let end_at = ts.get_untracked() + (timeout * 1000) as f64;

    let seconds_left = Signal::derive(move || {
        let d = (end_at - ts.get()) / 1000.0;
        (d as usize).to_string()
    });

    let title_view = {
        let title = title.clone();
        move || {
            let title = format!("⎡{}⎦", title.get());
            view! {
                <p class="mr-2 ml-0">
                    {title}
                </p>
            }
        }
    };

    on_cleanup(move || cancel_timeout.clear());

    view! {
        <div class="p-2 bg-stone-100 dark:bg-stone-900 border border-slate-400 rounded md:min-w-64 shadow-lg max-w-prose animate__animated animate__fadeInUp flex items-center">
            <button
                class="text-2xl pr-4 text-purple-800 font-bold"
                on:click={move |_| {
                    cancel_timeout.clear();
                    action.call(());
                }}
                >
                {"⤺"}
            </button>
            <p class="mr-2 ml-auto">
                {t!("util.undo_delete")}
            </p>
            <Show when=move || !title.get().is_empty()>
                {title_view.clone()}
            </Show>
            <button
                class="mr-0"
                on:click={move |_| {
                cancel_timeout.clear();
                on_timeout.call(());
                }}
            >
                {move || t!("util.time_left", seconds=seconds_left.get()).to_string()}
            </button>
        </div>
    }
}
