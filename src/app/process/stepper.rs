use leptos::*;
use leptos_router::*;
use strum::VariantArray;

use crate::app::{
    components::{ButtonView, IconView, RvArtboardView},
    state::{use_store, ProcessStep, SeqStep},
    use_lang,
};

#[component]
pub fn StepperView() -> impl IntoView {
    let state = use_store();
    let location = use_location();

    let step_idx = Signal::derive(move || {
        let current = location.pathname.get();
        state
            .get()
            .sequence
            .iter()
            .position(|s| s.href == current)
            .unwrap_or_default()
    });

    let step_data = Signal::derive(move || {
        let current = step_idx.get();
        state
            .get()
            .sequence
            .into_iter()
            .nth(current)
            .unwrap_or_default()
    });

    let prev_button_disabled = Signal::derive(move || step_idx.get() == 0);
    let prev_button_text = Signal::derive(move || {
        if prev_button_disabled.get() {
            None
        } else {
            state
                .get()
                .sequence
                .iter()
                .nth(step_idx.get().saturating_sub(1))
                .map(|s| {
                    if s.example.is_some() {
                        t!("worksheets.prev_ex")
                    } else {
                        t!("worksheets.prev_wk")
                    }
                    .to_string()
                })
        }
    });

    let next_button_disabled =
        Signal::derive(move || step_idx.get() == state.get().sequence.len().saturating_sub(1));

    let next_button_text = Signal::derive(move || {
        if next_button_disabled.get() {
            Some("✓".to_string())
        } else {
            state
                .get()
                .sequence
                .iter()
                .nth(step_idx.get().saturating_add(1))
                .map(|s| {
                    if s.example.is_some() {
                        t!("worksheets.next_ex")
                    } else {
                        t!("worksheets.next_wk")
                    }
                    .to_string()
                })
        }
    });

    let navigate = use_navigate();
    let on_next = move |_| {
        let seq = state.get().sequence;

        let next = seq.iter().nth(step_idx.get() + 1);
        if let Some(next) = next {
            navigate(next.href.as_str(), Default::default());
        }
    };

    let navigate = use_navigate();
    let on_prev = move |_| {
        let seq = state.get().sequence;

        let prev = seq.iter().nth(step_idx.get().saturating_sub(1));
        if let Some(prev) = prev {
            navigate(prev.href.as_str(), Default::default())
        }
    };

    view! {
        <aside class="flex flex-wrap lg:flex-nowrap flex-col md:justify-stretch items-stretch lg:items-center xl:items-stretch md:flex-row xl:flex-col pt-4 mx-4 md:mb-8 xl:pt-0 xl:pr-4 xl:ml-0 xl:mr-4 xl:my-12 border-t-2 xl:border-t-0 xl:border-r-2 border-solid border-slate-400">
            <ButtonView
                attr:class="md:basis-5/12 md:max-lg:ml-0 md:max-lg:mr-auto lg:basis-auto mb-2 xl:mb-4"
                on:click={on_prev}
                disabled={prev_button_disabled}
                attr:title=prev_button_text
            >
                <Show
                    when=move || !prev_button_disabled.get()
                    fallback={move || view!{
                        <IconView icon="Restart"/>
                    }}
                >
                    <IconView icon="Prev"/>
                </Show>
            </ButtonView>
            <ButtonView
                cta=1
                attr:class="md:basis-5/12 md:max-lg:ml-auto md:max-lg:mr-0 lg:basis-auto lg:order-last mb-2 xl:mb-4"
                on:click={on_next}
                disabled={next_button_disabled}
                attr:title=next_button_text
            >
                <Show
                    when=move || !next_button_disabled.get()
                    fallback={move || view!{
                        <IconView icon="Done"/>
                    }}
                >
                    <IconView icon="Next"/>
                </Show>
            </ButtonView>
            <ol class="md:basis-full md:max-xl:mx-auto lg:basis-auto lg:shrink lg:order-2 xl:order-first flex flex-col flex-wrap sm:flex-row xl:flex-col justify-center xl:gap-4 xl:mb-4">
                <For
                    each=move || ProcessStep::VARIANTS.into_iter()
                    key=|state| *state
                    let:child
                >
                    <li class="contents">
                        <StepView step=*child current_step_data=step_data/>
                    </li>
                </For>
            </ol>
        </aside>
    }
}

#[component]
fn StepView(step: ProcessStep, current_step_data: Signal<SeqStep>) -> impl IntoView {
    let lang = use_lang();
    let (input, set_input) = create_signal(None);

    let is_inactive = Signal::derive(move || current_step_data.get().process_step != step);

    let activate_cb = Callback::new(move |hover| {
        let is_inactive = is_inactive.get();
        set_input.set(Some(("Visible".to_string(), hover)));
        set_input.set(Some((
            "Inactive".to_string(),
            if hover { false } else { is_inactive },
        )));
    });

    let label = Signal::derive(move || {
        let l_id = format!("stepper.{}", step.to_string().to_lowercase());
        t!(l_id.as_str()).to_string()
    });

    let href = Signal::derive(move || {
        let lang = lang.get();
        let data = current_step_data.get();
        if let Some(example) = data.example.as_ref() {
            format!("/{lang}/process/{}/{example}", step as isize)
        } else {
            format!("/{lang}/process/{}", step as isize)
        }
    });

    create_render_effect(move |_| {
        set_input.set(Some(("Inactive".to_string(), is_inactive.get())));
    });

    view! {
        <A
            href=move || href.get()
            exact=true
            on:pointerenter=move |_| activate_cb.call(true)
            on:pointerleave=move |_| activate_cb.call(false)
            active_class="pointer-events-none"
            class="block flex flex-col xl:flex-row items-center px-6 xl:pl-0 hover:underline hover:text-purple-800 active:text-purple-950 text-center xl:text-left"
        >
            <RvArtboardView
                attr:class="mt-1 w-16 h-16 sm:w-8 sm:h-8 xl:w-12 xl:h-12"
                state_machine={format!("{step} State Machine")}
                name={format!("{step}")}
                input_bool=input
            />
            <span class="my-2 xl:ml-4 text-sm block w-full">{label}</span>
        </A>
    }
}
