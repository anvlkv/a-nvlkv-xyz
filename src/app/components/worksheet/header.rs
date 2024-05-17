use leptos::*;

use super::{Tab, WorksheetState};

pub struct WorksheetHeaderDescriptionProps {
    pub toggle_hidden: Callback<()>,
    pub hidden: Signal<bool>,
}

#[component]
pub fn WorksheetHeader<D, N>(
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into)] description_id: MaybeSignal<String>,
    #[prop(into, optional)] tabs: MaybeSignal<Vec<Tab>>,
    children: D,
) -> impl IntoView
where
    D: Fn(WorksheetHeaderDescriptionProps) -> N + 'static,
    N: IntoView,
{
    let ctx = use_context::<WorksheetState>().unwrap();

    create_render_effect({
        let title = title.clone();
        move |_| {
            let title = title.get();
            ctx.set_title.set(title);
        }
    });

    create_render_effect({
        let id = description_id.clone();
        move |_| {
            let id = id.get();
            ctx.set_current_description.set(id);
        }
    });

    create_render_effect({
        let tabs = tabs.clone();
        move |_| {
            let tabs = tabs.get();
            ctx.set_tabs.set(tabs);
        }
    });

    on_cleanup(move || {
        let title = title.get();
        let id = description_id.get();
        let tabs = tabs.get();

        ctx.set_title.update(move |t| {
            if t == &title {
                *t = Default::default();
            }
        });
        ctx.set_current_description.update(move |t| {
            if t == &id {
                *t = Default::default();
            }
        });
        ctx.set_tabs.update(move |t| {
            if t == &tabs {
                *t = Vec::new();
            }
        });
    });

    children(WorksheetHeaderDescriptionProps {
        toggle_hidden: ctx.toggle_description_hidden.clone(),
        hidden: ctx.description_hidden,
    })
    .into_view()
}
