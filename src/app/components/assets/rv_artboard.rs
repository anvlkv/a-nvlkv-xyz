use leptos::{html::canvas, *};

#[cfg(feature = "client")]
mod rv_animation {
    use js_sys::Promise;
    use wasm_bindgen::prelude::*;
    use web_sys::Element;

    #[wasm_bindgen(module = "/src/app/components/assets/rv_artboard.mjs")]
    extern "C" {

        #[derive(Clone, PartialEq)]
        pub type RvJs;

        #[wasm_bindgen(constructor)]
        pub fn new() -> RvJs;

        #[wasm_bindgen(js_name=mountAnimation, method)]
        pub fn mount_animation(
            this: &RvJs,
            file: &str,
            name: &str,
            state_machine: &str,
            el: &Element,
            fit: &str,
            alignment: &str,
        ) -> Promise;

        #[wasm_bindgen(js_name=cleanUp, method)]
        pub fn clean_up(this: &RvJs);

        #[wasm_bindgen(js_name=setInput, method)]
        pub fn set_input(this: &RvJs, name: &str, value: &JsValue);

        #[wasm_bindgen(js_name=trigerInput, method)]
        pub fn trigger_input(this: &RvJs, name: &str);
    }
}

#[component]
#[cfg_attr(feature = "ssr", allow(unused_variables))]
pub fn RvArtboardView(
    #[prop(attrs, into)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(into)] name: String,
    #[prop(into)] state_machine: String,
    #[prop(into, default = "/pkg/anvlkv-done-button.riv".to_string())] file: String,
    #[prop(into, default = "cover".to_string())] fit: String,
    #[prop(into, default = "center".to_string())] alignment: String,
    #[prop(into, optional)] input_bool: MaybeSignal<Option<(String, bool)>>,
    #[prop(into, optional)] input_num: MaybeSignal<Option<(String, f64)>>,
    #[prop(into, optional)] input_trigger: MaybeSignal<Option<String>>,
    #[prop(into, optional)] on_loaded: Option<Callback<String>>,
) -> impl IntoView {
    let canvas_el = create_node_ref::<html::Canvas>();

    #[cfg(feature = "client")]
    {
        use futures::StreamExt;
        use wasm_bindgen_futures::JsFuture;

        let rv = create_memo(|_| rv_animation::RvJs::new());

        create_render_effect(move |_| {
            let mut stream = input_bool.to_stream();
            let rv = rv.get();
            spawn_local(async move {
                while let Some(val) = stream.next().await {
                    if let Some((name, value)) = val {
                        rv.set_input(name.as_str(), &value.into());
                        log::debug!("send input_bool: {name} - {value}");
                    }
                }
                log::debug!("listener exited: input_bool");
            });
        });

        create_render_effect(move |_| {
            let mut stream = input_num.to_stream();
            let rv = rv.get();
            spawn_local(async move {
                while let Some(val) = stream.next().await {
                    if let Some((name, value)) = val {
                        rv.set_input(name.as_str(), &value.into());
                        log::debug!("send input_num: {name} - {value}");
                    }
                }
                log::debug!("listener exited: input_num");
            });
        });

        create_render_effect(move |_| {
            let mut stream = input_trigger.to_stream();
            let rv = rv.get();
            spawn_local(async move {
                while let Some(val) = stream.next().await {
                    if let Some(name) = val {
                        rv.trigger_input(name.as_str());
                        log::debug!("send trigger_input: {name}")
                    }
                }
                log::debug!("listener exited: input_trigger");
            });
        });

        create_effect(move |_| {
            if let Some(el) = canvas_el.get().as_deref() {
                let rv = rv.get();
                let promise = rv.mount_animation(
                    file.as_str(),
                    name.as_str(),
                    state_machine.as_str(),
                    el,
                    fit.as_str(),
                    alignment.as_str(),
                );
                let on_loaded = on_loaded.clone();
                let file = file.clone();
                let name = name.clone();
                let state_machine = state_machine.clone();
                spawn_local(async move {
                    let fut = JsFuture::from(promise);
                    if fut.await.is_ok() {
                        log::debug!("animation loaded: {file}/{name}/{state_machine}");
                        if let Some(on_loaded) = on_loaded {
                            on_loaded.call(name);
                        }
                    } else {
                        log::error!("error loading animation: {file}/{name}/{state_machine}")
                    }
                });
            }
        });

        on_cleanup(move || {
            let rv = rv.get();
            rv.clean_up();
        });
    }

    canvas().attrs(attrs).node_ref(canvas_el)
}
