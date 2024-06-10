use html::AnyElement;
use leptos::*;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    #[default]
    Base,
    Lg,
    Xl,
}

#[component]
pub fn ButtonView(
    #[prop(into, optional)] cta: MaybeSignal<u8>,
    #[prop(into, optional)] size: MaybeSignal<ButtonSize>,
    #[prop(into, optional)] disabled: MaybeSignal<bool>,
    #[prop(into, optional)] link: Option<MaybeSignal<String>>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(into, optional)] node_ref: Option<NodeRef<AnyElement>>,
    children: ChildrenFn,
) -> impl IntoView {
    let element = if let Some(href) = link {
        html::a()
            .attr("href", href.clone())
            .attr("type", "button")
            .child(children)
            .into_any()
    } else {
        html::button().child(children).into_any()
    };

    let element = element.attrs(attrs).attr("disabled", disabled)
        .classes("border-2 border-solid drop-shadow-sm rounded-full text-center")
        .dyn_classes(move || {
            match size.get() {
                ButtonSize::Sm => "text-sm px-1 py-1 md:px-2 font-bold",
                ButtonSize::Base => "text-base px-2 py-1 md:px-3 md:py-2 font-semibold",
                ButtonSize::Lg => "text-xl px-6 py-2 md:px-8 md:py-4 font-normal",
                ButtonSize::Xl => "text-4xl sm:text-5xl md:text-6xl lg:text-8xl 2xl:text-9xl px-10 md:px-16 py-2 lg:px-20 lg:py-3 2xl:px-24 2xl:py-6 font-light",
            }.split_whitespace()
        })
        .dyn_classes(move || {
            match cta.get() {
                0 => "bg-stone-300 dark:bg-stone-950 hover:bg-stone-200 dark:hover:bg-stone-800 active:bg-stone-300 dark:active:bg-stone:700 border-slate-50",
                1 => "bg-stone-300 dark:bg-stone-950 hover:bg-stone-200 dark:hover:bg-stone-800 active:bg-stone-300 dark:active:bg-stone:700 border-purple-800 text-purple-700 dark:text-purple:500",
                _ => "bg-purple-900 hover:bg-purple-800 text-stone-100 active:bg-purple-950 border-slate-50"
            }.split_whitespace()
        })
        .dyn_classes(move ||
            {
                if disabled.get() {
                    "pointer-events-none contrast-50 saturate-50"
                }
                else {
                    ""
                }
            }.split_whitespace()
        );

    if let Some(node_ref) = node_ref {
        element.node_ref(node_ref)
    } else {
        element
    }
}
