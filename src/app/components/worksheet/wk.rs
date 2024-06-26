use std::collections::HashMap;

use leptos::*;
use leptos_use::{
    signal_throttled,
    storage::{use_storage, use_storage_with_options, UseStorageOptions},
    utils::JsonCodec,
};
use strum::VariantArray;

use crate::app::{
    state::{use_store, ProcessStep, ProjectData, SeqStep, StorageMode, WorkSheets},
    use_lang, Language,
};

#[derive(PartialEq, Clone)]
pub struct Tab {
    pub title: String,
    pub href: String,
    pub is_example: bool,
}

#[derive(Clone)]
pub struct WorksheetState {
    pub wk_data: RwSignal<WorkSheets>,
    pub description_hidden: Signal<bool>,
    pub toggle_description_hidden: Callback<()>,
    pub set_current_description: WriteSignal<String>,
    pub toggle_fullscreen: Callback<()>,
    pub is_fullscreen: Signal<bool>,
}

pub const WK_STORAGE: &str = "worksheet_storage";
pub const WK_DESCRIPTION_HIDDEN_STORAGE: &str = "worksheet_description_hidden";

pub fn use_wk_ctx() -> WorksheetState {
    use_context::<WorksheetState>().unwrap()
}

#[cfg_attr(feature = "ssr", allow(unused))]
#[component]
pub fn WorksheetView(
    #[prop(into)] storage_type: StorageMode,
    #[prop(into, optional)] fs_element: Option<NodeRef<html::Div>>,
    #[prop(into, optional)] examples: Option<
        Resource<Language, Result<Vec<ProjectData>, ServerFnError<String>>>,
    >,
    children: ChildrenFn,
) -> impl IntoView {
    let (current_description, set_current_description) = create_signal(String::default());

    let (_, set_wk_storage, del_wk_storage) =
        use_storage_with_options::<Option<WorkSheets>, JsonCodec>(
            (&storage_type).into(),
            WK_STORAGE,
            UseStorageOptions::default().listen_to_storage_changes(false),
        );

    let (hidden_stored, set_hidden, del_hidden) = use_storage::<HashMap<String, bool>, JsonCodec>(
        (&storage_type).into(),
        WK_DESCRIPTION_HIDDEN_STORAGE,
    );

    let state = use_store();
    let lang = use_lang();
    // let wk_res = create_local_resource(move || state.get().wk.get(), |state| async move { state });

    let description_hidden = create_local_resource(
        move || hidden_stored.get(),
        |hidden| async move { hidden.clone() },
    );
    let description_hidden = Signal::derive(move || {
        let current = current_description.get();
        *description_hidden
            .get()
            .unwrap_or_default()
            .get(&current)
            .unwrap_or(&false)
    });

    let wk_data_throttled = signal_throttled(Signal::derive(move || state.get().wk.get()), 350.0);

    create_effect(move |_| {
        let wk = wk_data_throttled.get();
        if let Some(pref) = state
            .try_get()
            .map(|s| s.storage_preference.try_get())
            .flatten()
        {
            match pref {
                Some(StorageMode::Local) => set_wk_storage.update(|w| *w = Some(wk)),
                None => {
                    log::debug!("wk: {wk:#?}");

                    if wk != WorkSheets::default() {
                        state.get().show_privacy_prompt.set(true);
                    }
                }
                _ => {
                    del_wk_storage();
                    del_hidden();
                }
            }
        }
    });

    let on_toggle_hidden = Callback::new(move |_| {
        let current = current_description.get();
        if !current.is_empty() {
            set_hidden.update(move |s| {
                let val = !*s.get(&current).unwrap_or(&false);
                s.insert(current, val);
            })
        }
    });

    #[cfg_attr(feature = "ssr", allow(unused))]
    let (is_fullscreen, set_is_fullscreen) = create_signal(false);
    let on_tooggle_fullscreen = Callback::new(move |_| {
        if let Some(fs_element) = fs_element {
            #[cfg(feature = "client")]
            {
                let doc = document();
                if let Some(el) = doc.fullscreen_element() {
                    el.class_list().remove_1("full-screen").unwrap();
                    doc.exit_fullscreen();
                    set_is_fullscreen.set(false);
                } else if let Some(el) = fs_element.get().as_deref() {
                    match el.request_fullscreen() {
                        Ok(_) => {
                            el.class_list().add_1("full-screen").unwrap();
                            set_is_fullscreen.set(true);
                        }
                        Err(e) => {
                            log::error!("fullscreen error: {:?}", js_sys::Error::from(e));
                            set_is_fullscreen.set(false);
                        }
                    }
                } else {
                    log::warn!("not fullscreen; fullscreen element not provided");
                }
            }
        }
    });

    create_effect(move |_| {
        if let Some(examples) = examples {
            let lang = lang.get();
            if let Some(examples) = examples.get().map(|d| d.ok()).flatten() {
                state.update(|s| {
                    s.examples = examples;
                    s.sequence = vec![];
                    make_sequence(&mut s.sequence, &s.examples, lang);
                });
            } else {
                state.update(|s| {
                    s.sequence = ProcessStep::VARIANTS
                        .iter()
                        .enumerate()
                        .map(|(i, step)| SeqStep {
                            href: format!("/{}/process/{}", lang, i),
                            process_step: *step,
                            example: None,
                        })
                        .collect();
                });
            }
        }
    });

    move || {
        let wk = state.get().wk;
        let children = children.clone();
        log::debug!("render wk inner");
        view! {
            <WorkSheetProvider
                wk_data=wk
                toggle_description_hidden=on_toggle_hidden
                toggle_fullscreen=on_tooggle_fullscreen
                description_hidden
                set_current_description
                is_fullscreen
            >
                <div id="worksheets-root" class="flex flex-col">
                    {children()}
                </div>
            </WorkSheetProvider>
        }
    }
}

