use leptos::*;

use super::IconView;

#[derive(Clone, Copy, Debug)]
pub enum Status {
    Pending,
    Success,
}

#[component]
pub fn StatusView(
    #[prop(into)] status: MaybeSignal<Status>,
    #[prop(into, optional)] message: MaybeSignal<String>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let icon = move || match status.get() {
        Status::Pending => view! {
            <IconView
                attr:class="inline-block animate__animated animate__infinite animate__flip animate__slow"
                attr:style="--animate-delay:0.5s; animation-direction: alternate-reverse;"
                icon="Wait"
            />
        },
        Status::Success => view! {
            <IconView
                attr:class="inline-block animate__animated animate__fadeInUp dark:text-emerald-400 text-emerald-600"
                attr:style="--animate-delay:0.75s;"
                icon="Done"
            />
        },
    };

    let message = move || {
        if message.get().is_empty() {
            match status.get() {
                Status::Pending => view! {
                    <p>{t!("util.pending")}</p>
                },
                Status::Success => view! {
                    <p>{t!("util.done")}</p>
                },
            }
        } else {
            let msg = message.get();
            view! {
                <p>{msg}</p>
            }
        }
    };

    html::div()
        .attrs(attrs)
        .dyn_classes(move || {
            vec![
                "text-lg flex items-center gap-4 justify-center p-4 rounded border w-fit"
                    .to_string(),
                match status.get() {
                    Status::Pending => {
                        "bg-amber-200 border-amber-400 dark:bg-amber-950 dark:border-amber-800"
                            .to_string()
                    }
                    Status::Success => {
                        "bg-purple-200 border-purple-400 dark:bg-purple-950 dark:border-purple-800"
                            .to_string()
                    }
                },
            ]
        })
        .child(view! {
            <div class="overflow-visible shrink-0">
                {icon}
            </div>
        })
        .child(message)
}
