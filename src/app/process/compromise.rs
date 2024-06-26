use form_signal::FormSignal;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::app::{
    components::{
        use_example_ctx, use_wk_ctx, ButtonSize, ButtonView, CheckedOption, DescriptionView,
        ListSelectView, ReadOnlyListView, ReadOnlyView, StringInputView, WorksheetHeader,
    },
    process::FixedProblemStatement,
    state::{use_store, Completenes, ProblemWK, ProcessStep, SolutionsWK},
    tabs_signal, use_lang,
};

/// step 4
#[component]
pub fn CompromiseView() -> impl IntoView {
    let state = use_store();
    let wk_state = use_wk_ctx();
    let lang = use_lang();
    let link = Signal::derive(move || format!("/{}/process/4", lang.get()));
    let question_statement = FormSignal::new(
        wk_state.wk_data,
        |s| s.compromise.question,
        |s, q| s.compromise.question = q,
    );

    let solutions_list = Signal::derive(move || {
        let wk_data: SolutionsWK = state.get().wk.get().solutions;
        wk_data
            .solutions
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| CheckedOption {
                value: s.clone(),
                label: view! {
                    <ReadOnlyView>
                        {s.clone()}
                    </ReadOnlyView>
                },
            })
            .collect::<Vec<_>>()
    });

    let stakeholders_list = Signal::derive(move || {
        let wk_data: ProblemWK = state.get().wk.get().problem;
        wk_data
            .unique_stakeholders()
            .into_iter()
            .map(|s| CheckedOption {
                value: s.clone(),
                label: view! {
                    <ReadOnlyView>
                        {s.clone()}
                    </ReadOnlyView>
                },
            })
            .collect::<Vec<_>>()
    });

    let solution_choices = FormSignal::new(
        wk_state.wk_data,
        |s| s.compromise.solution_choices,
        |s, ch| s.compromise.solution_choices = ch,
    );

    let stakeholder_choices = FormSignal::new(
        wk_state.wk_data,
        |s| s.compromise.stakeholder_choices,
        |s, ch| s.compromise.stakeholder_choices = ch,
    );

    let tabs = tabs_signal(ProcessStep::Compromise);

    let disable_question = Signal::derive(move || {
        let data = wk_state.wk_data.get().compromise;
        data.solution_choices
            .iter()
            .filter(|e| !e.is_empty())
            .next()
            .is_none()
            || data
                .stakeholder_choices
                .iter()
                .filter(|e| !e.is_empty())
                .next()
                .is_none()
    });

    let disable_cta = Signal::derive(move || !wk_state.wk_data.get().compromise.is_complete());

    view! {
        <Title text={move || format!("{} | {} | {}", t!("worksheets.compromise.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title={t!("worksheets.compromise.title").to_string()}
            description_id="compromise"
            tabs
        />
        <div class="grow w-full">
            <DescriptionView
                hidden=wk_state.description_hidden
                toggle_hidden=wk_state.toggle_description_hidden
            >
                <p class="whitespace-pre-line">
                    {t!("worksheets.compromise.description")}
                </p>
            </DescriptionView>
            <form>
                <div class="max-w-prose mb-4 whitespace-pre-line">
                    <p>{t!("worksheets.compromise.instruction_1")}</p>
                </div>
                <FixedProblemStatement/>
                <div class="grid lg:grid-cols-2 gap-6 mt-8">
                    <div>
                        <h4 class="text-xl mb-4 w-full text-center">
                            {t!("worksheets.compromise.label_solutions")}
                        </h4>
                        <ListSelectView
                            max={Some(2)}
                            options={solutions_list}
                            value={solution_choices}
                        />
                    </div>
                    <div>
                        <h4 class="text-xl mb-4 w-full text-center">
                            {t!("worksheets.compromise.label_stakeholders")}
                        </h4>
                        <ListSelectView
                            max={Some(2)}
                            options={stakeholders_list}
                            value={stakeholder_choices}
                        />
                    </div>
                </div>
                <hr class="border-t border-slate-400 mt-4 mb-8"/>
                <div class="max-w-prose mb-4 whitespace-pre-line">
                    <p>{t!("worksheets.compromise.instruction_2")}</p>
                </div>
                <label>
                    <p class="mb-2">{t!("worksheets.compromise.label_question")}</p>
                    <StringInputView
                        input_type="textarea"
                        disabled={disable_question}
                        value=question_statement
                        placeholder={t!("worksheets.compromise.placeholder").to_string()}/>
                </label>
            </form>
            <div class="flex w-full mt-8 justify-center">
                <ButtonView
                    cta=2
                    size=ButtonSize::Lg
                    disabled={disable_cta}
                    link
                >
                    {t!("worksheets.compromise.cta")}
                </ButtonView>
            </div>
        </div>
    }
}

#[component]
pub fn FixedQuestionStatement() -> impl IntoView {
    let state = use_wk_ctx();
    let lang = use_lang();

    let question = Signal::derive(move || state.wk_data.get().compromise.question);

    let href = Signal::derive(move || Some(format!("/{}/process/3", lang.get())));
    let empty = Signal::derive(move || question.get().is_empty());

    view! {
        <ReadOnlyView
            fallback_title=t!("worksheets.compromise.empty").to_string()
            fallback_href=href
            label=t!("worksheets.compromise.label_question").to_string()
            empty={empty}
        >
            {question}
        </ReadOnlyView>
    }
}

#[component]
pub fn FixedSolutionsChoice() -> impl IntoView {
    let state = use_wk_ctx();

    let solutions = Signal::derive(move || state.wk_data.get().compromise.solution_choices);

    view! {
        <ReadOnlyListView
            value={solutions}
            label=t!("worksheets.compromise.label_solutions").to_string()
        />
    }
}

#[component]
pub fn FixedStakeholdersChoice() -> impl IntoView {
    let state = use_wk_ctx();

    let stakeholders = Signal::derive(move || state.wk_data.get().compromise.stakeholder_choices);

    view! {
        <ReadOnlyListView
            value={stakeholders}
            label=t!("worksheets.compromise.label_stakeholders").to_string()
        />
    }
}

#[component]
pub fn ExampleCompromiseView() -> impl IntoView {
    let lang = use_lang();
    let (wk, example) = use_example_ctx();
    let wk_ctx = use_wk_ctx();

    let tabs = tabs_signal(ProcessStep::Compromise);

    let question_statement = Signal::derive(move || wk.get().compromise.question);
    let problem_statement = Signal::derive(move || wk.get().problem.problem_statement);

    let solutions_data = Signal::derive(move || wk.get().solutions.solutions);
    let stakeholders_data = Signal::derive(move || wk.get().problem.stakeholders);

    let title = Signal::derive(move || {
        t!(
            "worksheets.compromise.example_title",
            title = example.get().title
        )
        .to_string()
    });
    let example_id = Signal::derive(move || example.get().id);

    let case_href = move || {
        let id = example_id.get();
        let lang = lang.get();
        format!("/{lang}/projects/{id}")
    };

    view! {
        <Title text={move || format!("{} | {} | {} | {}", example.get().title, t!("worksheets.compromise.title"), t!("process.title"), t!("name"))}/>
        <WorksheetHeader
            title
            description_id=example_id
            tabs
        />
        <div class="grow w-full">
            <DescriptionView
                hidden=wk_ctx.description_hidden
                toggle_hidden=wk_ctx.toggle_description_hidden
                alternative=1
            >
                <p class="whitespace-pre-line">
                    {move || example.get().description}
                </p>
                <A href=case_href attr:class="underline">
                    {move || t!("worksheets.view_example", title=example.get().title)}
                </A>
            </DescriptionView>
            <div role="form">
                <div class="max-w-prose mb-4 whitespace-pre-line italic">
                    <p>{t!("worksheets.compromise.instruction_1")}</p>
                </div>
                <ReadOnlyView
                        label=t!("worksheets.problem.label_statement").to_string()
                    >
                    {problem_statement}
                </ReadOnlyView>
                <div class="grid grid-cols-2 gap-6">
                    <div>
                        <h4 class="text-xl mb-4 w-full text-center">
                            {t!("worksheets.compromise.label_solutions")}
                        </h4>
                        <ReadOnlyListView
                            value=solutions_data
                        />
                    </div>
                    <div>
                        <h4 class="text-xl mb-4 w-full text-center">
                            {t!("worksheets.compromise.label_stakeholders")}
                        </h4>
                        <ReadOnlyListView
                            value=stakeholders_data
                        />
                    </div>
                </div>
                <hr class="border-t border-slate-400 mt-4 mb-8"/>
                <div class="max-w-prose mb-4 whitespace-pre-line italic">
                    <p>{t!("worksheets.compromise.instruction_2")}</p>
                </div>
                <ReadOnlyView
                    label=t!("worksheets.compromise.label_question").to_string()
                >
                    {question_statement}
                </ReadOnlyView>
            </div>
        </div>
    }
}
