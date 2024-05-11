use std::str::FromStr;

use form_signal::FormState;
use leptos::*;

use super::{CheckedOption, ModalView, RadioInputView};

use crate::app::state::{use_store, StorageMode};

#[cfg(any(feature = "csr", feature = "hydrate"))]
mod rv_animation {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/app/components/privacy_notice.mjs")]
    extern "C" {

        #[wasm_bindgen(js_name=privacyAnimation)]
        pub fn privacy_animation();

        #[wasm_bindgen(js_name=cleanUp)]
        pub fn clean_up();
    }
}

#[component]
pub fn PrivacyNoticeView() -> impl IntoView {
    let state = use_store();

    let when = create_read_slice(state, move |s| s.show_privacy_prompt.get());

    let (storage_option_form, _) = create_signal({
        FormState::<String>::new(
            state
                .get_untracked()
                .storage_preference
                .get_untracked()
                .unwrap_or_default()
                .to_string(),
        )
    });

    let on_resolve = move |accepted| {
        let state = state.get();
        let next = storage_option_form.get().get();

        if accepted {
            state
                .storage_preference
                .set(Some(StorageMode::from_str(next.as_str()).unwrap()));
        } else {
            let old = state.storage_preference.get();
            storage_option_form
                .get()
                .update(|o| *o = old.unwrap_or_default().to_string());
        }

        state.show_privacy_prompt.set(false);
    };

    view! {
        <ModalView when={when} curtain=true on_resolve=on_resolve>
            <PrivacyContent storage_option=storage_option_form/>
        </ModalView>
    }
}

#[component]
fn PrivacyContent(#[prop(into)] storage_option: Signal<FormState<String>>) -> impl IntoView {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        create_effect(move |_| {
            rv_animation::privacy_animation();
        });

        on_cleanup(move || {
            rv_animation::clean_up();
        });
    }

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

    view! {
        <div class="flex">
            <canvas id="privacy_animation" class="w-32 h-32 mr-4"/>
            <form>
                <p class="mb-4">{t!("privacy.notice")}</p>
                <p class="mb-4">{t!("privacy.prompt")}</p>
                <RadioInputView options={options} value={storage_option}/>
            </form>
        </div>
    }
}
