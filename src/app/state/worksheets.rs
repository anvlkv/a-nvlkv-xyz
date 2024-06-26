use std::{collections::HashSet, str::FromStr};

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantArray};

use super::Contact;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct WorkSheets {
    pub problem: ProblemWK,
    pub solutions: SolutionsWK,
    pub compromise: CompromiseWK,
    pub implement: ImplementWK,
    pub iterate: IterateWK,
    pub inquire: InquireWK,
}

impl std::fmt::Display for WorkSheets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wk = t!("worksheets.wk").to_string();
        write!(f, "{wk}")
    }
}

impl Completenes for WorkSheets {
    fn is_complete(&self) -> bool {
        self.problem.is_complete()
            && self.solutions.is_complete()
            && self.compromise.is_complete()
            && self.implement.is_complete()
            && self.iterate.is_complete()
    }

    fn is_empty(&self) -> bool {
        self.problem.is_empty()
            && self.solutions.is_empty()
            && self.compromise.is_empty()
            && self.implement.is_empty()
            && self.iterate.is_empty()
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ProblemWK {
    pub problems: Vec<String>,
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

    fn is_empty(&self) -> bool {
        self.problem_statement.is_empty()
            && self
                .problems
                .iter()
                .filter(|r| !r.is_empty())
                .next()
                .is_none()
            && self
                .stakeholders
                .iter()
                .filter(|r| !r.is_empty())
                .next()
                .is_none()
    }
}

impl ProblemWK {
    pub fn unique_stakeholders(&self) -> Vec<String> {
        let all_unique = HashSet::<String>::from_iter(self.stakeholders.iter().filter_map(|s| {
            if !s.is_empty() {
                Some(s.clone())
            } else {
                None
            }
        }));
        let mut entries = Vec::from_iter(all_unique);
        entries.sort_by(|a, b| {
            let a_w = self.stakeholders.iter().filter(|s| s == &a).count();
            let b_w = self.stakeholders.iter().filter(|s| s == &b).count();
            b_w.cmp(&a_w)
        });

        entries
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct SolutionsWK {
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
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
                .next()
                .is_some()
            && self
                .stakeholder_choices
                .iter()
                .filter(|r| !r.is_empty())
                .next()
                .is_some()
    }

    fn is_empty(&self) -> bool {
        self.question.is_empty()
            && self
                .solution_choices
                .iter()
                .filter(|r| !r.is_empty())
                .next()
                .is_none()
            && self
                .stakeholder_choices
                .iter()
                .filter(|r| !r.is_empty())
                .next()
                .is_none()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ImplementWK {
    pub now: Vec<String>,
    pub best: Vec<String>,
}

impl Completenes for ImplementWK {
    fn is_complete(&self) -> bool {
        self.now.iter().filter(|r| !r.is_empty()).next().is_some()
            && self.best.iter().filter(|r| !r.is_empty()).next().is_some()
    }

    fn is_empty(&self) -> bool {
        self.now.iter().filter(|r| !r.is_empty()).next().is_none()
            && self.best.iter().filter(|r| !r.is_empty()).next().is_none()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct IterateWK {
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub resources: Vec<String>,
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct InquireWK {
    pub inquery_option: String,
    pub custom_prompt: String,
    pub personalized: bool,
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

    fn is_empty(&self) -> bool {
        !self.is_complete()
    }
}
