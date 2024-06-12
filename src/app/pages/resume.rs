use leptos::*;
use leptos_meta::*;

use crate::app::{
    components::ErrorView,
    resume::{get_cv_entries, CvDummyView, CvView},
    use_lang,
};

#[component]
pub fn ResumeView() -> impl IntoView {
    let lang = use_lang();
    let entries = create_resource(
        move || lang.get(),
        |lang| async move { get_cv_entries(lang).await },
    );

    view! {
        <Title text={move || format!("{} | {}", t!("cv.title"), t!("name"))}/>
        <div class="mx-auto max-w-screen-xl px-6 md:px-8 lg:px-16 min-h-full flex flex-col justify-center items-center">
            <h2 class="text-2xl mt-8 mb-4 mx-auto text-center print:hidden">{t!("cv.title")}</h2>
            <Transition fallback=CvDummyView>
                <ErrorBoundary fallback=|err| view! { <ErrorView errors=err/>}>
                    {move || match entries.get() {
                        Some(d) => {
                            let data = d.map_err(|e| ServerFnErrorErr::from(e))?;
                            Result::<View, ServerFnErrorErr<String>>::Ok(view!{
                                <CvView data/>
                            }.into_view())
                        }
                        None => {
                            Ok(
                                CvDummyView.into_view()
                            )
                        }
                    }}
                </ErrorBoundary>
            </Transition>
        </div>
    }
}
