use leptos::*;
use uuid::Uuid;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub problem: FormState<ProblemForm>,
    pub examples_problem: Vec<ExampleState<ProblemForm>>,
    pub solutions: FormState<SolutionsForm>,
    pub examples_solutions: Vec<ExampleState<SolutionsForm>>,
    pub compromise: FormState<CompromiseForm>,
    pub examples_compromise: Vec<ExampleState<CompromiseForm>>,
    pub implement: FormState<ImplementForm>,
    pub examples_implement: Vec<ExampleState<ImplementForm>>,
    pub iterate: FormState<IterateForm>,
    pub examples_iterate: Vec<ExampleState<IterateForm>>,
    pub inquire: FormState<InquireForm>,
    pub sequence: Vec<String>,
}

#[derive(Clone)]
struct Store(RwSignal<State>);

#[component]
pub fn StoreProvider(children: Children) -> impl IntoView {
    let state = create_rw_signal(State::default());
    provide_context(Store(state));
    children().into_view()
}

pub fn use_store() -> RwSignal<State> {
    let ctx = use_context::<Store>().expect("State not provided");
    ctx.0
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ExampleState<T> {
    pub id: Uuid,
    pub visited: bool,
    pub value: T,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum FormStatus {
    #[default]
    Prestine,
    Touched,
    Dirty,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct FormState<T> {
    pub value: T,
    pub status: FormStatus,
    pub complete: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ProblemForm {
    pub problems: Vec<String>,
    pub stakeholders: Vec<String>,
    pub problem_statement: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SolutionsForm {
    pub solutions: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CompromiseForm {
    pub choices: Vec<Option<bool>>,
    pub assumption: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ImplementForm {
    pub now: Vec<String>,
    pub best: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct IterateForm {
    pub test: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct InquireForm {
    pub name: String,
    pub email: String,
    pub message: String,
}
