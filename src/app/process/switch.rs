use leptos::*;
use leptos_router::*;
use web_sys::{Element, ScrollBehavior, ScrollToOptions};

use crate::app::{
    components::{use_wk_ctx, ExampleView},
    process::*,
};

#[component]
pub fn ProcessSwitchView() -> impl IntoView {
    let params = use_params_map();
    let wk_ctx = use_wk_ctx();

    log::debug!("render ProcessSwitchView");

    let view = move || {
        log::debug!("render ProcessSwitchView inner");
        let p = params.get();
        let step: usize = p
            .get("step")
            .map(|s| s.parse().ok())
            .flatten()
            .unwrap_or_default();
        let example = p.get("example");

        create_effect(move |_| {
            let mut options = ScrollToOptions::default();
            options.behavior(ScrollBehavior::Smooth).top(0.0);

            if let Some(el) = document().fullscreen_element() {
                el.scroll_with_scroll_to_options(&options);
            } else {
                window().scroll_with_scroll_to_options(&options);
            }

            log::debug!("scroll to top");
        });

        match example {
            Some(_) => {
                view! {
                    <ExampleView>
                        {
                            match step {
                                1 => ExampleProblemView.into_view(),
                                2 => ExampleSolutionView.into_view(),
                                3 => ExampleCompromiseView.into_view(),
                                4 => ExampleImplementView.into_view(),
                                _ => AboutView.into_view()
                            }
                        }
                    </ExampleView>
                }
            }
            None => match step {
                1 => ProblemView.into_view(),
                2 => SolutionView.into_view(),
                3 => CompromiseView.into_view(),
                4 => ImplementView.into_view(),
                5 => IterateView.into_view(),
                6 => InquireView.into_view(),
                _ => AboutView.into_view(),
            },
        }
    };

    view.into_view()
}
