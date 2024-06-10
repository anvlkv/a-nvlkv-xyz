use form_signal::FormState;
use leptos::*;

use super::CheckedOption;

#[component]
pub fn ListSelectView(
    #[prop(into)] options: MaybeSignal<Vec<CheckedOption>>,
    #[prop(into)] value: Signal<FormState<Vec<String>>>,
    #[prop(into, optional)] max: MaybeSignal<Option<usize>>,
) -> impl IntoView {
    let on_change = move |(ev, attr_value): (ev::Event, String)| {
        let checked = event_target_checked(&ev);
        value.get().update(|val| {
            if checked {
                val.push(attr_value)
            } else {
                val.retain(|v| v != &attr_value)
            }
        });
    };

    let max = Signal::derive(move || max.get());

    view! {
        <ul>
            <For
                each=move || options.get()
                key=|state| state.value.clone()
                let:child
            >
                <ListSelectItem
                    value={child.value.clone()}
                    form={value}
                    max
                    on_change={on_change}
                >
                    {child.label.clone()}
                </ListSelectItem>
            </For>
        </ul>
    }
}

#[component]
pub fn ListSelectItem(
    #[prop(into)] form: Signal<FormState<Vec<String>>>,
    #[prop(into)] value: MaybeSignal<String>,
    #[prop(into)] max: Signal<Option<usize>>,
    #[prop(into)] on_change: Callback<(ev::Event, String)>,
    children: ChildrenFn,
) -> impl IntoView {
    let value = Signal::derive(move || value.get());
    let checked = Signal::derive(move || form.get().get().contains(&value.get()));
    let name = Signal::derive(move || form.get().id.to_string());
    let on_change = move |ev| on_change.call((ev, value.get()));

    let disabled = Signal::derive(move || {
        let val = value.get();
        let data = form.get().get();
        let len = data.len();
        let selected = data.contains(&val);
        !selected && max.get().map(|max| len >= max).unwrap_or(false)
    });

    view! {
        <li class="contents">
            <label class=move || format!("flex items-center justify-stretch w-full gap-4 {}", if disabled.get() {"opacity-50"} else {""})>
                <input
                    attr:type="checkbox"
                    disabled={disabled}
                    class="grow-0"
                    name={name}
                    value={value}
                    checked={checked}
                    on:change={on_change}
                />
                {children}
            </label>
        </li>
    }
}
