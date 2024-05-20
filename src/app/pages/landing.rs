use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::{components::RvArtboardView, use_lang};

/// step 0
#[component]
pub fn LandingView() -> impl IntoView {
    let lang = use_lang();

    #[allow(unused_variables)]
    let (button_height, set_button_height) = create_signal::<f64>(0.0);

    let button_element: NodeRef<html::Span> = create_node_ref();

    #[cfg(any(feature = "hydrate", feature = "csr"))]
    {
        use std::rc::Rc;

        use js_sys::Array;
        use wasm_bindgen::{closure::Closure, JsCast, JsValue};
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

        create_effect({
            let observer = observer.clone();
            move |_| {
                if let Some(btn) = button_element.get().as_deref() {
                    observer.observe(&btn);
                }
            }
        });

        on_cleanup(move || {
            observer.disconnect();
        });
    }

    let (hover_button, set_hover_button) = create_signal(None);
    let pointer_cb = Callback::new(move |value: bool| {
        log::debug!("set show: {value}");
        set_hover_button.set(Some(("Show".to_string(), value)))
    });

    view! {
        <Title text={move || format!("{} | {}", t!("name"), t!("specialty"))}/>
        <section class="contents">
            <div class="grow grid grid-cols-2 md:grid-cols-4 content-center">
                <div class="relative col-span-2 row-span-4 md:col-start-2 py-3 margin-0 flex flex-col-reverse justify-stretch items-stretch text-4xl sm:text-5xl md:text-6xl lg:text-8xl 2xl:text-9xl ">
                    <RvArtboardView
                        attr:id="process_animation"
                        attr:class="absolute box-border self-center bottom-0 mb-3 md:mb-7 min-w-full"
                        attr:style={move || format!("height: {}px;", button_height.get() * 2.75)}
                        name="Done"
                        state_machine="Done State Machine"
                        input_bool=hover_button
                        fit="fitHeight"
                        alignment="bottomCenter"
                    />
                    <A
                        id="the-done-button"
                        attr:type="button"
                        attr:class="contents"
                        href={move || format!("/{}/process/0", lang.get())}
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
                    <div class="flex flex-col mt-24 min-w-32 items-stretch self-center whitespace-nowrap">
                        <span class="px-16">{t!("letters.row_1")}</span>
                        <span class="px-16 text-right">{t!("letters.row_2")}</span>
                        <span class="px-16">{t!("letters.row_3")} </span>
                    </div>
                </div>
                <div id="process-intro" class="col-span-2 md:col-span-4 py-6 flex flex-col md:flex-row gap-16 text-base sm:text-lg">
                    <p class="basis-full md:basis-1/2">
                        {t!("landing.p1_s1")}{" "}
                        <A href={move || format!("/{}/process/0", lang.get())} attr:class="underline text-purple-800 dark:text-purple-200">
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
