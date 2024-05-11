use leptos::*;
use serde::{Deserialize, Serialize};
use strum::Display;

use form_signal::FormState;

use super::{CheckedOption, ModalView, RadioInputView};

#[derive(Serialize, Deserialize, Default, Display)]
pub enum StorageMode {
    #[default]
    #[strum(to_string = "local")]
    Local,
    #[strum(to_string = "session")]
    Session,
}

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
    let (storage, set_storage) = create_signal::<Option<StorageMode>>(None);

    let (storage_option, _) = create_signal(FormState::new(StorageMode::Local.to_string()));

    let when = Signal::derive(|| true);

    let on_resolve = move |accepted| {
        if accepted {
        } else {
        }
    };

    view! {
        <ModalView when={when} curtain=true on_resolve=on_resolve>
            <PrivacyContent storage_option/>
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
                value: StorageMode::Session.to_string(),
                label: view! {
                    <p class="font-lg font-bold">{t!("privacy.session")}</p>
                    <p>{t!("privacy.session_description")}</p>
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
