use form_signal::FormState;
use leptos::*;

use super::CheckedOption;

#[component]
pub fn CheckboxInputView(
    #[prop(into)] option: MaybeSignal<CheckedOption>,
    #[prop(into)] value: Signal<FormState<bool>>,
) -> impl IntoView {
    let option = Signal::derive(move || option.get());

    view! {
        <label
            class="flex items-start mb-2"
        >
            <input
                attr:type="checkbox"
                attr:name=move || value.get().id.to_string()
                attr:value=move || option.get().value
                on:change={move |e| {
                    let next = event_target_checked(&e);
                    value.get().set(next);
                }}
                checked={move || value.get().get()}
                class="mt-2 ml-2 scale-150"
            />
            <div class="ml-4">
                {move || option.get().label.into_view()}
            </div>
        </label>
    }
}
