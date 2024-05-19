use serde::{Deserialize, Serialize};

#[derive(FormState, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl Default for WorkSheets {
    fn default() -> Self {
        Self {
            problem: ProblemWK {
                problems: vec![Default::default()],
                stakeholders: vec![Default::default()],
                ..Default::default()
            },
            solutions: SolutionsWK {
                solutions: vec![Default::default()],
                ..Default::default()
            },
            compromise: CompromiseWK {
                ..Default::default()
            },
            implement: ImplementWK {
                now: vec![Default::default()],
                best: vec![Default::default()],
            },
            iterate: Default::default(),
            inquire: Default::default(),
        }
    }
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
    pub solution_choices: Vec<Option<bool>>,
    pub stakeholder_choices: Vec<Option<bool>>,
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
