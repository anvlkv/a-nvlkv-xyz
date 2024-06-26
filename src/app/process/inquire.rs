use leptos::*;
use leptos_meta::*;
use strum::VariantArray;
use uuid::Uuid;

use crate::app::{
    components::{
        use_wk_ctx, use_wk_state, ButtonSize, ButtonView, CheckboxInputView, CheckedOption,
        ContactForm, DescriptionView, ErrorView, IconView, RadioInputView, ReadOnlyView, Status,
        StatusView, StringInputView, WorksheetHeader,
    },
    process::inquire_personal,
    state::{Completenes, InqueryOption, InquireWK, WorkSheets},
    tracking::SessionId,
    use_lang,
};

use super::inquire_inferrence;

/// step 7
#[component]
pub fn InquireView() -> impl IntoView {
    let state = use_wk_state();
    let wk_ctx = use_wk_ctx();
    let session_id = use_context::<SessionId>().unwrap();

    let inquire_action = create_action(|data: &(WorkSheets, Option<Uuid>)| {
        let (wk, session_id) = data.clone();
        async move {
            inquire_inferrence(wk, session_id)
                .await
                .map_err(|e| ServerFnErrorErr::from(e))
        }
    });
    let inquire_personal_action = create_action(|data: &(WorkSheets, Option<Uuid>)| {
        let (wk, session_id) = data.clone();
        let contact = wk.inquire.contact.clone();
        async move {
            inquire_personal(Some(wk), contact, session_id)
                .await
                .map_err(|e| ServerFnErrorErr::from(e))
        }
    });

    let prompt_option = Signal::derive(move || state.get().inquire.get().inquery_option.clone());
    let show_prompt_input =
        Signal::derive(move || prompt_option.get().get() == InqueryOption::Custom.to_string());

    let share_value = Signal::derive(move || state.get().inquire.get().personalized.clone());

    let share_option = Signal::derive(|| CheckedOption {
        value: "share".to_string(),
        label: view! {
            <p class="max-w-prose whitespace-pre-line">
                {t!("worksheets.inquire.instruction_2")}
            </p>
        }
        .into_view(),
    });

    let inquery_options = Signal::derive(|| {
        InqueryOption::VARIANTS
            .iter()
            .map(|opt| {
                let option = format!("inquery_{opt}");
                let label_name = format!("worksheets.inquire.{option}");
                CheckedOption {
                    value: opt.to_string(),
                    label: view! {
                        <p class="max-w-prose whitespace-pre-line">
                            {t!(label_name.as_str()).to_string()}
                        </p>
                    }
                    .into_view(),
                }
            })
            .collect::<Vec<_>>()
    });

    let custom_prompt = Signal::derive(move || state.get().inquire.get().custom_prompt);

    let contact_value = Signal::derive(move || state.get().inquire.get().contact.get());

    let on_submit = Callback::new(move |_| {
        let inquery = state.get().inquire.get().get();
        let wk = state.get().get();
        let session_id = session_id.0.get();
        if inquery.personalized {
            inquire_personal_action.dispatch((wk.clone(), session_id.clone()));
        }
        inquire_action.dispatch((
            WorkSheets {
                inquire: InquireWK {
                    contact: Default::default(),
                    ..wk.inquire
                },
                ..wk
            },
            session_id,
        ));
    });

    let disable_inquire = Signal::derive(move || {
        let wk = state.get().get();
        !wk.is_complete() || !wk.inquire.is_complete()
    });

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.inquire.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.inquire.title").to_string()}
            description_id="inquire"
        />
        <div class="grow w-full">
            <Show
                when={move || inquire_action.value().get().is_none() && !inquire_action.pending().get()}
                fallback=move || view!{
                    <InquireResult inquire_action inquire_personal_action/>
                }
            >
                <DescriptionView
                    hidden=wk_ctx.description_hidden
                    toggle_hidden=wk_ctx.toggle_description_hidden
                >
                    <p class="whitespace-pre-line">
                        {t!("worksheets.inquire.description")}
                    </p>
                </DescriptionView>
                <form on:submit=move |e| {
                    e.prevent_default();

                    on_submit.call(())
                }>
                    <div class="max-w-prose mb-4 whitespace-pre-line">
                        <p>{t!("worksheets.inquire.instruction_1")}</p>
                    </div>
                    <RadioInputView options=inquery_options value=prompt_option />
                    <Show when=move || show_prompt_input.get()>
                        <StringInputView
                            class="mt-2"
                            attr:required=true
                            input_type="textarea"
                            value=custom_prompt
                            placeholder={t!("worksheets.inquire.placeholder").to_string()}
                        />
                    </Show>
                    <hr class="border-t border-slate-400 mt-4 mb-8" />
                    <CheckboxInputView option=share_option value=share_value />
                    <Show when=move || share_value.get().get()>
                        <ContactForm value=contact_value/>
                    </Show>
                    <div class="flex w-full mt-8 justify-center">
                        <ButtonView
                            cta=2
                            size=ButtonSize::Lg
                            attr:type="submit"
                            on:click=move |e| {
                                e.prevent_default();
                                on_submit.call(())
                            }
                            disabled={disable_inquire}
                        >
                            <IconView icon="Send"/>
                            {t!("worksheets.inquire.cta")}
                        </ButtonView>
                    </div>
                </form>
            </Show>
        </div>
    }
}

