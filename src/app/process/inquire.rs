use leptos::*;
use leptos_meta::*;

use crate::app::components::{use_wk_ctx, ContactFormView, DescriptionView, WorksheetHeader};

/// step 7
#[component]
pub fn InquireView() -> impl IntoView {
    let wk_ctx = use_wk_ctx();
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
                <p>{t!("worksheets.inquire.description")}</p>
            </DescriptionView>
            <ContactFormView/>
        </div>
    }
}
