use leptos::*;

use form_signal::{AllSignalTraits, FormSignal};

#[derive(Clone)]
pub struct CheckedOption {
    pub value: String,
    pub label: View,
}

#[component]
pub fn RadioInputView<T, Rw, R, W>(
    #[prop(into)] options: MaybeSignal<Vec<CheckedOption>>,
    #[prop(into)] value: FormSignal<T, String, Rw, R, W>,
) -> impl IntoView
where
    Rw: AllSignalTraits<T>,
    T: std::fmt::Debug + Default + PartialEq + Clone + 'static,
    R: Fn(T) -> String + Clone + 'static,
    W: Fn(&mut T, String) + Clone + 'static,
{
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
                    attr:name={move || value.id.to_string()}
                    attr:value=child.value.clone()
                    on:change={
                        let value = value.clone();
                        move |e| {
                            let val = event_target_value(&e);
                            value.set(val);
                        }
                    }
                    checked={
                        let value = value.clone();
                        move || value.get() == child.value
                    }
                    class="mt-2 ml-2 scale-150"
                />
                <div class="ml-4">
                    {child.label.into_view()}
                </div>
            </label>
        </For>
    }
}
