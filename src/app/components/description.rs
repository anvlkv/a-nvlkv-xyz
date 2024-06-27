use leptos::*;

use crate::app::components::IconView;

#[component]
pub fn DescriptionView(
    #[prop(into, optional)] toggle_hidden: Option<Callback<()>>,
    #[prop(into, optional)] hidden: Signal<bool>,
    #[prop(into, optional)] alternative: MaybeSignal<usize>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = move || {
        format!(
            "flex items-start -ml-4 mb-3 rounded-lg max-w-prose lg:w-fit p-4 {} text-lg",
            match alternative.get() {
                2 => "bg-indigo-200 dark:bg-indigo-950 border border-indigo-400 dark:brder-indigo-800 text-indigo-800 dark:text-indigo-200",
                1 => "bg-emerald-200 dark:bg-emerald-950 border border-emerald-400 dark:brder-emerald-800 text-emerald-800 dark:text-emerald-200",
                _ => "bg-sky-200 dark:bg-sky-950 border border-sky-400 dark:brder-sky-800 text-sky-800 dark:text-sky-200"
            }
        )
    };

    view! {
        <Show when={move || !hidden.get()}>
            <div class=class>
                <div>
                    {children()}
                </div>
                {move || if let Some(toggle_hidden) = toggle_hidden {
                  view!{
                      <button
                          on:click={move |_| toggle_hidden(())}
                          title=t!("util.close")
                          class="ml-1 mr-0 -mt-0.5 text-sm"
                      >
                          <IconView
                              icon="Close"
                          />
                      </button>
                  }.into_view()
                } else {
                    ().into_view()
                }}
            </div>
        </Show>
    }
}