#[component]
fn WorkSheetProvider(
    #[prop(into)] wk_data: RwSignal<WorkSheets>,
    #[prop(into)] description_hidden: Signal<bool>,
    #[prop(into)] toggle_description_hidden: Callback<()>,
    #[prop(into)] set_current_description: WriteSignal<String>,
    #[prop(into)] toggle_fullscreen: Callback<()>,
    #[prop(into)] is_fullscreen: Signal<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    provide_context(WorksheetState {
        wk_data,
        description_hidden,
        toggle_description_hidden,
        toggle_fullscreen,
        set_current_description,
        is_fullscreen,
    });
    log::debug!("WorkSheetProvider");
    children.into_view()
}

fn make_sequence(seq: &mut Vec<SeqStep>, examples: &Vec<ProjectData>, lang: Language) {
    // about
    seq.push(SeqStep {
        href: format!("/{}/process/{}", lang, 0),
        process_step: ProcessStep::About,
        example: None,
    });

    // all worksheets first example
    examples.first().iter().for_each(|ex| {
        seq.extend(
            ProcessStep::VARIANTS
                .iter()
                .enumerate()
                .filter_map(|(i, step)| {
                    if i > 0 && i < ProcessStep::VARIANTS.len() - 2 {
                        Some(SeqStep {
                            href: format!("/{}/process/{}/{}", lang, i, ex.id),
                            process_step: *step,
                            example: Some(ex.id.clone()),
                        })
                    } else {
                        None
                    }
                }),
        );
    });

    // each workshet examples
    seq.extend(
        ProcessStep::VARIANTS
            .iter()
            .enumerate()
            .fold(vec![], |mut acc, (i, step)| {
                if i > 0 && i < ProcessStep::VARIANTS.len() - 2 {
                    // example
                    acc.extend(examples.iter().skip(1).map(|ex| SeqStep {
                        href: format!("/{}/process/{}/{}", lang, i, ex.id),
                        process_step: *step,
                        example: Some(ex.id.clone()),
                    }));
                    // worksheet
                    acc.push(SeqStep {
                        href: format!("/{}/process/{}", lang, i),
                        process_step: *step,
                        example: None,
                    });
                }
                acc
            }),
    );

    // iterate
    seq.push(SeqStep {
        href: format!("/{}/process/{}", lang, 5),
        process_step: ProcessStep::Iterate,
        example: None,
    });

    // inquire
    seq.push(SeqStep {
        href: format!("/{}/process/{}", lang, 6),
        process_step: ProcessStep::Inquire,
        example: None,
    });
}
