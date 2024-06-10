use std::str::FromStr;

use leptos::*;
use strum::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString)]
pub enum Icon {
    Close,
    Delete,
    Done,
    Drag,
    Example,
    Info,
    Next,
    Prev,
    Restart,
    Restore,
    Send,
    Worksheet,
    Download,
}

#[component]
pub fn IconView(#[prop(into)] icon: MaybeSignal<String>) -> impl IntoView {
    view! {
        <i
            class=move || {
                let icon = Icon::from_str(icon.get().as_str()).expect("icon name");
                format!("uiw-{icon} mx-0.5")
            }
        ></i>
    }
}
