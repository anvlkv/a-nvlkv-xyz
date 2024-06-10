use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantArray};

use super::{Contact, ContactFormState};

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

impl Completenes for WorkSheets {
    fn is_complete(&self) -> bool {
        self.problem.is_complete()
            && self.solutions.is_complete()
            && self.compromise.is_complete()
            && self.implement.is_complete()
            && self.iterate.is_complete()
    }
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
            iterate: IterateWK {
                resources: vec![Default::default()],
                external_resources: vec![Default::default()],
                ..Default::default()
            },
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

impl Completenes for ProblemWK {
    fn is_complete(&self) -> bool {
        !self.problem_statement.is_empty()
            && self
                .problems
                .iter()
                .filter(|r| !r.is_empty())
                .next()
                .is_some()
            && self
                .stakeholders
                .iter()
                .filter(|r| !r.is_empty())
                .next()
                .is_some()
    }
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionsWK {
    #[iterable]
    pub solutions: Vec<String>,
}

impl Completenes for SolutionsWK {
    fn is_complete(&self) -> bool {
        self.solutions
            .iter()
            .filter(|r| !r.is_empty())
            .next()
            .is_some()
    }
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompromiseWK {
    pub solution_choices: Vec<String>,
    pub stakeholder_choices: Vec<String>,
    pub question: String,
}

impl Completenes for CompromiseWK {
    fn is_complete(&self) -> bool {
        !self.question.is_empty()
            && self
                .solution_choices
                .iter()
                .filter(|r| !r.is_empty())
                .count()
                > 0
            && self
                .stakeholder_choices
                .iter()
                .filter(|r| !r.is_empty())
                .count()
                > 0
    }
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImplementWK {
    #[iterable]
    pub now: Vec<String>,
    #[iterable]
    pub best: Vec<String>,
}

impl Completenes for ImplementWK {
    fn is_complete(&self) -> bool {
        self.now.iter().filter(|r| !r.is_empty()).next().is_some()
            && self.best.iter().filter(|r| !r.is_empty()).next().is_some()
    }
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterateWK {
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    #[iterable]
    pub resources: Vec<String>,
    #[iterable]
    pub external_resources: Vec<String>,
}

impl Completenes for IterateWK {
    fn is_complete(&self) -> bool {
        !self.title.is_empty()
            && !self.start_date.is_empty()
            && !self.end_date.is_empty()
            && self
                .resources
                .iter()
                .filter(|r| !r.is_empty())
                .next()
                .is_some()
    }
}

#[derive(FormState, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InquireWK {
    pub inquery_option: String,
    pub custom_prompt: String,
    pub personalized: bool,
    #[nested]
    pub contact: Contact,
}

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    EnumString,
    VariantArray,
    Display,
)]
pub enum InqueryOption {
    #[default]
    FirstTime,
    ScopeAndTime,
    EthicalDesign,
    Narrative,
    Custom,
}

impl Completenes for InquireWK {
    fn is_complete(&self) -> bool {
        let inquire = match InqueryOption::from_str(self.inquery_option.as_str()) {
            Ok(InqueryOption::Custom) => !self.custom_prompt.is_empty(),
            Ok(_) => true,
            Err(_) => false,
        };

        inquire && (!self.personalized || self.contact.is_complete())
    }
}

pub trait Completenes {
    fn is_complete(&self) -> bool;
}
