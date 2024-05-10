use leptos::*;
use strum::{Display, VariantArray};
use uuid::Uuid;

use super::form::*;

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
    pub sequence: Vec<SeqStep>,
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

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ProblemForm {
    pub problems: Vec<FormState<String>>,
    pub stakeholders: Vec<FormState<String>>,
    pub problem_statement: FormState<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SolutionsForm {
    pub solutions: Vec<FormState<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CompromiseForm {
    pub choices: Vec<FormState<Option<bool>>>,
    pub assumption: FormState<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ImplementForm {
    pub now: Vec<FormState<String>>,
    pub best: Vec<FormState<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct IterateForm {
    pub test: FormState<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct InquireForm {
    pub name: FormState<String>,
    pub email: FormState<String>,
    pub message: FormState<String>,
    pub include_worksheets: FormState<bool>,
}
