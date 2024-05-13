use leptos::*;
use leptos_meta::*;

use crate::app::components::{DescriptionView, WorksheetHeader};

/// step 6
#[component]
pub fn IterateView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.iterate.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.iterate.title").to_string()}
            description_id="iterate"
            let:child
        >
            <DescriptionView
                hidden=child.hidden
                toggle_hidden=child.toggle_hidden
            >
                <p class="whitespace-pre-line">
                    {t!("worksheets.iterate.description")}
                </p>
            </DescriptionView>
        </WorksheetHeader>
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
    }
}
