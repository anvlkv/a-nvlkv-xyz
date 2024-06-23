use leptos::*;
use leptos_meta::Title;
use leptos_router::ActionForm;

use crate::app::{
    components::{ButtonSize, ButtonView, ContactForm, ErrorView, IconView, Status, StatusView},
    process::InquireContact,
    state::{Completenes, ContactFormState},
    tracking::SessionId,
};

#[component]
pub fn ContactView() -> impl IntoView {
    let contact_value = create_rw_signal(ContactFormState::default());
    let (contact_value, _) = contact_value.split();
    let (loaded, set_loaded) = create_signal(false);

    create_effect(move |_| {
        set_loaded.set(true);
    });

    let disabled = Signal::derive(move || {
        let value = contact_value.get().get();
        !value.is_complete() && loaded.get()
    });

    let inquire_personal_action = create_server_action::<InquireContact>();
    let pending = inquire_personal_action.pending();
    let value = inquire_personal_action.value();
    let session_id = use_context::<SessionId>().unwrap();

    view! {
        <Title text={move || format!("{} | {}", t!("contact.title"), t!("name"))}/>
        <ActionForm action={inquire_personal_action} class="mx-auto max-w-screen-2xl px-6 md:px-8 lg:px-16 min-h-full w-full">
            <input
                attr:type="hidden"
                name="session_id"
                value={move || session_id.0.get().map(|id| id.to_string())}
            />

            <ErrorBoundary fallback=|err| view! { <ErrorView errors=err/>}>
                <div class="flex flex-col w-full items-stretch p-8 my-6 lg:my-8 bg-stone-200 dark:bg-stone-800 rounded-xl shadow">
                    <Show
                        when={move || !pending.get() && value.get().is_none()}
                        fallback=move || view!{
                            <ContactResult inquire_personal_action/>
                        }
                    >
                        <h2 class="text-xl mb-4">{t!("contact.title")}</h2>
                        <legend>
                            <p class="max-w-prose">{t!("contact.description")}</p>
                        </legend>
                        <ContactForm value={contact_value.into()}/>
                        <ButtonView
                            cta=2
                            size=ButtonSize::Lg
                            attr:type="submit"
                            attr:class="my-8"
                            disabled
                        >
                            <IconView icon="Send"/>
                            {t!("contact.send")}
                        </ButtonView>
                    </Show>
                </div>
            </ErrorBoundary>
        </ActionForm>
    }
}

#[component]
fn ContactResult(
    inquire_personal_action: Action<InquireContact, Result<String, ServerFnError<String>>>,
) -> impl IntoView {
    let pending = inquire_personal_action.pending();
    let done = inquire_personal_action.value();
    view! {
        {move || if pending.get() {
            view!{
                <StatusView
                    status=Status::Pending
                    attr:class="mb-4 mx-auto"
                />
            }.into_view()
        } else if let Some(r) = done.get() {
            view!{
                <StatusView
                    status=Status::Success
                    message={t!("contact.success.title").to_string()}
                    attr:class="mb-4 mx-auto"
                />
                <span class="hidden">
                    {r.map_err(|e| ServerFnErrorErr::from(e))}
                </span>
                <p class="max-w-prose mt-4 whitespace-pre-line">
                    {t!("contact.success.description")}
                </p>
            }.into_view()
        } else {
            ().into_view()
        }}
    }
}
