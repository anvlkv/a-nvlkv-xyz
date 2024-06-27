use std::str::FromStr;

use form_signal::FormSignal;
use leptos::*;

use super::{CheckedOption, ModalView, RadioInputView};

use crate::app::{
    components::RvArtboardView,
    state::{use_store, StorageMode},
};

#[component]
pub fn PrivacyNoticeView() -> impl IntoView {
    let state = use_store();

    let when = create_read_slice(state, move |s| s.show_privacy_prompt.get());

    let storage_option_form = create_rw_signal(
        state
            .get_untracked()
            .storage_preference
            .get_untracked()
            .unwrap_or_default()
            .to_string(),
    );

    let on_resolve = Callback::new(move |accepted| {
        let next = storage_option_form.get();
        let state = state.get();

        if accepted {
            state
                .storage_preference
                .set(Some(StorageMode::from_str(next.as_str()).unwrap()))
        } else {
            let old = state.storage_preference.get();
            storage_option_form.update(|o| *o = old.unwrap_or_default().to_string());
        }

        state.show_privacy_prompt.set(false);
    });

    view! {
        <ModalView
            when={when}
            curtain=true
            on_resolve=on_resolve
            cancel_btn=true
        >
            <PrivacyContent storage_option=storage_option_form/>
        </ModalView>
    }
}

#[component]
fn PrivacyContent(#[prop(into)] storage_option: RwSignal<String>) -> impl IntoView {
    let options = Signal::derive(|| {
        vec![
            CheckedOption {
                value: StorageMode::Local.to_string(),
                label: view! {
                    <p class="font-lg font-bold">{t!("privacy.local")}</p>
                    <p>{t!("privacy.local_description")}</p>
                }
                .into_view(),
            },
            CheckedOption {
                value: StorageMode::None.to_string(),
                label: view! {
                    <p class="font-lg font-bold">{t!("privacy.no_storage")}</p>
                    <p>{t!("privacy.no_storage_description")}</p>
                }
                .into_view(),
            },
        ]
    });

    let storage_option = FormSignal::new(storage_option, |v| v.clone(), |v, val| *v = val);

    view! {
        <div class="flex flex-col items-center lg:flex-row max-w-prose">
            <RvArtboardView
                attr:class="w-32 h-32 mr-4"
                state_machine="Privacy State Machine"
                name="Privacy"
            />
            <form>
                <p class="mb-4">{t!("privacy.notice")}</p>
                <p class="mb-4">{t!("privacy.prompt")}</p>
                <RadioInputView options={options} value={storage_option}/>
                <p>
                    {t!("privacy.link_before")}
                    <a
                        target="_blank"
                        class="underline"
                        href="https://github.com/anvlkv/a-nvlkv-xyz#privacy"
                    >
                        {t!("privacy.link")}
                    </a>
                    {t!("privacy.link_after")}
                </p>
            </form>
        </div>
    }
}
