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
    Wait,
}

#[component]
pub fn IconView(
    #[prop(into)] icon: MaybeSignal<String>,
    #[prop(attrs, into)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    html::i().attrs(attrs).attr("class", move || {
        let icon = Icon::from_str(icon.get().as_str()).expect("icon name");
        format!("uiw-{icon} mx-0.5")
    })
}
