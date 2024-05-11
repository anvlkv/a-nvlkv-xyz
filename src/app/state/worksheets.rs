use serde::{Deserialize, Serialize};

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemWK {
    #[iterable]
    pub problems: Vec<String>,
    #[iterable]
    pub stakeholders: Vec<String>,
    pub problem_statement: String,
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionsWK {
    #[iterable]
    pub solutions: Vec<String>,
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompromiseWK {
    #[iterable]
    pub choices: Vec<Option<bool>>,
    pub assumption: String,
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImplementWK {
    #[iterable]
    pub now: Vec<String>,
    #[iterable]
    pub best: Vec<String>,
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterateWK {
    pub test: String,
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InquireWK {
    pub name: String,
    pub email: String,
    pub message: String,
    pub include_worksheets: bool,
}
