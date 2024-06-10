use leptos::*;
use leptos_meta::*;
use strum::VariantArray;

use crate::app::{
    components::{
        use_wk_ctx, use_wk_state, ButtonSize, ButtonView, CheckboxInputView, CheckedOption,
        ContactForm, DescriptionView, ErrorView, IconView, RadioInputView, StringInputView,
        WorksheetHeader,
    },
    process::inquire_personal,
    state::{Completenes, InqueryOption, InquireWK, WorkSheets},
};

use super::inquire_inferrence;

/// step 7
#[component]
pub fn InquireView() -> impl IntoView {
    let state = use_wk_state();
    let wk_ctx = use_wk_ctx();
    let inquire_action = create_action(|wk: &WorkSheets| {
        let wk = wk.clone();
        async move { inquire_inferrence(wk).await.map_err(|e| ServerFnErrorErr::from(e)) }
    });
    let inquire_personal_action = create_action(|wk: &WorkSheets| {
        let wk = wk.clone();
        let contact = wk.inquire.contact.clone();
        async move { inquire_personal(Some(wk), contact).await.map_err(|e| ServerFnErrorErr::from(e)) }
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
        if inquery.personalized {
            inquire_personal_action.dispatch(wk.clone());
        }
        inquire_action.dispatch(WorkSheets {
            inquire: InquireWK {
                contact: Default::default(),
                ..wk.inquire
            },
            ..wk
        });
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
    inquire_action: Action<WorkSheets, Result<String, ServerFnErrorErr<String>>>,
    inquire_personal_action: Action<WorkSheets, Result<(), ServerFnErrorErr<String>>>,
) -> impl IntoView {
    let pending_personal = inquire_personal_action.pending();
    let pending = inquire_action.pending();
    let done_personal = inquire_personal_action.value();
    let response = inquire_action.value();
    view! {
        <ErrorBoundary fallback=|err| view! { <ErrorView errors=err/>}>
            {move || if pending_personal.get() {
                view!{
                    <p class="text-lg mb-4">
                        <IconView attr:class="animate__animated animate__infinite animate__rotateIn" icon="Wait"/>
                        <span>{t!("util.pending")}</span>
                    </p>
                }.into_view()
            } else if let Some(r) = done_personal.get() {
                view!{
                    <p class="text-lg">
                        <IconView icon="Done"/>
                        <span>{t!("contact.success.title")}</span>
                        <span class="hidden">{r}</span>
                    </p>
                    <p class="max-w-prose mt-4">
                        {t!("contact.success.description")}
                    </p>
                    <hr class="border-t border-slate-400 mt-4 mb-8"/>
                }.into_view()
            } else {
                ().into_view()
            }}
            {move || if pending.get() {
                view!{
                    <p class="text-lg">
                        <IconView attr:class="animate__animated animate__infinite animate__rotateIn" icon="Wait"/>
                        <span>{t!("util.pending")}</span>
                    </p>
                }.into_view()
            } else if let Some(r) = response.get() {
                view!{
                    <p class="max-w-prose my-4 text-sm whitespace-pre-line">
                        {t!("worksheets.inquire.ai_disclaimer")}
                    </p>
                    <p class="max-w-prose whitespace-pre-line">
                        {r}
                    </p>
                }.into_view()
            } else {
                ().into_view()
            }}
        </ErrorBoundary>
    }
}
