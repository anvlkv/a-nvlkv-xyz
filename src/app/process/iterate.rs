use form_signal::FormState;
use leptos::*;
use leptos_meta::*;
use leptos_router::use_navigate;
use uuid::Uuid;
use web_time::Instant;

use crate::app::{
    components::{
        use_wk_ctx, use_wk_state, ButtonSize, ButtonView, DescriptionView, HistoryEntry, IconView,
        ListInputView, ModalView, StringInputView, UndoRemove, WorksheetHeader,
    },
    process::{
        FixedBestList, FixedNowList, FixedProblemStatement, FixedQuestionStatement,
        FixedSolutionsChoice, FixedStakeholdersChoice,
    },
    state::Completenes,
    use_lang,
};

/// step 6
#[component]
pub fn IterateView() -> impl IntoView {
    let wk_ctx = use_wk_ctx();
    let state = use_wk_state();
    let lang = use_lang();
    let download_link = move || format!("/{}/process/download", lang.get());

    let title = Signal::derive(move || state.get().iterate.get().title);
    let start_date = Signal::derive(move || state.get().iterate.get().start_date);
    let end_date = Signal::derive(move || state.get().iterate.get().end_date);
    let resources = Signal::derive(move || state.get().iterate.get().resources);
    let external_resources = Signal::derive(move || state.get().iterate.get().external_resources);

    let resources_delete_history = create_rw_signal(vec![]);
    let externals_delete_history = create_rw_signal(vec![]);

    let externals_value_add = move |(next, index): (String, Option<usize>)| {
        let next = FormState::new(next);
        let id = next.id;
        state.get().iterate.update(move |p| {
            p.external_resources
                .insert(index.unwrap_or(p.external_resources.len()), next);
        });
        id
    };
    let resources_value_add = move |(next, index): (String, Option<usize>)| {
        let next = FormState::new(next);
        let id = next.id;
        state.get().iterate.update(move |p| {
            p.resources.insert(index.unwrap_or(p.resources.len()), next);
        });
        id
    };
    let externals_value_remove = move |id: Uuid| {
        state.get().iterate.update(move |p| {
            let i = p
                .external_resources
                .iter()
                .position(|v| v.id == id)
                .unwrap();
            let removed = p.external_resources.remove(i).get_untracked();
            externals_delete_history.update(|h| h.push((removed, i, Instant::now())));
        })
    };
    let resources_value_remove = move |id: Uuid| {
        state.get().iterate.update(move |p| {
            let i = p.resources.iter().position(|v| v.id == id).unwrap();
            let removed = p.resources.remove(i).get_untracked();
            resources_delete_history.update(|h| h.push((removed, i, Instant::now())));
        })
    };
    let externals_restore = move |(val, at, _): HistoryEntry<String>| {
        state.get().iterate.update(move |p| {
            if p.external_resources.len() >= at {
                p.external_resources.insert(at, FormState::new(val));
            } else {
                p.external_resources.push(FormState::new(val));
            }
        })
    };
    let resources_restore = move |(val, at, _): HistoryEntry<String>| {
        state.get().iterate.update(move |p| {
            if p.resources.len() >= at {
                p.resources.insert(at, FormState::new(val));
            } else {
                p.resources.push(FormState::new(val));
            }
        })
    };

    let min_start = Signal::derive(move || chrono::Local::now().date_naive().to_string());
    let min_end = Signal::derive(move || {
        let start = start_date.get().get();
        if start.is_empty() {
            min_start.get()
        } else {
            start
        }
    });

    let (show_download, set_show_download) = create_signal(false);

    let disable_download = Signal::derive(move || !state.get().get().is_complete());

    let on_download = move |_| {
        set_show_download.set(true);
    };

    let navigate = use_navigate();
    let on_download_complete = Callback::new(move |_| {
        set_show_download.set(false);
        let link = format!("/{}/process/6", lang.get());
        navigate(link.as_str(), Default::default());
    });

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.iterate.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.iterate.title").to_string()}
            description_id="iterate"
        />
        <div class="grow w-full">
            <DescriptionView
                hidden=wk_ctx.description_hidden
                toggle_hidden=wk_ctx.toggle_description_hidden
            >
                <p class="whitespace-pre-line">
                    {t!("worksheets.iterate.description")}
                </p>
            </DescriptionView>
            <form>
                <FixedProblemStatement/>
                <FixedSolutionsChoice/>
                <FixedStakeholdersChoice/>
                <FixedQuestionStatement/>
                <FixedNowList/>
                <FixedBestList/>
                <div class="max-w-prose mb-4 mt-6 whitespace-pre-line">
                    <p>{t!("worksheets.iterate.instruction_title")}</p>
                </div>
                <label class="block mb-4 mt-2">
                    <p class="mb-2">{t!("worksheets.iterate.label_title")}</p>
                    <StringInputView
                        input_type="text"
                        value=title
                        placeholder={t!("worksheets.iterate.placeholder_title").to_string()}
                    />
                </label>
                <div class="max-w-prose mb-4 whitespace-pre-line">
                    <p>{t!("worksheets.iterate.instruction_resources")}</p>
                </div>
                <div class="grid lg:grid-cols-2 gap-6 mb-4">
                    <div>
                        <h4 class="text-xl mb-4 w-full text-center">
                            {t!("worksheets.iterate.label_resources")}
                        </h4>
                        <ListInputView
                            input_type="text"
                            data=resources
                            add_value=resources_value_add
                            remove_value=resources_value_remove
                            add_entry_text={t!("worksheets.iterate.add_resource").to_string()}
                            placeholder={t!("worksheets.iterate.placeholder_resources").to_string()}
                        />
                    </div>
                    <div>
                        <h4 class="text-xl mb-4 w-full text-center">
                            {t!("worksheets.iterate.label_externals")}
                        </h4>
                        <ListInputView
                            input_type="text"
                            data=external_resources
                            add_value=externals_value_add
                            remove_value=externals_value_remove
                            add_entry_text={t!("worksheets.iterate.add_external").to_string()}
                            placeholder={t!("worksheets.iterate.placeholder_external").to_string()}
                        />
                    </div>
                </div>
                <div class="max-w-prose mb-4 whitespace-pre-line">
                    <p>{t!("worksheets.iterate.instruction_dates")}</p>
                </div>
                <div class="grid lg:grid-cols-2 gap-6 mb-4">
                    <label>
                        <p class="mb-2">{t!("worksheets.iterate.label_date_1")}</p>
                        <StringInputView
                            input_type="date"
                            value=start_date
                            attr:min=min_start
                            placeholder={t!("worksheets.iterate.placeholder_date_1").to_string()}
                        />
                    </label>
                    <label>
                        <p class="mb-2">{t!("worksheets.iterate.label_date_2")}</p>
                        <StringInputView
                            input_type="date"
                            value=end_date
                            attr:min=min_end
                            placeholder={t!("worksheets.iterate.placeholder_date_2").to_string()}
                        />
                    </label>
                </div>
            </form>
            <div class="flex w-full mt-8 justify-center">
                <ButtonView
                    cta=2
                    size=ButtonSize::Lg
                    on:click={on_download}
                    disabled={disable_download}
                >
                    <IconView icon="Download"/>
                    {t!("worksheets.iterate.cta")}
                </ButtonView>
            </div>
        </div>
        <ModalView
            curtain=true
            when=show_download
            on_resolve=on_download_complete
        >
            <h3 class="text-xl mb-2">{t!("worksheets.iterate.cta")}</h3>
            <DescriptionView>
                <p class="whitespace-pre-line">
                    <IconView icon="Info"/>
                    {t!("util.browser_pdf")}
                </p>
            </DescriptionView>
            <iframe src={download_link} class="w-[50vw] rounded-lg aspect-video"/>
        </ModalView>
        <UndoRemove
            history=resources_delete_history
            on_restore=resources_restore
        />
        <UndoRemove
            history=externals_delete_history
            on_restore=externals_restore
        />
    }
}
