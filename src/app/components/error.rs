use leptos::*;

#[component]
pub fn ErrorTemplate(errors: RwSignal<Errors>) -> impl IntoView {
    view! {
        <For each=move || errors.get()
            key=|state| state.0.clone()
            let:child
        >
            <p>{format!("{:?}", child.1)}</p>
        </For>
    }
}
