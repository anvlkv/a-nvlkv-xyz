use form_signal::{AllSignalTraits, FormSignal};
use leptos::*;

use super::CheckedOption;

#[component]
pub fn CheckboxInputView<T, Rw, R, W>(
    #[prop(into)] option: MaybeSignal<CheckedOption>,
    #[prop(into)] value: FormSignal<T, bool, Rw, R, W>,
) -> impl IntoView
where
    Rw: AllSignalTraits<T>,
    T: std::fmt::Debug + Default + PartialEq + Clone + 'static,
    R: Fn(T) -> bool + Clone + 'static,
    W: Fn(&mut T, bool) + Clone + 'static,
{
    let option = Signal::derive(move || option.get());

    view! {
        <label
            class="flex items-start mb-2"
        >
            <input
                attr:type="checkbox"
                attr:name=move || value.id.to_string()
                attr:value=move || option.get().value
                on:change={
                    let value = value.clone();
                    move |e| {
                    let next = event_target_checked(&e);
                    value.set(next);
                }}
                checked={move || value.get()}
                class="mt-2 ml-2 scale-150"
            />
            <div class="ml-4">
                {move || option.get().label.into_view()}
            </div>
        </label>
    }
}
