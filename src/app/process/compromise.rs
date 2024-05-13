use leptos::*;
use leptos_meta::*;

use crate::app::components::{DescriptionView, WorksheetHeader};

/// step 4
#[component]
pub fn CompromiseView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.compromise.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.compromise.title").to_string()}
            description_id="compromise"
            let:child
        >
            <DescriptionView
                hidden=child.hidden
                toggle_hidden=child.toggle_hidden
            >
                <p>{t!("worksheets.compromise.description")}</p>
            </DescriptionView>
        </WorksheetHeader>
        <div class="max-w-prose">
            <p>{t!("worksheets.compromise.instruction_1")}</p>
        </div>
        <div class="grid grid-cols-2 text-center">
            <h4>{t!("worksheets.compromise.label_solutions")}</h4>
            <h4>{t!("worksheets.compromise.label_stakeholders")}</h4>
        </div>
        <div class="max-w-prose">
            <p>{t!("worksheets.compromise.instruction_2")}</p>
        </div>
        <label>
            <p>{t!("worksheets.compromise.label_question")}</p>
            <textarea/>
        </label>
    }
}
