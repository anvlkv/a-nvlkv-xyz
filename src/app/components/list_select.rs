use form_signal::{AllSignalTraits, FormSignal};
use leptos::*;

use crate::app::components::ReadOnlyView;

use super::CheckedOption;

#[component]
pub fn ListSelectView<T, Rw, R, W>(
    #[prop(into)] options: Signal<Vec<CheckedOption>>,
    #[prop(into)] value: FormSignal<T, Vec<String>, Rw, R, W>,
    #[prop(into, optional)] max: MaybeSignal<Option<usize>>,
) -> impl IntoView
where
    Rw: AllSignalTraits<T>,
    T: std::fmt::Debug + Default + PartialEq + Clone + 'static,
    R: Fn(T) -> Vec<String> + Clone + 'static,
    W: Fn(&mut T, Vec<String>) + Clone + 'static,
{
    let on_change = Callback::new({
        let value = value.clone();
        move |(ev, attr_value): (ev::Event, String)| {
            let checked = event_target_checked(&ev);
            value.update(|val| {
                if checked {
                    val.push(attr_value)
                } else {
                    val.retain(|v| v != &attr_value)
                }
            });
        }
    });

    let max = Signal::derive(move || max.get());

    let not_in_scope = Signal::derive({
        let value = value.clone();
        move || {
            let options = options.get();
            value
                .get()
                .into_iter()
                .filter(|v| options.iter().find(|o| &o.value == v).is_none())
                .map(|v| CheckedOption {
                    value: v.clone(),
                    label: view! {
                        <ReadOnlyView>
                            <span class="line-through">{v.clone()}</span>
                        </ReadOnlyView>
                    },
                })
                .collect::<Vec<_>>()
        }
    });

    view! {
        <ul>
            <For
                each=move || not_in_scope.get()
                key=|state| state.value.clone()
                let:child
                clone:value
            >
                <ListSelectItem
                    value={child.value.clone()}
                    form={value.clone()}
                    max
                    on_change={on_change.clone()}
                >
                    {child.label.clone()}
                </ListSelectItem>
            </For>
            <For
                each=move || options.get()
                key=|state| state.value.clone()
                let:child
            >
                <ListSelectItem
                    value={child.value.clone()}
                    form={value.clone()}
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
pub fn ListSelectItem<T, Rw, R, W>(
    #[prop(into)] form: FormSignal<T, Vec<String>, Rw, R, W>,
    #[prop(into)] value: MaybeSignal<String>,
    #[prop(into)] max: Signal<Option<usize>>,
    #[prop(into)] on_change: Callback<(ev::Event, String)>,
    children: ChildrenFn,
) -> impl IntoView
where
    Rw: AllSignalTraits<T>,
    T: std::fmt::Debug + Default + PartialEq + Clone + 'static,
    R: Fn(T) -> Vec<String> + Clone + 'static,
    W: Fn(&mut T, Vec<String>) + Clone + 'static,
{
    let value = Signal::derive(move || value.get());
    let checked = Signal::derive({
        let form = form.clone();
        move || form.get().contains(&value.get())
    });
    let name = Signal::derive(move || form.id.to_string());
    let on_change = move |ev| on_change((ev, value.get()));

    let disabled = Signal::derive(move || {
        let val = value.get();
        let data = form.get();
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
