use std::collections::HashMap;

use leptos::*;
use leptos_router::*;
use leptos_use::{on_click_outside, use_document, use_event_listener};

use crate::app::{components::IconView, util::transform_xata_image, STYLED_ROOT};

#[component]
pub fn PictureView(
    #[prop(into)] src: MaybeSignal<String>,
    #[prop(into)] alt: MaybeSignal<String>,
    #[prop(attrs, into)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(into, optional)] node_ref: NodeRef<html::Picture>,
) -> impl IntoView {
    let srcsets = create_memo({
        let src = src.clone();
        move |_| {
            let src = src.get();
            [640, 768, 1024, 1280, 1536]
                .into_iter()
                .map(|w| {
                    let width = w.to_string();
                    let srcset = transform_xata_image(
                        src.as_str(),
                        HashMap::from_iter(vec![
                            ("width", width.as_str()),
                            ("fit", "scale-down"),
                            ("format", "webp"),
                        ]),
                    );

                    view! {
                        <source
                            media={format!("(min-width: {w}px)")}
                            srcset=srcset/>
                    }
                })
                .collect_view()
        }
    });

    html::picture()
        .child(srcsets)
        .child(html::img().attrs(attrs).attr("src", src).attr("alt", alt))
        .node_ref(node_ref)
        .attr("class", "contents")
}

#[component]
pub fn PictureModalView(
    #[prop(into)] src: MaybeSignal<String>,
    #[prop(into)] alt: MaybeSignal<String>,
    #[prop(into)] thumbnail_size: i32,
    #[prop(into, optional)] thumbnail_class: MaybeSignal<String>,
) -> impl IntoView {
    let (show, set_show) = create_signal(false);
    let src = Signal::derive(move || src.get());
    let alt = Signal::derive(move || alt.get());
    let target = create_node_ref::<html::Picture>();
    let clear_click_listener = on_click_outside(target, move |_| {
        if show.get() {
            set_show.set(false)
        }
    });

    let on_click_thumbnail = move |e: ev::MouseEvent| {
        e.prevent_default();
        set_show.set(true);
    };
    let thumbnail_src = create_memo(move |_| {
        transform_xata_image(
            src.get().as_str(),
            HashMap::from_iter(vec![
                ("width", thumbnail_size.to_string().as_str()),
                ("height", thumbnail_size.to_string().as_str()),
                ("fit", "cover"),
                ("format", "webp"),
            ]),
        )
    });

    let clear_kbd_listener = use_event_listener(use_document(), ev::keydown, move |evt| {
        if show.get() {
            if evt.key().to_lowercase() == "escape".to_string() {
                set_show.set(false);
            }
        }
    });

    on_cleanup(move || {
        clear_kbd_listener();
        clear_click_listener();
    });

    create_render_effect(move |_| {
        if let Some(root_el) = document().get_element_by_id(STYLED_ROOT) {
            if show.get() {
                root_el.class_list().add_1("blur").unwrap();
            } else {
                root_el.class_list().remove_1("blur").unwrap();
            }
        }
    });

    view! {
        <A
            href=move || src.get()
            on:click=on_click_thumbnail
            class={thumbnail_class}
        >
            <img src=thumbnail_src alt=alt/>
        </A>
        <Show when=move || show.get()>
            <Portal>
                <div
                    role="presentation"
                    class="absolute min-w-screen min-h-dvh h-full w-full bg-gray-950 opacity-75 top-0 left-0"
                ></div>
                <div
                    class="fixed top-0 left-0 min-w-screen min-h-dvh w-full h-full flex justify-center items-center font-sans text-slate-950 dark:text-slate-50 pt-10 lg:p-15"
                >
                    <button
                        on:click=move |e| {
                            e.prevent_default();
                            set_show.set(false);
                        }
                        class="text-lg absolute top-10 right-10 text-gray-50"
                        title=t!("util.close")
                    >
                        <IconView icon="Close"/>
                    </button>
                    <PictureView
                        src=src
                        alt=alt
                        attr:class="max-h-dvh w-auto max-w-full animate__animated animate__fadeInDown"
                        node_ref=target
                    />
                </div>
            </Portal>
        </Show>
    }
}
