use leptos::*;
use leptos_meta::*;

use crate::app::{
    components::{
        use_wk_ctx, use_wk_state, ButtonSize, ButtonView, CheckboxInputView, CheckedOption,
        DescriptionView, IconView, RadioInputView, StringInputView, WorksheetHeader,
    },
    process::inquire_personal,
    state::WorkSheets,
};

use super::inquire_inferrence;

/// step 7
#[component]
pub fn InquireView() -> impl IntoView {
    let state = use_wk_state();
    let wk_ctx = use_wk_ctx();
    let (inquery_in_progress, set_inquery_in_progress) = create_signal(false);
    let inquire_action = create_action(|wk: &WorkSheets| {
        let wk = wk.clone();
        async move { inquire_inferrence(wk).await }
    });
    let inquire_perssonal_action = create_action(|wk: &WorkSheets| {
        let wk = wk.clone();
        async move { inquire_personal(wk).await }
    });

    let prompt_option = Signal::derive(move || state.get().inquire.get().inquery_option.clone());
    let show_prompt_input =
        Signal::derive(move || prompt_option.get().get().as_str() == "inquery_4");

    let share_value = Signal::derive(move || state.get().inquire.get().personalized.clone());

    let share_option = CheckedOption {
        value: "share".to_string(),
        label: view! {
            <p class="max-w-prose whitespace-pre-line">
                {t!("worksheets.inquire.instruction_2")}
            </p>
        }
        .into_view(),
    };

    let inquery_options = (1..=4)
        .map(|i| {
            let option = format!("inquery_{i}");
            let label_name = format!("worksheets.inquire.{option}");
            CheckedOption {
                value: option.clone(),
                label: view! {
                    <p class="max-w-prose whitespace-pre-line">
                        {t!(label_name.as_str()).to_string()}
                    </p>
                }
                .into_view(),
            }
        })
        .collect::<Vec<_>>();

    let custom_prompt = Signal::derive(move || state.get().inquire.get().custom_prompt);

    let contact_name = Signal::derive(move || state.get().inquire.get().contact.get().name.clone());
    let contact_email =
        Signal::derive(move || state.get().inquire.get().contact.get().email.clone());
    let contact_message =
        Signal::derive(move || state.get().inquire.get().contact.get().message.clone());

    let on_submit = Callback::new(move |_| {
        set_inquery_in_progress.set(true);

        let inquery = state.get().inquire.get().get();
        let wk = state.get().get();
        if inquery.personalized {
            inquire_perssonal_action.dispatch(wk.clone());
        }
        inquire_action.dispatch(wk);
    });

    view! {
        <Title text={move || format!("{} | {}", t!("contact.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.inquire.title").to_string()}
            description_id="inquire"
        />
        <div class="grow w-full">
            <DescriptionView
                hidden=wk_ctx.description_hidden
                toggle_hidden=wk_ctx.toggle_description_hidden
            >
                <p class="whitespace-pre-line">
                    {t!("worksheets.inquire.description")}
                </p>
            </DescriptionView>
            <form on:submit=move |_| on_submit.call(())>
                <div class="max-w-prose mb-4 whitespace-pre-line">
                    <p>{t!("worksheets.inquire.instruction_1")}</p>
                </div>
                <RadioInputView options=inquery_options value=prompt_option />
                <Show when=move || show_prompt_input.get()>
                    <StringInputView
                        input_type="textarea"
                        value=custom_prompt
                        placeholder={t!("worksheets.inquire.placeholder").to_string()}
                    />
                </Show>
                <hr class="border-t border-slate-400 mt-4 mb-8" />
                <CheckboxInputView option=share_option value=share_value />
                <Show when=move || share_value.get().get()>
                    <label class="block my-2">
                        <p class="mb-1">{t!("contact.name.label")}</p>
                        <StringInputView
                            attr:required=true
                            attr:autocomplete="given-name"
                            input_type="text"
                            value=contact_name
                            placeholder=t!("contact.name.placeholder").to_string()
                        />
                    </label>
                    <label class="block my-2">
                        <p class="mb-1">{t!("contact.email.label")}</p>
                        <StringInputView
                            attr:required=true
                            attr:autocomplete="email"
                            input_type="email"
                            value=contact_email
                            placeholder=t!("contact.email.placeholder").to_string()
                        />
                    </label>
                    <label class="block my-2">
                        <p class="mb-1">{t!("contact.message.label")}</p>
                        <StringInputView
                            attr:required=true
                            input_type="textarea"
                            value=contact_message
                            placeholder=t!("contact.message.placeholder").to_string()
                        />
                    </label>
                </Show>
                <ButtonView
                    cta=2
                    size=ButtonSize::Lg
                    attr:type="submit"
                    on:click=move |_| on_submit.call(())
                >
                    <IconView icon="Send"/>
                    {t!("worksheets.inquire.cta")}
                </ButtonView>
            </form>
        </div>
    }
}
