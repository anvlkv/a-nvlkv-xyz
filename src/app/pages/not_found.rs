use leptos::*;
use leptos_meta::*;

#[component]
pub fn NotFound() -> impl IntoView {
    // #[cfg(feature = "ssr")]
    // {
    //     let resp = expect_context::<leptos_spin::ResponseOptions>();
    //     resp.set_status(404);
    // }

    view! {
        <Title text={move || t!("not_found")}/>
        <h1>"Not Found"</h1>
    }
}
