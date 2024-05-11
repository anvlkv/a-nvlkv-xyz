use leptos::*;
use strum::{Display, VariantArray};
use uuid::Uuid;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub wk: WorkSheetsFormState,
    pub examples: Examples,
    pub sequence: Vec<SeqStep>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Examples {
    pub examples_problem: Vec<ExampleState<ProblemWK>>,
    pub examples_solutions: Vec<ExampleState<SolutionsWK>>,
    pub examples_compromise: Vec<ExampleState<CompromiseWK>>,
    pub examples_implement: Vec<ExampleState<ImplementWK>>,
    pub examples_iterate: Vec<ExampleState<IterateWK>>,
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq)]
pub struct WorkSheets {
    #[nested]
    pub problem: ProblemWK,
    #[nested]
    pub solutions: SolutionsWK,
    #[nested]
    pub compromise: CompromiseWK,
    #[nested]
    pub implement: ImplementWK,
    #[nested]
    pub iterate: IterateWK,
    #[nested]
    pub inquire: InquireWK,
}

#[derive(Clone)]
struct Store(RwSignal<State>);

#[component]
pub fn StoreProvider(children: Children) -> impl IntoView {
    let state = create_rw_signal(State::default());
    provide_context(Store(state));

    create_effect(move |_| {
        let s = state.get();
        let wk: WorkSheets = (&s.wk).into();
        log::debug!("wk: {wk:#?}");
    });

    children().into_view()
}

pub fn use_store() -> RwSignal<State> {
    let ctx = use_context::<Store>().expect("State not provided");
    ctx.0
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SeqStep {
    pub href: String,
    pub process_step: ProcessStep,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Display, VariantArray)]
pub enum ProcessStep {
    #[default]
    About,
    Problem,
    Solution,
    Compromise,
    Implement,
    Iterate,
    Inquire,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ExampleState<T> {
    pub id: Uuid,
    pub visited: bool,
    pub value: T,
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq)]
pub struct ProblemWK {
    #[iterable]
    pub problems: Vec<String>,
    #[iterable]
    pub stakeholders: Vec<String>,
    pub problem_statement: String,
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq)]
pub struct SolutionsWK {
    #[iterable]
    pub solutions: Vec<String>,
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq)]
pub struct CompromiseWK {
    #[iterable]
    pub choices: Vec<Option<bool>>,
    pub assumption: String,
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq)]
pub struct ImplementWK {
    #[iterable]
    pub now: Vec<String>,
    #[iterable]
    pub best: Vec<String>,
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq)]
pub struct IterateWK {
    pub test: String,
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq)]
pub struct InquireWK {
    pub name: String,
    pub email: String,
    pub message: String,
    pub include_worksheets: bool,
}
