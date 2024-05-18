use leptos::*;
use leptos_meta::*;

use crate::app::{
    components::{use_wk_ctx, DescriptionView, WorksheetHeader},
    state::ProcessStep,
    tabs_signal,
};

/// step 6
#[component]
pub fn IterateView() -> impl IntoView {
    let tabs = tabs_signal(ProcessStep::Iterate);
    let wk_ctx = use_wk_ctx();

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.iterate.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.iterate.title").to_string()}
            description_id="iterate"
            tabs
        />
        <div class="grow w-full">
            <DescriptionView
                hidden=wk_ctx.description_hidden
                toggle_hidden=wk_ctx.toggle_description_hidden
            >
                <p class="whitespace-pre-line">
                    {t!("worksheets.iterate.description")}
                </p>
            </DescriptionView>
            <label>
                <p>{t!("worksheets.iterate.instruction_1")}</p>
                <input type="checkbox"/>
            </label>
            <label>
                <p>{t!("worksheets.iterate.instruction_2")}</p>
                <input type="checkbox"/>
            </label>
            <label>
                <p>{t!("worksheets.iterate.instruction_3")}</p>
                <input type="checkbox"/>
            </label>
            <label>
                <p>{t!("worksheets.iterate.instruction_4")}</p>
                <input type="checkbox"/>
            </label>
        </div>
    }
}
