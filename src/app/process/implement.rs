use leptos::*;
use leptos_meta::*;

use crate::app::components::{DescriptionView, WorksheetHeader};

/// step 5
#[component]
pub fn ImplementView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.implement.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.implement.title").to_string()}
            description_id="implement"
            let:child
        >
            <DescriptionView
                hidden=child.hidden
                toggle_hidden=child.toggle_hidden
            >
                <p class="pb-4">{t!("worksheets.implement.description_1")}</p>
                <p>{t!("worksheets.implement.description_2")}</p>
            </DescriptionView>
        </WorksheetHeader>
        <div class="max-w-prose">
            <p>{t!("worksheets.implement.instruction")}</p>
        </div>
        <div class="grid grid-cols-2 text-center">
            <div>
                <h4>{t!("worksheets.implement.label_col_1")}</h4>
                <p>{t!("worksheets.implement.hint_col_1")}</p>
            </div>
            <div>
                <h4>{t!("worksheets.implement.label_col_2")}</h4>
                <p>{t!("worksheets.implement.hint_col_2")}</p>
            </div>
        </div>
    }
}
