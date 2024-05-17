use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::use_lang;

#[cfg(any(feature = "csr", feature = "hydrate"))]
mod rv_animation {
    use js_sys::Promise;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/app/pages/landing.mjs")]
    extern "C" {

        #[wasm_bindgen(js_name=landingAnimation)]
        pub fn landing_animation() -> Promise;

        #[wasm_bindgen(js_name=setActive)]
        pub fn set_active(value: bool);

        #[wasm_bindgen(js_name=cleanUp)]
        pub fn clean_up();
    }
}

/// step 0
#[component]
pub fn LandingView() -> impl IntoView {
    let lang = use_lang();

    #[allow(unused_variables)]
    let (button_height, set_button_height) = create_signal::<f64>(0.0);
    #[allow(unused_variables)]
    let (rv_ready, set_rv_ready) = create_signal(false);

    let button_element: NodeRef<html::Span> = create_node_ref();

    #[cfg(any(feature = "hydrate", feature = "csr"))]
    {
        use std::rc::Rc;

        use js_sys::Array;
        use wasm_bindgen::{closure::Closure, JsCast, JsValue};
        use wasm_bindgen_futures::JsFuture;
        use web_sys::{ResizeObserver, ResizeObserverEntry};

        let on_resize_button = Closure::wrap(Box::new(move |entries: Array, _| {
            let entry = ResizeObserverEntry::from(entries.at(0));
            let rect = entry.content_rect();
            let height = rect.height();
            set_button_height.set(height);
        }) as Box<dyn FnMut(Array, JsValue)>)
        .into_js_value();

        let observer = Rc::new(
            ResizeObserver::new(on_resize_button.as_ref().unchecked_ref())
                .expect("create observer"),
        );

        create_effect(move |_| {
            if let Some(btn) = button_element.get().as_deref() {
                observer.observe(&btn);
            }
        });

        create_effect(move |_| {
            let promise = rv_animation::landing_animation();
            spawn_local(async move {
                _ = JsFuture::from(promise).await;
                set_rv_ready.set(true);
            })
        });

        on_cleanup(move || {
            rv_animation::clean_up();
        });
    }

    #[cfg_attr(feature = "ssr", allow(unused_variables))]
    let pointer_cb = Callback::new(move |value: bool| {
        #[cfg(any(feature = "csr", feature = "hydrate"))]
        {
            rv_animation::set_active(value)
        }
    });

    view! {
        <Title text={move || format!("{} | {}", t!("name"), t!("specialty"))}/>
        <section class="contents">
            <div class="grow grid grid-cols-2 md:grid-cols-4 content-center">
                <div class="relative col-span-2 row-span-4 md:col-start-2 py-3 margin-0 flex flex-col-reverse justify-stretch items-stretch text-4xl sm:text-5xl md:text-6xl lg:text-8xl 2xl:text-9xl ">
                    <canvas id="process_animation" style={move || format!("height: {}px;", button_height.get() * 2.75)} class="absolute box-border self-center bottom-0 mb-3 md:mb-7 min-w-full"/>
                    <A
                        id="the-done-button"
                        attr:type="button"
                        href={move || format!("/{}/process/0", lang.get())}
                        class={move || if rv_ready.get() {"contents opacity-1"} else {"contents opacity-75"}}
                    >
                        <span
                            node_ref=button_element
                            on:pointerover={move |_| {
                                pointer_cb.call(true)
                            }}
                            on:pointerenter={move |_| {
                                pointer_cb.call(true)
                            }}
                            on:pointerleave={move |_| {
                                pointer_cb.call(false)
                            }}
                            class={
                                "mt-4 block md:mt-8 md:mb-4 mx-auto shrink-0 px-10 md:px-16 py-2 lg:px-20 lg:py-3 2xl:px-24 2xl:py-6 rounded-full bg-purple-900 text-stone-100 border-4 border-solid border-slate-50 drop-shadow-md text-center"
                            }
                        >
                            {t!("letters.done")}
                        </span>
                    </A>
                    <div class="flex flex-col min-w-32 items-stretch self-center whitespace-nowrap">
                        <span class="px-16">{t!("letters.row_1")}</span>
                        <span class="px-16 text-right">{t!("letters.row_2")}</span>
                        <span class="px-16">{t!("letters.row_3")} </span>
                    </div>
                </div>
                <div id="process-intro" class="col-span-2 md:col-span-4 py-6 flex flex-col md:flex-row gap-16 text-base sm:text-lg">
                    <p class="basis-full md:basis-1/2">
                        {t!("landing.p1_s1")}{" "}
                        <A href={move || format!("/{}/process/1", lang.get())} attr:class="underline text-purple-800 dark:text-purple-200">
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
