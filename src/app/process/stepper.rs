use leptos::*;
use leptos_router::*;

use crate::app::state::use_store;

#[component]
pub fn StepperView() -> impl IntoView {
    let state = use_store();

    let mut prev_button_disabled = false;
    let mut prev_button_text = "Previous";

    let mut next_button_disabled = false;
    let mut next_button_text = "Next";

    // match route.unwrap() {
    //     Route::Process { step: 1 } => {
    //         prev_button_disabled = true;
    //         next_button_text = "Continue";
    //     }
    //     Route::Process { step: 2 } => {
    //         prev_button_text = "Return to introduction";
    //         next_button_text = if !state.problem.complete {
    //             "See examples"
    //         } else {
    //             "Create solutions"
    //         };
    //     }
    //     Route::ProcessExample { step: 2, id } => {
    //         prev_button_text = "Return to problem worksheet";
    //         if let Some((example, pos)) = state
    //             .examples_problem
    //             .iter()
    //             .position(|e| e.id == id)
    //             .map(|pos| (state.examples_problem.iter().nth(pos).unwrap(), pos))
    //         {
    //             next_button_text = if !state.examples_problem.last().map {
    //                 "See examples"
    //             } else {
    //                 "Create solutions"
    //             };
    //         }
    //     }
    //     Route::Process { step: 3 } => {
    //         prev_button_text = "Return to problem space";
    //         next_button_text = if !state.solutions.complete {
    //             "See examples"
    //         } else {
    //             "Find the compromise"
    //         };
    //     }
    //     Route::Process { step: 4 } => {
    //         prev_button_text = "Return to solutions";
    //         next_button_text = if !state.solutions.complete {
    //             "See examples"
    //         } else {
    //             "Find the compromise"
    //         };
    //     }
    //     Route::Process { step: 4 } => {
    //         prev_button_text = "Return to solutions";
    //         next_button_text = if !state.solutions.complete {
    //             "See examples"
    //         } else {
    //             "Find the compromise"
    //         };
    //     }
    //     Route::ProcessExample { step, id } => {}
    //     _ => {}
    // }

    let navigate = use_navigate();
    let route = use_route();
    let on_next = move |_| {
        let route = route.path();
        let seq = state.get().sequence;

        let next = seq.iter().skip_while(|r| *r != &route).skip(1).next();
        if let Some(next) = next {
            navigate(next.as_str(), Default::default())
        } else if let Some(first) = seq.first() {
            navigate(first.as_str(), Default::default())
        } else {
            log::warn!("current step not found")
        }
    };

    let navigate = use_navigate();
    let route = use_route();
    let on_prev = move |_| {
        let route = route.path();
        let seq = state.get().sequence;

        let prev = seq.iter().take_while(|r| *r != &route).last();
        if let Some(prev) = prev {
            navigate(prev.as_str(), Default::default())
        }
    };

    let prev_button = view! {<button on:click={on_prev} disabled={prev_button_disabled}>{prev_button_text}</button>};
    let next_button = view! {<button on:click={on_next} disabled={next_button_disabled}>{next_button_text}</button>};

    view! {
        <aside class="flex justify-between items-center w-full pt-6 pb-3 border-t-2 border-solid border-slate-400">
            {prev_button}
            {next_button}
        </aside>
    }
}
