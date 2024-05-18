use leptos::*;
use leptos_router::*;

use crate::app::use_lang;

#[component]
pub fn HeaderView() -> impl IntoView {
    let location = use_location();
    #[cfg_attr(feature = "ssr", allow(unused_variables))]
    let navigate = use_navigate();
    let lang = use_lang();

    let title = move || {
        let lang = format!("/{}", lang.get());

        match location.pathname.get() {
            l if l == lang => view! {
                <h1>{t!("name")}</h1>
            }
            .into_view(),
            _ => view! {
                <A href={lang} attr:class="underline hover:text-purple-800 active:text-purple-950">
                    {t!("name")}
                </A>
            }
            .into_view(),
        }
    };

    let options = move || {
        available_locales!()
            .into_iter()
            .map(|locale| {
                let lc = format!("lang_{locale}.long");
                let lc_short = format!("lang_{locale}.short");
                let label = t!(lc.as_str()).into_owned();
                let label_short = t!(lc_short.as_str(), locale = "en").into_owned();

                view! {
                    <option value={locale} selected={move || locale == lang.get().to_string().as_str()}>
                        {label}{format!(" ({label_short})")}
                    </option>
                }
            })
            .collect_view()
    };

    #[cfg_attr(feature = "ssr", allow(unused_variables))]
    let onchange_lang = move |e: ev::Event| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::JsCast;
            use web_sys::{EventTarget, HtmlSelectElement};

            let target: Option<EventTarget> = e.target();
            let select = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());

            if let Some(lang) = select.map(|s| s.value()) {
                let path = location.pathname.get();
                let it = path.split("/").skip(2);
                let next_path = it.fold(format!("/{lang}"), |acc, param| format!("{acc}/{param}"));
                navigate(next_path.as_str(), Default::default());
            }
        }
    };

    view! {
        <header class="flex justify-center bg-stone-100 dark:bg-stone-900 shadow-sm">
            <div class="max-w-screen-2xl w-full px-6 md:px-8 lg:px-16 py-3 flex flex-wrap justify-between grow shrink-0">
                {title}
                <div class="flex gap-2">
                    <A class="underline hover:text-purple-800 active:text-purple-950" exact=true href={move || format!("/{}/contact", lang.get())} >{ t!("let_talk") }</A>
                    <select
                    name="language"
                    on:change={onchange_lang}
                    value={move || lang.get().to_string()}
                    class="bg-transparent max-w-min pr-2 border-r-2 border-solid border-slate-400">
                        {options}
                    </select>
                </div>
            </div>
        </header>
    }
}