#[component]
fn InquireResult(
    inquire_action: Action<(WorkSheets, Option<Uuid>), Result<String, ServerFnErrorErr<String>>>,
    inquire_personal_action: Action<
        (WorkSheets, Option<Uuid>),
        Result<(), ServerFnErrorErr<String>>,
    >,
) -> impl IntoView {
    let state = use_wk_state();
    let pending_personal = inquire_personal_action.pending();
    let pending = inquire_action.pending();
    let done_personal = inquire_personal_action.value();
    let response = inquire_action.value();
    let lang = use_lang();
    let link = Signal::derive(move || format!("/{}/process/1", lang.get()));
    let (last_chance, set_last_chance) = create_signal(false);
    let contact_value = Signal::derive(move || state.get().inquire.get().contact.get());
    let send_disabled =
        Signal::derive(move || !state.get().inquire.get().contact.get().get().is_complete());
    let session_id = use_context::<SessionId>().unwrap();

    let submit = move |e: ev::SubmitEvent| {
        e.prevent_default();
        let wk = state.get().get();
        let session_id = session_id.0.get();
        inquire_personal_action.dispatch((wk, session_id));
        set_last_chance.set(false);
    };

    create_effect(move |_| {
        window().scroll_to_with_x_and_y(0.0, 0.0);
    });

    view! {
        <ErrorBoundary fallback=|err| view! { <ErrorView errors=err/>}>
            {move || if pending.get() {
                view!{
                    <StatusView
                        status=Status::Pending
                        attr:class="mb-4 mx-auto"
                    />
                }.into_view()
            } else if let Some(r) = response.get() {
                view!{
                    <StatusView
                        status=Status::Success
                        message={t!("worksheets.inquire.ai_disclaimer").to_string()}
                        attr:class="mb-4 mx-auto"
                    />
                    <ReadOnlyView>
                        {r.clone()}
                    </ReadOnlyView>
                    <hr class="border-t border-slate-400 mt-4 mb-8"/>
                }.into_view()
            } else {
                ().into_view()
            }}
            {move || if pending_personal.get() {
                view!{
                    <StatusView
                        status=Status::Pending
                        attr:class="mb-4 mx-auto"
                    />
                }.into_view()
            } else if let Some(r) = done_personal.get() {
                view!{
                    <StatusView
                        status=Status::Success
                        message={t!("contact.success.title").to_string()}
                        attr:class="mb-4 mx-auto"
                    />
                    <span class="hidden">{r}</span>
                    <p class="max-w-prose mt-4">
                        {t!("contact.success.description")}
                    </p>
                }.into_view()
            } else {
                ().into_view()
            }}
            <Show when=move || last_chance.get()>
                <form on:submit={submit} class="flex flex-col items-stretch">
                    <h3 class="text-lg mb-4">{t!("worksheets.inquire.cta_3")}</h3>
                    <ContactForm value=contact_value/>
                    <div class="flex justify-center">
                        <ButtonView
                            cta=2
                            size=ButtonSize::Lg
                            attr:type="submit"
                            attr:class="my-8"
                            disabled={send_disabled}
                        >
                            <IconView icon="Send"/>
                            {t!("contact.send")}
                        </ButtonView>
                    </div>
                </form>
            </Show>
            <Show when={move|| !pending_personal.get() && !pending.get()}>
                <div class="flex w-full mt-8 gap-8 justify-center">
                    <ButtonView
                        cta=1
                        size=ButtonSize::Lg
                        link
                    >
                        <IconView icon="Restart"/>
                        {t!("worksheets.inquire.cta_2")}
                    </ButtonView>
                    <Show when={move || done_personal.get().is_none() && !last_chance.get()}>
                        <ButtonView
                            cta=2
                            size=ButtonSize::Lg
                            on:click={move |_| set_last_chance.set(true)}
                        >
                            <IconView icon="Send"/>
                            {t!("worksheets.inquire.cta_3")}
                        </ButtonView>
                    </Show>
                </div>
            </Show>
        </ErrorBoundary>
    }
}
