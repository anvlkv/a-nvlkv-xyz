use leptos::*;

use form_signal::FormState;

#[derive(Clone)]
pub struct CheckedOption {
    pub value: String,
    pub label: View,
}

#[component]
pub fn RadioInputView(
    #[prop(into)] options: MaybeSignal<Vec<CheckedOption>>,
    #[prop(into)] value: Signal<FormState<String>>,
) -> impl IntoView {
    view! {
        <For each=move || options.get()
            key=|state| state.value.clone()
            let:child
        >
            <label
                class="flex items-start mb-2"
            >
                <input
                    attr:type="radio"
                    attr:name={move || value.get().id.to_string()}
                    attr:value=child.value.clone()
                    on:change={move |e| {
                        let val = event_target_value(&e);
                        value.get().set(val);
                    }}
                    checked={move || value.get().get() == child.value}
                    class="mt-2 scale-150"
                />
                <div class="ml-4">
                    {child.label.into_view()}
                </div>
            </label>
        </For>
    }
}
