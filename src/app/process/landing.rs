use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::Language;

/// step 0
#[component]
pub fn LandingView() -> impl IntoView {
    // let onclick = props.on_click.clone();
    // let LocaleLang(selected_lang) = use_context::<LocaleLang>().unwrap();
    // rust_i18n::set_locale(selected_lang.as_str());
    let lang = use_context::<Signal<Language>>().unwrap();

    view! {
        <Title text={move || format!("{} | {}", t!("name"), t!("specialty"))}/>
        <section>
            <div class="grid grid-cols-2 md:grid-cols-4 content-center">
                <div class="relative col-span-2 row-span-4 md:col-start-2 py-3 margin-0 flex flex-col-reverse justify-stretch items-stretch text-4xl sm:text-5xl md:text-6xl lg:text-8xl 2xl:text-9xl ">
                    <button id="the-done-button" class="mt-4 md:mt-8 md:mb-4 mx-auto shrink-0 px-10 md:px-16 py-2 lg:px-20 lg:py-3 2xl:px-24 2xl:py-6 rounded-full bg-purple-900 text-stone-100 border-4 border-solid border-slate-50 drop-shadow-md">{t!("letters.done")}</button>
                    <div class="flex flex-col min-w-32 items-stretch self-center whitespace-nowrap">
                        <span class="px-16">{t!("letters.row_1")}</span>
                        <span class="px-16 text-right">{t!("letters.row_2")}</span>
                        <span class="px-16">{t!("letters.row_3")} </span>
                    </div>
                    <div id="emojies" class="absolute left-0">
                        <span title="research" >{"🗒"}</span>
                        <span title="ideate" >{"💡"}</span>
                        <span title="compromise" >{"📌"}</span>
                        <span title="implement" >{"🛠"}</span>
                        <span title="iterate" >{"♻️"}</span>
                        <span title="lotus" >{"🪷"}</span>
                    </div>
                </div>
                <div id="process-intro" class="col-span-2 md:col-span-4 py-6 flex flex-col md:flex-row gap-16 text-base sm:text-lg">
                    <p class="basis-full md:basis-1/2">
                        {t!("landing.p1_s1")}{" "}
                        <A href={move || format!("/{}/1", lang.get().0)} attr:class="underline text-purple-800 dark:text-purple-200">
                            {t!("landing.p1_link")}
                        </A>
                        {" "}{t!("landing.p1_s2")}
                        <br/>
                        {t!("landing.p1_s3")}
                    </p>
                    <p class="basis-full md:basis-1/2">
                        {t!("landing.p2_s1")}
                        <br/>
                        {t!("landing.p2_s2")}
                    </p>
                </div>
            </div>
        </section>
    }
}
