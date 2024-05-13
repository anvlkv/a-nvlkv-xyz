use leptos::*;
use leptos_meta::*;

use crate::app::components::{ContactFormView, DescriptionView, WorksheetHeader};

/// step 7
#[component]
pub fn InquireView() -> impl IntoView {
    view! {
        <Title text={move || format!("{} | {}", t!("contact.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.inquire.title").to_string()}
            description_id="inquire"
            let:child
        >
            <DescriptionView
                hidden=child.hidden
                toggle_hidden=child.toggle_hidden
            >
                <p>{t!("worksheets.inquire.description")}</p>
            </DescriptionView>
        </WorksheetHeader>
        <ContactFormView/>
    }
}
